//! Tests for MY-39, the rosters command on the CLI.
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
//! binary there refuses rosters as no command at all
//! and exits with a usage error at status two, where the
//! golden pin expects status zero and the malformed pin
//! expects status one with the lineage command's own
//! output. The naming pin fails there in two halves: its
//! header and string halves fail by assertion on the
//! printed text, because neither the choices header nor
//! the choices string names rosters on that tree, and
//! its command half, the help listing line "print the
//! rosters of a chain of atlases", fails there too,
//! because no rosters line stands in the listing. The
//! naming pin is born asserting the twentieth command
//! last, as pin 16 of Round 18 was born asserting the
//! nineteenth, so the genus the Round 15 sweep names is
//! designed against in this round's own file from its
//! first standing.
//!
//! Round 20 states this file's split in the open.
//! One pin here is restated in this round, the naming
//! pin, whose choices header and choices string
//! assertions now name fellows last, as the named
//! mechanical consequence of the twenty-first command
//! joining the surface. On the tree before this slice
//! the file stands at two passed and one failed. The
//! failure is the naming pin,
//! the_help_and_choices_name_rosters, failing by
//! assertion on its restated text against the base
//! surface, because neither the choices header nor
//! the choices string names fellows on that tree.
//! The golden pin and the malformed pin pass there
//! by design, as they hold standing surface the round
//! does not move.

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
fn the_rosters_command_prints_the_golden_text_and_exits_zero() {
    let path = repo_root()
        .join("examples")
        .join("constructed-departures-lineage.json");
    let outcome = run_cli(&["rosters", &path.to_string_lossy()]);
    assert_eq!(outcome.code, 0);
    assert_eq!(outcome.stderr, Vec::<String>::new());
    let golden_path = repo_root()
        .join("examples")
        .join("constructed-departures-rosters.txt");
    let golden = std::fs::read_to_string(&golden_path).expect("the rosters golden reads");
    assert_eq!(stdout_text(&outcome), golden);
}

#[test]
fn a_malformed_file_prints_what_the_lineage_command_prints() {
    let member = jobj(&[("entries", jarr(&[]))]);
    let text = lineage_text(&[atlas_text("One", &["only"]), member]);
    let file = write_temp("r19cli-badmember", "lineage.json", &text);
    let rosters = run_cli(&["rosters", &file.to_string_lossy()]);
    let lineage = run_cli(&["lineage", &file.to_string_lossy()]);
    assert_eq!(rosters.code, 1);
    assert_eq!(rosters.code, lineage.code);
    assert_eq!(rosters.stdout, lineage.stdout);
    assert_eq!(rosters.stderr, lineage.stderr);
    assert_eq!(rosters.stdout[0], "invalid lineage");
}

#[test]
fn the_help_and_choices_name_rosters() {
    let help = run_cli(&["--help"]);
    assert_eq!(help.code, 0);
    let listing = stdout_text(&help);
    assert!(
        listing.contains("print the rosters of a chain of atlases"),
        "the help listing names the rosters command: {listing}"
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
