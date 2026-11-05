//! Tests for MY-14, the lineage command on the CLI.
//!
//! The command loads the lineage file whole through the
//! MY-13 loader before printing anything, and a malformed
//! lineage prints invalid lineage and then its defects,
//! exactly as the atlas command prints the defects of a
//! malformed atlas.

mod common;

use common::*;

fn atlas_text(title: &str, label: &str, question: &str) -> String {
    jobj(&[
        ("title", jstr(title)),
        (
            "entries",
            jarr(&[jobj(&[
                ("label", jstr(label)),
                ("exploration", exploration_json(question, &[])),
            ])]),
        ),
    ])
}

fn lineage_text(atlases: &[String]) -> String {
    jobj(&[("atlases", jarr(atlases))])
}

fn sound_file(tag: &str) -> String {
    let text = lineage_text(&[
        atlas_text("One", "only", "Q"),
        atlas_text("Two", "only", "Q"),
    ]);
    write_temp(tag, "lineage.json", &text)
        .to_string_lossy()
        .into_owned()
}

#[test]
fn sound_file_prints_the_lineage_and_exits_zero() {
    let file = sound_file("r6cli-sound");
    let outcome = run_cli(&["lineage", &file]);
    assert_eq!(outcome.code, 0);
    assert_eq!(outcome.stderr, Vec::<String>::new());
    assert_eq!(
        stdout_text(&outcome),
        "earlier atlas, One\nlater atlas, Two\nentry only, unchanged\n\
         changelog counts, entries arrived 0, departed 0, matched 1, branches changed 0\n"
    );
}

#[test]
fn a_chain_of_three_prints_the_lineage_counts_line() {
    let text = lineage_text(&[
        atlas_text("One", "only", "Q"),
        atlas_text("Two", "only", "Q"),
        atlas_text("Three", "only", "Q"),
    ]);
    let file = write_temp("r6cli-three", "lineage.json", &text);
    let outcome = run_cli(&["lineage", &file.to_string_lossy()]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        outcome.stdout.last().map(String::as_str),
        Some(
            "lineage counts, steps 2, entries arrived 0, departed 0, matched 2, branches changed 0"
        )
    );
}

#[test]
fn malformed_lineage_prints_invalid_lineage_and_its_defects() {
    let member = jobj(&[("entries", jarr(&[]))]);
    let text = lineage_text(&[atlas_text("One", "only", "Q"), member]);
    let defects = {
        let (found, defects) = myriad::atlas::load_lineage_text(&text);
        assert!(found.is_none());
        defects
    };
    let file = write_temp("r6cli-badmember", "lineage.json", &text);
    let outcome = run_cli(&["lineage", &file.to_string_lossy()]);
    assert_eq!(outcome.code, 1);
    let mut expected = vec!["invalid lineage".to_string()];
    expected.extend(defects.iter().map(|defect| format!("defect {defect}")));
    assert_eq!(outcome.stdout, expected);
    assert_eq!(outcome.stdout[1], "defect atlas 1, missing field title");
}

#[test]
fn a_chain_of_one_prints_the_fewer_than_two_defect() {
    let text = lineage_text(&[atlas_text("One", "only", "Q")]);
    let file = write_temp("r6cli-one", "lineage.json", &text);
    let outcome = run_cli(&["lineage", &file.to_string_lossy()]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        strings(&[
            "invalid lineage",
            "defect field atlases holds fewer than two atlases",
        ])
    );
}

#[test]
fn unreadable_file_names_file_cannot_be_read() {
    let missing = temp_dir("r6cli-unreadable").join("gone.json");
    let outcome = run_cli(&["lineage", &missing.to_string_lossy()]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        strings(&["invalid lineage", "defect file cannot be read"])
    );
}

#[test]
fn malformed_lineage_prints_no_reading_line() {
    let text = lineage_text(&[atlas_text("One", "only", "Q")]);
    let file = write_temp("r6cli-noreading", "lineage.json", &text);
    let outcome = run_cli(&["lineage", &file.to_string_lossy()]);
    assert_eq!(outcome.stdout[0], "invalid lineage");
    assert!(outcome
        .stdout
        .iter()
        .skip(1)
        .all(|line| line.starts_with("defect ")));
    assert!(!stdout_text(&outcome).contains("earlier atlas,"));
}

#[test]
fn lineage_help_names_the_file_argument() {
    let outcome = run_cli(&["lineage", "--help"]);
    assert_eq!(outcome.code, 0);
    let text = stdout_text(&outcome);
    assert!(text.contains("usage: myriad lineage [-h] file"));
    assert!(text.contains("file"));
}

#[test]
fn lineage_with_no_file_names_the_file_argument_missing() {
    let outcome = run_cli(&["lineage"]);
    assert_eq!(outcome.code, 2);
    assert!(outcome
        .stderr
        .join("\n")
        .contains("the following arguments are required: file"));
}

#[test]
fn second_file_is_an_unrecognized_argument() {
    let file = sound_file("r6cli-second");
    let outcome = run_cli(&["lineage", &file, &file]);
    assert_eq!(outcome.code, 2);
    assert!(outcome
        .stderr
        .join("\n")
        .contains("unrecognized arguments:"));
}
