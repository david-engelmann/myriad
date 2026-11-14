//! Tests for MY-40, the rosters reading.
//!
//! The reading takes a lineage, reads the presence of
//! every label in the one presence pass MY-18 states,
//! gathers the atlases by the set of entries each
//! holds, in one walk over the chain, and states one
//! section for each distinct roster, the section
//! opening with the standing line of each entry of
//! its roster, in the order of first standing, and
//! holding one line for each atlas holding the
//! roster, in chain order, closing with the rosters
//! counts. The pins here read the reading over the
//! shipped chains and over hand built chains, and
//! bind it to the presence, standing, standings, and
//! sets readings through the five relations MY-40
//! states, each computed from the neighbouring
//! readings' own outputs.
//!
//! The mutations the plan names against the reading
//! stand here once, as exact edits to the built
//! reading, and the pins below cite them by number.
//! MR-1, first entry grouping: the gathering keys
//! each atlas by the first entry of its roster, in
//! the order of first standing, atlases sharing a
//! section whenever their rosters' first entries
//! agree, their rosters never compared. MR-2, size
//! grouping: the gathering keys each atlas by the
//! count of entries it holds, atlases sharing a
//! section whenever their counts agree, their
//! rosters never compared. MR-3, member order
//! headings: the heading lines of a section stand in
//! the order the entries stand in the section's
//! earliest atlas, the member order the presence
//! reading prints, in place of the order of first
//! standing. MR-4, the atlases closing: the closing's
//! rosters figure counts the sections' atlas lines,
//! reading atlases in place of rosters. MR-5, the one
//! line heading: each section opens with a single
//! line naming the count of its roster's entries in
//! place of the roster's standing lines. MR-6, the
//! containment leak: an atlas stands in every section
//! whose roster contains its own roster, not only in
//! its own roster's section.
//!
//! The discrimination split of these pins stands as
//! follows, pin by pin. Pins 1 to 11 fail on the tree
//! before this slice by execution, because
//! render_rosters stands nowhere on that tree, so
//! the file does not compile there and no pin of 1 to
//! 11 can pass there. Pin 1, the winter golden byte
//! for byte, fails under MR-1, the three atlases
//! sharing one section, every winter roster first
//! standing at the ferry link entry; under MR-2, the
//! winter rosters standing uniform at three entries
//! each, so the grouping by count gathers the whole
//! chain into one section, the posture stated here
//! at design time against its sets ancestor, the
//! winter chain having passed the size grouping on
//! the label side by the coincidence of its labels'
//! counts and failing it on the atlas side by the
//! uniformity of its rosters; under MR-4, the closing
//! reading rosters 3, pin 1 standing as MR-4's
//! designated killer; and under MR-5. Under MR-3 the
//! pin passes, and the posture is stated here from
//! the executed transcription, not found later: the
//! member order of every winter atlas agrees with
//! the order of first standing over its roster.
//! Under MR-6 the pin passes, no winter roster
//! containing another, computed over the shipped
//! chain. Pin 2, the radio golden byte for byte,
//! fails under MR-1, every radio roster first
//! standing at the radio check entry; under MR-2,
//! the second and third atlases sharing a section,
//! their counts agreeing at three while their
//! rosters differ; under MR-6, the first atlas
//! standing also in the third atlas's section, its
//! roster standing inside the third's; and under
//! MR-5. The closing half of the pin passes under
//! MR-4 by the coincidence of its figures, the radio
//! chain holding three rosters over three atlases,
//! and that posture is stated here too. Under MR-3
//! the pin passes as pin 1 does, the two orders
//! agreeing on every radio atlas. Pin 3, the
//! constructed golden byte for byte, fails under
//! MR-2, the first and second atlases sharing a
//! section, their counts agreeing at three; under
//! MR-6, the third atlas standing also in the second
//! atlas's section, its roster standing inside the
//! second's; and under MR-5. Under MR-1 the pin
//! passes, its rosters first standing at the solo
//! line, middle line, and keeper line entries, all
//! distinct, the posture stated at design time,
//! MR-1's killers being pins 1, 2, 4, 5, 6, 8, 9,
//! 10, 11, 18, 20, and 21. Under
//! MR-3 and MR-4 the pin passes as pin 2 does. Pin 4,
//! the partition relation, read against the presence
//! reading's own output, fails under MR-1, MR-2, and
//! MR-6, and under MR-5, its sections' headings
//! standing as the one line counts in place of the
//! standing lines the presence sections name. Pin 5,
//! the equality relation, computed both
//! ways against the presence output, fails under
//! MR-1 and MR-2, and under MR-5 and MR-6, its
//! section parse reading the printed headings.
//! Pin 6, the heading agreement, fails under MR-5,
//! and under MR-1, MR-2, and MR-6, in each case a
//! merged or leaked section printing a heading whose
//! entries do not stand in every atlas beneath it.
//! Pin 7, the line agreement and order,
//! passes under MR-3 on its own chains, the member
//! order of each section's earliest atlas agreeing
//! with the order of first standing there by the
//! executed transcription, and the round's MR-3
//! discrimination stands at pins 11, 18, 20, and 21,
//! where the orders part. The pin fails under MR-5
//! and MR-6, the one line heading standing in place
//! of the standing lines under the first, and an
//! atlas standing in a section beyond its own
//! roster's under the second. Pin 8, the closing
//! agreement, fails under MR-4 on its winter arm and
//! under MR-2, and under MR-1, MR-5, and MR-6, its
//! closing and heading line sums reading the mutated
//! gatherings directly. Pin 9, the sets agreement,
//! computed
//! against the sets reading's own output, fails
//! under MR-1 and MR-2, and under MR-5 and MR-6,
//! under MR-5 no heading standing as an entry line,
//! so no section opens to compare, and under MR-6
//! an atlas standing in a section beyond its own,
//! setting pairs together the sets reading keeps
//! apart. Pin 10, the widest roster,
//! passes under MR-2 by design, its sizes standing
//! 3, 3, and 2 and the grouping by count standing as
//! the gathering itself stands, and it fails under
//! MR-6, the part roster standing inside the full,
//! and under MR-1, MR-4, and MR-5, its closing
//! assertion and its section shape both reading the
//! mutated form.
//! Pin 11, the parting pin, fails under MR-3, its
//! heading reversing; under MR-1, both rosters first
//! standing at the one early entry; under MR-5, its
//! heading standing as the one line naming its count
//! in place of the standing lines it renders whole;
//! and under MR-6, the first roster standing inside
//! the second; and
//! it passes under MR-4 by design, its rosters and
//! its atlases both counting two, the posture stated
//! here so MR-4's designated killer stands as pin 1
//! among the golden and hand built pins. Pins 12 and
//! 13 state the refusal battery, and their split is
//! stated in the form the Round 16 deviation D2
//! settled: the behaviour the pins pin stands whole
//! on the tree before this slice, in the one
//! presence pass the reading consumes unchanged, the
//! fewer than two atlases refusal and the surrogate
//! read before the line break read, and inside a
//! file that imports render_rosters the two pins
//! share the file's compile absence on that tree,
//! exactly as the pins of every reading file of the
//! family do. Under MR-1 to MR-6 the two pins pass,
//! the refusals they pin standing in the one presence
//! pass none of the six edits touches, and pin 12
//! alone catches the further edit the mutation pass
//! probed beyond the named six, the fewer than two
//! atlases refusal dropped in that same pass.
//!
//! Round 23 adds four pins to this file, and their
//! discrimination split stands as follows. The fold
//! of MY-47 is behaviour neutral, so the behaviour
//! the four pins pin stands whole on the tree before
//! this slice, in the shipped render_rosters, and
//! the four pins pass there by design, as guards of
//! a restatement and not as witnesses of an absence.
//! Two forms of the fold stand behaviour identical
//! to the correct fold on every chain, MO-1, the
//! roster scan restored, and MO-2, the lines from
//! the helper, the scan and the walk gathering the
//! same rosters in the same order and the helper and
//! the inversion stating the same figures, so every
//! pin of this file passes under both, and their
//! discrimination stands on the contract text of
//! MY-47, on the reviews' line read of the one
//! function, and on the fold record's measured
//! times, as MY-48 states. Under MO-3, the text
//! order roster, 6 pins of this file fail, pins 1,
//! 2, and 3, the three counted golden pins, pin 4,
//! the partition relation, pin 7, the line agreement
//! and order, and pin 10, the widest and part roster
//! pin, beside the generated chain rosters golden
//! and rosters relations. Under MO-4, the reversed
//! sections, 6 pins of this file fail, pins 1, 2,
//! and 3, pin 7, pin 10, and pin 11, the parting
//! heading pin, beside the same two generated chain
//! pins. Under MO-5, the earliest atlas only, 6 pins
//! of this file fail, pins 1, 4, 5, 8, 9, and 10,
//! and no generated chain pin fails, every generated
//! roster standing held by one atlas. Under MO-6,
//! the span heading, 5 pins of this file fail, pins
//! 1 and 2, pin 4, pin 6, and pin 7, beside the same
//! two generated chain pins, the constructed counted
//! pin passing, no constructed label's standings
//! gapping, so span and count agree on every heading
//! of that chain. Pins 12 and 13, the refusal
//! battery, pass under all six edits, the refusals
//! they pin standing in the one presence pass none
//! of the six edits touches. The thin chain pin
//! fails under MO-4, its sections standing reversed,
//! the last section first, and under MO-6, its first
//! heading reading the label-1 span of 302 against
//! its standings of two, and it passes under MO-5 by
//! design, every thin roster standing held by one
//! atlas, so the earliest atlas only stands as the
//! whole of each section's atlas lines already.
//! Under MO-3 the thin chain pin passes by the shape
//! of its fixture, and the posture stands stated
//! here in the open, the pin counting verbatim the
//! sections of atlases 1, 152, and 302, none of
//! which the text order parts, no thin roster
//! holding label-10 and label-2 together, so the
//! flip the edit works elsewhere in the chain cannot
//! occur in the sections the pin counts, and its
//! counts, its section shape, and its closing stand
//! indifferent to heading order inside a section.
//! The window chain pin fails under MO-3, its
//! headings standing in label text order, under
//! MO-4, its sections standing reversed, under MO-5,
//! each section holding one atlas line of its four
//! and the reading standing at 181 lines against its
//! 271, and under MO-6, its first heading reading the
//! label-1 span of 120 against its standings of
//! twenty. The paired chain pin fails under MO-3,
//! its first section's headings standing in label
//! text order as label-1, label-2, and label-4
//! against the order of first standing, label-2,
//! label-1, and label-4, under MO-4, its two sections
//! standing reversed, under MO-5, each section
//! holding one atlas line of its sixty, and under
//! MO-6, its first section's label-1 heading reading
//! the span of 119 against its standings of sixty.
//! The gathering relation pin fails under MO-3, its
//! headings standing out of the order of first
//! standing the relation reads from the standing
//! reading, under MO-4, its sections standing out of
//! their earliest atlases' places, under MO-5, the
//! winter chain's third atlas standing in no section
//! and the paired chain's atlases standing in none
//! but their sections' first, and under MO-6, its
//! heading lines standing as span lines wherever a
//! label's standings gap, the harbour lights line of
//! the winter chain among them. Under MI-1 to MI-5,
//! MF-1 to MF-6, and MC-1 to MC-6 all four pins pass,
//! those edits standing in render_standings,
//! render_fellows, and render_company, and in no
//! function the pins call. The thirteen pins above
//! stand unmoved.
//!

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_presence, render_rosters, render_sets, render_standing,
    render_standings, Atlas, AtlasEntry, Lineage,
};
use myriad::explorations::Exploration;
use myriad::pytext::PText;

fn expl() -> Exploration {
    Exploration {
        question: pt("q"),
        branches: vec![branch("a", None, "p", "open", None, None, None)],
    }
}

fn atlas(title: &str, labels: &[&str]) -> Atlas {
    Atlas {
        title: pt(title),
        entries: labels
            .iter()
            .map(|label| AtlasEntry {
                label: pt(label),
                exploration: expl(),
            })
            .collect(),
    }
}

fn load(name: &str) -> Lineage {
    let path = repo_root().join("examples").join(name);
    let text = std::fs::read_to_string(&path).expect("a shipped example reads");
    let (lineage, defects) = load_lineage_text(&text);
    assert!(
        defects.is_empty(),
        "a shipped lineage loads clean: {defects:?}"
    );
    lineage.expect("a shipped lineage value stands")
}

const CHAINS: [&str; 3] = [
    "winter-spring-summer-lineage.json",
    "radio-investigation-lineage.json",
    "constructed-departures-lineage.json",
];

/// The figure after a head in a line, read back out of
/// the rendered text.
fn figure(line: &str, head: &str, tail: &str) -> usize {
    let rest = line
        .split(head)
        .nth(1)
        .unwrap_or_else(|| panic!("the figure head {head} stands in the line {line}"));
    rest.split(tail)
        .next()
        .expect("the figure stands in the line")
        .parse()
        .expect("a count")
}

/// The label an entry line names.
fn label_of(line: &str) -> String {
    line.strip_prefix("entry ")
        .expect("an entry line opens with the word entry")
        .split(',')
        .next()
        .expect("a label stands")
        .to_string()
}

/// The sections of a presence reading, each the entry
/// lines standing under its opening line.
fn presence_sections(lines: &[String]) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = Vec::new();
    for line in lines {
        if line.starts_with("atlas ") && !line.contains("standing") {
            out.push(Vec::new());
        } else if line.starts_with("entry ") {
            out.last_mut()
                .expect("an entry line stands inside a section")
                .push(line.clone());
        }
    }
    out
}

/// The sections of a sets reading, the closing set
/// aside, each section the numbers of its atlas lines
/// in order and its entry lines. A section opens at
/// an atlas line standing first or following an entry
/// line, the atlas lines of one set standing in a row
/// ahead of the entry lines holding it.
fn sets_sections(lines: &[String]) -> Vec<(Vec<usize>, Vec<String>)> {
    let mut out: Vec<(Vec<usize>, Vec<String>)> = Vec::new();
    for line in &lines[..lines.len() - 1] {
        if line.starts_with("atlas ") {
            let opens = match out.last() {
                None => true,
                Some((_, entries)) => !entries.is_empty(),
            };
            if opens {
                out.push((Vec::new(), Vec::new()));
            }
            let number: usize = line
                .strip_prefix("atlas ")
                .expect("an atlas line")
                .parse()
                .expect("an atlas number");
            out.last_mut()
                .expect("an atlas line stands inside a section")
                .0
                .push(number);
        } else if line.starts_with("entry ") {
            out.last_mut()
                .expect("an entry line stands inside a section")
                .1
                .push(line.clone());
        }
    }
    out
}

/// The sections of a rosters reading, the closing set
/// aside, each section its heading entry lines and the
/// numbers of its atlas lines in order. A section
/// opens at an entry line standing first or following
/// an atlas line, the heading lines of one roster
/// standing in a row ahead of the atlas lines holding
/// it.
fn rosters_sections(lines: &[String]) -> Vec<(Vec<String>, Vec<usize>)> {
    let mut out: Vec<(Vec<String>, Vec<usize>)> = Vec::new();
    for line in &lines[..lines.len() - 1] {
        if line.starts_with("entry ") {
            let opens = match out.last() {
                None => true,
                Some((_, atlases)) => !atlases.is_empty(),
            };
            if opens {
                out.push((Vec::new(), Vec::new()));
            }
            out.last_mut()
                .expect("a heading line stands inside a section")
                .0
                .push(line.clone());
        } else if line.starts_with("atlas ") {
            let number: usize = line
                .strip_prefix("atlas ")
                .expect("an atlas line")
                .parse()
                .expect("an atlas number");
            out.last_mut()
                .expect("an atlas line stands inside a section")
                .1
                .push(number);
        }
    }
    out
}

/// A thin chain of the given length built in process,
/// the atlas at each place holding the label of its
/// place and the label of the next, cyclically, so
/// each label stands in exactly two atlases.
fn thin_chain(n: usize) -> Lineage {
    let mut atlases = Vec::new();
    for i in 0..n {
        let first = format!("label-{}", i + 1);
        let second = format!("label-{}", (i + 1) % n + 1);
        let title = format!("atlas-{}", i + 1);
        atlases.push(atlas(&title, &[&first, &second]));
    }
    Lineage { atlases }
}

/// A window chain of one hundred and twenty atlases
/// built in process over thirty labels, each atlas
/// holding the five labels of a window turning by one
/// label per atlas, so each roster is held by the four
/// atlases one full turn apart.
fn window_chain() -> Lineage {
    let mut atlases = Vec::new();
    for i in 0..120usize {
        let labels: Vec<String> = (0..5)
            .map(|j| format!("label-{}", (i + j) % 30 + 1))
            .collect();
        let refs: Vec<&str> = labels.iter().map(String::as_str).collect();
        let title = format!("atlas-{}", i + 1);
        atlases.push(atlas(&title, &refs));
    }
    Lineage { atlases }
}

/// A paired chain of one hundred and twenty atlases
/// built in process over five labels, its odd atlases
/// holding label-2, label-1, and label-4 in that member
/// order and its even atlases holding label-3, label-5,
/// and label-2, so each of its two rosters is held by
/// sixty atlases.
fn paired_chain() -> Lineage {
    let mut atlases = Vec::new();
    for i in 0..120 {
        let title = format!("atlas-{}", i + 1);
        if i % 2 == 0 {
            atlases.push(atlas(&title, &["label-2", "label-1", "label-4"]));
        } else {
            atlases.push(atlas(&title, &["label-3", "label-5", "label-2"]));
        }
    }
    Lineage { atlases }
}

#[test]
fn the_winter_rosters_stand_as_counted() {
    let lines =
        render_rosters(&load("winter-spring-summer-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "atlas 1",
            "atlas 3",
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "atlas 2",
            "rosters counts, rosters 2, atlases 3, entries 4",
        ])
    );
}

#[test]
fn the_radio_rosters_stand_as_counted() {
    let lines = render_rosters(&load("radio-investigation-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry radio check, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry battery drain, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "atlas 1",
            "entry radio check, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry aerial icing, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "entry log gaps, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "atlas 2",
            "entry radio check, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry battery drain, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry log gaps, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "atlas 3",
            "rosters counts, rosters 3, atlases 3, entries 4",
        ])
    );
}

#[test]
fn the_constructed_rosters_stand_as_counted() {
    let lines =
        render_rosters(&load("constructed-departures-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry solo line, first stood at atlas 1, last stood at atlas 1, stands in 1 atlases, does not stand at the close",
            "entry middle line, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, does not stand at the close",
            "entry keeper line, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "atlas 1",
            "entry middle line, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, does not stand at the close",
            "entry keeper line, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry late line, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "atlas 2",
            "entry keeper line, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry late line, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "atlas 3",
            "rosters counts, rosters 3, atlases 3, entries 4",
        ])
    );
}

#[test]
fn each_atlas_stands_in_exactly_one_section_matching_its_presence_section() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_rosters(&chain).expect("the rosters read");
        let found = rosters_sections(&lines);
        let presence = render_presence(&chain).expect("the presence reads");
        let by_atlas = presence_sections(&presence);
        let standing = render_standing(&chain).expect("the standing reads");
        let standing_lines = &standing[..standing.len() - 1];
        for (index, section) in by_atlas.iter().enumerate() {
            let number = index + 1;
            let holding: Vec<&(Vec<String>, Vec<usize>)> = found
                .iter()
                .filter(|(_, atlases)| atlases.contains(&number))
                .collect();
            assert_eq!(
                holding.len(),
                1,
                "the atlas {number} stands in exactly one section over {name}"
            );
            let heading = &holding[0].0;
            let mut named: Vec<String> = heading.clone();
            named.sort();
            let mut present: Vec<String> = section.clone();
            present.sort();
            assert_eq!(
                named, present,
                "the heading of the section holding atlas {number} names the entries presence names for it over {name}"
            );
            let mut ordered: Vec<&String> = heading.iter().collect();
            ordered.sort_by_key(|line| {
                standing_lines
                    .iter()
                    .position(|held| held == *line)
                    .expect("a heading line is a standing line")
            });
            let ordered: Vec<String> = ordered.into_iter().cloned().collect();
            assert_eq!(
                heading, &ordered,
                "the heading lines stand in the order of first standing over {name}"
            );
        }
        let named: usize = found.iter().map(|(_, atlases)| atlases.len()).sum();
        assert_eq!(
            named,
            figure(lines.last().expect("a closing line"), "atlases ", ","),
            "the atlas lines over all sections count the atlases figure over {name}"
        );
    }
}

#[test]
fn two_atlases_share_a_section_exactly_when_presence_names_the_same_entries() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_rosters(&chain).expect("the rosters read");
        let found = rosters_sections(&lines);
        let presence = render_presence(&chain).expect("the presence reads");
        let by_atlas = presence_sections(&presence);
        for first in 1..=by_atlas.len() {
            for second in 1..=by_atlas.len() {
                let together = found
                    .iter()
                    .any(|(_, atlases)| atlases.contains(&first) && atlases.contains(&second));
                let mut first_labels: Vec<String> = by_atlas[first - 1]
                    .iter()
                    .map(|line| label_of(line))
                    .collect();
                first_labels.sort();
                let mut second_labels: Vec<String> = by_atlas[second - 1]
                    .iter()
                    .map(|line| label_of(line))
                    .collect();
                second_labels.sort();
                assert_eq!(
                    together,
                    first_labels == second_labels,
                    "atlases {first} and {second} stand together exactly on the equality of their rosters over {name}"
                );
            }
        }
    }
}

#[test]
fn each_sections_heading_agrees_with_every_atlas_line_under_it() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_rosters(&chain).expect("the rosters read");
        let found = rosters_sections(&lines);
        let presence = render_presence(&chain).expect("the presence reads");
        let by_atlas = presence_sections(&presence);
        let standing = render_standing(&chain).expect("the standing reads");
        let standing_lines = &standing[..standing.len() - 1];
        for (heading, atlases) in &found {
            assert!(
                !atlases.is_empty(),
                "a section holds its atlases over {name}"
            );
            for line in heading {
                assert!(
                    standing_lines.contains(line),
                    "a heading line is the standing line of its entry byte for byte over {name}"
                );
            }
            let mut heading_labels: Vec<String> =
                heading.iter().map(|line| label_of(line)).collect();
            heading_labels.sort();
            for number in atlases {
                let mut present_labels: Vec<String> = by_atlas[number - 1]
                    .iter()
                    .map(|line| label_of(line))
                    .collect();
                present_labels.sort();
                assert_eq!(
                    heading_labels, present_labels,
                    "the heading names every entry standing in atlas {number} and no other over {name}"
                );
                assert_eq!(
                    heading.len(),
                    by_atlas[number - 1].len(),
                    "the heading's count is the presence count of atlas {number} over {name}"
                );
            }
        }
    }
}

#[test]
fn every_heading_line_is_the_standing_line_of_its_entry_and_the_sections_stand_in_order() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_rosters(&chain).expect("the rosters read");
        let found = rosters_sections(&lines);
        let standing = render_standing(&chain).expect("the standing reads");
        let standing_lines = &standing[..standing.len() - 1];
        let place_of = |line: &String| {
            standing_lines
                .iter()
                .position(|held| held == line)
                .expect("a heading line is a standing line of the reading")
        };
        let mut earliest: Vec<usize> = Vec::new();
        for (heading, atlases) in &found {
            let mut places: Vec<usize> = Vec::new();
            for line in heading {
                assert!(
                    standing_lines.contains(line),
                    "a heading line is the standing line of its entry byte for byte over {name}"
                );
                places.push(place_of(line));
            }
            let mut ordered = places.clone();
            ordered.sort_unstable();
            assert_eq!(
                places, ordered,
                "the heading lines stand in first standing order over {name}"
            );
            earliest.push(atlases[0]);
        }
        let mut ordered = earliest.clone();
        ordered.sort_unstable();
        assert_eq!(
            earliest, ordered,
            "the sections stand at their earliest atlas's place over {name}"
        );
    }
}

#[test]
fn the_closing_agrees_with_the_sections_and_the_standings() {
    let expected_singletons = [1usize, 3, 3];
    let expected_sums = [9usize, 8, 8];
    let expected_headings = [6usize, 8, 8];
    for (chain_index, name) in CHAINS.iter().enumerate() {
        let chain = load(name);
        let lines = render_rosters(&chain).expect("the rosters read");
        let found = rosters_sections(&lines);
        let closing = lines.last().expect("a closing line");
        let presence = render_presence(&chain).expect("the presence reads");
        let by_atlas = presence_sections(&presence);
        let distinct: std::collections::BTreeSet<Vec<String>> = by_atlas
            .iter()
            .map(|section| {
                let mut labels: Vec<String> = section.iter().map(|line| label_of(line)).collect();
                labels.sort();
                labels
            })
            .collect();
        assert_eq!(
            figure(closing, "rosters counts, rosters ", ","),
            distinct.len(),
            "the rosters figure is the count of the distinct rosters over {name}"
        );
        assert_eq!(
            figure(closing, "rosters counts, rosters ", ","),
            found.len(),
            "the rosters figure is the count of the sections over {name}"
        );
        let standing = render_standing(&chain).expect("the standing reads");
        let standing_closing = standing.last().expect("a closing line");
        assert_eq!(
            figure(closing, "entries ", ","),
            figure(standing_closing, "entries ", ","),
            "the entries figure is the standing reading's over {name}"
        );
        assert_eq!(
            figure(closing, "atlases ", ","),
            chain.atlases.len(),
            "the atlases figure over {name}"
        );
        let singletons = found
            .iter()
            .filter(|(_, atlases)| atlases.len() == 1)
            .count();
        assert_eq!(
            singletons, expected_singletons[chain_index],
            "the sections holding one atlas stand as counted over {name}"
        );
        let summed: usize = by_atlas.iter().map(|section| section.len()).sum();
        let standings = render_standings(&chain).expect("the standings read");
        let standings_closing = standings.last().expect("a closing line");
        assert_eq!(
            summed,
            figure(standings_closing, "standings counts, standings ", ","),
            "each atlas counted at the size of its own roster sums to the standings figure over {name}"
        );
        assert_eq!(summed, expected_sums[chain_index], "the sum over {name}");
        let printed: usize = found.iter().map(|(heading, _)| heading.len()).sum();
        assert_eq!(
            printed, expected_headings[chain_index],
            "the heading lines printed over {name}"
        );
    }
}

#[test]
fn two_atlases_share_a_section_exactly_when_their_sets_sections_agree() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_rosters(&chain).expect("the rosters read");
        let found = rosters_sections(&lines);
        let sets = render_sets(&chain).expect("the sets read");
        let sections = sets_sections(&sets);
        let naming = |number: usize| -> Vec<usize> {
            sections
                .iter()
                .enumerate()
                .filter(|(_, (atlases, _))| atlases.contains(&number))
                .map(|(index, _)| index)
                .collect()
        };
        for first in 1..=chain.atlases.len() {
            for second in 1..=chain.atlases.len() {
                let together = found
                    .iter()
                    .any(|(_, atlases)| atlases.contains(&first) && atlases.contains(&second));
                assert_eq!(
                    together,
                    naming(first) == naming(second),
                    "atlases {first} and {second} stand together exactly when the sets sections naming them agree over {name}"
                );
            }
        }
    }
}

#[test]
fn the_widest_roster_holds_its_atlases_and_the_part_roster_its_one() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", &["wide a", "wide b", "narrow"]),
            atlas("Second", &["wide a", "wide b", "narrow"]),
            atlas("Third", &["wide a", "wide b"]),
        ],
    };
    let lines = render_rosters(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry wide a, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry wide b, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry narrow, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, does not stand at the close",
            "atlas 1",
            "atlas 2",
            "entry wide a, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry wide b, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "atlas 3",
            "rosters counts, rosters 2, atlases 3, entries 3",
        ])
    );
}

#[test]
fn the_parting_heading_stands_in_first_standing_order_against_the_member_order() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", &["early"]),
            atlas("Second", &["late", "early"]),
        ],
    };
    let lines = render_rosters(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry early, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, stands at the close",
            "atlas 1",
            "entry early, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, stands at the close",
            "entry late, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, stands at the close",
            "atlas 2",
            "rosters counts, rosters 2, atlases 2, entries 2",
        ])
    );
}

#[test]
fn a_lineage_holding_fewer_than_two_atlases_is_refused() {
    let chain = Lineage {
        atlases: vec![atlas("Only", &["kept"])],
    };
    assert_eq!(
        render_rosters(&chain).expect_err("the short chain is refused"),
        "the lineage holds fewer than two atlases"
    );
}

#[test]
fn a_label_holding_an_unpaired_surrogate_is_refused_first() {
    let both = Atlas {
        title: pt("First"),
        entries: vec![AtlasEntry {
            label: PText(vec![0x61, 0xD800, 0x0A]),
            exploration: expl(),
        }],
    };
    let chain = Lineage {
        atlases: vec![both, atlas("Second", &["kept"])],
    };
    assert_eq!(
        render_rosters(&chain).expect_err("the doubly broken label names the surrogate first"),
        "an entry label holds an unpaired surrogate"
    );
}

#[test]
fn the_thin_chain_rosters_stand_as_counted() {
    let lines = render_rosters(&thin_chain(302)).expect("the chain reads");
    assert_eq!(lines.len(), 907);
    assert_eq!(
        lines.last().expect("a closing line"),
        "rosters counts, rosters 302, atlases 302, entries 302"
    );
    let sections = rosters_sections(&lines);
    assert_eq!(sections.len(), 302);
    assert!(sections
        .iter()
        .all(|(heading, atlases)| heading.len() == 2 && atlases.len() == 1));
    assert_eq!(
        sections[0].0,
        strings(&[
            "entry label-1, first stood at atlas 1, last stood at atlas 302, stands in 2 atlases, stands at the close",
            "entry label-2, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, does not stand at the close",
        ])
    );
    assert_eq!(sections[0].1, vec![1]);
    assert_eq!(
        sections[151].0,
        strings(&[
            "entry label-152, first stood at atlas 151, last stood at atlas 152, stands in 2 atlases, does not stand at the close",
            "entry label-153, first stood at atlas 152, last stood at atlas 153, stands in 2 atlases, does not stand at the close",
        ])
    );
    assert_eq!(sections[151].1, vec![152]);
    assert_eq!(
        sections[301].0,
        strings(&[
            "entry label-1, first stood at atlas 1, last stood at atlas 302, stands in 2 atlases, stands at the close",
            "entry label-302, first stood at atlas 301, last stood at atlas 302, stands in 2 atlases, stands at the close",
        ])
    );
    assert_eq!(sections[301].1, vec![302]);
}

#[test]
fn the_window_chain_rosters_stand_as_counted() {
    let lines = render_rosters(&window_chain()).expect("the chain reads");
    assert_eq!(lines.len(), 271);
    assert_eq!(
        lines.last().expect("a closing line"),
        "rosters counts, rosters 30, atlases 120, entries 30"
    );
    let sections = rosters_sections(&lines);
    assert_eq!(sections.len(), 30);
    assert!(sections
        .iter()
        .all(|(heading, atlases)| heading.len() == 5 && atlases.len() == 4));
    assert_eq!(
        sections[0].0,
        strings(&[
            "entry label-1, first stood at atlas 1, last stood at atlas 120, stands in 20 atlases, stands at the close",
            "entry label-2, first stood at atlas 1, last stood at atlas 120, stands in 20 atlases, stands at the close",
            "entry label-3, first stood at atlas 1, last stood at atlas 120, stands in 20 atlases, stands at the close",
            "entry label-4, first stood at atlas 1, last stood at atlas 120, stands in 20 atlases, stands at the close",
            "entry label-5, first stood at atlas 1, last stood at atlas 95, stands in 20 atlases, does not stand at the close",
        ])
    );
    assert_eq!(sections[0].1, vec![1, 31, 61, 91]);
    assert_eq!(
        sections[15].0,
        strings(&[
            "entry label-16, first stood at atlas 12, last stood at atlas 106, stands in 20 atlases, does not stand at the close",
            "entry label-17, first stood at atlas 13, last stood at atlas 107, stands in 20 atlases, does not stand at the close",
            "entry label-18, first stood at atlas 14, last stood at atlas 108, stands in 20 atlases, does not stand at the close",
            "entry label-19, first stood at atlas 15, last stood at atlas 109, stands in 20 atlases, does not stand at the close",
            "entry label-20, first stood at atlas 16, last stood at atlas 110, stands in 20 atlases, does not stand at the close",
        ])
    );
    assert_eq!(sections[15].1, vec![16, 46, 76, 106]);
    assert_eq!(
        sections[29].0,
        strings(&[
            "entry label-1, first stood at atlas 1, last stood at atlas 120, stands in 20 atlases, stands at the close",
            "entry label-2, first stood at atlas 1, last stood at atlas 120, stands in 20 atlases, stands at the close",
            "entry label-3, first stood at atlas 1, last stood at atlas 120, stands in 20 atlases, stands at the close",
            "entry label-4, first stood at atlas 1, last stood at atlas 120, stands in 20 atlases, stands at the close",
            "entry label-30, first stood at atlas 26, last stood at atlas 120, stands in 20 atlases, stands at the close",
        ])
    );
    assert_eq!(sections[29].1, vec![30, 60, 90, 120]);
}

#[test]
fn the_paired_chain_rosters_stand_as_counted() {
    let lines = render_rosters(&paired_chain()).expect("the chain reads");
    assert_eq!(lines.len(), 127);
    assert_eq!(
        lines.last().expect("a closing line"),
        "rosters counts, rosters 2, atlases 120, entries 5"
    );
    let sections = rosters_sections(&lines);
    assert_eq!(sections.len(), 2);
    assert_eq!(
        sections[0].0,
        strings(&[
            "entry label-2, first stood at atlas 1, last stood at atlas 120, stands in 120 atlases, stands at the close",
            "entry label-1, first stood at atlas 1, last stood at atlas 119, stands in 60 atlases, does not stand at the close",
            "entry label-4, first stood at atlas 1, last stood at atlas 119, stands in 60 atlases, does not stand at the close",
        ])
    );
    assert_eq!(sections[0].1, (1..=120).step_by(2).collect::<Vec<usize>>());
    assert_eq!(
        sections[1].0,
        strings(&[
            "entry label-2, first stood at atlas 1, last stood at atlas 120, stands in 120 atlases, stands at the close",
            "entry label-3, first stood at atlas 2, last stood at atlas 120, stands in 60 atlases, stands at the close",
            "entry label-5, first stood at atlas 2, last stood at atlas 120, stands in 60 atlases, stands at the close",
        ])
    );
    assert_eq!(sections[1].1, (2..=120).step_by(2).collect::<Vec<usize>>());
}

#[test]
fn the_rosters_gathering_relation_holds_at_every_shape() {
    let mut chains: Vec<Lineage> = CHAINS.iter().map(|name| load(name)).collect();
    chains.push(load("generated-chain-lineage.json"));
    chains.push(thin_chain(302));
    chains.push(window_chain());
    chains.push(paired_chain());
    for chain in &chains {
        let lines = render_rosters(chain).expect("the rosters read");
        let sections = rosters_sections(&lines);
        let presence = render_presence(chain).expect("the presence reads");
        let by_atlas = presence_sections(&presence);
        let standing = render_standing(chain).expect("the standing reads");
        let standing_lines = &standing[..standing.len() - 1];
        let distinct: std::collections::BTreeSet<Vec<String>> = by_atlas
            .iter()
            .map(|section| {
                let mut labels: Vec<String> = section.iter().map(|line| label_of(line)).collect();
                labels.sort();
                labels
            })
            .collect();
        assert_eq!(sections.len(), distinct.len());
        for (index, section) in by_atlas.iter().enumerate() {
            let number = index + 1;
            let holding: Vec<&(Vec<String>, Vec<usize>)> = sections
                .iter()
                .filter(|(_, atlases)| atlases.contains(&number))
                .collect();
            assert_eq!(holding.len(), 1);
            let heading = &holding[0].0;
            let mut heading_labels: Vec<String> =
                heading.iter().map(|line| label_of(line)).collect();
            heading_labels.sort();
            let mut present_labels: Vec<String> =
                section.iter().map(|line| label_of(line)).collect();
            present_labels.sort();
            assert_eq!(heading_labels, present_labels);
            let places: Vec<usize> = heading
                .iter()
                .map(|line| {
                    standing_lines
                        .iter()
                        .position(|held| held == line)
                        .expect("a heading line is a standing line")
                })
                .collect();
            let mut ordered = places.clone();
            ordered.sort_unstable();
            assert_eq!(places, ordered);
        }
        let earliest: Vec<usize> = sections.iter().map(|(_, atlases)| atlases[0]).collect();
        let mut ordered_earliest = earliest.clone();
        ordered_earliest.sort_unstable();
        assert_eq!(earliest, ordered_earliest);
        let closing = lines.last().expect("a closing line");
        assert_eq!(
            figure(closing, "rosters counts, rosters ", ","),
            distinct.len()
        );
        assert_eq!(figure(closing, "atlases ", ","), chain.atlases.len());
        let entries: usize = closing
            .rsplit("entries ")
            .next()
            .expect("an entries figure")
            .parse()
            .expect("a count");
        let distinct_labels: std::collections::BTreeSet<String> = by_atlas
            .iter()
            .flat_map(|section| section.iter().map(|line| label_of(line)))
            .collect();
        assert_eq!(entries, distinct_labels.len());
    }
}
