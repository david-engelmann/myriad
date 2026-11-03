//! Pins for the myriad command line surface, translated
//! from tests/test_cli.py.
//!
//! Each command is pinned on its printed output and its
//! exit status together, on a sound file and on a malformed
//! one. The Python corpus calls the in-process main, so the
//! Rust corpus calls the dispatch core at the width the
//! Python main reads on a captured stream, 80 columns, over
//! real files.

mod common;

use common::{run_cli, stdout_text, temp_dir, write_temp};

const SOUND: &str = "{\"question\": \"q\", \"branches\": [\
    {\"id\": \"a\", \"parent\": null, \"prompt\": \"p\", \"state\": \"open\"}, \
    {\"id\": \"b\", \"parent\": \"a\", \"prompt\": \"p2\", \"state\": \"taken\", \
     \"finding\": \"f\"}, \
    {\"id\": \"c\", \"parent\": null, \"prompt\": \"p3\", \"state\": \"abandoned\", \
     \"reason\": \"r\"}]}";

fn write(tag: &str, contents: &str) -> String {
    write_temp(tag, "exploration.json", contents)
        .to_string_lossy()
        .into_owned()
}

/// The printed text of the expected lines, each closed by
/// a newline, the shape the Python capture reads.
fn printed(lines: &[&str]) -> String {
    lines.iter().map(|line| format!("{line}\n")).collect()
}

#[test]
fn map_prints_the_map_and_exits_zero() {
    let path = write("cli-map", SOUND);
    let outcome = run_cli(&["map", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        printed(&[
            "question, q",
            "a (open) p",
            "  b (taken) p2",
            "    finding, f",
            "c (abandoned) p3",
            "  reason, r",
            "branches 3, open 1, taken 1, abandoned 1",
        ])
    );
}

#[test]
fn check_prints_the_counts_and_exits_zero() {
    let path = write("cli-check", SOUND);
    let outcome = run_cli(&["check", &path]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        printed(&["branches 3", "open 1", "taken 1", "abandoned 1"])
    );
}

#[test]
fn map_on_a_malformed_file_prints_defects_and_exits_one() {
    let bad = "{\"question\": \"q\", \"branches\": [{\"id\": \"a\", \
        \"parent\": \"ghost\", \"prompt\": \"p\", \"state\": \"open\"}]}";
    let path = write("cli-map-bad", bad);
    let outcome = run_cli(&["map", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        stdout_text(&outcome),
        printed(&[
            "invalid exploration",
            "defect branch a parent ghost does not exist",
        ])
    );
}

#[test]
fn check_on_a_malformed_file_prints_defects_and_exits_one() {
    let path = write("cli-check-bad", "{\"branches\": []}");
    let outcome = run_cli(&["check", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        stdout_text(&outcome),
        printed(&["invalid exploration", "defect missing field question",])
    );
}

#[test]
fn map_prints_a_chain_of_five_thousand_in_full() {
    let branches: Vec<String> = (0..5000)
        .map(|i| {
            let parent = if i == 0 {
                None
            } else {
                Some(format!("b{}", i - 1))
            };
            common::branch_json(
                &format!("b{i}"),
                parent.as_deref(),
                &format!("p{i}"),
                "open",
            )
        })
        .collect();
    let path = write("cli-chain", &common::exploration_json("q", &branches));
    let outcome = run_cli(&["map", &path]);
    assert_eq!(outcome.code, 0);
    let text = stdout_text(&outcome);
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 5002);
    assert_eq!(lines[0], "question, q");
    assert_eq!(lines[1], "b0 (open) p0");
    assert_eq!(
        lines[lines.len() - 2],
        format!("{}b4999 (open) p4999", "  ".repeat(4999))
    );
    assert_eq!(
        lines[lines.len() - 1],
        "branches 5000, open 5000, taken 0, abandoned 0"
    );
}

#[test]
fn a_missing_file_exits_one_with_the_read_defect() {
    let dir = temp_dir("cli-missing");
    let path = dir.join("missing.json").to_string_lossy().into_owned();
    let outcome = run_cli(&["map", &path]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        stdout_text(&outcome),
        printed(&["invalid exploration", "defect file cannot be read",])
    );
}
