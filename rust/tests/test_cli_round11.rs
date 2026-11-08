//! Tests for MY-24, the arrivals command on the CLI.
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

#[test]
fn sound_file_prints_the_arrivals_and_exits_zero() {
    let text = lineage_text(&[atlas_text("One", &["only"]), atlas_text("Two", &["only"])]);
    let file = write_temp("r11cli-sound", "lineage.json", &text);
    let outcome = run_cli(&["arrivals", &file.to_string_lossy()]);
    assert_eq!(outcome.code, 0);
    assert_eq!(outcome.stderr, Vec::<String>::new());
    assert_eq!(
        stdout_text(&outcome),
        "atlas 1\n\
         entry only, first stood at atlas 1, last stood at atlas 2, \
         stands in 2 atlases, stands at the close\n\
         atlas 2\n\
         arrivals counts, atlas 1 arrivals 1, atlas 2 arrivals 0, \
         entries 1, atlases 2\n"
    );
}

#[test]
fn a_chain_with_a_late_entry_prints_its_arrival() {
    let text = lineage_text(&[
        atlas_text("One", &["kept"]),
        atlas_text("Two", &["kept", "new"]),
        atlas_text("Three", &["kept", "new"]),
    ]);
    let file = write_temp("r11cli-late", "lineage.json", &text);
    let outcome = run_cli(&["arrivals", &file.to_string_lossy()]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        "atlas 1\n\
         entry kept, first stood at atlas 1, last stood at atlas 3, \
         stands in 3 atlases, stands at the close\n\
         atlas 2\n\
         entry new, first stood at atlas 2, last stood at atlas 3, \
         stands in 2 atlases, stands at the close\n\
         atlas 3\n\
         arrivals counts, atlas 1 arrivals 1, atlas 2 arrivals 1, \
         atlas 3 arrivals 0, entries 2, atlases 3\n"
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
    let file = write_temp("r11cli-badmember", "lineage.json", &text);
    let outcome = run_cli(&["arrivals", &file.to_string_lossy()]);
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
    let file = write_temp("r11cli-shared", "lineage.json", &text);
    let arrivals = run_cli(&["arrivals", &file.to_string_lossy()]);
    let lineage = run_cli(&["lineage", &file.to_string_lossy()]);
    assert_eq!(arrivals.code, lineage.code);
    assert_eq!(arrivals.stdout, lineage.stdout);
    assert_eq!(arrivals.stderr, lineage.stderr);
}

#[test]
fn a_chain_of_one_prints_the_fewer_than_two_defect() {
    let text = lineage_text(&[atlas_text("One", &["only"])]);
    let file = write_temp("r11cli-one", "lineage.json", &text);
    let outcome = run_cli(&["arrivals", &file.to_string_lossy()]);
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
    let missing = temp_dir("r11cli-unreadable").join("gone.json");
    let outcome = run_cli(&["arrivals", &missing.to_string_lossy()]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        strings(&["invalid lineage", "defect file cannot be read"])
    );
}

#[test]
fn malformed_lineage_prints_no_reading_line() {
    let text = lineage_text(&[atlas_text("One", &["only"])]);
    let file = write_temp("r11cli-noreading", "lineage.json", &text);
    let outcome = run_cli(&["arrivals", &file.to_string_lossy()]);
    assert_eq!(outcome.stdout[0], "invalid lineage");
    assert!(outcome
        .stdout
        .iter()
        .all(|line| !line.starts_with("atlas ") && !line.starts_with("arrivals counts")));
}

#[test]
fn the_command_writes_nothing() {
    let dir = temp_dir("r11cli-writes");
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
    let outcome = run_cli(&["arrivals", &file.to_string_lossy()]);
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
