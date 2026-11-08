//! Setup pins, translated from tests/test_setup.py. Each test
//! can fail for a real reason against a repo that lost a setup
//! file or let its status drift from its contract.
//!
//! Two pins stand at the end that the Python corpus does not
//! carry. They pin the Rust port status line in the README
//! and the cargo steps in the CI gate, the additions PORT-5
//! and PORT-7 ask the port to make.

mod common;

use std::fs;

use common::repo_root;

fn read(relative: &str) -> String {
    fs::read_to_string(repo_root().join(relative)).expect("a standing file")
}

fn carries_line(text: &str, expected: &str) -> bool {
    text.lines().any(|line| line == expected)
}

#[test]
fn agents_carries_the_canonical_marker() {
    let text = read("AGENTS.md");
    assert!(text.contains("<!-- repo annex below this line -->"));
    assert!(text.contains("39 repo storefront perfection program"));
}

#[test]
fn readme_states_the_closed_round_count() {
    let text = read("README.md");
    assert!(text.contains("Closed perfection rounds, 10 of 100"));
}

#[test]
fn contract_carries_the_setup_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["SETUP-1", "SETUP-2", "SETUP-3", "SETUP-4", "MY-1"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn contract_carries_the_round_five_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["MY-11", "MY-12"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn contract_carries_the_round_six_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["MY-13", "MY-14"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn contract_carries_the_round_seven_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["MY-15", "MY-16"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn contract_carries_the_round_eight_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["MY-17", "MY-18"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn contract_carries_the_round_nine_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["MY-19", "MY-20"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn contract_carries_the_round_ten_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["MY-21", "MY-22", "FIX-MY-13"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn contract_carries_the_round_eleven_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["MY-23", "MY-24", "FIX-MY-14"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn readme_reports_the_contract_status() {
    let text = read("README.md");
    let expected = "- Contract, SETUP-1 to SETUP-4 and MY-1 to MY-4 met. \
        MY-1 to MY-4 stand in full form in CONTRACT.md with the Round 1 \
        repair amendments, and Round 1 closed on the merged slice 2 after \
        three reviews, a repair pass, and an approving re-review.";
    assert!(carries_line(&text, expected));
}

#[test]
fn readme_reports_the_round_two_contract_status() {
    let text = read("README.md");
    let expected = "- Contract, MY-5 and MY-6 met. \
        They stand in full form in CONTRACT.md with the Round 2 \
        repair amendments, and Round 2 closed on the merged slice 3 after \
        three reviews, two repair passes, and two re-reviews, the second \
        approving with no findings.";
    assert!(carries_line(&text, expected));
}

#[test]
fn readme_reports_the_round_three_contract_status() {
    let text = read("README.md");
    let expected = "- Contract, MY-7 and MY-8 met. They stand in full form in \
        CONTRACT.md with the Round 3 repair amendments, and Round 3 \
        closed on the merged slice 4 after three reviews, a repair \
        pass, and an approving re-review.";
    assert!(carries_line(&text, expected));
}

#[test]
fn readme_reports_the_slice_four_standing() {
    let text = read("README.md");
    let expected = "- Slice 4 has landed. It carries the atlas \
        loader that loads a file of labelled explorations only \
        whole and names the entry standing behind every defect, \
        and the atlas rendering that walks the entries in file \
        order with each section holding the map and the claim \
        lines of its exploration and the closing counts read over \
        the union of every entry, with a synthetic example atlas \
        of three explorations of differing shapes and its golden \
        rendering under examples.";
    assert!(carries_line(&text, expected));
    assert!(!text.contains("Slice 4 is built and in review"));
}

#[test]
fn readme_reports_the_round_four_contract_status() {
    let text = read("README.md");
    let expected = "- Contract, MY-9 and MY-10 met. They stand in full form in \
        CONTRACT.md, and Round 4 closed on the merged slice 5 after \
        three reviews approving with no findings.";
    assert!(carries_line(&text, expected));
}

#[test]
fn readme_reports_the_slice_five_standing() {
    let text = read("README.md");
    let expected = "- Slice 5 has landed. It carries the atlas \
        index that reads one atlas as a table of contents, one \
        line for each entry carrying its label, its question, and \
        the counts of its exploration alone, standing between the \
        title line and the atlas counts line of the atlas \
        rendering, with the golden index of the winter atlas \
        under examples.";
    assert!(carries_line(&text, expected));
    // The count stood at one, not zero, as a mechanical
    // consequence of the slice 6 status line above it in
    // the README, which read built and in review while
    // the round 5 slice stood unmerged, until the Round 5
    // closure landed slice 6 and the count returned to
    // zero. It stands at one again as the named mechanical
    // consequence of the slice 7 status line, which read
    // built and in review while Round 6 was built, until
    // the Round 6 closure landed slice 7 and the count
    // returned to zero. It stands at one again as the
    // named mechanical consequence of the slice 8 status
    // line, which read built and in review while Round 7
    // was built, until the Round 7 closure landed slice 8
    // and the count returned to zero. It stands at zero
    // again as the named mechanical consequence of the
    // slice 9 status line, which read built and in review
    // while Round 8 was built, until the Round 8 closure
    // landed slice 9 and the count returned to zero. It
    // stands at zero again as the named mechanical
    // consequence of the slice 10 status line, which
    // read built and in review while Round 9 was built,
    // until the Round 9 closure landed slice 10 and the
    // count returned to zero. It stands at one again as
    // the named mechanical consequence of the slice 11
    // status line, which reads built and in review while
    // Round 10 is built, until the Round 10 closure
    // lands slice 11 and the count returns to zero. It
    // stands at zero again as the named mechanical
    // consequence of the slice 11 status line, which
    // read built and in review while Round 10 was built,
    // until the Round 10 closure landed slice 11 and the
    // count returned to zero. It stands at one again as
    // the named mechanical consequence of the slice 12
    // status line, which reads built and in review while
    // Round 11 is built, until the Round 11 closure
    // lands slice 12 and the count returns to zero.
    assert_eq!(text.matches("built and in review").count(), 1);
    // The count stands at twelve, not eleven, as a mechanical
    // consequence of the Round 10 closure. The slice 11 line
    // now also reads has landed, beside the port status
    // line below, where it read built and in review while
    // the slice stood unmerged.
    assert_eq!(text.matches("has landed").count(), 12);
}

#[test]
fn readme_reports_the_round_six_status() {
    let text = read("README.md");
    let standing = "- Contract, MY-13 and MY-14 met. They stand in full form in \
        CONTRACT.md, and Round 6 closed on the merged slice 7 after \
        three reviews approving with no findings.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 7 has landed. It carries the atlas \
        lineage that reads a chain of atlases as the changelogs of its \
        steps composed and closes a longer chain with the summed counts \
        of its steps, with the winter atlas, its spring successor, and \
        the summer successor and their golden lineage under examples.";
    assert!(carries_line(&text, expected));
    assert!(text.contains("Closed perfection rounds, 10 of 100"));
}

#[test]
fn readme_reports_the_round_five_standing() {
    let text = read("README.md");
    let standing = "- Contract, MY-11 and MY-12 met. They stand in full form in \
        CONTRACT.md, and Round 5 closed on the merged slice 6 after \
        three reviews, a repair pass, and an approving re-review.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 6 has landed. It carries the atlas \
        changelog that reads two atlases against each other and \
        names the entries and branches that arrived, departed, or \
        changed between them, with the winter atlas and its \
        stated successor and their golden changelog under \
        examples.";
    assert!(carries_line(&text, expected));
}

#[test]
fn readme_reports_the_round_seven_status() {
    let text = read("README.md");
    let standing = "- Contract, MY-15 and MY-16 met. They stand in full \
        form in CONTRACT.md, and Round 7 closed on the merged \
        slice 8 after three reviews approving with no findings.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 8 has landed. It carries the returns \
        reading that joins the presences of the entries of a chain \
        across the whole chain and names each entry that stands again \
        after an absence, with the golden returns of the winter, \
        spring, and summer chain under examples.";
    assert!(carries_line(&text, expected));
}

#[test]
fn readme_reports_the_round_eight_status() {
    let text = read("README.md");
    let standing = "- Contract, MY-17 and MY-18 met. They stand in full form in \
        CONTRACT.md, and Round 8 closed on the merged slice 9 after \
        three reviews, a repair pass, and an approving re-review.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 9 has landed. It carries the standing \
        reading that reads the presence of every entry of a chain in \
        every atlas of the chain and names the first atlas each entry \
        stands in, the last atlas it stands in, the count of atlases \
        it stands in, and whether it stands at the close, with the \
        golden standing of the winter, spring, and summer chain \
        under examples.";
    assert!(carries_line(&text, expected));
    assert!(text.contains("Closed perfection rounds, 10 of 100"));
}

#[test]
fn readme_reports_the_round_nine_status() {
    let text = read("README.md");
    let standing = "- Contract, MY-19 and MY-20 met. They stand in full form in \
        CONTRACT.md, and Round 9 closed on the merged slice 10 after \
        three reviews, with the red team's one library surface \
        finding carried to Round 10 as a fencing component.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 10 has landed. It carries the absence \
        reading that reads the presence of every entry of a chain in \
        every atlas of the chain and names each run of atlases an entry \
        stands absent from after it has stood, the inside runs and the \
        closing run told apart, with the golden absence of the winter, \
        spring, and summer chain under examples, and it brings the \
        README to the fleet README standard, with an install and a \
        quickstart that run and the standing reading of the example \
        chain standing as the captured output of its first screen.";
    assert!(carries_line(&text, expected));
    assert!(text.contains("Closed perfection rounds, 10 of 100"));
}

#[test]
fn readme_reports_the_round_ten_status() {
    let text = read("README.md");
    let standing = "- Contract, MY-21 and MY-22 met. They stand in full form in \
        CONTRACT.md, and Round 10 closed on the merged slice 11 after \
        three reviews, with the red team's observation on the non \
        chain renderings carried to a later round as a fencing \
        candidate.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 11 has landed. It carries the chain \
        fencing closure of FIX-MY-13 that refuses a hand built lineage \
        whose entry label holds a line break in the returns, standing, \
        and absence readings, the investigation log example of the three \
        loggings of one branching investigation with its golden standing, \
        returns, and absence under examples, and the investigation log \
        framing of the first screen and this status in the scoped terms \
        of MY-22.";
    assert!(carries_line(&text, expected));
    assert!(text.contains("Closed perfection rounds, 10 of 100"));
}

#[test]
fn readme_reports_the_round_eleven_slice() {
    let text = read("README.md");
    let standing = "- Contract, MY-23 and MY-24 now stand in full form in \
        CONTRACT.md, and their standing is not claimed here, as \
        Round 11 has not closed.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 12 is built and in review. It carries the rendering \
        fencing closure of FIX-MY-14 that refuses a hand built value \
        whose printed field holds a line break in the map, claims, \
        census, atlas, index, and changelog renderings and in the \
        lineage through them, and the arrivals reading that reads the \
        presence of every entry of a chain in the one presence pass \
        and names the entries arriving at each atlas in the lines of \
        their standing, with the golden arrivals of the winter, spring, \
        and summer chain and of the radio investigation chain under \
        examples.";
    assert!(carries_line(&text, expected));
    assert!(text.contains("Closed perfection rounds, 10 of 100"));
}

#[test]
fn the_one_line_description_stands_identical_in_the_lede_and_the_metadata() {
    let description = "Myriad is a Rust library and command line tool that \
        reads logs of branching investigations of a question, single \
        investigations, sets of them, and chains of them logged over \
        time, and prints readings over them.";
    let readme = read("README.md");
    assert!(carries_line(&readme, description));
    let manifest = read("Cargo.toml");
    let field = manifest
        .lines()
        .find(|line| line.starts_with("description = "))
        .expect("the metadata carries a description field");
    let carried = field
        .trim_start_matches("description = \"")
        .trim_end_matches('"');
    assert_eq!(carried, description);
}

#[test]
fn readme_states_the_investigation_log_scoping() {
    let text = read("README.md");
    assert!(text.contains("An exploration is the log of one branching investigation of a question"));
    assert!(text.contains("An atlas is a set of such logs standing together"));
    assert!(text.contains("a lineage is the same ground logged again at successive times"));
}

#[test]
fn ci_runs_the_same_gate_as_the_local_gate() {
    let text = read(".github/workflows/ci.yml");
    let live: String = text
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .map(|line| line.split('#').next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(live.contains("cargo run --bin codepoint_scan"));
    assert!(!live.contains("pytest"));
    assert!(!live.contains("scripts/codepoint_scan.py"));
}

#[test]
fn readme_reports_the_rust_port_status() {
    let text = read("README.md");
    let line = "- The Rust port has landed. It carries \
        the same behavior as the Python package it replaces, checked \
        against the same goldens, and the Python tree stands removed \
        from the repo.";
    assert!(carries_line(&text, line));
}

#[test]
fn ci_runs_the_cargo_gate() {
    let text = read(".github/workflows/ci.yml");
    let live: String = text
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .map(|line| line.split('#').next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(live.contains("cargo test"));
    assert!(live.contains("cargo clippy --all-targets -- -D warnings"));
    assert!(live.contains("cargo fmt --check"));
}

// The manifest pin. The integration tests live beside the
// library under rust/tests, outside the root level tests
// directory cargo discovers, so each one is named in the
// manifest with its path. This pin fails the moment a test
// file stands in the tree without a manifest entry naming
// it, or an entry names a file that does not stand, so no
// test file can stand silently unrun.

#[test]
fn every_rust_test_file_is_named_in_the_cargo_manifest() {
    let manifest = read("Cargo.toml");
    let mut named: Vec<String> = Vec::new();
    let mut in_test = false;
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed == "[[test]]" {
            in_test = true;
            continue;
        }
        if trimmed.starts_with('[') {
            in_test = false;
            continue;
        }
        if in_test && trimmed.starts_with("name = ") {
            named.push(
                trimmed
                    .trim_start_matches("name = ")
                    .trim_matches('"')
                    .to_string(),
            );
        }
    }
    let mut on_disk: Vec<String> = Vec::new();
    for entry in fs::read_dir(repo_root().join("rust/tests")).expect("the test tree") {
        let entry = entry.expect("a test tree entry");
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".rs") {
            on_disk.push(name.trim_end_matches(".rs").to_string());
        }
    }
    named.sort();
    on_disk.sort();
    assert_eq!(named, on_disk);
}

// The gate line pin. The README gate line names the
// whole gate from the removal on, the cargo gate and the
// codepoint scan, as the CI gate pin beside it already
// holds the workflow to the same commands.

#[test]
fn readme_names_the_gate_in_its_gate_line() {
    let text = read("README.md");
    let expected = "- Local gate, cargo test, \
        cargo clippy --all-targets -- -D warnings, and cargo fmt --check, \
        and the codepoint scan, cargo run --bin codepoint_scan. \
        CI runs the same commands.";
    assert!(carries_line(&text, expected));
}
