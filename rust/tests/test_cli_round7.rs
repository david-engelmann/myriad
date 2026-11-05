//! Tests for MY-16, the returns command on the CLI.
//!
//! The command loads the lineage file whole through the
//! MY-13 loader before printing anything, and a malformed
//! lineage prints exactly what the lineage command prints
//! for that file, the line invalid lineage and then its
//! defects, because the file is a lineage and the defects
//! are the lineage loader's own.

mod common;

use common::*;

fn atlas_text(title: &str, labels: &[&str]) -> String {
    let entries: Vec<String> = labels
        .iter()
        .map(|label| {
            jobj(&[
                ("label", jstr(label)),
                ("exploration", exploration_json("Q", &[])),
            ])
        })
        .collect();
    jobj(&[("title", jstr(title)), ("entries", jarr(&entries))])
}

fn lineage_text(atlases: &[String]) -> String {
    jobj(&[("atlases", jarr(atlases))])
}

fn sound_file(tag: &str) -> String {
    let text = lineage_text(&[atlas_text("One", &["only"]), atlas_text("Two", &["only"])]);
    write_temp(tag, "lineage.json", &text)
        .to_string_lossy()
        .into_owned()
}

#[test]
fn sound_file_prints_the_returns_and_exits_zero() {
    let file = sound_file("r7cli-sound");
    let outcome = run_cli(&["returns", &file]);
    assert_eq!(outcome.code, 0);
    assert_eq!(outcome.stderr, Vec::<String>::new());
    assert_eq!(
        stdout_text(&outcome),
        "returns counts, returns 0, entries returned 0\n"
    );
}

#[test]
fn a_chain_with_a_return_prints_the_return_line() {
    let text = lineage_text(&[
        atlas_text("One", &["kept", "gone"]),
        atlas_text("Two", &["kept"]),
        atlas_text("Three", &["kept", "gone"]),
    ]);
    let file = write_temp("r7cli-return", "lineage.json", &text);
    let outcome = run_cli(&["returns", &file.to_string_lossy()]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        "entry gone, returned at step 2, absent 1 atlases\n\
         returns counts, returns 1, entries returned 1\n"
    );
}

#[test]
fn malformed_lineage_prints_invalid_lineage_and_its_defects() {
    let member = jobj(&[("entries", jarr(&[]))]);
    let text = lineage_text(&[atlas_text("One", &["only"]), member]);
    let defects = {
        let (found, defects) = myriad::atlas::load_lineage_text(&text);
        assert!(found.is_none());
        defects
    };
    let file = write_temp("r7cli-badmember", "lineage.json", &text);
    let outcome = run_cli(&["returns", &file.to_string_lossy()]);
    assert_eq!(outcome.code, 1);
    let mut expected = vec!["invalid lineage".to_string()];
    expected.extend(defects.iter().map(|defect| format!("defect {defect}")));
    assert_eq!(outcome.stdout, expected);
    assert_eq!(outcome.stdout[1], "defect atlas 1, missing field title");
}

#[test]
fn a_malformed_file_prints_what_the_lineage_command_prints() {
    let member = jobj(&[("entries", jarr(&[]))]);
    let text = lineage_text(&[atlas_text("One", &["only"]), member]);
    let file = write_temp("r7cli-shared", "lineage.json", &text);
    let returns = run_cli(&["returns", &file.to_string_lossy()]);
    let lineage = run_cli(&["lineage", &file.to_string_lossy()]);
    assert_eq!(returns.code, lineage.code);
    assert_eq!(returns.stdout, lineage.stdout);
    assert_eq!(returns.stderr, lineage.stderr);
}

#[test]
fn a_chain_of_one_prints_the_fewer_than_two_defect() {
    let text = lineage_text(&[atlas_text("One", &["only"])]);
    let file = write_temp("r7cli-one", "lineage.json", &text);
    let outcome = run_cli(&["returns", &file.to_string_lossy()]);
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
    let missing = temp_dir("r7cli-unreadable").join("gone.json");
    let outcome = run_cli(&["returns", &missing.to_string_lossy()]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        strings(&["invalid lineage", "defect file cannot be read"])
    );
}

#[test]
fn malformed_lineage_prints_no_reading_line() {
    let text = lineage_text(&[atlas_text("One", &["only"])]);
    let file = write_temp("r7cli-noreading", "lineage.json", &text);
    let outcome = run_cli(&["returns", &file.to_string_lossy()]);
    assert_eq!(outcome.stdout[0], "invalid lineage");
    assert!(outcome
        .stdout
        .iter()
        .skip(1)
        .all(|line| line.starts_with("defect ")));
    assert!(!stdout_text(&outcome).contains("returns counts"));
}

#[test]
fn returns_help_names_the_file_argument() {
    let outcome = run_cli(&["returns", "--help"]);
    assert_eq!(outcome.code, 0);
    let text = stdout_text(&outcome);
    assert!(text.contains("usage: myriad returns [-h] file"));
    assert!(text.contains("file"));
}

#[test]
fn returns_with_no_file_names_the_file_argument_missing() {
    let outcome = run_cli(&["returns"]);
    assert_eq!(outcome.code, 2);
    assert!(outcome
        .stderr
        .join("\n")
        .contains("the following arguments are required: file"));
}

#[test]
fn second_file_is_an_unrecognized_argument() {
    let file = sound_file("r7cli-second");
    let outcome = run_cli(&["returns", &file, &file]);
    assert_eq!(outcome.code, 2);
    assert!(outcome
        .stderr
        .join("\n")
        .contains("unrecognized arguments:"));
}

#[test]
fn the_command_writes_nothing() {
    let dir = temp_dir("r7cli-writes");
    let text = lineage_text(&[
        atlas_text("One", &["kept", "gone"]),
        atlas_text("Two", &["kept"]),
        atlas_text("Three", &["kept", "gone"]),
    ]);
    let file = dir.join("lineage.json");
    std::fs::write(&file, &text).expect("a temporary file");
    let outcome = run_cli(&["returns", &file.to_string_lossy()]);
    assert_eq!(outcome.code, 0);
    let names: Vec<String> = std::fs::read_dir(&dir)
        .expect("the directory reads")
        .map(|entry| {
            entry
                .expect("a directory entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(names, vec!["lineage.json".to_string()]);
}
