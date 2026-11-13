//! Tests for MY-41, the fellows command on the CLI.
//!
//! The command loads the lineage file whole through the
//! MY-13 loader before printing anything, and a malformed
//! lineage prints exactly what the lineage command prints
//! for that file, the line invalid lineage and then its
//! defects, because the file is a lineage and the defects
//! are the lineage loader's own.
//!
//! The discrimination split of these pins stands as
//! follows. The golden pin and the malformed pin fail on
//! the tree before this slice by execution, because the
//! binary there refuses fellows as no command at all
//! and exits with a usage error at status two, where the
//! golden pin expects status zero and the malformed pin
//! expects status one with the lineage command's own
//! output. The naming pin fails there in two halves: its
//! header and string halves fail by assertion on the
//! printed text, because neither the choices header nor
//! the choices string names fellows on that tree, and
//! its command half, the help listing line "print the
//! fellows of a chain of atlases", fails there too,
//! because no fellows line stands in the listing. The
//! naming pin is born asserting the twenty-first command
//! last, as pin 16 of Round 19 was born asserting the
//! twentieth, so the genus the Round 15 sweep names is
//! designed against in this round's own file from its
//! first standing.

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
fn the_fellows_command_prints_the_golden_text_and_exits_zero() {
    let path = repo_root()
        .join("examples")
        .join("constructed-departures-lineage.json");
    let outcome = run_cli(&["fellows", &path.to_string_lossy()]);
    assert_eq!(outcome.code, 0);
    assert_eq!(outcome.stderr, Vec::<String>::new());
    let golden_path = repo_root()
        .join("examples")
        .join("constructed-departures-fellows.txt");
    let golden = std::fs::read_to_string(&golden_path).expect("the fellows golden reads");
    assert_eq!(stdout_text(&outcome), golden);
}

#[test]
fn a_malformed_file_prints_what_the_lineage_command_prints() {
    let member = jobj(&[("entries", jarr(&[]))]);
    let text = lineage_text(&[atlas_text("One", &["only"]), member]);
    let file = write_temp("r20cli-badmember", "lineage.json", &text);
    let fellows = run_cli(&["fellows", &file.to_string_lossy()]);
    let lineage = run_cli(&["lineage", &file.to_string_lossy()]);
    assert_eq!(fellows.code, 1);
    assert_eq!(fellows.code, lineage.code);
    assert_eq!(fellows.stdout, lineage.stdout);
    assert_eq!(fellows.stderr, lineage.stderr);
    assert_eq!(fellows.stdout[0], "invalid lineage");
}

#[test]
fn the_help_and_choices_name_fellows() {
    let help = run_cli(&["--help"]);
    assert_eq!(help.code, 0);
    let listing = stdout_text(&help);
    assert!(
        listing.contains("print the fellows of a chain of atlases"),
        "the help listing names the fellows command: {listing}"
    );
    assert!(
        listing.contains("standings,company,sets,rosters,fellows}"),
        "the choices header names fellows last: {listing}"
    );
    let refused = run_cli(&["bogus"]);
    assert_eq!(refused.code, 2);
    let error = refused.stderr.join("\n");
    assert!(
        error.contains("'standings', 'company', 'sets', 'rosters', 'fellows'"),
        "the choices string names fellows last: {error}"
    );
}
