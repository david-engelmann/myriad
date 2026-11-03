//! Pins for the Round 3 command of the myriad command line
//! surface, translated from tests/test_cli_round3.py.
//!
//! The atlas command is pinned on its printed output and
//! its exit status together, on a sound file and on a
//! malformed one, exactly as the earlier commands were
//! pinned.

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

fn write(tag: &str, contents: &str) -> String {
    write_temp(tag, "atlas.json", contents)
        .to_string_lossy()
        .into_owned()
}

fn printed(lines: &[&str]) -> String {
    lines.iter().map(|line| format!("{line}\n")).collect()
}

#[test]
fn atlas_prints_the_rendering_and_exits_zero() {
    let path = write("cli3-atlas", SOUND);
    let outcome = run_cli(&["atlas", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        printed(&[
            "atlas, the title",
            "entry first",
            "question, q one",
            "a (taken) p",
            "  claimed by crew",
            "  finding, f",
            "  b (open) p2",
            "branches 2, open 1, taken 1, abandoned 0",
            "claim crew, branches 1, open 0, taken 1, abandoned 0",
            "unclaimed, branches 1, open 1, taken 0, abandoned 0",
            "entry second",
            "question, q two",
            "branches 0, open 0, taken 0, abandoned 0",
            "atlas counts, entries 2, branches 2, open 1, taken 1, abandoned 0",
        ])
    );
}

#[test]
fn atlas_on_a_malformed_file_prints_defects_and_exits_one() {
    let broken = "{\"title\": \"t\", \"entries\": [\
        {\"label\": \"ferry\", \"exploration\": {\"branches\": []}}, \
        {\"label\": \"ferry\", \"exploration\": {\"question\": \"q\", \
         \"branches\": []}}]}";
    let path = write("cli3-atlas-bad", broken);
    let outcome = run_cli(&["atlas", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        stdout_text(&outcome),
        printed(&[
            "invalid atlas",
            "defect entry ferry, missing field question",
            "defect duplicate entry label ferry, entries 0 and 1",
        ])
    );
}

#[test]
fn atlas_on_a_missing_file_prints_the_read_defect() {
    let dir = temp_dir("cli3-missing");
    let path = dir.join("missing.json").to_string_lossy().into_owned();
    let outcome = run_cli(&["atlas", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        stdout_text(&outcome),
        printed(&["invalid atlas", "defect file cannot be read"])
    );
}
