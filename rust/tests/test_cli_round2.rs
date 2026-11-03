//! Pins for the Round 2 commands of the myriad command
//! line surface, translated from tests/test_cli_round2.py.
//!
//! The claims and census commands are pinned on their
//! printed output and their exit status together, on a
//! sound file and on a malformed one, exactly as the map
//! and check commands were pinned in Round 1.

mod common;

use common::{run_cli, stdout_text, temp_dir, write_temp};

const SOUND: &str = "{\"question\": \"q\", \"branches\": [\
    {\"id\": \"a\", \"parent\": null, \"prompt\": \"p\", \"state\": \"open\", \
     \"claim\": \"north fork\"}, \
    {\"id\": \"b\", \"parent\": \"a\", \"prompt\": \"p2\", \"state\": \"taken\", \
     \"finding\": \"f\", \"claim\": \"north fork\"}, \
    {\"id\": \"c\", \"parent\": null, \"prompt\": \"p3\", \"state\": \"abandoned\", \
     \"reason\": \"r\"}]}";

fn write(tag: &str, contents: &str) -> String {
    write_temp(tag, "exploration.json", contents)
        .to_string_lossy()
        .into_owned()
}

fn printed(lines: &[&str]) -> String {
    lines.iter().map(|line| format!("{line}\n")).collect()
}

#[test]
fn claims_prints_the_listing_and_exits_zero() {
    let path = write("cli2-claims", SOUND);
    let outcome = run_cli(&["claims", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        printed(&[
            "question, q",
            "claim north fork, branches 2, open 1, taken 1, abandoned 0",
            "unclaimed, branches 1, open 0, taken 0, abandoned 1",
        ])
    );
}

#[test]
fn census_prints_the_census_and_exits_zero() {
    let path = write("cli2-census", SOUND);
    let outcome = run_cli(&["census", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        printed(&[
            "question, q",
            "taken",
            "  claim north fork",
            "    b, finding, f",
            "abandoned",
            "  unclaimed",
            "    c, reason, r",
            "open",
            "  a, prompt, p",
            "branches 3, open 1, taken 1, abandoned 1",
        ])
    );
}

#[test]
fn claims_on_a_malformed_file_prints_defects_and_exits_one() {
    let bad = "{\"question\": \"q\", \"branches\": [{\"id\": \"a\", \
        \"parent\": null, \"prompt\": \"p\", \"state\": \"open\", \"claim\": \"\"}]}";
    let path = write("cli2-claims-bad", bad);
    let outcome = run_cli(&["claims", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        stdout_text(&outcome),
        printed(&[
            "invalid exploration",
            "defect branch 0 field claim is empty",
        ])
    );
}

#[test]
fn census_on_a_malformed_file_prints_defects_and_exits_one() {
    let path = write("cli2-census-bad", "{\"branches\": []}");
    let outcome = run_cli(&["census", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        stdout_text(&outcome),
        printed(&["invalid exploration", "defect missing field question",])
    );
}

#[test]
fn census_on_a_missing_file_exits_one_with_the_read_defect() {
    let dir = temp_dir("cli2-missing");
    let path = dir.join("missing.json").to_string_lossy().into_owned();
    let outcome = run_cli(&["census", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        stdout_text(&outcome),
        printed(&["invalid exploration", "defect file cannot be read",])
    );
}
