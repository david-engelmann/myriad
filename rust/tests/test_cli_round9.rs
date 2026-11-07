//! Tests for MY-20, the absence command on the CLI.
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
fn sound_file_prints_the_absence_and_exits_zero() {
    let file = sound_file("r9cli-sound");
    let outcome = run_cli(&["absence", &file]);
    assert_eq!(outcome.code, 0);
    assert_eq!(outcome.stderr, Vec::<String>::new());
    assert_eq!(
        stdout_text(&outcome),
        "absence counts, absences 0, entries with absences 0, \
         absent at the close 0\n"
    );
}

#[test]
fn a_chain_with_a_departed_entry_prints_its_absence_lines() {
    let text = lineage_text(&[
        atlas_text("One", &["kept", "gone"]),
        atlas_text("Two", &["kept"]),
        atlas_text("Three", &["kept", "gone"]),
    ]);
    let file = write_temp("r9cli-absence", "lineage.json", &text);
    let outcome = run_cli(&["absence", &file.to_string_lossy()]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        "entry gone, absent from atlas 2, for 1 atlases\n\
         absence counts, absences 1, entries with absences 1, \
         absent at the close 0\n"
    );
}

#[test]
fn a_chain_with_an_entry_gone_at_the_close_marks_the_run() {
    let text = lineage_text(&[
        atlas_text("One", &["kept", "gone"]),
        atlas_text("Two", &["kept"]),
    ]);
    let file = write_temp("r9cli-closing", "lineage.json", &text);
    let outcome = run_cli(&["absence", &file.to_string_lossy()]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        "entry gone, absent from atlas 2, for 1 atlases, to the close\n\
         absence counts, absences 1, entries with absences 1, \
         absent at the close 1\n"
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
    let file = write_temp("r9cli-badmember", "lineage.json", &text);
    let outcome = run_cli(&["absence", &file.to_string_lossy()]);
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
    let file = write_temp("r9cli-shared", "lineage.json", &text);
    let absence = run_cli(&["absence", &file.to_string_lossy()]);
    let lineage = run_cli(&["lineage", &file.to_string_lossy()]);
    assert_eq!(absence.code, lineage.code);
    assert_eq!(absence.stdout, lineage.stdout);
    assert_eq!(absence.stderr, lineage.stderr);
}

#[test]
fn a_chain_of_one_prints_the_fewer_than_two_defect() {
    let text = lineage_text(&[atlas_text("One", &["only"])]);
    let file = write_temp("r9cli-one", "lineage.json", &text);
    let outcome = run_cli(&["absence", &file.to_string_lossy()]);
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
    let missing = temp_dir("r9cli-unreadable").join("gone.json");
    let outcome = run_cli(&["absence", &missing.to_string_lossy()]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        strings(&["invalid lineage", "defect file cannot be read"])
    );
}

#[test]
fn malformed_lineage_prints_no_reading_line() {
    let text = lineage_text(&[atlas_text("One", &["only"])]);
    let file = write_temp("r9cli-noreading", "lineage.json", &text);
    let outcome = run_cli(&["absence", &file.to_string_lossy()]);
    assert_eq!(outcome.stdout[0], "invalid lineage");
    assert!(outcome
        .stdout
        .iter()
        .all(|line| !line.starts_with("entry ") && !line.starts_with("absence counts")));
}

#[test]
fn the_command_writes_nothing() {
    let dir = temp_dir("r9cli-writes");
    let text = lineage_text(&[atlas_text("One", &["only"]), atlas_text("Two", &["only"])]);
    let file = dir.join("lineage.json");
    std::fs::write(&file, &text).expect("a temporary file");
    let before: Vec<String> = std::fs::read_dir(&dir)
        .expect("the directory reads")
        .map(|item| {
            item.expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    let outcome = run_cli(&["absence", &file.to_string_lossy()]);
    assert_eq!(outcome.code, 0);
    let after: Vec<String> = std::fs::read_dir(&dir)
        .expect("the directory reads")
        .map(|item| {
            item.expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(before, after);
    assert_eq!(
        std::fs::read_to_string(&file).expect("the file reads back"),
        text
    );
}
