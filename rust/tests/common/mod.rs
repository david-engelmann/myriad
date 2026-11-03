//! Shared helpers for the translated test corpus.
//!
//! The Python corpus builds its inputs as dictionaries and
//! hands them to the loaders through json.dumps. The Rust
//! corpus builds the same JSON text directly, so the loaders
//! read exactly the bytes the Python loaders read.

#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use myriad::atlas::{load_atlas_text, Atlas};
use myriad::cli::{dispatch_with_columns, Outcome};
use myriad::explorations::{load_exploration_text, Branch, Exploration, State};
use myriad::pytext::PText;

/// The root of the repository, where the examples and the
/// standing files live.
pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Text from a Rust string literal.
pub fn pt(text: &str) -> PText {
    PText::from_text(text)
}

/// Expected lines as owned strings.
pub fn strings(lines: &[&str]) -> Vec<String> {
    lines.iter().map(|line| line.to_string()).collect()
}

/// Quote and escape a string the way json.dumps writes it.
pub fn jstr(value: &str) -> String {
    let mut out = String::from("\"");
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            ch if (ch as u32) < 0x20 || (ch as u32) > 0x7e => {
                out.push_str(&format!("\\u{:04x}", ch as u32));
            }
            ch => out.push(ch),
        }
    }
    out.push('"');
    out
}

/// A JSON object from raw field values, in the dumps layout.
pub fn jobj(fields: &[(&str, String)]) -> String {
    let inner: Vec<String> = fields
        .iter()
        .map(|(key, value)| format!("{}: {}", jstr(key), value))
        .collect();
    format!("{{{}}}", inner.join(", "))
}

/// A JSON array from raw item values, in the dumps layout.
pub fn jarr(items: &[String]) -> String {
    format!("[{}]", items.join(", "))
}

/// The JSON null literal.
pub fn jnull() -> String {
    "null".to_string()
}

/// One plain branch object as JSON text.
pub fn branch_json(id: &str, parent: Option<&str>, prompt: &str, state: &str) -> String {
    jobj(&[
        ("id", jstr(id)),
        ("parent", parent.map(jstr).unwrap_or_else(jnull)),
        ("prompt", jstr(prompt)),
        ("state", jstr(state)),
    ])
}

/// One exploration as JSON text from a question and branch
/// objects.
pub fn exploration_json(question: &str, branches: &[String]) -> String {
    jobj(&[("question", jstr(question)), ("branches", jarr(branches))])
}

/// The defects of an exploration text that must not load.
pub fn defects_of(data_json: &str) -> Vec<String> {
    let (found, defects) = load_exploration_text(data_json);
    assert!(found.is_none(), "expected defects, found a clean load");
    defects
}

/// An exploration text that must load clean.
pub fn loads(data_json: &str) -> Exploration {
    let (found, defects) = load_exploration_text(data_json);
    assert_eq!(defects, Vec::<String>::new());
    found.expect("a clean load holds an exploration")
}

/// The defects of an atlas text that must not load.
pub fn atlas_defects_of(data_json: &str) -> Vec<String> {
    let (found, defects) = load_atlas_text(data_json);
    assert!(found.is_none(), "expected defects, found a clean load");
    defects
}

/// An atlas text that must load clean.
pub fn atlas_loads(data_json: &str) -> Atlas {
    let (found, defects) = load_atlas_text(data_json);
    assert_eq!(defects, Vec::<String>::new());
    found.expect("a clean load holds an atlas")
}

/// Apply overrides and drops to a field list, the way the
/// Python helpers update a base dictionary.
fn apply<'a>(
    mut fields: Vec<(&'a str, String)>,
    overrides: &[(&'a str, String)],
    drop: &[&str],
) -> Vec<(String, String)> {
    fields.retain(|(key, _)| !drop.contains(key));
    for (key, value) in overrides {
        if let Some(slot) = fields.iter_mut().find(|(held, _)| held == key) {
            slot.1 = value.clone();
        } else {
            fields.push((key, value.clone()));
        }
    }
    fields
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect()
}

fn fields_json(fields: &[(String, String)]) -> String {
    let refs: Vec<(&str, String)> = fields
        .iter()
        .map(|(key, value)| (key.as_str(), value.clone()))
        .collect();
    jobj(&refs)
}

/// One branch object with the base fields, overridden and
/// dropped as the test asks.
pub fn branch_obj(overrides: &[(&str, String)], drop: &[&str]) -> String {
    let base = vec![
        ("id", jstr("a")),
        ("parent", jnull()),
        ("prompt", jstr("p")),
        ("state", jstr("open")),
    ];
    fields_json(&apply(base, overrides, drop))
}

/// One exploration object with the base fields, overridden
/// and dropped as the test asks.
pub fn exploration_obj(branches: &[String], overrides: &[(&str, String)], drop: &[&str]) -> String {
    let base = vec![("question", jstr("q")), ("branches", jarr(branches))];
    fields_json(&apply(base, overrides, drop))
}

/// One branch value built directly, for the rendering pins
/// that hand a tree to a renderer without loading it.
pub fn branch(
    id: &str,
    parent: Option<&str>,
    prompt: &str,
    state: &str,
    claim: Option<&str>,
    finding: Option<&str>,
    reason: Option<&str>,
) -> Branch {
    Branch {
        id: pt(id),
        parent: parent.map(pt),
        prompt: pt(prompt),
        state: match state {
            "open" => State::Open,
            "taken" => State::Taken,
            _ => State::Abandoned,
        },
        claim: claim.map(pt),
        finding: finding.map(pt),
        reason: reason.map(pt),
    }
}

/// The words no rendering in this repo may print, held in one
/// place so every vocabulary scan in the tests reads the same
/// list, as the Python conftest holds them.
pub const FORBIDDEN_WORDS: [&str; 18] = [
    "winner",
    "verdict",
    "vote",
    "rank",
    "ranked",
    "ranking",
    "best",
    "worst",
    "largest",
    "smallest",
    "most",
    "least",
    "beats",
    "versus",
    "compared",
    "comparison",
    "top",
    "leading",
];

fn is_word_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

/// Whether a text holds no forbidden word at a word boundary,
/// the reading of the Python regular expression scans.
pub fn holds_vocabulary_bar(text: &str) -> bool {
    let lower: Vec<char> = text.to_lowercase().chars().collect();
    for word in FORBIDDEN_WORDS {
        let wanted: Vec<char> = word.chars().collect();
        if lower.len() < wanted.len() {
            continue;
        }
        for start in 0..=(lower.len() - wanted.len()) {
            if lower[start..start + wanted.len()] == wanted[..] {
                let before_ok = start == 0 || !is_word_char(lower[start - 1]);
                let after = start + wanted.len();
                let after_ok = after == lower.len() || !is_word_char(lower[after]);
                if before_ok && after_ok {
                    return false;
                }
            }
        }
    }
    true
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A fresh temporary directory for one test.
pub fn temp_dir(tag: &str) -> PathBuf {
    let number = COUNTER.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("myriad-{tag}-{}-{number}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("a temporary directory");
    dir
}

/// Write one temporary file and return its path.
pub fn write_temp(tag: &str, name: &str, contents: &str) -> PathBuf {
    let path = temp_dir(tag).join(name);
    std::fs::write(&path, contents).expect("a temporary file");
    path
}

/// A file reader over the real filesystem at plain paths,
/// standing in the place the binary entry stands.
pub fn real_reader(path: &PText) -> Option<String> {
    let text: String = path
        .0
        .iter()
        .map(|&unit| char::from_u32(unit).unwrap_or('\u{fffd}'))
        .collect();
    std::fs::read_to_string(text).ok()
}

/// Run the command line the way the in-process Python main
/// runs at a terminal width of 80 columns.
pub fn run_cli(argv: &[&str]) -> Outcome {
    let args: Vec<PText> = argv.iter().map(|arg| pt(arg)).collect();
    dispatch_with_columns(&args, Some(80), &real_reader)
}

/// The standard output of an outcome as one text, the shape
/// the Python capture reads.
pub fn stdout_text(outcome: &Outcome) -> String {
    outcome
        .stdout
        .iter()
        .map(|line| format!("{line}\n"))
        .collect()
}
