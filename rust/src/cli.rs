//! The command line, mirroring the Python argparse surface.
//!
//! The binary carries eight commands, seven over one file
//! argument each and the changelog over two. The dispatch
//! mirrors the Python argparse run token
//! for token, including the layouts of help and of errors, the
//! width those layouts wrap at, and the exit statuses, as the
//! probes of this repo confirmed against the Python 3.12
//! argparse on the programme machine. Arguments are handled
//! as operating system bytes end to end, decoded with the
//! same error recovery the Python runtime applies, so a file
//! name holding a raw byte opens and an unprintable argument
//! echoes byte for byte as the Python echoes it.

use std::ffi::OsString;

use crate::atlas::{
    load_atlas_text, load_lineage_text, render_atlas, render_changelog, render_index,
    render_lineage, Atlas, Lineage,
};
use crate::explorations::{
    count_by_state, load_exploration_text, render_census, render_claims, render_map, Exploration,
};
use crate::pytext::PText;

/// The outcome of one command line run.
pub struct Outcome {
    /// Lines for the standard output.
    pub stdout: Vec<String>,
    /// Lines for the standard error.
    pub stderr: Vec<String>,
    /// The exit status.
    pub code: i32,
}

/// Decode operating system bytes into text the way the Python
/// runtime decodes an argument.
///
/// Bytes that do not form valid UTF-8 are recovered one byte
/// at a time into the surrogate escape range, exactly as the
/// Python decoder recovers them. A truncated or malformed
/// sequence escapes its lead byte alone, because the bytes
/// that follow it can never open a valid sequence themselves,
/// so the recovered text is the same either way.
pub fn decode_os_bytes(bytes: &[u8]) -> PText {
    let mut units: Vec<u32> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        if byte < 0x80 {
            units.push(u32::from(byte));
            i += 1;
            continue;
        }
        let shape: Option<(usize, u8, u8)> = match byte {
            0xC2..=0xDF => Some((2, 0x80, 0xBF)),
            0xE0 => Some((3, 0xA0, 0xBF)),
            0xE1..=0xEC | 0xEE..=0xEF => Some((3, 0x80, 0xBF)),
            0xED => Some((3, 0x80, 0x9F)),
            0xF0 => Some((4, 0x90, 0xBF)),
            0xF1..=0xF3 => Some((4, 0x80, 0xBF)),
            0xF4 => Some((4, 0x80, 0x8F)),
            _ => None,
        };
        let valid = match shape {
            Some((width, low, high)) => {
                i + width <= bytes.len()
                    && (low..=high).contains(&bytes[i + 1])
                    && (2..width).all(|k| (0x80..=0xBF).contains(&bytes[i + k]))
            }
            None => false,
        };
        if valid {
            let width = shape.expect("a valid shape stands").0;
            let chunk = std::str::from_utf8(&bytes[i..i + width]).expect("a checked chunk");
            units.push(chunk.chars().next().expect("a checked char") as u32);
            i += width;
        } else {
            units.push(0xDC00 + u32::from(byte));
            i += 1;
        }
    }
    PText(units)
}

/// Decode one operating system argument into text.
pub fn decode_os_arg(arg: &OsString) -> PText {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        decode_os_bytes(arg.as_os_str().as_bytes())
    }
    #[cfg(not(unix))]
    {
        PText::from_text(&arg.to_string_lossy())
    }
}

/// Encode text back into operating system bytes, the inverse
/// of the argument decoding for values that came from it.
pub fn encode_os_bytes(text: &PText) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    for &unit in &text.0 {
        if (0xDC80..=0xDCFF).contains(&unit) {
            out.push((unit - 0xDC00) as u8);
        } else if let Some(ch) = char::from_u32(unit) {
            let mut buf = [0u8; 4];
            out.extend_from_slice(ch.encode_utf8(&mut buf).as_bytes());
        } else {
            out.extend_from_slice("\u{FFFD}".as_bytes());
        }
    }
    out
}

/// Render text the way the Python error stream renders it.
///
/// The Python writes its error text with unrecoverable
/// characters replaced by their backslash escape text, so a
/// surrogate escape unit stands as the six characters of its
/// escape, and every other character stands as itself.
pub fn stderr_text(text: &PText) -> String {
    let mut out = String::new();
    for &unit in &text.0 {
        if (0xD800..=0xDFFF).contains(&unit) {
            out.push_str(&format!("\\u{unit:04x}"));
        } else if let Some(ch) = char::from_u32(unit) {
            out.push(ch);
        }
    }
    out
}

/// The one range search comparator the table readings
/// share, reading a unit against a range as below the
/// range, inside it, or above it.
fn range_order(&(lo, hi): &(u32, u32), unit: u32) -> std::cmp::Ordering {
    if unit < lo {
        std::cmp::Ordering::Greater
    } else if unit > hi {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Equal
    }
}

fn in_ranges(ranges: &[(u32, u32)], unit: u32) -> bool {
    ranges
        .binary_search_by(|range| range_order(range, unit))
        .is_ok()
}

fn py_printable(unit: u32) -> bool {
    !in_ranges(crate::pyprint_table::NON_PRINTABLE, unit)
}

/// The Python repr of a text value, as argparse prints an
/// offending argument inside its error lines.
pub fn py_repr(text: &PText) -> String {
    let has_single = text.0.contains(&0x27);
    let has_double = text.0.contains(&0x22);
    let quote = if has_single && !has_double { '"' } else { '\'' };
    let mut out = String::new();
    out.push(quote);
    for &unit in &text.0 {
        if unit == quote as u32 || unit == 0x5C {
            out.push('\\');
            out.push(char::from_u32(unit).expect("a quote or a backslash"));
        } else if unit == 0x0A {
            out.push_str("\\n");
        } else if unit == 0x0D {
            out.push_str("\\r");
        } else if unit == 0x09 {
            out.push_str("\\t");
        } else if py_printable(unit) {
            out.push(char::from_u32(unit).expect("a printable unit is a char"));
        } else if unit < 0x100 {
            out.push_str(&format!("\\x{unit:02x}"));
        } else if unit <= 0xFFFF {
            out.push_str(&format!("\\u{unit:04x}"));
        } else {
            out.push_str(&format!("\\U{unit:08x}"));
        }
    }
    out.push(quote);
    out
}

/// Whether a code point is a decimal digit in the Python
/// reading, which accepts every Unicode decimal digit.
fn is_nd(unit: u32) -> bool {
    in_ranges(crate::pyprint_table::ND_RANGES, unit)
}

/// The decimal value of a Unicode decimal digit code point.
fn nd_value(unit: u32) -> u32 {
    // Every decimal digit block in Unicode is a run of ten
    // code points whose values ascend from zero, and merged
    // ranges begin at a zero, so the offset inside the range
    // taken ten at a time is the value.
    let ranges = crate::pyprint_table::ND_RANGES;
    let index = ranges
        .binary_search_by(|range| range_order(range, unit))
        .expect("a digit stands in the table");
    let (lo, _) = ranges[index];
    (unit - lo) % 10
}

/// Whether a code point stands in the strip set of the
/// Python int reading.
///
/// The set is narrower than the string whitespace set, as
/// PORT-14 states, holding no separator controls from
/// U+001C to U+001F, which the string set holds and the
/// int reading does not strip.
fn py_isspace(unit: u32) -> bool {
    matches!(
        unit,
        0x09..=0x0D
            | 0x20
            | 0x85
            | 0xA0
            | 0x1680
            | 0x2000..=0x200A
            | 0x2028
            | 0x2029
            | 0x202F
            | 0x205F
            | 0x3000
    )
}

/// Read a text value as the Python int reading reads it, for
/// the COLUMNS environment value.
///
/// The reading strips the int reading strip set, takes an
/// optional sign, and takes digits in any Unicode decimal
/// digit block, with single underscores standing between
/// digits. Anything else fails the reading.
pub fn parse_python_int(text: &PText) -> Option<i64> {
    let units = &text.0;
    let mut start = 0;
    let mut end = units.len();
    while start < end && py_isspace(units[start]) {
        start += 1;
    }
    while end > start && py_isspace(units[end - 1]) {
        end -= 1;
    }
    let body = &units[start..end];
    if body.is_empty() {
        return None;
    }
    let (negative, digits) = match body[0] {
        0x2B => (false, &body[1..]),
        0x2D => (true, &body[1..]),
        _ => (false, body),
    };
    if digits.is_empty() {
        return None;
    }
    let mut value: i64 = 0;
    let mut prev_digit = false;
    for (index, &unit) in digits.iter().enumerate() {
        if unit == 0x5F {
            // An underscore stands only between two digits.
            if !prev_digit {
                return None;
            }
            let next = digits.get(index + 1)?;
            if !is_nd(*next) {
                return None;
            }
            prev_digit = false;
            continue;
        }
        if !is_nd(unit) {
            return None;
        }
        value = value
            .saturating_mul(10)
            .saturating_add(i64::from(nd_value(unit)));
        prev_digit = true;
    }
    if !prev_digit {
        return None;
    }
    Some(if negative { -value } else { value })
}

#[cfg(unix)]
fn terminal_columns() -> Option<usize> {
    unsafe {
        let mut size: libc::winsize = std::mem::zeroed();
        if libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut size) == 0 && size.ws_col > 0 {
            Some(size.ws_col as usize)
        } else {
            None
        }
    }
}

#[cfg(not(unix))]
fn terminal_columns() -> Option<usize> {
    None
}

/// The width the help layout wraps at, read as the Python
/// reads it for its own terminal size.
///
/// The argument is the COLUMNS value under the Python integer
/// reading, as a count of columns when the reading produced a
/// positive figure, zero when it produced a figure that is
/// not positive, and nothing when the value is unset or does
/// not read as an integer.
pub fn formatter_width(columns_env: Option<usize>) -> usize {
    const FALLBACK: usize = 80;
    let columns = match columns_env {
        Some(cols) => {
            if cols > 0 {
                Some(cols)
            } else {
                terminal_columns()
            }
        }
        None => terminal_columns(),
    };
    let width = columns.unwrap_or(FALLBACK);
    // The Python takes two columns off the width it read, and
    // the formatter width stands at two at the smallest.
    width.saturating_sub(2).max(2)
}

/// Read the COLUMNS environment value the way the Python
/// terminal size reading reads it.
pub fn read_columns_env() -> Option<usize> {
    let raw = std::env::var_os("COLUMNS")?;
    let text = decode_os_arg(&raw);
    match parse_python_int(&text) {
        Some(value) if value > 0 => Some(value as usize),
        Some(_) => Some(0),
        None => None,
    }
}

// ----------------------------------------------------------------
// The help and usage formatter, mirroring the Python
// HelpFormatter over the two parser shapes this program has.

struct ActionInfo {
    /// The invocation as it stands in the help listing.
    invocation: String,
    /// The indent the action stands at in the listing, two
    /// for a parser action and four for a subcommand entry.
    indent: usize,
    /// The help text, when the action carries one.
    help: Option<String>,
    /// The part this action contributes to the usage line.
    usage_part: String,
}

struct ParserInfo {
    prog: String,
    description: Option<String>,
    positionals: Vec<ActionInfo>,
    optionals: Vec<ActionInfo>,
    subactions: Vec<(String, String)>,
}

fn len_of(text: &str) -> usize {
    text.chars().count()
}

impl ParserInfo {
    fn action_max_length(&self) -> usize {
        // The Python formatter measures each invocation with
        // the indent the action stands at when it is measured.
        let mut max_len = 0;
        for action in self.positionals.iter().chain(self.optionals.iter()) {
            max_len = max_len.max(len_of(&action.invocation) + action.indent);
        }
        for (name, _) in &self.subactions {
            max_len = max_len.max(len_of(name) + 4);
        }
        max_len
    }

    fn help_position(&self, width: usize) -> usize {
        let max_help_position = (width as i64 - 20).clamp(4, 24) as usize;
        (self.action_max_length() + 2).min(max_help_position)
    }

    fn opt_usage(&self) -> String {
        self.optionals
            .iter()
            .map(|a| a.usage_part.clone())
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn pos_usage(&self) -> String {
        self.positionals
            .iter()
            .map(|a| a.usage_part.clone())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// The usage text, without its trailing newline.
    fn format_usage(&self, width: usize) -> String {
        let prefix = "usage: ";
        let opt_usage = self.opt_usage();
        let pos_usage = self.pos_usage();
        let action_usage = [opt_usage.clone(), pos_usage.clone()]
            .into_iter()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        let usage = if action_usage.is_empty() {
            self.prog.clone()
        } else {
            format!("{} {}", self.prog, action_usage)
        };
        let text_width = width;
        if len_of(prefix) + len_of(&usage) <= text_width {
            return format!("{prefix}{usage}");
        }
        // Break the usage into wrappable parts the way the
        // Python part pattern breaks it, bracketed groups and
        // plain runs of non space characters.
        let opt_parts = split_usage_parts(&opt_usage);
        let pos_parts = split_usage_parts(&pos_usage);
        let lines: Vec<String>;
        if (len_of(prefix) + len_of(&self.prog)) as f64 <= 0.75 * text_width as f64 {
            let indent = " ".repeat(len_of(prefix) + len_of(&self.prog) + 1);
            if !opt_parts.is_empty() {
                let mut head = vec![self.prog.clone()];
                head.extend(opt_parts.iter().cloned());
                lines = {
                    let mut all = get_lines(&head, &indent, Some(prefix), text_width);
                    all.extend(get_lines(&pos_parts, &indent, None, text_width));
                    all
                };
            } else if !pos_parts.is_empty() {
                let mut head = vec![self.prog.clone()];
                head.extend(pos_parts.iter().cloned());
                lines = get_lines(&head, &indent, Some(prefix), text_width);
            } else {
                lines = vec![self.prog.clone()];
            }
        } else {
            let indent = " ".repeat(len_of(prefix));
            let mut parts = opt_parts.clone();
            parts.extend(pos_parts.iter().cloned());
            let mut grouped = get_lines(&parts, &indent, None, text_width);
            if grouped.len() > 1 {
                grouped = get_lines(&opt_parts, &indent, None, text_width);
                grouped.extend(get_lines(&pos_parts, &indent, None, text_width));
            }
            lines = {
                let mut all = vec![self.prog.clone()];
                all.extend(grouped);
                all
            };
        }
        format!("{prefix}{}", lines.join("\n"))
    }

    /// The full help text, ending with a newline.
    fn format_help(&self, width: usize) -> String {
        let help_position = self.help_position(width);
        let mut out = String::new();
        out.push_str(&self.format_usage(width));
        out.push_str("\n\n");
        if let Some(description) = &self.description {
            let text_width = width.max(11);
            for line in wrap_text(description, text_width) {
                out.push_str(&line);
                out.push('\n');
            }
            out.push('\n');
        }
        let mut first_section = true;
        if !self.positionals.is_empty() || !self.subactions.is_empty() {
            if !first_section {
                out.push('\n');
            }
            first_section = false;
            out.push_str("positional arguments:\n");
            for action in &self.positionals {
                out.push_str(&format_action(action, help_position, width));
            }
            for (name, help) in &self.subactions {
                let subaction = ActionInfo {
                    invocation: name.clone(),
                    indent: 4,
                    help: Some(help.clone()),
                    usage_part: String::new(),
                };
                out.push_str(&format_action(&subaction, help_position, width));
            }
        }
        if !self.optionals.is_empty() {
            if !first_section {
                out.push('\n');
            }
            out.push_str("options:\n");
            for action in &self.optionals {
                out.push_str(&format_action(action, help_position, width));
            }
        }
        out
    }
}

/// Split a usage fragment into the parts the Python part
/// pattern finds in it.
fn split_usage_parts(text: &str) -> Vec<String> {
    // The pattern takes a parenthesised run, a bracketed run,
    // or a plain run of non space characters, in that order.
    let chars: Vec<char> = text.chars().collect();
    let mut parts: Vec<String> = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == ' ' {
            i += 1;
            continue;
        }
        if chars[i] == '(' || chars[i] == '[' {
            let close = if chars[i] == '(' { ')' } else { ']' };
            // The pattern is lazy up to the first close that
            // a space or the end of the text follows.
            let mut end: Option<usize> = None;
            let mut j = i + 1;
            while j < chars.len() {
                if chars[j] == close && (j + 1 == chars.len() || chars[j + 1] == ' ') {
                    end = Some(j);
                    break;
                }
                j += 1;
            }
            match end {
                Some(j) => {
                    parts.push(chars[i..=j].iter().collect());
                    i = j + 1;
                }
                None => {
                    // Fall back to the plain run.
                    let mut j = i;
                    while j < chars.len() && chars[j] != ' ' {
                        j += 1;
                    }
                    parts.push(chars[i..j].iter().collect());
                    i = j;
                }
            }
        } else {
            let mut j = i;
            while j < chars.len() && chars[j] != ' ' {
                j += 1;
            }
            parts.push(chars[i..j].iter().collect());
            i = j;
        }
    }
    parts
}

fn get_lines(
    parts: &[String],
    indent: &str,
    prefix: Option<&str>,
    text_width: usize,
) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut line: Vec<&str> = Vec::new();
    let indent_length = len_of(indent) as i64;
    let mut line_len: i64 = match prefix {
        Some(text) => len_of(text) as i64 - 1,
        None => indent_length - 1,
    };
    for part in parts {
        if line_len + 1 + len_of(part) as i64 > text_width as i64 && !line.is_empty() {
            lines.push(format!("{indent}{}", line.join(" ")));
            line = Vec::new();
            line_len = indent_length - 1;
        }
        line.push(part);
        line_len += len_of(part) as i64 + 1;
    }
    if !line.is_empty() {
        lines.push(format!("{indent}{}", line.join(" ")));
    }
    if prefix.is_some() && !lines.is_empty() {
        let first = lines[0].clone();
        lines[0] = first[indent.len()..].to_string();
    }
    lines
}

/// Wrap a help text the way the Python text wrapper wraps
/// it, after the whitespace collapse the formatter applies.
fn wrap_text(text: &str, width: usize) -> Vec<String> {
    // The formatter collapses every run of ASCII whitespace
    // to a single space and strips the ends before wrapping.
    let mut collapsed = String::new();
    let mut in_ws = false;
    for ch in text.chars() {
        if matches!(ch, ' ' | '\t' | '\n' | '\r' | '\u{0B}' | '\u{0C}') {
            in_ws = true;
        } else {
            if in_ws && !collapsed.is_empty() {
                collapsed.push(' ');
            }
            in_ws = false;
            collapsed.push(ch);
        }
    }
    let words: Vec<&str> = collapsed.split(' ').filter(|w| !w.is_empty()).collect();
    wrap_words(&words, width)
}

/// Break a word longer than the width across lines,
/// pushing the full lines and returning the remainder to
/// stand as the current line.
fn break_long_word(word: &str, width: usize, lines: &mut Vec<String>) -> String {
    let mut rest: String = word.to_string();
    loop {
        let room = width.max(1);
        if len_of(&rest) <= room {
            return rest;
        }
        let head: String = rest.chars().take(room).collect();
        lines.push(head);
        rest = rest.chars().skip(room).collect();
    }
}

fn wrap_words(words: &[&str], width: usize) -> Vec<String> {
    // A greedy word wrap. A word longer than the width is
    // broken across lines, filling the current line first,
    // as the Python wrapper breaks long words. No text this
    // program formats holds such a word at the widths the
    // floors allow, but the rule stands for completeness.
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    for word in words {
        let word_len = len_of(word);
        if current.is_empty() {
            if word_len <= width {
                current = word.to_string();
            } else {
                current = break_long_word(word, width, &mut lines);
            }
            continue;
        }
        if len_of(&current) + 1 + word_len <= width {
            current.push(' ');
            current.push_str(word);
        } else if word_len > width {
            // The long word fills the room left on this line
            // before it breaks.
            let room = width - len_of(&current) - 1;
            let mut rest: String = word.to_string();
            if room > 0 {
                let head: String = rest.chars().take(room).collect();
                current.push(' ');
                current.push_str(&head);
                rest = rest.chars().skip(room).collect();
                lines.push(std::mem::take(&mut current));
            } else {
                lines.push(std::mem::take(&mut current));
            }
            current = break_long_word(&rest, width, &mut lines);
        } else {
            lines.push(std::mem::take(&mut current));
            current = word.to_string();
        }
    }
    if !current.is_empty() || lines.is_empty() {
        lines.push(current);
    }
    lines
}

fn format_action(action: &ActionInfo, help_position: usize, width: usize) -> String {
    let indent = " ".repeat(action.indent);
    let header = format!("{indent}{}", action.invocation);
    let help = match &action.help {
        None => return format!("{header}\n"),
        Some(help) if help.is_empty() => return format!("{header}\n"),
        Some(help) => help,
    };
    let action_width = help_position as i64 - action.indent as i64 - 2;
    let help_width = (width as i64 - help_position as i64).max(11) as usize;
    let help_lines = wrap_text(help, help_width);
    let mut out = String::new();
    if (len_of(&action.invocation) as i64) <= action_width {
        let padded = format!("{header:<width$}", width = help_position);
        out.push_str(&padded);
        for (index, line) in help_lines.iter().enumerate() {
            if index > 0 {
                out.push_str(&" ".repeat(help_position));
            }
            out.push_str(line);
            out.push('\n');
        }
    } else {
        out.push_str(&header);
        out.push('\n');
        for line in help_lines {
            out.push_str(&" ".repeat(help_position));
            out.push_str(&line);
            out.push('\n');
        }
    }
    out
}

// ----------------------------------------------------------------
// The dispatch, mirroring the Python argparse run over the
// two parser shapes.

const COMMANDS: [&str; 8] = [
    "map",
    "check",
    "claims",
    "census",
    "atlas",
    "index",
    "changelog",
    "lineage",
];
const CHOICES_DISPLAY: &str =
    "'map', 'check', 'claims', 'census', 'atlas', 'index', 'changelog', 'lineage'";
const CHOICES_HEADER: &str = "{map,check,claims,census,atlas,index,changelog,lineage}";

fn help_action() -> ActionInfo {
    ActionInfo {
        invocation: "-h, --help".to_string(),
        indent: 2,
        help: Some("show this help message and exit".to_string()),
        usage_part: "[-h]".to_string(),
    }
}

fn top_info() -> ParserInfo {
    ParserInfo {
        prog: "myriad".to_string(),
        description: Some("Fork one hard question into branches and map them.".to_string()),
        positionals: vec![ActionInfo {
            invocation: CHOICES_HEADER.to_string(),
            indent: 2,
            help: None,
            usage_part: format!("{CHOICES_HEADER} ..."),
        }],
        optionals: vec![help_action()],
        subactions: [
            ("map", "print the map of a tree"),
            ("check", "print the branch counts of a tree"),
            ("claims", "print the claims listing of a tree"),
            ("census", "print the territory census of a tree"),
            ("atlas", "print the rendering of an atlas of trees"),
            ("index", "print the index of an atlas of trees"),
            ("changelog", "print the changelog of two atlases"),
            ("lineage", "print the lineage of a chain of atlases"),
        ]
        .into_iter()
        .map(|(name, help)| (name.to_string(), help.to_string()))
        .collect(),
    }
}

/// The positional names a subcommand takes, in the order
/// its arguments stand.
fn sub_positional_names(command: &str) -> &'static [&'static str] {
    if command == "changelog" {
        &["earlier", "later"]
    } else {
        &["file"]
    }
}

fn sub_info(command: &str) -> ParserInfo {
    ParserInfo {
        prog: format!("myriad {command}"),
        description: None,
        positionals: sub_positional_names(command)
            .iter()
            .map(|name| ActionInfo {
                invocation: name.to_string(),
                indent: 2,
                help: None,
                usage_part: name.to_string(),
            })
            .collect(),
        optionals: vec![help_action()],
        subactions: Vec::new(),
    }
}

fn error_outcome(info: &ParserInfo, width: usize, message: &str) -> Outcome {
    let mut stderr = Vec::new();
    for line in info.format_usage(width).lines() {
        stderr.push(line.to_string());
    }
    stderr.push(format!("{}: error: {}", info.prog, message));
    Outcome {
        stdout: Vec::new(),
        stderr,
        code: 2,
    }
}

fn help_outcome(info: &ParserInfo, width: usize) -> Outcome {
    Outcome {
        stdout: info
            .format_help(width)
            .lines()
            .map(str::to_string)
            .collect(),
        stderr: Vec::new(),
        code: 0,
    }
}

/// How one argument token classifies in a parser whose only
/// option is the help option.
#[derive(Clone, Debug, PartialEq)]
enum Class {
    /// A positional argument.
    Positional,
    /// The separator token, after which every token is a
    /// positional.
    Marker,
    /// The help option with no explicit argument.
    Help,
    /// The help option carrying an explicit argument.
    HelpExplicit {
        /// Whether the matched option string is the short
        /// one, which the cluster rule can split further.
        single: bool,
        /// Whether an equals sign carried the argument.
        sep: bool,
        /// The explicit argument text.
        explicit: PText,
    },
    /// An option-like token the parser does not know.
    Unknown,
}

fn starts_with_dashdash(units: &[u32]) -> bool {
    units.len() >= 2 && units[0] == 0x2D && units[1] == 0x2D
}

fn eq_units(units: &[u32], text: &str) -> bool {
    units.len() == text.chars().count()
        && units
            .iter()
            .zip(text.chars())
            .all(|(&unit, ch)| unit == ch as u32)
}

/// The argparse negative number pattern, matched in full,
/// over Unicode decimal digits.
fn looks_like_negative_number(units: &[u32]) -> bool {
    if units.first() != Some(&0x2D) {
        return false;
    }
    let rest = &units[1..];
    if !rest.is_empty() && rest.iter().all(|&unit| is_nd(unit)) {
        return true;
    }
    // The second alternative, digits then a point then
    // digits, with the leading digits allowed to be empty.
    let dot = rest.iter().position(|&unit| unit == 0x2E);
    match dot {
        Some(at) => {
            let head = &rest[..at];
            let tail = &rest[at + 1..];
            head.iter().all(|&unit| is_nd(unit))
                && !tail.is_empty()
                && tail.iter().all(|&unit| is_nd(unit))
        }
        None => false,
    }
}

fn classify(token: &PText) -> Class {
    let units = &token.0;
    if units.is_empty() {
        return Class::Positional;
    }
    if units[0] != 0x2D {
        return Class::Positional;
    }
    if eq_units(units, "-h") || eq_units(units, "--help") {
        return Class::Help;
    }
    if units.len() == 1 {
        return Class::Positional;
    }
    // The equals split the Python tries first.
    if let Some(at) = units.iter().position(|&unit| unit == 0x3D) {
        let head = &units[..at];
        if eq_units(head, "-h") {
            return Class::HelpExplicit {
                single: true,
                sep: true,
                explicit: PText(units[at + 1..].to_vec()),
            };
        }
        if eq_units(head, "--help") {
            return Class::HelpExplicit {
                single: false,
                sep: true,
                explicit: PText(units[at + 1..].to_vec()),
            };
        }
    }
    // The prefix and cluster searches.
    if starts_with_dashdash(units) {
        // Long options match by prefix up to any equals sign.
        let prefix_end = units
            .iter()
            .position(|&unit| unit == 0x3D)
            .unwrap_or(units.len());
        let prefix = &units[..prefix_end];
        let help_units: Vec<u32> = "--help".chars().map(|ch| ch as u32).collect();
        if help_units.starts_with(prefix) {
            if prefix_end < units.len() {
                return Class::HelpExplicit {
                    single: false,
                    sep: true,
                    explicit: PText(units[prefix_end + 1..].to_vec()),
                };
            }
            return Class::Help;
        }
    } else {
        // A short cluster whose first option is the help
        // option carries the rest as its explicit argument.
        if units.len() >= 2 && units[1] == 0x68 {
            return Class::HelpExplicit {
                single: true,
                sep: false,
                explicit: PText(units[2..].to_vec()),
            };
        }
    }
    if looks_like_negative_number(units) {
        return Class::Positional;
    }
    if units.contains(&0x20) {
        return Class::Positional;
    }
    Class::Unknown
}

/// Build the classification pattern for one parser level,
/// where the first separator turns every later token into a
/// positional.
fn classify_level(tokens: &[PText]) -> Vec<Class> {
    let mut classes: Vec<Class> = Vec::new();
    let mut after_marker = false;
    for token in tokens {
        if after_marker {
            classes.push(Class::Positional);
        } else if eq_units(&token.0, "--") {
            classes.push(Class::Marker);
            after_marker = true;
        } else {
            classes.push(classify(token));
        }
    }
    classes
}

/// The outcome of consuming one option token, mirroring the
/// argparse optional consumption for the help option.
enum OptStep {
    /// The index after the consumed token.
    Next(usize),
    /// A finished outcome, help printed or an error raised.
    Done(Outcome),
}

fn consume_optional(
    info: &ParserInfo,
    width: usize,
    class: &Class,
    token: &PText,
    index: usize,
    extras: &mut Vec<PText>,
) -> OptStep {
    match class {
        Class::Unknown => {
            extras.push(token.clone());
            OptStep::Next(index + 1)
        }
        Class::Help => OptStep::Done(help_outcome(info, width)),
        Class::HelpExplicit {
            single,
            sep,
            explicit,
        } => {
            // The help action takes no argument. A short
            // option carrying a tail with no equals sign
            // splits, the help fires, and any tail the split
            // cannot place is set aside unseen because the
            // help exits first. Every other explicit argument
            // is refused.
            let fires =
                *single && !sep && !explicit.is_empty() && explicit.0.first() != Some(&0x2D);
            if fires {
                return OptStep::Done(help_outcome(info, width));
            }
            let message = format!(
                "argument -h/--help: ignored explicit argument {}",
                py_repr(explicit)
            );
            OptStep::Done(error_outcome(info, width, &message))
        }
        Class::Positional | Class::Marker => {
            unreachable!("an option index never holds a positional class")
        }
    }
}

/// One parser level's parse, over the token classes.
struct Level {
    /// The command token, at the top level.
    command: Option<PText>,
    /// The file tokens, at a sub level, one for a plain
    /// command and two for the changelog.
    files: Vec<PText>,
    /// The tokens the sub level hands back, at the top level.
    sub_tokens: Vec<PText>,
    /// Extra tokens in the order they were met.
    extras: Vec<PText>,
}

enum LevelStep {
    Parsed(Level),
    Done(Outcome),
}

/// Consume positionals from an index at the top level.
///
/// The subcommand action consumes greedily to the end of
/// the token list once its first positional stands, exactly
/// as its pattern matches. Returns the index after the
/// consumed span, having run the sub parse when the action
/// fired, or the start index when nothing matched.
fn top_consume_positionals(
    tokens: &[PText],
    classes: &[Class],
    start: usize,
    width: usize,
    level: &mut Level,
    taken: &mut bool,
) -> Result<usize, Outcome> {
    if *taken {
        return Ok(start);
    }
    // The pattern skips separators, then needs one
    // positional.
    let mut at = start;
    while at < tokens.len() && classes[at] == Class::Marker {
        at += 1;
    }
    if at >= tokens.len() || classes[at] != Class::Positional {
        return Ok(start);
    }
    let span = &tokens[start..];
    let command = span[0].clone();
    level.command = Some(command.clone());
    *taken = true;
    // The choice check fires inside the value reading,
    // before the sub parse runs.
    if !COMMANDS.iter().any(|name| command.eq_str(name)) {
        let info = top_info();
        let message = format!(
            "argument command: invalid choice: {} (choose from {CHOICES_DISPLAY})",
            py_repr(&command)
        );
        return Err(error_outcome(&info, width, &message));
    }
    level.sub_tokens = span[1..].to_vec();
    Ok(tokens.len())
}

/// Consume positionals from an index at a sub level.
///
/// The file actions match separators around their
/// positionals and strip the first separator from their
/// values. A plain command takes one positional and the
/// changelog takes two, each standing after the separators
/// that precede it.
fn sub_consume_positionals(
    tokens: &[PText],
    classes: &[Class],
    start: usize,
    level: &mut Level,
    names: &[&str],
    taken: &mut bool,
) -> usize {
    if *taken {
        return start;
    }
    let mut at = start;
    let mut took = false;
    while level.files.len() < names.len() {
        let mut probe = at;
        while probe < tokens.len() && classes[probe] == Class::Marker {
            probe += 1;
        }
        if probe >= tokens.len() || classes[probe] != Class::Positional {
            break;
        }
        level.files.push(tokens[probe].clone());
        took = true;
        at = probe + 1;
    }
    if !took {
        return start;
    }
    if level.files.len() == names.len() {
        *taken = true;
    }
    while at < tokens.len() && classes[at] == Class::Marker {
        at += 1;
    }
    at
}

/// Run one parser level's alternation loop.
///
/// The top level and the sub level share the loop shape. The
/// caller chooses the positional consumer and the required
/// name. A finished outcome (help or an error) returns at
/// once, as the Python exits inside the parse.
fn run_level(
    tokens: &[PText],
    info: &ParserInfo,
    width: usize,
    top: bool,
    names: &[&str],
) -> LevelStep {
    let classes = classify_level(tokens);
    let option_indices: Vec<usize> = classes
        .iter()
        .enumerate()
        .filter(|(_, class)| {
            matches!(
                class,
                Class::Help | Class::HelpExplicit { .. } | Class::Unknown
            )
        })
        .map(|(index, _)| index)
        .collect();
    let max_option = option_indices.iter().max().copied();
    let mut level = Level {
        command: None,
        files: Vec::new(),
        sub_tokens: Vec::new(),
        extras: Vec::new(),
    };
    let mut taken = false;
    let mut start = 0usize;
    if let Some(max_index) = max_option {
        while start <= max_index {
            let next_option = option_indices
                .iter()
                .copied()
                .filter(|index| *index >= start)
                .min()
                .expect("an option index stands ahead");
            if start != next_option {
                let end = if top {
                    match top_consume_positionals(
                        tokens, &classes, start, width, &mut level, &mut taken,
                    ) {
                        Ok(end) => end,
                        Err(outcome) => return LevelStep::Done(outcome),
                    }
                } else {
                    sub_consume_positionals(tokens, &classes, start, &mut level, names, &mut taken)
                };
                if end > start {
                    start = end;
                    continue;
                }
                start = end;
            }
            if !option_indices.contains(&start) {
                level
                    .extras
                    .extend(tokens[start..next_option].iter().cloned());
                start = next_option;
            }
            match consume_optional(
                info,
                width,
                &classes[start],
                &tokens[start],
                start,
                &mut level.extras,
            ) {
                OptStep::Next(next) => start = next,
                OptStep::Done(outcome) => return LevelStep::Done(outcome),
            }
        }
    }
    let stop = if top {
        match top_consume_positionals(tokens, &classes, start, width, &mut level, &mut taken) {
            Ok(end) => end,
            Err(outcome) => return LevelStep::Done(outcome),
        }
    } else {
        sub_consume_positionals(tokens, &classes, start, &mut level, names, &mut taken)
    };
    level.extras.extend(tokens[stop..].iter().cloned());
    if !taken {
        let name = if top {
            "command".to_string()
        } else {
            names[level.files.len()..].join(", ")
        };
        return LevelStep::Done(error_outcome(
            info,
            width,
            &format!("the following arguments are required: {name}"),
        ));
    }
    LevelStep::Parsed(level)
}

fn load_exploration_arg(
    path: &PText,
    read_file: &dyn Fn(&PText) -> Option<String>,
) -> (Option<Exploration>, Vec<String>) {
    match read_file(path) {
        None => (None, vec!["file cannot be read".to_string()]),
        Some(text) => load_exploration_text(&text),
    }
}

fn load_atlas_arg(
    path: &PText,
    read_file: &dyn Fn(&PText) -> Option<String>,
) -> (Option<Atlas>, Vec<String>) {
    match read_file(path) {
        None => (None, vec!["file cannot be read".to_string()]),
        Some(text) => load_atlas_text(&text),
    }
}

fn load_lineage_arg(
    path: &PText,
    read_file: &dyn Fn(&PText) -> Option<String>,
) -> (Option<Lineage>, Vec<String>) {
    match read_file(path) {
        None => (None, vec!["file cannot be read".to_string()]),
        Some(text) => load_lineage_text(&text),
    }
}

fn exploration_outcome(
    loaded: (Option<Exploration>, Vec<String>),
    render: fn(&Exploration) -> Result<Vec<String>, String>,
) -> Outcome {
    match loaded {
        (Some(exploration), _) => Outcome {
            stdout: render(&exploration)
                .expect("a loaded exploration satisfies the reading preconditions"),
            stderr: Vec::new(),
            code: 0,
        },
        (None, defects) => {
            let mut stdout = vec!["invalid exploration".to_string()];
            for defect in &defects {
                stdout.push(format!("defect {defect}"));
            }
            Outcome {
                stdout,
                stderr: Vec::new(),
                code: 1,
            }
        }
    }
}

/// Run the changelog command over its two files.
///
/// Both files load whole through the atlas loader before
/// anything prints, the later file even when the earlier
/// file draws defects, and a malformed pair prints the
/// defects of both files under the word naming each file,
/// as MY-12 states.
fn run_changelog(
    earlier_path: &PText,
    later_path: &PText,
    read_file: &dyn Fn(&PText) -> Option<String>,
) -> Outcome {
    let (earlier, earlier_defects) = load_atlas_arg(earlier_path, read_file);
    let (later, later_defects) = load_atlas_arg(later_path, read_file);
    match (earlier, later) {
        (Some(earlier), Some(later)) => Outcome {
            stdout: render_changelog(&earlier, &later)
                .expect("loaded atlases satisfy the reading preconditions"),
            stderr: Vec::new(),
            code: 0,
        },
        _ => {
            let mut stdout = vec!["invalid changelog".to_string()];
            for defect in &earlier_defects {
                stdout.push(format!("defect earlier, {defect}"));
            }
            for defect in &later_defects {
                stdout.push(format!("defect later, {defect}"));
            }
            Outcome {
                stdout,
                stderr: Vec::new(),
                code: 1,
            }
        }
    }
}

fn run_command(
    command: &str,
    file: &PText,
    read_file: &dyn Fn(&PText) -> Option<String>,
) -> Outcome {
    match command {
        "check" => {
            let loaded = load_exploration_arg(file, read_file);
            match loaded {
                (Some(exploration), _) => {
                    let counts = count_by_state(&exploration);
                    Outcome {
                        stdout: vec![
                            format!("branches {}", counts.total),
                            format!("open {}", counts.open),
                            format!("taken {}", counts.taken),
                            format!("abandoned {}", counts.abandoned),
                        ],
                        stderr: Vec::new(),
                        code: 0,
                    }
                }
                (None, defects) => exploration_outcome((None, defects), render_map),
            }
        }
        "map" => exploration_outcome(load_exploration_arg(file, read_file), render_map),
        "claims" => exploration_outcome(load_exploration_arg(file, read_file), render_claims),
        "census" => exploration_outcome(load_exploration_arg(file, read_file), render_census),
        "atlas" | "index" => {
            let loaded = load_atlas_arg(file, read_file);
            match loaded {
                (Some(atlas), _) => Outcome {
                    stdout: if command == "atlas" {
                        render_atlas(&atlas)
                            .expect("a loaded atlas satisfies the reading preconditions")
                    } else {
                        render_index(&atlas)
                            .expect("a loaded atlas satisfies the reading preconditions")
                    },
                    stderr: Vec::new(),
                    code: 0,
                },
                (None, defects) => {
                    let mut stdout = vec!["invalid atlas".to_string()];
                    for defect in &defects {
                        stdout.push(format!("defect {defect}"));
                    }
                    Outcome {
                        stdout,
                        stderr: Vec::new(),
                        code: 1,
                    }
                }
            }
        }
        "lineage" => {
            let loaded = load_lineage_arg(file, read_file);
            match loaded {
                (Some(lineage), _) => Outcome {
                    stdout: render_lineage(&lineage)
                        .expect("a loaded lineage satisfies the reading preconditions"),
                    stderr: Vec::new(),
                    code: 0,
                },
                (None, defects) => {
                    let mut stdout = vec!["invalid lineage".to_string()];
                    for defect in &defects {
                        stdout.push(format!("defect {defect}"));
                    }
                    Outcome {
                        stdout,
                        stderr: Vec::new(),
                        code: 1,
                    }
                }
            }
        }
        _ => unreachable!("the choice check admits only the eight commands"),
    }
}

/// Dispatch one command line at an explicit formatter
/// width, the testable core of the binary entry.
pub fn dispatch_with_columns(
    argv: &[PText],
    columns: Option<usize>,
    read_file: &dyn Fn(&PText) -> Option<String>,
) -> Outcome {
    let width = formatter_width(columns);
    dispatch_at_width(argv, width, read_file)
}

/// Dispatch one command line at an explicit formatter
/// width.
pub fn dispatch_at_width(
    argv: &[PText],
    width: usize,
    read_file: &dyn Fn(&PText) -> Option<String>,
) -> Outcome {
    let info = top_info();
    let top = match run_level(argv, &info, width, true, &[]) {
        LevelStep::Done(outcome) => return outcome,
        LevelStep::Parsed(level) => level,
    };
    let command = top.command.clone().expect("the top level holds a command");
    let command_name = COMMANDS
        .iter()
        .find(|name| command.eq_str(name))
        .expect("the choice check admitted the command");
    let sub = sub_info(command_name);
    let mut extras = top.extras.clone();
    let parsed = match run_level(
        &top.sub_tokens,
        &sub,
        width,
        false,
        sub_positional_names(command_name),
    ) {
        LevelStep::Done(outcome) => return outcome,
        LevelStep::Parsed(level) => level,
    };
    extras.extend(parsed.extras.iter().cloned());
    if !extras.is_empty() {
        let joined = extras.iter().map(stderr_text).collect::<Vec<_>>().join(" ");
        return error_outcome(&info, width, &format!("unrecognized arguments: {joined}"));
    }
    if *command_name == "changelog" {
        return run_changelog(&parsed.files[0], &parsed.files[1], read_file);
    }
    let file = parsed.files[0].clone();
    run_command(command_name, &file, read_file)
}

/// Dispatch one command line, reading the width from the
/// environment and the terminal as the Python does.
pub fn dispatch(argv: &[PText], read_file: &dyn Fn(&PText) -> Option<String>) -> Outcome {
    dispatch_with_columns(argv, read_columns_env(), read_file)
}
