//! Tests for MY-12, the changelog command on the CLI.
//!
//! The command loads both atlas files whole through the
//! MY-7 loader before printing anything, and a malformed
//! pair prints the defects of both files under the word
//! naming each file, the earlier file's defects first.

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

fn sound_pair(tag: &str) -> (String, String) {
    let earlier = write_temp(tag, "earlier.json", &atlas_text("One", "only", "Q"));
    let later = write_temp(tag, "later.json", &atlas_text("Two", "only", "Q"));
    (
        earlier.to_string_lossy().into_owned(),
        later.to_string_lossy().into_owned(),
    )
}

#[test]
fn sound_pair_prints_the_changelog_and_exits_zero() {
    let (earlier, later) = sound_pair("r5cli-sound");
    let outcome = run_cli(&["changelog", &earlier, &later]);
    assert_eq!(outcome.code, 0);
    assert_eq!(outcome.stderr, Vec::<String>::new());
    assert_eq!(
        stdout_text(&outcome),
        "earlier atlas, One\nlater atlas, Two\nentry only, unchanged\n\
         changelog counts, entries arrived 0, departed 0, matched 1, branches changed 0\n"
    );
}

#[test]
fn malformed_earlier_file_prints_its_defects_under_earlier() {
    let bad_text = atlas_text("One", "only", "Q").replace("\"title\"", "\"heading\"");
    let defects = atlas_defects_of(&bad_text);
    assert!(!defects.is_empty());
    let earlier = write_temp("r5cli-badearlier", "earlier.json", &bad_text);
    let (_, later) = sound_pair("r5cli-badearlier-later");
    let outcome = run_cli(&["changelog", &earlier.to_string_lossy(), &later]);
    assert_eq!(outcome.code, 1);
    let mut expected = vec!["invalid changelog".to_string()];
    expected.extend(
        defects
            .iter()
            .map(|defect| format!("defect earlier, {defect}")),
    );
    assert_eq!(outcome.stdout, expected);
}

#[test]
fn malformed_later_file_prints_its_defects_under_later() {
    let (earlier, _) = sound_pair("r5cli-badlater-earlier");
    let bad_text = jobj(&[("title", jstr("Two")), ("entries", jstr("nope"))]);
    let defects = atlas_defects_of(&bad_text);
    assert!(!defects.is_empty());
    let later = write_temp("r5cli-badlater", "later.json", &bad_text);
    let outcome = run_cli(&["changelog", &earlier, &later.to_string_lossy()]);
    assert_eq!(outcome.code, 1);
    let mut expected = vec!["invalid changelog".to_string()];
    expected.extend(
        defects
            .iter()
            .map(|defect| format!("defect later, {defect}")),
    );
    assert_eq!(outcome.stdout, expected);
}

#[test]
fn both_files_malformed_prints_earlier_defects_before_later_defects() {
    let bad_earlier = atlas_text("One", "only", "Q").replace("\"title\"", "\"heading\"");
    let bad_later = jobj(&[("title", jstr("Two")), ("entries", jstr("nope"))]);
    let earlier_defects = atlas_defects_of(&bad_earlier);
    let later_defects = atlas_defects_of(&bad_later);
    let earlier = write_temp("r5cli-both", "earlier.json", &bad_earlier);
    let later = write_temp("r5cli-both", "later.json", &bad_later);
    let outcome = run_cli(&[
        "changelog",
        &earlier.to_string_lossy(),
        &later.to_string_lossy(),
    ]);
    assert_eq!(outcome.code, 1);
    let mut expected = vec!["invalid changelog".to_string()];
    expected.extend(
        earlier_defects
            .iter()
            .map(|defect| format!("defect earlier, {defect}")),
    );
    expected.extend(
        later_defects
            .iter()
            .map(|defect| format!("defect later, {defect}")),
    );
    assert_eq!(outcome.stdout, expected);
}

#[test]
fn later_file_loads_even_when_the_earlier_file_is_unreadable() {
    let (_, later) = sound_pair("r5cli-unreadable");
    let missing = temp_dir("r5cli-unreadable-missing").join("gone.json");
    let outcome = run_cli(&["changelog", &missing.to_string_lossy(), &later]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        strings(&["invalid changelog", "defect earlier, file cannot be read"])
    );
}

#[test]
fn unreadable_later_file_names_the_later_file() {
    let (earlier, _) = sound_pair("r5cli-unreadable-later");
    let missing = temp_dir("r5cli-unreadable-later-missing").join("gone.json");
    let outcome = run_cli(&["changelog", &earlier, &missing.to_string_lossy()]);
    assert_eq!(outcome.code, 1);
    assert_eq!(
        outcome.stdout,
        strings(&["invalid changelog", "defect later, file cannot be read"])
    );
}

#[test]
fn malformed_pair_prints_no_reading_line() {
    let bad_text = atlas_text("One", "only", "Q").replace("\"title\"", "\"heading\"");
    let earlier = write_temp("r5cli-noreading", "earlier.json", &bad_text);
    let (_, later) = sound_pair("r5cli-noreading-later");
    let outcome = run_cli(&["changelog", &earlier.to_string_lossy(), &later]);
    assert_eq!(outcome.stdout[0], "invalid changelog");
    assert!(outcome
        .stdout
        .iter()
        .skip(1)
        .all(|line| line.starts_with("defect earlier, ")));
    assert!(!stdout_text(&outcome).contains("earlier atlas,"));
}

#[test]
fn changelog_help_names_the_two_positional_arguments() {
    let outcome = run_cli(&["changelog", "--help"]);
    assert_eq!(outcome.code, 0);
    let text = stdout_text(&outcome);
    assert!(text.contains("usage: myriad changelog [-h] earlier later"));
    assert!(text.contains("earlier"));
    assert!(text.contains("later"));
}

#[test]
fn changelog_with_no_file_names_both_arguments_missing() {
    let outcome = run_cli(&["changelog"]);
    assert_eq!(outcome.code, 2);
    assert!(outcome
        .stderr
        .join("\n")
        .contains("the following arguments are required: earlier, later"));
}

#[test]
fn changelog_with_one_file_names_the_later_argument_missing() {
    let (earlier, _) = sound_pair("r5cli-onefile");
    let outcome = run_cli(&["changelog", &earlier]);
    assert_eq!(outcome.code, 2);
    assert!(outcome
        .stderr
        .join("\n")
        .contains("the following arguments are required: later"));
}

#[test]
fn third_file_is_an_unrecognized_argument() {
    let (earlier, later) = sound_pair("r5cli-third");
    let outcome = run_cli(&["changelog", &earlier, &later, &earlier]);
    assert_eq!(outcome.code, 2);
    assert!(outcome
        .stderr
        .join("\n")
        .contains("unrecognized arguments:"));
}
