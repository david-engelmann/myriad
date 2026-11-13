//! Tests for MY-33, the standings command on the CLI.
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
//! binary there refuses standings as no command at all
//! and exits with a usage error at status two, where the
//! golden pin expects status zero and the malformed pin
//! expects status one with the lineage command's own
//! output. The naming pin fails there in two halves: its
//! header and string halves fail by assertion on the
//! printed text, because neither the choices header nor
//! the choices string names standings on that tree, and
//! its command half, the help listing line "print the
//! standings of a chain of atlases", fails there too,
//! because no standings line stands in the listing. The
//! naming pin is born asserting the seventeenth command
//! last, as pin 14 of Round 15 was born asserting the
//! sixteenth, so the genus the Round 15 sweep names is
//! designed against in this round's own file from its
//! first standing.
//!
//! Round 17 states this file's split in the open.
//! One pin here is restated in this round, the naming
//! pin, whose choices header and choices string
//! assertions now name company last, as the named
//! mechanical consequence of the eighteenth command
//! joining the surface. On the tree before this slice
//! the file stands at two passed and one failed. The
//! failure is the naming pin,
//! the_help_and_choices_name_standings, failing by
//! assertion on its restated text against the base
//! surface, because neither the choices header nor
//! the choices string names company on that tree.
//! The golden pin and the malformed pin pass there
//! by design, as they hold standing surface the round
//! does not move.
//!
//! Round 18 states this file's split in the open.
//! One pin here is restated in this round, the naming
//! pin, whose choices header and choices string
//! assertions now name sets last, as the named
//! mechanical consequence of the nineteenth command
//! joining the surface. On the tree before this slice
//! the file stands at two passed and one failed. The
//! failure is the naming pin,
//! the_help_and_choices_name_standings, failing by
//! assertion on its restated text against the base
//! surface, because neither the choices header nor
//! the choices string names sets on that tree.
//! The golden pin and the malformed pin pass there
//! by design, as they hold standing surface the round
//! does not move.
//!
//! Round 19 states this file's split in the open.
//! One pin here is restated in this round, the naming
//! pin, whose choices header and choices string
//! assertions now name rosters last, as the named
//! mechanical consequence of the twentieth command
//! joining the surface. On the tree before this slice
//! the file stands at two passed and one failed. The
//! failure is the naming pin,
//! the_help_and_choices_name_standings, failing by
//! assertion on its restated text against the base
//! surface, because neither the choices header nor
//! the choices string names rosters on that tree.
//! The golden pin and the malformed pin pass there
//! by design, as they hold standing surface the round
//! does not move.
//!
//! Round 20 states this file's split in the open.
//! One pin here is restated in this round, the naming
//! pin, whose choices header and choices string
//! assertions now name fellows last, as the named
//! mechanical consequence of the twenty-first command
//! joining the surface. On the tree before this slice
//! the file stands at two passed and one failed. The
//! failure is the naming pin,
//! the_help_and_choices_name_standings, failing by
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
fn the_standings_command_prints_the_golden_text_and_exits_zero() {
    let path = repo_root()
        .join("examples")
        .join("constructed-departures-lineage.json");
    let outcome = run_cli(&["standings", &path.to_string_lossy()]);
    assert_eq!(outcome.code, 0);
    assert_eq!(outcome.stderr, Vec::<String>::new());
    let golden_path = repo_root()
        .join("examples")
        .join("constructed-departures-standings.txt");
    let golden = std::fs::read_to_string(&golden_path).expect("the standings golden reads");
    assert_eq!(stdout_text(&outcome), golden);
}

#[test]
fn a_malformed_file_prints_what_the_lineage_command_prints() {
    let member = jobj(&[("entries", jarr(&[]))]);
    let text = lineage_text(&[atlas_text("One", &["only"]), member]);
    let file = write_temp("r16cli-badmember", "lineage.json", &text);
    let standings = run_cli(&["standings", &file.to_string_lossy()]);
    let lineage = run_cli(&["lineage", &file.to_string_lossy()]);
    assert_eq!(standings.code, 1);
    assert_eq!(standings.code, lineage.code);
    assert_eq!(standings.stdout, lineage.stdout);
    assert_eq!(standings.stderr, lineage.stderr);
    assert_eq!(standings.stdout[0], "invalid lineage");
}

#[test]
fn the_help_and_choices_name_standings() {
    let help = run_cli(&["--help"]);
    assert_eq!(help.code, 0);
    let listing = stdout_text(&help);
    assert!(
        listing.contains("print the standings of a chain of atlases"),
        "the help listing names the standings command: {listing}"
    );
    assert!(
        listing.contains("presence,standings,company,sets,rosters,fellows}"),
        "the choices header names fellows last: {listing}"
    );
    let refused = run_cli(&["bogus"]);
    assert_eq!(refused.code, 2);
    let error = refused.stderr.join("\n");
    assert!(
        error.contains("'presence', 'standings', 'company', 'sets', 'rosters', 'fellows'"),
        "the choices string names fellows last: {error}"
    );
}
