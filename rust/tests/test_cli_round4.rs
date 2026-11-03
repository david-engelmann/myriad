//! Pins for the Round 4 command of the myriad command line
//! surface, translated from tests/test_cli_round4.py.
//!
//! The index command is pinned on its printed output and
//! its exit status together, on a sound file and on a
//! malformed one, exactly as the earlier commands were
//! pinned. A malformed atlas prints the defects the atlas
//! command prints and no line of the index reading, because
//! the command loads the atlas whole before it prints.

mod common;

use common::{run_cli, stdout_text, temp_dir, write_temp};

const SOUND: &str = "{\"title\": \"the title\", \"entries\": [\
    {\"label\": \"first\", \"exploration\": {\"question\": \"q one\", \
     \"branches\": [\
        {\"id\": \"a\", \"parent\": null, \"prompt\": \"p\", \"state\": \"taken\", \
         \"finding\": \"f\", \"claim\": \"crew\"}, \
        {\"id\": \"b\", \"parent\": \"a\", \"prompt\": \"p2\", \"state\": \"open\"}]}}, \
    {\"label\": \"second\", \"exploration\": {\"question\": \"q two\", \
     \"branches\": []}}]}";

const MALFORMED: &str = "{\"title\": \"the title\", \"entries\": [\
    {\"label\": \"first\", \"exploration\": {\"question\": \"q one\", \
     \"branches\": [\
        {\"id\": \"a\", \"parent\": null, \"prompt\": \"p\", \"state\": \"taken\", \
         \"finding\": \"f\"}]}}, \
    {\"label\": \"first\", \"exploration\": {\"question\": \"q two\", \
     \"branches\": []}}, \
    {\"exploration\": {\"question\": \"q three\", \"branches\": []}}]}";

fn write(tag: &str, contents: &str) -> String {
    write_temp(tag, "atlas.json", contents)
        .to_string_lossy()
        .into_owned()
}

fn printed(lines: &[&str]) -> String {
    lines.iter().map(|line| format!("{line}\n")).collect()
}

#[test]
fn index_prints_the_reading_and_exits_zero() {
    let path = write("cli4-index", SOUND);
    let outcome = run_cli(&["index", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        printed(&[
            "atlas, the title",
            "entry first, question, q one, branches 2, open 1, taken 1, abandoned 0",
            "entry second, question, q two, branches 0, open 0, taken 0, abandoned 0",
            "atlas counts, entries 2, branches 2, open 1, taken 1, abandoned 0",
        ])
    );
}

#[test]
fn index_on_a_malformed_file_prints_defects_and_exits_one() {
    let path = write("cli4-index-bad", MALFORMED);
    let outcome = run_cli(&["index", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        stdout_text(&outcome),
        printed(&[
            "invalid atlas",
            "defect entry 2 is missing field label",
            "defect duplicate entry label first, entries 0 and 1",
        ])
    );
}

#[test]
fn index_prints_the_same_defects_as_the_atlas_command() {
    let path = write("cli4-same", MALFORMED);
    let atlas_outcome = run_cli(&["atlas", &path]);
    assert_eq!(atlas_outcome.code, 1);
    let index_outcome = run_cli(&["index", &path]);
    assert_eq!(index_outcome.code, 1);
    assert_eq!(stdout_text(&index_outcome), stdout_text(&atlas_outcome));
}

#[test]
fn index_prints_no_index_line_for_a_malformed_atlas() {
    let path = write("cli4-nolines", MALFORMED);
    let outcome = run_cli(&["index", &path]);
    assert_eq!(outcome.code, 1);
    for line in outcome.stdout.iter() {
        assert!(
            line == "invalid atlas" || line.starts_with("defect "),
            "{line}"
        );
    }
}

#[test]
fn index_on_a_missing_file_exits_one() {
    let dir = temp_dir("cli4-missing");
    let path = dir.join("absent.json").to_string_lossy().into_owned();
    let outcome = run_cli(&["index", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        stdout_text(&outcome),
        printed(&["invalid atlas", "defect file cannot be read"])
    );
}
