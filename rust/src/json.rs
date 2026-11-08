//! A JSON value and a reading of JSON text that mirrors the
//! Python json reading the loaders rest on.
//!
//! The mirror covers the semantics the loaders depend on. A
//! repeated object key keeps its last value. A number is
//! classified from its raw literal, so an integer literal of
//! more than 4300 digits fails the reading as the Python
//! integer string limit fails it, and a float literal that
//! overflows stands as infinity. The NaN and Infinity literals
//! parse. A string is a sequence of code points, so an unpaired
//! surrogate is representable and the loaders can refuse it by
//! name. Input nested past the open container boundary fails
//! the reading, as input nested past the Python recursion
//! boundary fails it. The parse walks an explicit stack and
//! the value drops through an explicit work list, so neither
//! direction leans on the call stack.

use std::fmt;
use std::fmt::Write as _;

use crate::pytext::PText;

/// The deepest nesting the reading accepts, measured through
/// the console entry point.
pub const MAX_OPEN_CONTAINERS: usize = 9997;

/// The integer digit limit of the Python reading.
pub const MAX_INTEGER_DIGITS: usize = 4300;

/// A JSON number, classified from its raw literal.
///
/// The loaders never read a number as a figure, so an integer
/// stands as its literal text, which is exact, and a float
/// stands as the double the Python reading computes.
#[derive(Clone, Debug, PartialEq)]
pub enum Num {
    /// An integer literal, held as its raw text.
    Int(String),
    /// A float literal, held as its double value.
    Float(f64),
}

/// A parsed JSON value.
///
/// The Clone, PartialEq, and Debug behaviours stand as
/// manual implementations below, over explicit stacks, so
/// each settles at the reading's own depth boundary under
/// FIX-MY-16, where the derived forms recursed.
pub enum JsonValue {
    /// The null literal.
    Null,
    /// A boolean literal.
    Bool(bool),
    /// A number literal.
    Num(Num),
    /// A string, held as code points.
    Str(PText),
    /// An array of values.
    Arr(Vec<JsonValue>),
    /// An object, as key and value pairs in document order.
    Obj(Vec<(PText, JsonValue)>),
}

impl JsonValue {
    /// The last value held under a key, mirroring the Python
    /// dictionary the reading builds, where a repeated key
    /// keeps its last value.
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Obj(pairs) => pairs
                .iter()
                .rev()
                .find(|(held, _)| held.eq_str(key))
                .map(|(_, value)| value),
            _ => None,
        }
    }

    /// Whether the value is an object.
    pub fn is_object(&self) -> bool {
        matches!(self, JsonValue::Obj(_))
    }
}

impl Drop for JsonValue {
    fn drop(&mut self) {
        // Drain the children through a work list, so dropping a
        // deeply nested value cannot grow the call stack.
        let mut pending: Vec<JsonValue> = Vec::new();
        drain_children(self, &mut pending);
        while let Some(mut value) = pending.pop() {
            drain_children(&mut value, &mut pending);
            // The emptied shell drops here without recursion,
            // because its own children already moved out.
        }
    }
}

impl Clone for JsonValue {
    fn clone(&self) -> JsonValue {
        // The copy is assembled bottom up over an explicit
        // stack of steps, so cloning a value nested to the
        // reading's boundary cannot grow the call stack, the
        // protection FIX-MY-16 settles beside the explicit
        // stack of the parse and the work list of the Drop.
        enum Step<'a> {
            Enter(&'a JsonValue),
            FinishArray(usize),
            FinishObject(&'a [(PText, JsonValue)]),
        }
        let mut steps: Vec<Step<'_>> = vec![Step::Enter(self)];
        let mut done: Vec<JsonValue> = Vec::new();
        while let Some(step) = steps.pop() {
            match step {
                Step::Enter(value) => match value {
                    JsonValue::Null => done.push(JsonValue::Null),
                    JsonValue::Bool(flag) => done.push(JsonValue::Bool(*flag)),
                    JsonValue::Num(number) => done.push(JsonValue::Num(number.clone())),
                    JsonValue::Str(text) => done.push(JsonValue::Str(text.clone())),
                    JsonValue::Arr(items) => {
                        steps.push(Step::FinishArray(items.len()));
                        for item in items.iter().rev() {
                            steps.push(Step::Enter(item));
                        }
                    }
                    JsonValue::Obj(pairs) => {
                        steps.push(Step::FinishObject(pairs));
                        for (_, item) in pairs.iter().rev() {
                            steps.push(Step::Enter(item));
                        }
                    }
                },
                Step::FinishArray(count) => {
                    let children = done.split_off(done.len() - count);
                    done.push(JsonValue::Arr(children));
                }
                Step::FinishObject(pairs) => {
                    let values = done.split_off(done.len() - pairs.len());
                    let copied: Vec<(PText, JsonValue)> = pairs
                        .iter()
                        .map(|(key, _)| key.clone())
                        .zip(values)
                        .collect();
                    done.push(JsonValue::Obj(copied));
                }
            }
        }
        done.pop().expect("the walk completes the root value")
    }
}

impl PartialEq for JsonValue {
    fn eq(&self, other: &JsonValue) -> bool {
        // The derived semantics stand exactly, the same
        // variant, object pairs in document order, an integer
        // by its literal text, a float by the equality of its
        // double, so a NaN stands unequal to itself. The pairs
        // walk an explicit stack, so the comparison settles at
        // the reading's boundary as FIX-MY-16 states.
        let mut pairs: Vec<(&JsonValue, &JsonValue)> = vec![(self, other)];
        while let Some((left, right)) = pairs.pop() {
            match (left, right) {
                (JsonValue::Null, JsonValue::Null) => {}
                (JsonValue::Bool(a), JsonValue::Bool(b)) => {
                    if a != b {
                        return false;
                    }
                }
                (JsonValue::Num(a), JsonValue::Num(b)) => {
                    if a != b {
                        return false;
                    }
                }
                (JsonValue::Str(a), JsonValue::Str(b)) => {
                    if a != b {
                        return false;
                    }
                }
                (JsonValue::Arr(a), JsonValue::Arr(b)) => {
                    if a.len() != b.len() {
                        return false;
                    }
                    pairs.extend(a.iter().zip(b.iter()));
                }
                (JsonValue::Obj(a), JsonValue::Obj(b)) => {
                    if a.len() != b.len() {
                        return false;
                    }
                    for ((key_a, value_a), (key_b, value_b)) in a.iter().zip(b.iter()) {
                        if key_a != key_b {
                            return false;
                        }
                        pairs.push((value_a, value_b));
                    }
                }
                _ => return false,
            }
        }
        true
    }
}

impl fmt::Debug for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The output is the derived form's plain output byte
        // for byte, the container structure written
        // iteratively over an explicit stack of jobs while
        // each leaf formats through its own Debug, so a value
        // nested to the boundary formats whole. The derived
        // alternate pretty form is not reproduced, and the
        // plain form stands under both flags, the one
        // deliberate deviation FIX-MY-16 names.
        enum Job<'a> {
            Value(&'a JsonValue),
            Literal(&'static str),
            Number(&'a Num),
            Text(&'a PText),
        }
        let mut out = String::new();
        let mut jobs: Vec<Job<'_>> = vec![Job::Value(self)];
        while let Some(job) = jobs.pop() {
            match job {
                Job::Literal(text) => out.push_str(text),
                Job::Number(number) => {
                    let _ = write!(out, "{number:?}");
                }
                Job::Text(text) => {
                    let _ = write!(out, "{text:?}");
                }
                Job::Value(value) => match value {
                    JsonValue::Null => out.push_str("Null"),
                    JsonValue::Bool(flag) => {
                        out.push_str(if *flag { "Bool(true)" } else { "Bool(false)" });
                    }
                    JsonValue::Num(number) => {
                        jobs.push(Job::Literal(")"));
                        jobs.push(Job::Number(number));
                        jobs.push(Job::Literal("Num("));
                    }
                    JsonValue::Str(text) => {
                        jobs.push(Job::Literal(")"));
                        jobs.push(Job::Text(text));
                        jobs.push(Job::Literal("Str("));
                    }
                    JsonValue::Arr(items) => {
                        if items.is_empty() {
                            out.push_str("Arr([])");
                        } else {
                            jobs.push(Job::Literal("])"));
                            for (index, item) in items.iter().enumerate().rev() {
                                jobs.push(Job::Value(item));
                                if index > 0 {
                                    jobs.push(Job::Literal(", "));
                                }
                            }
                            jobs.push(Job::Literal("Arr(["));
                        }
                    }
                    JsonValue::Obj(pairs) => {
                        if pairs.is_empty() {
                            out.push_str("Obj([])");
                        } else {
                            jobs.push(Job::Literal("])"));
                            for (index, (key, item)) in pairs.iter().enumerate().rev() {
                                jobs.push(Job::Literal(")"));
                                jobs.push(Job::Value(item));
                                jobs.push(Job::Literal(", "));
                                jobs.push(Job::Text(key));
                                jobs.push(Job::Literal("("));
                                if index > 0 {
                                    jobs.push(Job::Literal(", "));
                                }
                            }
                            jobs.push(Job::Literal("Obj(["));
                        }
                    }
                },
            }
        }
        f.write_str(&out)
    }
}

fn drain_children(value: &mut JsonValue, pending: &mut Vec<JsonValue>) {
    match value {
        JsonValue::Arr(items) => pending.append(items),
        JsonValue::Obj(pairs) => {
            for (_, child) in pairs.drain(..) {
                pending.push(child);
            }
        }
        _ => {}
    }
}

/// Parse JSON text the way the Python json reading does.
///
/// The result is the parsed value, or nothing when the text is
/// not valid JSON under the mirrored semantics.
pub fn parse_json(text: &str) -> Option<JsonValue> {
    let mut parser = Parser {
        bytes: text.as_bytes(),
        pos: 0,
    };
    parser.skip_ws();
    let value = parser.parse_value()?;
    parser.skip_ws();
    if parser.pos != parser.bytes.len() {
        return None;
    }
    Some(value)
}

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl Parser<'_> {
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn skip_ws(&mut self) {
        while let Some(byte) = self.peek() {
            if matches!(byte, b' ' | b'\t' | b'\n' | b'\r') {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn parse_value(&mut self) -> Option<JsonValue> {
        // The container stack keeps the parse iterative, as the
        // mirrored depth boundary sits far past the depth a
        // recursive descent could hold on a small thread stack.
        let mut stack: Vec<Frame> = Vec::new();
        let mut completed: Option<JsonValue> = None;
        'outer: loop {
            if let Some(done) = completed.take() {
                match stack.last_mut() {
                    None => return Some(done),
                    Some(Frame::Arr(items)) => {
                        items.push(done);
                        self.skip_ws();
                        match self.peek()? {
                            b',' => {
                                self.pos += 1;
                                self.skip_ws();
                            }
                            b']' => {
                                self.pos += 1;
                                match stack.pop() {
                                    Some(Frame::Arr(items)) => {
                                        completed = Some(JsonValue::Arr(items));
                                    }
                                    _ => return None,
                                }
                            }
                            _ => return None,
                        }
                        continue 'outer;
                    }
                    Some(Frame::Obj(pairs, pending_key)) => {
                        let key = pending_key.take()?;
                        pairs.push((key, done));
                        self.skip_ws();
                        match self.peek()? {
                            b',' => {
                                self.pos += 1;
                                self.skip_ws();
                                let key = self.parse_key()?;
                                match stack.last_mut() {
                                    Some(Frame::Obj(_, pending)) => *pending = Some(key),
                                    _ => return None,
                                }
                            }
                            b'}' => {
                                self.pos += 1;
                                match stack.pop() {
                                    Some(Frame::Obj(done_pairs, _)) => {
                                        completed = Some(JsonValue::Obj(done_pairs));
                                    }
                                    _ => return None,
                                }
                            }
                            _ => return None,
                        }
                        continue 'outer;
                    }
                }
            }
            self.skip_ws();
            match self.peek()? {
                b'{' => {
                    if stack.len() >= MAX_OPEN_CONTAINERS {
                        return None;
                    }
                    self.pos += 1;
                    self.skip_ws();
                    if self.peek() == Some(b'}') {
                        self.pos += 1;
                        completed = Some(JsonValue::Obj(Vec::new()));
                        continue 'outer;
                    }
                    let key = self.parse_key()?;
                    stack.push(Frame::Obj(Vec::new(), Some(key)));
                }
                b'[' => {
                    if stack.len() >= MAX_OPEN_CONTAINERS {
                        return None;
                    }
                    self.pos += 1;
                    self.skip_ws();
                    if self.peek() == Some(b']') {
                        self.pos += 1;
                        completed = Some(JsonValue::Arr(Vec::new()));
                        continue 'outer;
                    }
                    stack.push(Frame::Arr(Vec::new()));
                }
                b'"' => {
                    completed = Some(JsonValue::Str(self.parse_string()?));
                }
                b't' => {
                    self.expect_word("true")?;
                    completed = Some(JsonValue::Bool(true));
                }
                b'f' => {
                    self.expect_word("false")?;
                    completed = Some(JsonValue::Bool(false));
                }
                b'n' => {
                    self.expect_word("null")?;
                    completed = Some(JsonValue::Null);
                }
                b'N' => {
                    self.expect_word("NaN")?;
                    completed = Some(JsonValue::Num(Num::Float(f64::NAN)));
                }
                b'I' => {
                    self.expect_word("Infinity")?;
                    completed = Some(JsonValue::Num(Num::Float(f64::INFINITY)));
                }
                b'-' => {
                    if self.bytes.get(self.pos + 1) == Some(&b'I') {
                        self.pos += 1;
                        self.expect_word("Infinity")?;
                        completed = Some(JsonValue::Num(Num::Float(f64::NEG_INFINITY)));
                    } else {
                        completed = Some(self.parse_number()?);
                    }
                }
                byte if byte.is_ascii_digit() => {
                    completed = Some(self.parse_number()?);
                }
                _ => return None,
            }
        }
    }

    fn parse_key(&mut self) -> Option<PText> {
        self.skip_ws();
        if self.peek() != Some(b'"') {
            return None;
        }
        let key = self.parse_string()?;
        self.skip_ws();
        if self.peek() != Some(b':') {
            return None;
        }
        self.pos += 1;
        Some(key)
    }

    fn expect_word(&mut self, word: &str) -> Option<()> {
        if self.bytes[self.pos..].starts_with(word.as_bytes()) {
            self.pos += word.len();
            Some(())
        } else {
            None
        }
    }

    fn parse_string(&mut self) -> Option<PText> {
        // The cursor is on the opening quote. The Python reading
        // takes the raw characters of the text between escapes,
        // so chunks are read as UTF-8 text, which the input
        // already is, the file reading having refused text that
        // is not UTF-8.
        debug_assert_eq!(self.peek(), Some(b'"'));
        self.pos += 1;
        let mut units: Vec<u32> = Vec::new();
        loop {
            let byte = self.peek()?;
            match byte {
                b'"' => {
                    self.pos += 1;
                    return Some(PText(units));
                }
                b'\\' => {
                    self.pos += 1;
                    let escape = self.peek()?;
                    self.pos += 1;
                    match escape {
                        b'"' => units.push(0x22),
                        b'\\' => units.push(0x5C),
                        b'/' => units.push(0x2F),
                        b'b' => units.push(0x08),
                        b'f' => units.push(0x0C),
                        b'n' => units.push(0x0A),
                        b'r' => units.push(0x0D),
                        b't' => units.push(0x09),
                        b'u' => {
                            let first = self.parse_hex4()?;
                            if (0xD800..=0xDBFF).contains(&first) {
                                // A high surrogate combines only
                                // with an immediately following low
                                // surrogate escape.
                                if self.peek() == Some(b'\\')
                                    && self.bytes.get(self.pos + 1) == Some(&b'u')
                                {
                                    self.pos += 2;
                                    let mark = self.pos;
                                    let second = self.parse_hex4()?;
                                    if (0xDC00..=0xDFFF).contains(&second) {
                                        let combined =
                                            0x10000 + ((first - 0xD800) << 10) + (second - 0xDC00);
                                        units.push(combined);
                                    } else {
                                        units.push(first);
                                        self.pos = mark - 2;
                                    }
                                } else {
                                    units.push(first);
                                }
                            } else {
                                units.push(first);
                            }
                        }
                        _ => return None,
                    }
                }
                0x00..=0x1F => return None,
                _ => {
                    // Take the whole UTF-8 char at the cursor.
                    let start = self.pos;
                    let width = utf8_width(byte)?;
                    let end = start + width;
                    let chunk = self.bytes.get(start..end)?;
                    let text = std::str::from_utf8(chunk).ok()?;
                    let ch = text.chars().next()?;
                    units.push(ch as u32);
                    self.pos = end;
                }
            }
        }
    }

    fn parse_hex4(&mut self) -> Option<u32> {
        let mut value: u32 = 0;
        for _ in 0..4 {
            let byte = self.peek()?;
            let digit = match byte {
                b'0'..=b'9' => u32::from(byte - b'0'),
                b'a'..=b'f' => u32::from(byte - b'a') + 10,
                b'A'..=b'F' => u32::from(byte - b'A') + 10,
                _ => return None,
            };
            value = value * 16 + digit;
            self.pos += 1;
        }
        Some(value)
    }

    fn parse_number(&mut self) -> Option<JsonValue> {
        // The grammar is the JSON number grammar the Python
        // reading accepts, and the classification turns on the
        // raw literal, as the Python reading classifies it.
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.pos += 1;
        }
        match self.peek()? {
            b'0' => {
                self.pos += 1;
            }
            b'1'..=b'9' => {
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }
            _ => return None,
        }
        let mut is_float = false;
        if self.peek() == Some(b'.') {
            is_float = true;
            self.pos += 1;
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return None;
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        if matches!(self.peek(), Some(b'e') | Some(b'E')) {
            is_float = true;
            self.pos += 1;
            if matches!(self.peek(), Some(b'+') | Some(b'-')) {
                self.pos += 1;
            }
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return None;
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        let literal = std::str::from_utf8(&self.bytes[start..self.pos]).ok()?;
        if is_float {
            let value: f64 = literal.parse().ok()?;
            Some(JsonValue::Num(Num::Float(value)))
        } else {
            let digits = literal.strip_prefix('-').unwrap_or(literal).len();
            if digits > MAX_INTEGER_DIGITS {
                return None;
            }
            Some(JsonValue::Num(Num::Int(literal.to_string())))
        }
    }
}

enum Frame {
    Arr(Vec<JsonValue>),
    Obj(Vec<(PText, JsonValue)>, Option<PText>),
}

fn utf8_width(lead: u8) -> Option<usize> {
    match lead {
        0x00..=0x7F => Some(1),
        0xC2..=0xDF => Some(2),
        0xE0..=0xEF => Some(3),
        0xF0..=0xF4 => Some(4),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_repeated_key_keeps_its_last_value() {
        let value = parse_json("{\"a\": 1, \"a\": 2}").expect("parses");
        assert_eq!(value.get("a"), Some(&JsonValue::Num(Num::Int("2".into()))));
    }

    #[test]
    fn literals_parse_to_their_values() {
        assert_eq!(parse_json("null"), Some(JsonValue::Null));
        assert_eq!(parse_json("true"), Some(JsonValue::Bool(true)));
        assert_eq!(parse_json("false"), Some(JsonValue::Bool(false)));
        assert_eq!(
            parse_json("-12"),
            Some(JsonValue::Num(Num::Int("-12".into())))
        );
    }

    #[test]
    fn an_integer_at_the_digit_limit_parses_and_one_past_it_fails() {
        let at_limit = "9".repeat(MAX_INTEGER_DIGITS);
        assert!(parse_json(&at_limit).is_some());
        let past_limit = "9".repeat(MAX_INTEGER_DIGITS + 1);
        assert_eq!(parse_json(&past_limit), None);
        let negative_past = format!("-{}", "9".repeat(MAX_INTEGER_DIGITS + 1));
        assert_eq!(parse_json(&negative_past), None);
    }

    #[test]
    fn a_float_literal_that_overflows_stands_as_infinity() {
        match parse_json("1e400") {
            Some(JsonValue::Num(Num::Float(value))) => assert!(value.is_infinite()),
            other => panic!("expected an infinite float, got {other:?}"),
        }
        match parse_json("NaN") {
            Some(JsonValue::Num(Num::Float(value))) => assert!(value.is_nan()),
            other => panic!("expected a NaN float, got {other:?}"),
        }
        assert_eq!(
            parse_json("-Infinity"),
            Some(JsonValue::Num(Num::Float(f64::NEG_INFINITY)))
        );
    }

    #[test]
    fn the_number_grammar_refuses_malformed_literals() {
        assert_eq!(parse_json("01"), None);
        assert_eq!(parse_json("1."), None);
        assert_eq!(parse_json(".5"), None);
        assert_eq!(parse_json("+1"), None);
        assert_eq!(parse_json("1e"), None);
        assert_eq!(parse_json("-"), None);
        assert_eq!(parse_json("0x10"), None);
    }

    #[test]
    fn a_paired_surrogate_escape_combines() {
        let value = parse_json("\"\\ud83d\\ude00\"").expect("parses");
        assert_eq!(value, JsonValue::Str(PText(vec![0x1F600])));
    }

    #[test]
    fn an_unpaired_surrogate_escape_stands_as_its_unit() {
        let value = parse_json("\"\\ud800\"").expect("parses");
        assert_eq!(value, JsonValue::Str(PText(vec![0xD800])));
        let value = parse_json("\"\\ud800\\u0041\"").expect("parses");
        assert_eq!(value, JsonValue::Str(PText(vec![0xD800, 0x41])));
        let value = parse_json("\"\\udc00\"").expect("parses");
        assert_eq!(value, JsonValue::Str(PText(vec![0xDC00])));
    }

    #[test]
    fn malformed_inputs_fail() {
        assert_eq!(parse_json(""), None);
        assert_eq!(parse_json("{"), None);
        assert_eq!(parse_json("[1,]"), None);
        assert_eq!(parse_json("{\"a\"}"), None);
        assert_eq!(parse_json("{\"a\":1} extra"), None);
        assert_eq!(parse_json("'single'"), None);
        assert_eq!(parse_json("\"bad \\x escape\""), None);
        assert_eq!(parse_json("\"raw\nnewline\""), None);
        assert_eq!(parse_json("tru"), None);
    }

    #[test]
    fn nesting_at_the_boundary_parses_and_past_it_fails() {
        let at = format!(
            "{}1{}",
            "[".repeat(MAX_OPEN_CONTAINERS),
            "]".repeat(MAX_OPEN_CONTAINERS)
        );
        assert!(parse_json(&at).is_some());
        let past = format!(
            "{}1{}",
            "[".repeat(MAX_OPEN_CONTAINERS + 1),
            "]".repeat(MAX_OPEN_CONTAINERS + 1)
        );
        assert_eq!(parse_json(&past), None);
    }

    #[test]
    fn a_momentary_peak_at_the_boundary_parses() {
        // A closed container frees its slot, as in the probe.
        let inner = "[".repeat(MAX_OPEN_CONTAINERS - 2);
        let text = format!("[\"s\", {inner}1{}]", "]".repeat(MAX_OPEN_CONTAINERS - 2));
        assert!(parse_json(&text).is_some());
    }

    #[test]
    fn object_keys_read_through_the_object() {
        let value = parse_json("{\"question\": \"q\", \"branches\": []}").expect("parses");
        assert!(value.is_object());
        assert_eq!(
            value.get("question"),
            Some(&JsonValue::Str(PText::from_text("q")))
        );
        assert_eq!(value.get("missing"), None);
    }
}
