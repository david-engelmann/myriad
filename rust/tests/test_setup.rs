//! Setup pins, translated from tests/test_setup.py. Each test
//! can fail for a real reason against a repo that lost a setup
//! file or let its status drift from its contract.
//!
//! Two pins stand at the end that the Python corpus does not
//! carry. They pin the Rust port status line in the README
//! and the cargo steps in the CI gate, the additions PORT-5
//! and PORT-7 ask the port to make.
//!
//! Round 14 adds three pins to this file and extends a
//! fourth, and their discrimination split stands as
//! follows. The matched Usage pin and the runs assertion
//! of the doc head enumeration pin fail on the tree
//! before this slice by assertion, because no matched
//! section stands in the README's Usage there and the
//! doc head names seven of the eight chain readings
//! there. The round fourteen slice pin and the round
//! fourteen obligations pin fail there by assertion too,
//! because the README carries no round fourteen standing
//! or slice line and the contract carries no MY-29 or
//! MY-30 there. The built and in review count moves to
//! one as the named mechanical consequence of the slice
//! 15 status line. No pin added here passes on that tree
//! by design.
//!
//! Round 15 adds three pins to this file and extends a
//! fourth, and their discrimination split stands as
//! follows. The runs Usage pin and the presence assertion
//! of the doc head enumeration pin fail on the tree
//! before this slice by assertion, because no runs
//! section stands in the README's Usage there and the
//! doc head names eight of the nine chain readings
//! there. The round fifteen status pin and the round
//! fifteen obligations pin fail there by assertion too,
//! because the README carries no round fifteen standing
//! or slice line and the contract carries no MY-31 or
//! MY-32 there. The built and in review count moves to
//! one as the named mechanical consequence of the slice
//! 16 status line. No pin added here passes on that tree
//! by design.
//!
//! Round 16 adds three pins to this file, extends a
//! fourth, and restates a fifth, and their
//! discrimination split stands as follows. The presence
//! Usage pin and the standings assertion of the doc
//! head enumeration pin fail on the tree before this
//! slice by assertion, because no presence section
//! stands in the README's Usage there and the doc head
//! names nine of the ten chain readings there. The
//! round sixteen status pin and the round sixteen
//! obligations pin fail there by assertion too, because
//! the README carries no round sixteen standing or
//! slice line and the contract carries no MY-33 or
//! MY-34 there. The built and in review count moves to
//! one as the named mechanical consequence of the slice
//! 17 status line, restated inside the slice five
//! standing pin whose comment chain names it. No pin
//! added here passes on that tree by design.
//!
//! Round 17 adds three pins to this file, extends a
//! fourth, and restates a fifth, and their
//! discrimination split stands as follows. The standings
//! Usage pin and the company assertion of the doc
//! head enumeration pin fail on the tree before this
//! slice by assertion, because no standings section
//! stands in the README's Usage there and the doc head
//! names ten of the eleven chain readings there. The
//! round seventeen status pin and the round seventeen
//! obligations pin fail there by assertion too, because
//! the README carries no round seventeen standing
//! or slice line and the contract carries no MY-35 or
//! MY-36 there. The built and in review count moves to
//! one as the named mechanical consequence of the slice
//! 18 status line, restated inside the slice five
//! standing pin whose comment chain names it. No pin
//! added here passes on that tree by design.
//!
//! Round 18 adds three pins to this file, extends a
//! fourth, and restates a fifth, and their
//! discrimination split stands as follows. The company
//! Usage pin and the sets assertion of the doc
//! head enumeration pin fail on the tree before this
//! slice by assertion, because no company section
//! stands in the README's Usage there and the doc head
//! names eleven of the twelve chain readings there.
//! The round eighteen status pin and the round
//! eighteen obligations pin fail there by assertion
//! too, because the README carries no round eighteen
//! standing or slice line and the contract carries
//! no MY-37 or MY-38 there. The built and in review
//! count moves to one as the named mechanical
//! consequence of the slice 19 status line, restated
//! inside the slice five standing pin whose comment
//! chain names it. No pin added here passes on that
//! tree by design.
//!
//! Round 19 adds three pins to this file, extends a
//! fourth, and restates a fifth, and their
//! discrimination split stands as follows. The sets
//! Usage pin and the rosters assertion of the doc
//! head enumeration pin fail on the tree before this
//! slice by assertion, because no sets section
//! stands in the README's Usage there and the doc head
//! names twelve of the thirteen chain readings
//! there. The round nineteen status pin and the round
//! nineteen obligations pin fail there by assertion
//! too, because the README carries no round nineteen
//! standing or slice line and the contract carries
//! no MY-39 or MY-40 there. The built and in review
//! count moves to one as the named mechanical
//! consequence of the slice 20 status line, restated
//! inside the slice five standing pin whose comment
//! chain names it. No pin added here passes on that
//! tree by design.
//!
//! Round 20 adds three pins to this file, extends a
//! fourth, and restates a fifth, and their
//! discrimination split stands as follows. The rosters
//! Usage pin and the fellows assertion of the doc
//! head enumeration pin fail on the tree before this
//! slice by assertion, because no rosters section
//! stands in the README's Usage there and the doc head
//! names thirteen of the fourteen chain readings
//! there. The round twenty status pin and the round
//! twenty obligations pin fail there by assertion
//! too, because the README carries no round twenty
//! standing or slice line and the contract carries
//! no MY-41 or MY-42 there. The built and in review
//! count moves to one as the named mechanical
//! consequence of the slice 21 status line, restated
//! inside the slice five standing pin whose comment
//! chain names it. No pin added here passes on that
//! tree by design.
//!
//! Round 13 adds four pins to this file and alters a
//! fifth, and their discrimination split stands as
//! follows. The departures
//! Usage pin and the doc head enumeration pin fail on the
//! tree before this slice by assertion, because no
//! departures section stands in the README's Usage there
//! and the doc head names five of the seven chain readings
//! there. The round thirteen slice pin and the round
//! thirteen obligations pin fail there by assertion too,
//! because the README carries no round thirteen standing
//! or slice line and the contract carries no MY-27 or
//! MY-28 there. The built and in review count moves to one
//! as the named mechanical consequence of the slice 14
//! status line. No pin added here passes on that tree by
//! design.

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
    assert!(text.contains("Closed perfection rounds, 20 of 100"));
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
    // lands slice 12 and the count returns to zero. It
    // stands at zero again as the named mechanical
    // consequence of the slice 12 status line, which
    // read built and in review while Round 11 was built,
    // until the Round 11 closure landed slice 12 and the
    // count returned to zero. It stood at one again as
    // the named mechanical consequence of the slice 13
    // status line, which read built and in review while
    // Round 12 was built, until the Round 12 closure
    // landed slice 13 and the count returned to zero. It
    // stands at one again as the named mechanical
    // consequence of the slice 14 status line, which
    // read built and in review while Round 13 was built,
    // until the Round 13 closure landed slice 14 and the
    // count returned to zero. It stands at one again as
    // the named mechanical consequence of the slice 15
    // status line, which read built and in review while
    // Round 14 was built, until the Round 14 closure
    // landed slice 15 and the count returned to zero.
    // It stands at one again as the named mechanical
    // consequence of the slice 16 status line, which
    // read built and in review while Round 15 was
    // built, until the Round 15 closure landed slice 16
    // and the count returned to zero. It stands at one
    // again as the named mechanical consequence of the
    // slice 17 status line, which read built and in
    // review while Round 16 was built, until the Round
    // 16 closure landed slice 17 and the count returned
    // to zero. It stands at one again as the named
    // mechanical consequence of the slice 18 status
    // line, which read built and in review while
    // Round 17 was built, until the Round 17 closure
    // landed slice 18 and the count returned to zero.
    // It stands at one again as the named mechanical
    // consequence of the slice 19 status line, which
    // read built and in review while Round 18 was
    // built, until the Round 18 closure landed slice 19
    // and the count returned to zero. It stands at one
    // again as the named mechanical consequence of the
    // slice 20 status line, which read built and in
    // review while Round 19 was built, until the Round
    // 19 closure landed slice 20 and the count returned
    // to zero. It stands at one again as the named
    // mechanical consequence of the slice 21 status
    // line, which reads built and in review while
    // Round 20 is built, until the Round 20 closure
    // lands slice 21 and the count returns to zero.
    assert_eq!(text.matches("built and in review").count(), 1);
    // The count stands at twenty one, not twenty, as a mechanical
    // consequence of the Round 19 closure. The slice 20 line
    // now also reads has landed, beside the port status
    // line below, where it read built and in review while
    // the slice stood unmerged.
    assert_eq!(text.matches("has landed").count(), 21);
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
    assert!(text.contains("Closed perfection rounds, 20 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 20 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 20 of 100"));
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
    assert!(text.contains("Closed perfection rounds, 20 of 100"));
}

#[test]
fn readme_reports_the_round_eleven_status() {
    let text = read("README.md");
    let standing = "- Contract, MY-23 and MY-24 met. They stand in full form in \
        CONTRACT.md, and Round 11 closed on the merged slice 12 after \
        three reviews, with the red team's observation on the JsonValue \
        value semantics carried to a later round as a repair \
        candidate.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 12 has landed. It carries the rendering \
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
    assert!(text.contains("Closed perfection rounds, 20 of 100"));
}

#[test]
fn readme_reports_the_round_twelve_status() {
    let text = read("README.md");
    let standing = "- Contract, MY-25 and MY-26 met. They stand in full form in \
        CONTRACT.md, and Round 12 closed on the merged slice 13 after \
        three reviews, with the red team's duplicated label finding \
        repaired under FIX-MY-17.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 13 has landed. It carries the formats \
        amendment of FIX-MY-15 that names the three formats at version 1 \
        in one consolidated section of the contract, the value semantics \
        settlement of FIX-MY-16 that settles the cloning, the equality, \
        and the formatting of a value at the depth the parser admits, \
        the departures reading of MY-26 that names the entries that \
        stand in no later atlas after the atlas they were last seen in, \
        with its goldens under examples, the documentation repairs of \
        the arrivals section, the worked example, the toolchain floor, \
        the annex, and the settled wording, and the generated fixture \
        family, one chain of eight loggings at the stated shapes with \
        the goldens of its five chain readings under examples.";
    assert!(carries_line(&text, expected));
    assert!(text.contains("Closed perfection rounds, 20 of 100"));
}

#[test]
fn readme_reports_the_round_thirteen_status() {
    let text = read("README.md");
    let standing = "- Contract, MY-27 and MY-28 met. They stand in full form in \
        CONTRACT.md, and Round 13 closed on the merged slice 14 \
        after three reviews, with the red team's duplicated label \
        counting finding repaired under FIX-MY-19.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 14 has landed. It carries the \
        matched reading of MY-28 that names the entries standing on \
        both sides of each step of a chain in the lines of their \
        standing, with its goldens under examples, the posture \
        paragraph that records the judgement of the two observations \
        of the Round 12 red team with its two posture pins, the \
        documentation repairs of the departures section of the Usage \
        and the enumeration of the doc head, and the generated \
        fixture family extended with the matched golden and the two \
        measured bands at the stated shapes.";
    assert!(carries_line(&text, expected));
    assert!(text.contains("Closed perfection rounds, 20 of 100"));
}

#[test]
fn readme_reports_the_round_fourteen_status() {
    let text = read("README.md");
    let standing = "- Contract, MY-29 and MY-30 met. They stand in full form in \
        CONTRACT.md, and Round 14 closed on the merged slice 15 \
        after three reviews, with the branch level counting of \
        the changelog repaired under FIX-MY-20.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 15 has landed. It carries the \
        runs reading of MY-30 that names each standing run of a chain, \
        one line for each run of atlases in a row an entry stands in, \
        with its goldens under examples, the changelog branch counting \
        repair of FIX-MY-20 that reads each branch id once at its first \
        standing inside a matched entry, the documentation repairs of \
        the matched section of the Usage and the enumeration of the \
        doc head, and the generated fixture family extended with the \
        runs golden and its relations at the stated shapes.";
    assert!(carries_line(&text, expected));
    assert!(text.contains("Closed perfection rounds, 20 of 100"));
}

#[test]
fn readme_reports_the_round_fifteen_status() {
    let text = read("README.md");
    let standing = "- Contract, MY-31 and MY-32 met. They stand in full form in \
        CONTRACT.md, and Round 15 closed on the merged slice 16 after \
        three reviews, with the third relation restated under the \
        owner's ruling and the restated headers' split paragraphs \
        repaired.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 16 has landed. It carries the \
        presence reading of MY-32 that names the entries standing in \
        each atlas of a chain in the lines of their standing, one \
        section for each atlas, with its goldens under examples, the \
        documentation repairs of the runs section of the Usage and \
        the enumeration of the doc head carried to nine chain \
        readings, and the generated fixture family extended with the \
        presence golden and its relations at the stated shapes.";
    assert!(carries_line(&text, expected));
    assert!(text.contains("Closed perfection rounds, 20 of 100"));
}

#[test]
fn readme_reports_the_round_sixteen_status() {
    let text = read("README.md");
    let standing = "- Contract, MY-33 and MY-34 met. They stand in full form in \
        CONTRACT.md, and Round 16 closed on the merged slice 17 after \
        three reviews, with the three recorded deviations judged \
        faithful reconciliations and the version 2 forms amendment \
        deferred under its named trigger.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 17 has landed. It carries the \
        standings reading of MY-34 that names the atlases each entry \
        of a chain stands in, in one section for each entry under the \
        line of its standing, with its goldens under examples, the \
        documentation repairs of the presence section of the Usage \
        and the enumeration of the doc head carried to ten chain \
        readings, and the generated fixture family extended with the \
        standings golden and its relations at the stated shapes.";
    assert!(carries_line(&text, expected));
    assert!(text.contains("Closed perfection rounds, 20 of 100"));
    // The number FIX-MY-18 stands unspent, as the Round 13
    // section of the contract records and the Round 16
    // ratification holds, the version 2 amendment deferred.
    let contract = read("CONTRACT.md");
    assert!(!contract.contains("### FIX-MY-18"));
}

#[test]
fn readme_reports_the_round_seventeen_status() {
    let text = read("README.md");
    let standing = "- Contract, MY-35 and MY-36 met. They stand in full form in \
        CONTRACT.md, and Round 17 closed on the merged slice 18 after \
        three reviews, with the two recorded deviations judged \
        faithful, the standards Minor repaired in the slice, and the \
        forced clippy gate repair landed in the slice.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 18 has landed. It carries the \
        company reading of MY-36 that names the entries each entry \
        of a chain stands with, in one section for each entry under \
        the line of its standing, with its goldens under examples, \
        the documentation repairs of the standings section of the \
        Usage and the enumeration of the doc head carried to eleven \
        chain readings, and the generated fixture family extended \
        with the company golden and its relations at the stated \
        shapes.";
    assert!(carries_line(&text, expected));
    assert!(text.contains("Closed perfection rounds, 20 of 100"));
    // The number FIX-MY-18 stands unspent, as the Round 13
    // section of the contract records and the Round 17
    // ratification holds, the version 2 amendment deferred.
    let contract = read("CONTRACT.md");
    assert!(!contract.contains("### FIX-MY-18"));
}

#[test]
fn readme_reports_the_round_eighteen_status() {
    let text = read("README.md");
    let standing = "- Contract, MY-37 and MY-38 met. They stand in full form in \
        CONTRACT.md, and Round 18 closed on the merged slice 19 after \
        three reviews, with the spec review clean and no repair \
        required, and the red team's two named evidence voids carried \
        to the Round 19 design warm-up.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 19 has landed. It carries the sets \
        reading of MY-38 that gathers the entries of a chain by the \
        set of atlases each stands in, in one section for each \
        distinct set under the atlas lines of its set and the \
        standing lines of the entries holding it, with its goldens \
        under examples, the documentation repairs of the company \
        section of the Usage and the enumeration of the doc head \
        carried to twelve chain readings, and the generated fixture \
        family extended with the sets golden and its relations at \
        the stated shapes.";
    assert!(carries_line(&text, expected));
    assert!(text.contains("Closed perfection rounds, 20 of 100"));
    // The number FIX-MY-18 stands unspent, as the Round 13
    // section of the contract records and the Round 18
    // ratification holds, the version 2 amendment deferred.
    let contract = read("CONTRACT.md");
    assert!(!contract.contains("### FIX-MY-18"));
}

#[test]
fn readme_reports_the_round_nineteen_status() {
    let text = read("README.md");
    let standing = "- Contract, MY-39 and MY-40 met. They stand in full form in \
        CONTRACT.md, and Round 19 closed on the merged slice 20 after \
        three reviews, with the red team clean, the figure repair and \
        the header restatement repair landed in the slice, and the \
        record corrections of the round carried in its closure record.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 20 has landed. It carries the \
        rosters reading of MY-40 that gathers the atlases of a chain by \
        the set of entries each holds, in one section for each distinct \
        roster under the standing lines of its entries and the atlas \
        lines of the atlases holding it, with its goldens under \
        examples, the documentation repairs of the sets section of \
        the Usage and the enumeration of the doc head carried to \
        thirteen chain readings, and the generated fixture family \
        extended with the rosters golden and its relations at the \
        stated shapes.";
    assert!(carries_line(&text, expected));
    assert!(text.contains("Closed perfection rounds, 20 of 100"));
    // The number FIX-MY-18 stands unspent, as the Round 13
    // section of the contract records and the Round 19
    // ratification holds, the version 2 amendment deferred.
    let contract = read("CONTRACT.md");
    assert!(!contract.contains("### FIX-MY-18"));
}

#[test]
fn readme_reports_the_round_twenty_status() {
    let text = read("README.md");
    let standing = "- Contract, MY-41 and MY-42 met. They stand in full form in CONTRACT.md, and Round 20 closed on the merged slice 21 after three reviews, with the red team approving on recomputation of every fellows golden, FIX-MY-22 amending the reconstruction direction and the sharing clause of MY-42, and the empty atlas pinned beside it.";
    assert!(carries_line(&text, standing));
    let expected = "- Slice 21 has landed. It carries the \
        fellows reading of MY-42 that names for each atlas of a chain \
        the atlases standing with it, in one section for each atlas \
        under its bare atlas line and the atlas lines of the atlases \
        standing with it, with its goldens under examples, the \
        documentation repairs of the rosters section of the Usage and \
        the enumeration of the doc head carried to fourteen chain \
        readings, and the generated fixture family extended with the \
        fellows golden and its relations at the stated shapes.";
    assert!(carries_line(&text, expected));
    assert!(text.contains("Closed perfection rounds, 20 of 100"));
    // The number FIX-MY-18 stands unspent, as the Round 13
    // section of the contract records and the Round 20
    // ratification holds, the version 2 amendment deferred.
    let contract = read("CONTRACT.md");
    assert!(!contract.contains("### FIX-MY-18"));
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

// The Round 12 formats pins, FIX-MY-15. The amendment names
// the three file forms and states each at version 1 in the
// consolidated formats section, and repoints the operative
// pointers of the earlier sections at the Rust files that
// carry their pins now. The trailer walk fails the gate the
// moment any path the contract names stops standing in the
// tree, so a dead pointer can never stand silently again.

#[test]
fn contract_carries_the_round_twelve_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["FIX-MY-15", "FIX-MY-16", "MY-25", "MY-26"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn contract_carries_the_round_thirteen_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["MY-27", "MY-28"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn contract_carries_the_round_fourteen_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["MY-29", "MY-30", "FIX-MY-20"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn contract_carries_the_round_fifteen_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["MY-31", "MY-32"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn contract_carries_the_round_sixteen_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["MY-33", "MY-34"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn contract_carries_the_round_seventeen_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["MY-35", "MY-36"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn contract_carries_the_round_eighteen_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["MY-37", "MY-38"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn contract_carries_the_round_nineteen_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["MY-39", "MY-40"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn contract_carries_the_round_twenty_obligations() {
    let text = read("CONTRACT.md");
    for oid in ["MY-41", "MY-42"] {
        assert!(text.contains(oid), "{oid}");
    }
}

#[test]
fn the_exploration_format_stands_named_at_version_one() {
    let text = read("CONTRACT.md");
    assert!(text.contains("myriad-exploration at version 1"));
}

#[test]
fn the_atlas_format_stands_named_at_version_one() {
    let text = read("CONTRACT.md");
    assert!(text.contains("myriad-atlas at version 1"));
}

#[test]
fn the_lineage_format_stands_named_at_version_one() {
    let text = read("CONTRACT.md");
    assert!(text.contains("myriad-lineage at version 1"));
    assert!(text.contains("A change that adds an optional field is not a new version"));
}

// Round 12, the documentation repairs. The arrivals
// section stands in Usage after the absence section,
// the worked JSON loads through the loader it teaches,
// the Install sentence stands in its settled form, and
// the two visible sentences after the README stand in
// the read voice of MY-22.

#[test]
fn the_arrivals_section_stands_in_usage_after_the_absence_section() {
    let readme = read("README.md");
    let absence = readme
        .find("### absence")
        .expect("the absence section stands");
    let arrivals = readme
        .find("### arrivals")
        .expect("the arrivals section stands");
    assert!(
        arrivals > absence,
        "the arrivals section follows the absence section"
    );
    assert!(readme
        .contains("./target/release/myriad arrivals examples/winter-spring-summer-lineage.json"));
    assert!(readme.contains(
        "arrivals counts, atlas 1 arrivals 3, atlas 2 arrivals 1, \
         atlas 3 arrivals 0, entries 4, atlases 3"
    ));
}

// Round 13, the documentation repairs. The departures
// section stands in Usage after the arrivals section,
// and the doc head of the library root names every
// chain reading, each pin failing by assertion on the
// tree before this slice, where no departures section
// stood and the doc head named five of the seven.

#[test]
fn the_departures_section_stands_in_usage_after_the_arrivals_section() {
    let readme = read("README.md");
    let arrivals = readme
        .find("### arrivals")
        .expect("the arrivals section stands");
    let departures = readme
        .find("### departures")
        .expect("the departures section stands");
    assert!(
        departures > arrivals,
        "the departures section follows the arrivals section"
    );
    assert!(readme
        .contains("./target/release/myriad departures examples/winter-spring-summer-lineage.json"));
    assert!(readme.contains(
        "departures counts, atlas 1 departures 0, atlas 2 departures 1, \
         atlas 3 departures 0, entries departed 1, entries 4, atlases 3"
    ));
}

// Round 14, the documentation repairs. The matched
// section stands in Usage after the departures section,
// and the doc head of the library root names every
// chain reading, each pin failing by assertion on the
// tree before this slice, where no matched section
// stood and the doc head named seven of the eight.

#[test]
fn the_matched_section_stands_in_usage_after_the_departures_section() {
    let readme = read("README.md");
    let departures = readme
        .find("### departures")
        .expect("the departures section stands");
    let matched = readme
        .find("### matched")
        .expect("the matched section stands");
    assert!(
        matched > departures,
        "the matched section follows the departures section"
    );
    assert!(readme
        .contains("./target/release/myriad matched examples/winter-spring-summer-lineage.json"));
    assert!(readme.contains(
        "matched counts, step 1 matched 2, step 2 matched 2, \
         entries matched 2, entries 4, steps 2"
    ));
}

#[test]
fn the_lib_doc_head_names_every_chain_reading() {
    let lib = read("rust/src/lib.rs");
    // The doc head wraps over comment lines, so the pin
    // reads it as the rendered text flows.
    let flowed = lib.replace("\n//! ", " ");
    for reading in [
        "the lineage reading",
        "the returns reading",
        "the standing reading",
        "the absence reading",
        "the arrivals reading",
        "the departures reading",
        "the matched reading",
        "the runs reading",
        "the presence reading",
        "the standings reading",
        "the company reading",
        "the sets reading",
        "the rosters reading",
        "the fellows reading",
    ] {
        assert!(flowed.contains(reading), "the doc head names {reading}");
    }
}

// Round 15, the documentation repairs. The runs section
// stands in Usage after the matched section, and the doc
// head of the library root names every chain reading,
// each pin failing by assertion on the tree before this
// slice, where no runs section stood and the doc head
// named eight of the nine.

#[test]
fn the_runs_section_stands_in_usage_after_the_matched_section() {
    let readme = read("README.md");
    let matched = readme
        .find("### matched")
        .expect("the matched section stands");
    let runs = readme.find("### runs").expect("the runs section stands");
    assert!(
        runs > matched,
        "the runs section follows the matched section"
    );
    assert!(
        readme.contains("./target/release/myriad runs examples/winter-spring-summer-lineage.json")
    );
    assert!(readme.contains("runs counts, runs 5, entries with runs 4, standing at the close 3"));
}

// Round 16, the documentation repairs. The presence
// section stands in Usage after the runs section, and
// the doc head of the library root names every chain
// reading, each pin failing by assertion on the tree
// before this slice, where no presence section stood
// and the doc head named nine of the ten.

#[test]
fn the_presence_section_stands_in_usage_after_the_runs_section() {
    let readme = read("README.md");
    let runs = readme.find("### runs").expect("the runs section stands");
    let presence = readme
        .find("### presence")
        .expect("the presence section stands");
    assert!(
        presence > runs,
        "the presence section follows the runs section"
    );
    assert!(readme
        .contains("./target/release/myriad presence examples/winter-spring-summer-lineage.json"));
    assert!(readme.contains(
        "presence counts, atlas 1 standing 3, atlas 2 standing 3, atlas 3 standing 3, entries 4, atlases 3"
    ));
}

// Round 17, the documentation repairs. The standings
// section stands in Usage after the presence section,
// and the doc head of the library root names every chain
// reading, each pin failing by assertion on the tree
// before this slice, where no standings section stood
// and the doc head named ten of the eleven.

#[test]
fn the_standings_section_stands_in_usage_after_the_presence_section() {
    let readme = read("README.md");
    let presence = readme
        .find("### presence")
        .expect("the presence section stands");
    let standings = readme
        .find("### standings")
        .expect("the standings section stands");
    assert!(
        standings > presence,
        "the standings section follows the presence section"
    );
    assert!(readme
        .contains("./target/release/myriad standings examples/winter-spring-summer-lineage.json"));
    assert!(readme.contains("standings counts, standings 9, entries 4, atlases 3"));
}

// Round 18, the documentation repairs. The company
// section stands in Usage after the standings section,
// and the doc head of the library root names every
// chain reading, each pin failing by assertion on the
// tree before this slice, where no company section
// stood and the doc head named eleven of the twelve.

#[test]
fn the_company_section_stands_in_usage_after_the_standings_section() {
    let readme = read("README.md");
    let standings = readme
        .find("### standings")
        .expect("the standings section stands");
    let company = readme
        .find("### company")
        .expect("the company section stands");
    assert!(
        company > standings,
        "the company section follows the standings section"
    );
    assert!(readme
        .contains("./target/release/myriad company examples/winter-spring-summer-lineage.json"));
    assert!(readme.contains("company counts, company 10, entries 4, atlases 3"));
}

// Round 19, the documentation repairs. The sets
// section stands in Usage after the company section,
// and the doc head of the library root names every
// chain reading, each pin failing by assertion on the
// tree before this slice, where no sets section
// stood and the doc head named twelve of the thirteen.

#[test]
fn the_sets_section_stands_in_usage_after_the_company_section() {
    let readme = read("README.md");
    let company = readme
        .find("### company")
        .expect("the company section stands");
    let sets = readme.find("### sets").expect("the sets section stands");
    assert!(
        sets > company,
        "the sets section follows the company section"
    );
    assert!(
        readme.contains("./target/release/myriad sets examples/winter-spring-summer-lineage.json")
    );
    assert!(readme.contains("sets counts, sets 3, entries 4, atlases 3"));
}

// Round 20, the documentation repairs. The rosters
// section stands in Usage after the sets section,
// and the doc head of the library root names every
// chain reading, each pin failing by assertion on
// the tree before this slice, where no rosters section
// stood and the doc head named thirteen of the fourteen.

#[test]
fn the_rosters_section_stands_in_usage_after_the_sets_section() {
    let readme = read("README.md");
    let sets = readme.find("### sets").expect("the sets section stands");
    let rosters = readme
        .find("### rosters")
        .expect("the rosters section stands");
    assert!(
        rosters > sets,
        "the rosters section follows the sets section"
    );
    assert!(readme
        .contains("./target/release/myriad rosters examples/winter-spring-summer-lineage.json"));
    assert!(readme.contains("rosters counts, rosters 2, atlases 3, entries 4"));
}

#[test]
fn the_worked_json_of_the_readme_loads_clean_through_the_loader() {
    let readme = read("README.md");
    let start = readme
        .find("```json")
        .expect("the worked example stands fenced");
    let rest = &readme[start + "```json".len()..];
    let end = rest.find("```").expect("the fence closes");
    let text = rest[..end].trim();
    let (exploration, defects) = myriad::explorations::load_exploration_text(text);
    assert!(
        defects.is_empty(),
        "the worked example loads clean: {defects:?}"
    );
    let exploration = exploration.expect("the worked example loads");
    assert!(exploration
        .question
        .eq_str("Which week does the harbour ice survey run"));
    assert_eq!(exploration.branches.len(), 3);
}

#[test]
fn the_install_sentence_states_the_toolchain_floor() {
    let readme = read("README.md");
    assert!(readme.contains("Build from source with a current stable Rust toolchain."));
    assert!(readme.contains("The crate is edition 2021"));
    assert!(readme.contains("the gate tests on Rust 1.91.1, the toolchain the workflow pins"));
}

#[test]
fn the_cli_description_and_the_lib_doc_head_stand_in_the_read_voice() {
    let cli = read("rust/src/cli.rs");
    assert!(cli.contains(
        "Read logs of branching investigations of a question and print readings over them."
    ));
    assert!(!cli.contains("Fork one hard question into branches and map them."));
    let lib = read("rust/src/lib.rs");
    // The doc head wraps over comment lines, so the pin
    // reads it as the rendered text flows.
    let flowed = lib.replace("\n//! ", " ");
    assert!(flowed.contains(
        "Myriad reads logs of branching investigations of a question and prints \
         readings over them."
    ));
    assert!(!lib.contains("forking one hard question"));
}

#[test]
fn every_path_the_contract_names_stands_in_the_tree() {
    let text = read("CONTRACT.md");
    let root = repo_root();
    let mut named: Vec<String> = Vec::new();
    for token in text.split(|ch: char| ch.is_whitespace() || matches!(ch, ',' | '(' | ')')) {
        let path = token.trim_end_matches('.');
        if path.starts_with("rust/") && !named.iter().any(|held| held == path) {
            named.push(path.to_string());
        }
    }
    assert!(named.len() > 40, "the walk reads the contract's paths");
    for path in &named {
        assert!(root.join(path).exists(), "the contract names {path}");
    }
}
