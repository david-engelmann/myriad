//! Tests for MY-36, the company reading.
//!
//! The reading takes a lineage, reads the presence of
//! every label in the one presence pass MY-18 states,
//! gathers for each label the labels that share an
//! atlas with it, and states one section for each
//! label of the chain, the section opening with the
//! entry's standing line and holding the standing
//! line of each entry it stands with, in the order of
//! first standing, closing with the company counts.
//! The pins here read the reading over the shipped
//! chains and over hand built chains, and bind it to
//! the presence, standing, and standings readings
//! through the five relations MY-36 states, each
//! computed from the neighbouring readings' own
//! outputs.
//!
//! The discrimination split of these pins stands as
//! follows, pin by pin. Pins 1 to 11 fail on the tree
//! before this slice by execution, because
//! render_company stands nowhere on that tree, so
//! the file does not compile there and no pin of 1 to
//! 11 can pass there. Pin 1, the winter golden byte
//! for byte, kills a reading that sections by atlas,
//! by step, or in any order but first standing, and a
//! company gathered in atlas walk order. Pin 2, the
//! radio golden byte for byte, kills a battery drain
//! section that names the aerial icing entry, which
//! it shares no atlas with, and a log gaps section
//! in walk order. Pin 3, the constructed golden byte
//! for byte, kills a solo line section that names
//! the late line entry, which stands in no atlas
//! beside it. Pin 4, the first relation, kills a
//! section that names an entry the presence sections
//! never gather with its heading, or omits one they
//! gather, read as the transpose against the
//! presence reading's own output. Pin 5, the second
//! relation, kills a company line that stands
//! without its mirror line in the other entry's
//! section, a section that names its own entry, and
//! a closing that counts the unordered pairs once
//! rather than once from each end. Pin 6, the third
//! relation, kills a company gathered from anything
//! but the standings sets of the two entries, read
//! as the overlap of the standings reading's own
//! sections. Pin 7, the fourth relation, kills a
//! heading taken from any construction but the one
//! standing line helper, and company lines standing
//! in any order but first standing. Pin 8, the
//! fifth relation, kills a closing whose company
//! figure is not the sum of its sections' company
//! lines, or whose entries figure is but the
//! standing reading's, and a section counted empty
//! that is not a solo standing. Pin 9 kills a
//! section for an entry that shares no atlas with
//! another holding any company line at all. Pin 10
//! kills a section of an entry standing in every
//! atlas that misses an entry of the chain, and a
//! section of any other entry that misses it. Pin
//! 11 kills a label held twice in one atlas named
//! twice in a company, held from birth here under
//! the presence pass's set posture, as the family's
//! doubled label pins hold their readings. Pins 12
//! and 13 are the refusal battery, and the design
//! states their split in the open. The refusals
//! they pin, fewer than two atlases refused and a
//! label of one unpaired surrogate beside a line
//! break refused for the surrogate, stand in the
//! one presence pass the reading consumes
//! unchanged, so the behaviour the two pins pin
//! stands whole on the tree before this slice in
//! the pass the family's readings share, and the
//! pins pass there by design on that ground. Inside
//! this file the two pins share the file's compile
//! absence on that tree, because render_company
//! stands nowhere there, as the pins of every
//! reading file of the family do.

//! Round 22 adds three pins to this file, and their
//! discrimination split stands as follows. The fold
//! of MY-45 is behaviour neutral, so the behaviour
//! the three pins pin stands whole on the tree
//! before this slice, in the shipped render_company,
//! and the three pins pass there by design, as
//! guards of a restatement and not as witnesses of
//! an absence. Two forms of the fold stand behaviour
//! identical to the correct fold on every chain,
//! MC-1, the pair scan restored, and MC-2, the lines
//! taken from the standing line helper, the scan and
//! the walk gathering the same company and the
//! helper and the inversion stating the same
//! figures, so every pin of this file passes under
//! both, and their discrimination stands on the
//! contract text of MY-45, on the reviews' line
//! read of the one function, and on the fold
//! record's measured times, as MY-46 states. Under
//! MC-3, the self company, 11 pins of this file
//! fail, the three counted golden pins among them,
//! beside the generated chain company golden and
//! company relations. Under MC-4, the reversed
//! company, 7 pins of this file fail, the three
//! counted golden pins among them, beside the same
//! two generated chain pins. Under MC-5, the first
//! atlas gathering, 9 pins of this file fail, the
//! three counted golden pins among them, beside
//! the same two. Under MC-6, the span line, 3 pins
//! of this file fail, the winter and radio counted
//! pins and the standing line order pin, the
//! constructed chain's figures standing span equal
//! to count throughout, beside the same two. The
//! thin chain pin fails under MC-3, its sections
//! holding their own headings' labels and the
//! closing reading company 906 over 1,209 lines,
//! under MC-4, its first label's company lines
//! standing reversed, under MC-5, its first label
//! holding one company line and the closing reading
//! company 302, and under MC-6, its first label's
//! heading reading a span of 302 against standings
//! of two. The window chain pin fails under MC-3,
//! the closing reading company 270 over 301 lines,
//! under MC-4, its first label's company lines
//! standing reversed, under MC-5, its first label
//! holding four company lines in place of eight and
//! the closing reading company 120, and under MC-6,
//! its first label's heading reading a span of 120
//! against standings of twenty. The gathering
//! relation pin fails under MC-3, its computed
//! company holding no self line where the edit
//! prints one, under MC-4, its sections standing in
//! an order its computed order is not, under MC-5,
//! its computed company standing gathered from
//! every atlas where the edit gathers from one, and
//! under MC-6, its heading figures standing at
//! counts where the edit reads spans. Under MI-1
//! to MI-5 and MF-1 to MF-6 all three pins pass,
//! those edits standing in render_standings and
//! render_fellows and in no function the pins call.
//! The thirteen pins above stand unmoved.
//!
mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_company, render_presence, render_standing, render_standings, Atlas,
    AtlasEntry, Lineage,
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

/// The figure after a head that ends the line.
fn figure_last(line: &str, head: &str) -> usize {
    line.split(head)
        .nth(1)
        .unwrap_or_else(|| panic!("the figure head {head} stands in the line {line}"))
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

/// The sections of a standings reading, the closing
/// set aside, each section its heading line and the
/// numbers of its atlas lines in order.
fn standings_sections(lines: &[String]) -> Vec<(String, Vec<usize>)> {
    let mut out: Vec<(String, Vec<usize>)> = Vec::new();
    for line in lines {
        if line.starts_with("entry ") {
            out.push((line.clone(), Vec::new()));
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

/// The sections of a company reading, each its
/// heading line and its company lines. A company
/// line and a heading are both standing lines and no
/// mark on a line tells them apart, so the sections
/// are cut by the sizes the presence reading's own
/// sections gather for each label in first standing
/// order, the reading the transpose relation reads.
fn company_sections(lines: &[String], chain: &Lineage) -> Vec<(String, Vec<String>)> {
    let standing = render_standing(chain).expect("the standing reads");
    let order: Vec<String> = standing[..standing.len() - 1]
        .iter()
        .map(|line| label_of(line))
        .collect();
    let presence = render_presence(chain).expect("the presence reads");
    let by_atlas = presence_sections(&presence);
    let mut out: Vec<(String, Vec<String>)> = Vec::new();
    let mut cursor = 0usize;
    for label in &order {
        let heading = lines.get(cursor).expect("a section heading stands").clone();
        cursor += 1;
        let companions = order
            .iter()
            .filter(|other| {
                other.as_str() != label.as_str()
                    && by_atlas.iter().any(|section| {
                        section
                            .iter()
                            .any(|line| line.starts_with(&format!("entry {label},")))
                            && section
                                .iter()
                                .any(|line| line.starts_with(&format!("entry {other},")))
                    })
            })
            .count();
        let mut section_lines: Vec<String> = Vec::new();
        for _ in 0..companions {
            section_lines.push(lines.get(cursor).expect("a company line stands").clone());
            cursor += 1;
        }
        out.push((heading, section_lines));
    }
    assert_eq!(
        cursor,
        lines.len() - 1,
        "the sections cover the reading but its closing"
    );
    out
}

/// A hand built thin chain of n atlases over n
/// labels, atlas i holding labels i and i plus one
/// cyclically, each label standing in exactly two
/// atlases.
fn thin_chain(n: usize) -> Lineage {
    let mut atlases = Vec::new();
    for i in 0..n {
        let first = format!("label-{}", i + 1);
        let second = format!("label-{}", (i + 1) % n + 1);
        atlases.push(Atlas {
            title: pt(&format!("atlas-{}", i + 1)),
            entries: vec![
                AtlasEntry {
                    label: pt(&first),
                    exploration: expl(),
                },
                AtlasEntry {
                    label: pt(&second),
                    exploration: expl(),
                },
            ],
        });
    }
    Lineage { atlases }
}

/// A hand built window chain of 120 atlases over 30
/// labels, each atlas holding the five labels of a
/// window turning by one label per atlas, each label
/// standing in exactly twenty atlases.
fn window_chain() -> Lineage {
    let mut atlases = Vec::new();
    for i in 0..120usize {
        let mut entries = Vec::new();
        for j in 0..5usize {
            entries.push(AtlasEntry {
                label: pt(&format!("label-{}", (i + j) % 30 + 1)),
                exploration: expl(),
            });
        }
        atlases.push(Atlas {
            title: pt(&format!("atlas-{}", i + 1)),
            entries,
        });
    }
    Lineage { atlases }
}

#[test]
fn the_winter_company_stands_as_counted() {
    let lines =
        render_company(&load("winter-spring-summer-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "company counts, company 10, entries 4, atlases 3",
        ])
    );
}

#[test]
fn the_radio_company_stands_as_counted() {
    let lines = render_company(&load("radio-investigation-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry radio check, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry battery drain, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry aerial icing, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "entry log gaps, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry battery drain, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry radio check, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry log gaps, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry aerial icing, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "entry radio check, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry log gaps, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry log gaps, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry radio check, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry battery drain, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry aerial icing, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "company counts, company 10, entries 4, atlases 3",
        ])
    );
}

#[test]
fn the_constructed_company_stands_as_counted() {
    let lines =
        render_company(&load("constructed-departures-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry solo line, first stood at atlas 1, last stood at atlas 1, stands in 1 atlases, does not stand at the close",
            "entry middle line, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, does not stand at the close",
            "entry keeper line, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry middle line, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, does not stand at the close",
            "entry solo line, first stood at atlas 1, last stood at atlas 1, stands in 1 atlases, does not stand at the close",
            "entry keeper line, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry late line, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry keeper line, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry solo line, first stood at atlas 1, last stood at atlas 1, stands in 1 atlases, does not stand at the close",
            "entry middle line, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, does not stand at the close",
            "entry late line, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry late line, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry middle line, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, does not stand at the close",
            "entry keeper line, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "company counts, company 10, entries 4, atlases 3",
        ])
    );
}

#[test]
fn each_entry_stands_with_the_entries_the_presence_sections_gather() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_company(&chain).expect("the company reads");
        let found = company_sections(&lines, &chain);
        let presence = render_presence(&chain).expect("the presence reads");
        let by_atlas = presence_sections(&presence);
        let standing = render_standing(&chain).expect("the standing reads");
        let order: Vec<String> = standing[..standing.len() - 1]
            .iter()
            .map(|line| label_of(line))
            .collect();
        for (heading, company_lines) in &found {
            let label = label_of(heading);
            let expected: Vec<String> = order
                .iter()
                .filter(|other| {
                    other.as_str() != label.as_str()
                        && by_atlas.iter().any(|section| {
                            section
                                .iter()
                                .any(|line| line.starts_with(&format!("entry {label},")))
                                && section
                                    .iter()
                                    .any(|line| line.starts_with(&format!("entry {other},")))
                        })
                })
                .cloned()
                .collect();
            let named: Vec<String> = company_lines.iter().map(|line| label_of(line)).collect();
            assert_eq!(
                named, expected,
                "the company of {label} transposes the presence sections over {name}"
            );
        }
        let named: std::collections::BTreeSet<String> = by_atlas
            .iter()
            .flatten()
            .map(|line| label_of(line))
            .collect();
        assert_eq!(
            found.len(),
            named.len(),
            "the sections cover the named labels over {name}"
        );
    }
}

#[test]
fn standing_with_is_symmetric_and_the_closing_counts_pairs_twice() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_company(&chain).expect("the company reads");
        let found = company_sections(&lines, &chain);
        let heads: Vec<String> = found.iter().map(|(heading, _)| label_of(heading)).collect();
        let named: Vec<Vec<String>> = found
            .iter()
            .map(|(_, company_lines)| company_lines.iter().map(|line| label_of(line)).collect())
            .collect();
        for (index, head) in heads.iter().enumerate() {
            assert!(
                !named[index].contains(head),
                "an entry never stands in its own company over {name}"
            );
            for other in &named[index] {
                let mirror = heads
                    .iter()
                    .position(|held| held == other)
                    .expect("the named entry holds a section");
                assert!(
                    named[mirror].contains(head),
                    "{other} stands in the company of {head}, so {head} stands in the company of {other} over {name}"
                );
            }
        }
        let mut pairs = 0usize;
        for (first, named_first) in named.iter().enumerate() {
            for second in heads.iter().skip(first + 1) {
                if named_first.contains(second) {
                    pairs += 1;
                }
            }
        }
        let closing = lines.last().expect("a closing line");
        assert_eq!(
            figure(closing, "company counts, company ", ","),
            2 * pairs,
            "the closing counts each standing pair once from each end over {name}"
        );
    }
}

#[test]
fn two_entries_stand_with_each_other_when_their_standings_sections_meet() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_company(&chain).expect("the company reads");
        let found = company_sections(&lines, &chain);
        let standings = render_standings(&chain).expect("the standings read");
        let sets = standings_sections(&standings);
        for (heading, company_lines) in &found {
            let label = label_of(heading);
            let own = sets
                .iter()
                .find(|(held, _)| label_of(held) == label)
                .expect("the label holds a standings section");
            let expected: Vec<String> = sets
                .iter()
                .filter(|(held, numbers)| {
                    label_of(held) != label && numbers.iter().any(|number| own.1.contains(number))
                })
                .map(|(held, _)| label_of(held))
                .collect();
            let named: Vec<String> = company_lines.iter().map(|line| label_of(line)).collect();
            assert_eq!(
                named, expected,
                "the company of {label} is the entries whose standings sets meet its own over {name}"
            );
        }
    }
}

#[test]
fn every_line_is_the_standing_line_of_its_entry_in_first_standing_order() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_company(&chain).expect("the company reads");
        let found = company_sections(&lines, &chain);
        let standing = render_standing(&chain).expect("the standing reads");
        let standing_lines = &standing[..standing.len() - 1];
        let headings: Vec<String> = found.iter().map(|(heading, _)| heading.clone()).collect();
        assert_eq!(
            headings,
            standing_lines.to_vec(),
            "the headings are the standing lines in first standing order over {name}"
        );
        for (_, company_lines) in &found {
            let mut places: Vec<usize> = Vec::new();
            for line in company_lines {
                let place = standing_lines
                    .iter()
                    .position(|held| held == line)
                    .expect("a company line is a standing line of the reading");
                places.push(place);
            }
            let mut ordered = places.clone();
            ordered.sort_unstable();
            assert_eq!(
                places, ordered,
                "the company lines stand in first standing order over {name}"
            );
        }
    }
}

#[test]
fn the_closing_sums_agree_and_solo_entries_hold_empty_sections() {
    let expected_company = [10usize, 10, 10];
    for (chain_index, name) in CHAINS.iter().enumerate() {
        let chain = load(name);
        let lines = render_company(&chain).expect("the company reads");
        let found = company_sections(&lines, &chain);
        let closing = lines.last().expect("a closing line");
        let company = figure(closing, "company counts, company ", ",");
        assert_eq!(company, expected_company[chain_index], "over {name}");
        let summed: usize = found
            .iter()
            .map(|(_, company_lines)| company_lines.len())
            .sum();
        assert_eq!(
            company, summed,
            "the closing company figure is the sum of its sections over {name}"
        );
        let standing = render_standing(&chain).expect("the standing reads");
        let standing_closing = standing.last().expect("a closing line");
        assert_eq!(
            figure(closing, "entries ", ","),
            figure(standing_closing, "entries ", ","),
            "the entries figure is the standing reading's over {name}"
        );
        assert_eq!(
            figure_last(closing, "atlases "),
            chain.atlases.len(),
            "the atlases figure over {name}"
        );
        let presence = render_presence(&chain).expect("the presence reads");
        let by_atlas = presence_sections(&presence);
        let solo = found
            .iter()
            .filter(|(heading, _)| {
                let label = label_of(heading);
                by_atlas
                    .iter()
                    .filter(|section| {
                        section
                            .iter()
                            .any(|line| line.starts_with(&format!("entry {label},")))
                    })
                    .all(|section| section.len() == 1)
            })
            .count();
        let empty = found
            .iter()
            .filter(|(_, company_lines)| company_lines.is_empty())
            .count();
        assert_eq!(
            empty, solo,
            "the empty sections are the entries whose every atlas holds them alone over {name}"
        );
    }
}

#[test]
fn an_entry_sharing_no_atlas_with_another_holds_its_heading_alone() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", &["solo act"]),
            atlas("Second", &["pair a", "pair b"]),
        ],
    };
    let lines = render_company(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry solo act, first stood at atlas 1, last stood at atlas 1, stands in 1 atlases, does not stand at the close",
            "entry pair a, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, stands at the close",
            "entry pair b, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, stands at the close",
            "entry pair b, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, stands at the close",
            "entry pair a, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, stands at the close",
            "company counts, company 2, entries 3, atlases 2",
        ])
    );
}

#[test]
fn an_entry_standing_in_every_atlas_stands_with_every_other_entry() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", &["hub", "a"]),
            atlas("Second", &["hub", "b"]),
            atlas("Third", &["hub", "c"]),
        ],
    };
    let lines = render_company(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry hub, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry a, first stood at atlas 1, last stood at atlas 1, stands in 1 atlases, does not stand at the close",
            "entry b, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "entry c, first stood at atlas 3, last stood at atlas 3, stands in 1 atlases, stands at the close",
            "entry a, first stood at atlas 1, last stood at atlas 1, stands in 1 atlases, does not stand at the close",
            "entry hub, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry b, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "entry hub, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry c, first stood at atlas 3, last stood at atlas 3, stands in 1 atlases, stands at the close",
            "entry hub, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "company counts, company 6, entries 4, atlases 3",
        ])
    );
}

#[test]
fn a_label_held_twice_in_one_atlas_stands_once_in_each_company() {
    let doubled = Atlas {
        title: pt("First"),
        entries: vec![
            AtlasEntry {
                label: pt("held twice"),
                exploration: expl(),
            },
            AtlasEntry {
                label: pt("held twice"),
                exploration: expl(),
            },
            AtlasEntry {
                label: pt("companion"),
                exploration: expl(),
            },
        ],
    };
    let chain = Lineage {
        atlases: vec![doubled, atlas("Second", &["held twice", "companion"])],
    };
    let lines = render_company(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry held twice, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, stands at the close",
            "entry companion, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, stands at the close",
            "entry companion, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, stands at the close",
            "entry held twice, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, stands at the close",
            "company counts, company 2, entries 2, atlases 2",
        ])
    );
}

#[test]
fn a_lineage_holding_fewer_than_two_atlases_is_refused() {
    let chain = Lineage {
        atlases: vec![atlas("Only", &["kept"])],
    };
    assert_eq!(
        render_company(&chain).expect_err("the short chain is refused"),
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
        render_company(&chain).expect_err("the doubly broken label names the surrogate first"),
        "an entry label holds an unpaired surrogate"
    );
}

#[test]
fn the_thin_chain_company_sections_stand_as_counted() {
    let chain = thin_chain(302);
    let lines = render_company(&chain).expect("the chain reads");
    assert_eq!(lines.len(), 907);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("company counts, company 604, entries 302, atlases 302")
    );
    let found = company_sections(&lines, &chain);
    assert_eq!(found.len(), 302);
    assert_eq!(
        found[0].0.as_str(),
        "entry label-1, first stood at atlas 1, last stood at atlas 302, \
         stands in 2 atlases, stands at the close"
    );
    assert_eq!(
        found[0].1,
        strings(&[
            "entry label-2, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, does not stand at the close",
            "entry label-302, first stood at atlas 301, last stood at atlas 302, \
             stands in 2 atlases, stands at the close",
        ])
    );
    assert_eq!(
        found[149].0.as_str(),
        "entry label-150, first stood at atlas 149, last stood at atlas 150, \
         stands in 2 atlases, does not stand at the close"
    );
    assert_eq!(
        found[149]
            .1
            .iter()
            .map(|line| label_of(line))
            .collect::<Vec<_>>(),
        vec!["label-149".to_string(), "label-151".to_string()]
    );
    assert_eq!(
        found[301].0.as_str(),
        "entry label-302, first stood at atlas 301, last stood at atlas 302, \
         stands in 2 atlases, stands at the close"
    );
    assert_eq!(
        found[301]
            .1
            .iter()
            .map(|line| label_of(line))
            .collect::<Vec<_>>(),
        vec!["label-1".to_string(), "label-301".to_string()]
    );
    for (_, company_lines) in &found {
        assert_eq!(company_lines.len(), 2, "each label stands with two others");
    }
}

#[test]
fn the_window_chain_company_sections_stand_as_counted() {
    let chain = window_chain();
    let lines = render_company(&chain).expect("the chain reads");
    assert_eq!(lines.len(), 271);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("company counts, company 240, entries 30, atlases 120")
    );
    let found = company_sections(&lines, &chain);
    assert_eq!(found.len(), 30);
    for (index, (heading, company_lines)) in found.iter().enumerate() {
        assert_eq!(label_of(heading), format!("label-{}", index + 1));
        assert_eq!(figure(heading, "stands in ", " atlases,"), 20);
        assert_eq!(
            company_lines.len(),
            8,
            "each label stands with eight others"
        );
    }
    assert_eq!(
        found[0].0.as_str(),
        "entry label-1, first stood at atlas 1, last stood at atlas 120, \
         stands in 20 atlases, stands at the close"
    );
    assert_eq!(
        found[0]
            .1
            .iter()
            .map(|line| label_of(line))
            .collect::<Vec<_>>(),
        vec![
            "label-2".to_string(),
            "label-3".to_string(),
            "label-4".to_string(),
            "label-5".to_string(),
            "label-27".to_string(),
            "label-28".to_string(),
            "label-29".to_string(),
            "label-30".to_string(),
        ]
    );
}

#[test]
fn the_company_sections_agree_with_the_presence_and_standings_readings() {
    let mut chains: Vec<Lineage> = CHAINS.iter().map(|name| load(name)).collect();
    chains.push(thin_chain(302));
    for chain in &chains {
        let total = chain.atlases.len();
        let lines = render_company(chain).expect("the company reads");
        let found = company_sections(&lines, chain);
        let standing = render_standing(chain).expect("the standing reads");
        let standing_lines = &standing[..standing.len() - 1];
        let order: Vec<String> = standing_lines.iter().map(|line| label_of(line)).collect();
        let standings = render_standings(chain).expect("the standings read");
        let sets = standings_sections(&standings);
        let presence = render_presence(chain).expect("the presence reads");
        let by_atlas = presence_sections(&presence);
        assert_eq!(found.len(), order.len());
        for (index, (heading, company_lines)) in found.iter().enumerate() {
            let label = label_of(heading);
            assert_eq!(label, order[index]);
            let own = sets
                .iter()
                .find(|(held, _)| label_of(held) == label)
                .expect("the label holds a standings section");
            assert_eq!(
                figure(heading, "first stood at atlas ", ", last stood"),
                own.1[0]
            );
            assert_eq!(
                figure(heading, "last stood at atlas ", ", stands in"),
                own.1[own.1.len() - 1]
            );
            assert_eq!(figure(heading, "stands in ", " atlases,"), own.1.len());
            assert_eq!(
                heading.ends_with("stands at the close"),
                own.1[own.1.len() - 1] == total
            );
            let mut gathered: Vec<String> = Vec::new();
            for atlas_no in &own.1 {
                for line in &by_atlas[atlas_no - 1] {
                    let other = label_of(line);
                    if other != label && !gathered.contains(&other) {
                        gathered.push(other);
                    }
                }
            }
            let expected: Vec<String> = order
                .iter()
                .filter(|held| gathered.contains(held))
                .cloned()
                .collect();
            let named: Vec<String> = company_lines.iter().map(|line| label_of(line)).collect();
            assert_eq!(named, expected, "the company of {label} as gathered");
            for line in company_lines {
                let named_label = label_of(line);
                let standing_line = standing_lines
                    .iter()
                    .find(|held| label_of(held) == named_label)
                    .expect("the named entry holds a standing line");
                assert_eq!(line, standing_line, "a company line is a standing line");
            }
        }
    }
}
