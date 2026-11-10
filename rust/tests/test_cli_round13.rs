//! Tests for MY-28, the matched command on the CLI.
//!
//! The command loads the lineage file whole through the
//! MY-13 loader before printing anything, and a malformed
//! lineage prints exactly what the lineage command prints
//! for that file, the line invalid lineage and then its
//! defects, because the file is a lineage and the defects
//! are the lineage loader's own.
//!
//! The discrimination split of these pins stands as
//! follows. Pins 11 and 12 fail on the tree before this
//! slice by execution, because the binary there refuses
//! matched as no command at all and exits with a usage
//! error at status two, where pin 11 expects status zero
//! and pin 12 expects status one with the lineage
//! command's own output. Pin 13 fails there by assertion
//! on the printed text, because neither the help listing
//! nor the choices strings name matched on that tree.

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
fn the_matched_command_prints_the_golden_text_and_exits_zero() {
    let path = repo_root()
        .join("examples")
        .join("winter-spring-summer-lineage.json");
    let outcome = run_cli(&["matched", &path.to_string_lossy()]);
    assert_eq!(outcome.code, 0);
    assert_eq!(outcome.stderr, Vec::<String>::new());
    let golden_path = repo_root()
        .join("examples")
        .join("winter-spring-summer-matched.txt");
    let golden = std::fs::read_to_string(&golden_path).expect("the matched golden reads");
    assert_eq!(stdout_text(&outcome), golden);
}

#[test]
fn a_malformed_file_prints_what_the_lineage_command_prints() {
    let member = jobj(&[("entries", jarr(&[]))]);
    let text = lineage_text(&[atlas_text("One", &["only"]), member]);
    let file = write_temp("r13cli-badmember", "lineage.json", &text);
    let matched = run_cli(&["matched", &file.to_string_lossy()]);
    let lineage = run_cli(&["lineage", &file.to_string_lossy()]);
    assert_eq!(matched.code, 1);
    assert_eq!(matched.code, lineage.code);
    assert_eq!(matched.stdout, lineage.stdout);
    assert_eq!(matched.stderr, lineage.stderr);
    assert_eq!(matched.stdout[0], "invalid lineage");
}

#[test]
fn the_help_and_choices_name_matched() {
    let help = run_cli(&["--help"]);
    assert_eq!(help.code, 0);
    let listing = stdout_text(&help);
    assert!(
        listing.contains("print the matched entries of a chain of atlases"),
        "the help listing names the matched command: {listing}"
    );
    assert!(
        listing.contains("departures,matched,runs}"),
        "the choices header names runs last: {listing}"
    );
    let refused = run_cli(&["bogus"]);
    assert_eq!(refused.code, 2);
    let error = refused.stderr.join("\n");
    assert!(
        error.contains("'departures', 'matched', 'runs'"),
        "the choices string names runs last: {error}"
    );
}
