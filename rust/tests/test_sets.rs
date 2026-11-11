//! Tests for MY-38, the sets reading.
//!
//! The reading takes a lineage, reads the presence of
//! every label in the one presence pass MY-18 states,
//! gathers the labels by the set of atlases each
//! stands in, in one walk over the order of first
//! standing, and states one section for each distinct
//! set, the section opening with the atlas lines of
//! its set in chain order and holding the standing
//! line of each label holding the set, in the order
//! of first standing, closing with the sets counts.
//! The pins here read the reading over the shipped
//! chains and over hand built chains, and bind it to
//! the standings, standing, and company readings
//! through the five relations MY-38 states, each
//! computed from the neighbouring readings' own
//! outputs.
//!
//! The discrimination split of these pins stands as
//! follows, pin by pin. Pins 1 to 11 fail on the tree
//! before this slice by execution, because
//! render_sets stands nowhere on that tree, so the
//! file does not compile there and no pin of 1 to 11
//! can pass there. Pin 1, the winter golden byte for
//! byte, kills a reading that sections by first
//! atlas, whose first section would hold the harbour
//! lights entry beside the ferry link and lifeboat
//! cover entries, a reading that prints the entries
//! figure for its sets figure, and a reading that
//! lets a section hold a label whose set only meets
//! its own. The pin passes against a grouping by
//! count, and the posture is stated here in the
//! open: on the winter chain the counts 3, 3, 2, and
//! 1 partition the four entries exactly as their
//! sets do, so no winter pin can kill that grouping,
//! and pin 2 is its designated killer. Pin 2, the
//! radio golden byte for byte, kills the grouping by
//! count, under which the battery drain and log gaps
//! entries would share a section, their counts
//! agreeing at two while their sets stand at atlases
//! 1 and 3 against atlases 2 and 3, and it kills the
//! grouping by first atlas likewise. The closing
//! half of the pin passes against a closing that
//! prints the entries figure for the sets figure,
//! the radio chain holding four sets over four
//! entries, whose killers are pins 1, 8, and 21.
//! Pin 3, the constructed golden byte for byte,
//! kills the grouping by first atlas, under which
//! the solo line, middle line, and keeper line
//! entries would share the first section. The pin
//! passes against a section order by set size, and
//! that posture is stated here too: on the
//! constructed chain the size order and the earliest
//! label order agree, whose killers are pins 1 and
//! 2, on whose chains the two orders part. Pin 4,
//! the first relation, read as the partition against
//! the standings reading's own output, kills a label
//! standing in two sections or in none, and a
//! section whose atlas lines are not the atlas lines
//! of its labels' standings sections. Pin 5, the
//! second relation, kills a gathering that joins
//! labels whose sets differ or parts labels whose
//! sets agree, computed both ways against the
//! standings sections. Pin 6, the third relation,
//! kills a section whose atlas lines name a first
//! atlas, a last atlas, or a count the standing
//! lines under them do not state, and a one line
//! heading in place of the atlas lines. Pin 7, the
//! fourth relation, kills a body line that is not
//! the standing line of its entry byte for byte, a
//! section standing at any place but its earliest
//! label's place, and body lines standing out of
//! first standing order. Pin 8, the fifth relation,
//! kills a closing whose sets figure is not the
//! count of the distinct sets the standings output
//! names, whose entries figure is not the standing
//! closing's, or whose stated sum the sections do
//! not bear out. Pin 9 kills a gathering whose
//! sections join entries holding company apart from
//! each other that differs, read against the company
//! reading's own output, its winter carriers the
//! ferry link and lifeboat cover entries. Pin 10
//! kills a widest set's section that misses an entry
//! standing in every atlas, and a single atlas set's
//! section holding any but its one entry. Pin 11
//! kills a set that names an atlas twice for a label
//! held twice in it, held from birth here under the
//! presence pass's set posture, as the family's
//! doubled label pins hold their readings. Pins 12
//! and 13 are the refusal battery, and the design
//! states their split in the open. The refusals they
//! pin, fewer than two atlases refused and a label
//! of one unpaired surrogate beside a line break
//! refused for the surrogate, stand in the one
//! presence pass the reading consumes unchanged, so
//! the behaviour the two pins pin stands whole on
//! the tree before this slice in the pass the
//! family's readings share, and the pins pass there
//! by design on that ground. Inside this file the
//! two pins share the file's compile absence on that
//! tree, because render_sets stands nowhere there,
//! as the pins of every reading file of the family
//! do.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_company, render_presence, render_sets, render_standing,
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

/// The sections of a company reading, each its
/// heading line and its company lines, cut by the
/// sizes the presence reading's own sections gather
/// for each label in first standing order, as the
/// company file's own cutter cuts them.
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

#[test]
fn the_winter_sets_stand_as_counted() {
    let lines = render_sets(&load("winter-spring-summer-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "atlas 2",
            "atlas 3",
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "atlas 1",
            "atlas 3",
            "entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "atlas 2",
            "entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "sets counts, sets 3, entries 4, atlases 3",
        ])
    );
}

#[test]
fn the_radio_sets_stand_as_counted() {
    let lines = render_sets(&load("radio-investigation-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "atlas 2",
            "atlas 3",
            "entry radio check, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "atlas 1",
            "atlas 3",
            "entry battery drain, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "atlas 2",
            "entry aerial icing, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "atlas 2",
            "atlas 3",
            "entry log gaps, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "sets counts, sets 4, entries 4, atlases 3",
        ])
    );
}

#[test]
fn the_constructed_sets_stand_as_counted() {
    let lines = render_sets(&load("constructed-departures-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "entry solo line, first stood at atlas 1, last stood at atlas 1, stands in 1 atlases, does not stand at the close",
            "atlas 1",
            "atlas 2",
            "entry middle line, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, does not stand at the close",
            "atlas 1",
            "atlas 2",
            "atlas 3",
            "entry keeper line, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "atlas 2",
            "atlas 3",
            "entry late line, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "sets counts, sets 4, entries 4, atlases 3",
        ])
    );
}

#[test]
fn each_label_stands_in_exactly_one_section_matching_its_standings_section() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_sets(&chain).expect("the sets read");
        let found = sets_sections(&lines);
        let standings = render_standings(&chain).expect("the standings read");
        let sets = standings_sections(&standings);
        for (heading, numbers) in &sets {
            let label = label_of(heading);
            let holding: Vec<&(Vec<usize>, Vec<String>)> = found
                .iter()
                .filter(|(_, entries)| entries.iter().any(|line| label_of(line) == label))
                .collect();
            assert_eq!(
                holding.len(),
                1,
                "the label {label} stands in exactly one section over {name}"
            );
            assert_eq!(
                &holding[0].0, numbers,
                "the section of {label} carries the atlas lines of its standings section over {name}"
            );
        }
        let named: usize = found.iter().map(|(_, entries)| entries.len()).sum();
        assert_eq!(
            named,
            sets.len(),
            "the labels over all sections count the chain's labels over {name}"
        );
        assert_eq!(
            found.len(),
            sets.iter()
                .map(|(_, numbers)| numbers)
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            "the sections count the distinct sets over {name}"
        );
    }
}

#[test]
fn two_labels_share_a_section_exactly_when_their_standings_sections_agree() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_sets(&chain).expect("the sets read");
        let found = sets_sections(&lines);
        let standings = render_standings(&chain).expect("the standings read");
        let sets = standings_sections(&standings);
        for (first_heading, first_numbers) in &sets {
            for (second_heading, second_numbers) in &sets {
                let first = label_of(first_heading);
                let second = label_of(second_heading);
                let together = found.iter().any(|(_, entries)| {
                    entries.iter().any(|line| label_of(line) == first)
                        && entries.iter().any(|line| label_of(line) == second)
                });
                assert_eq!(
                    together,
                    first_numbers == second_numbers,
                    "{first} and {second} stand together exactly on the equality of their sets over {name}"
                );
            }
        }
    }
}

#[test]
fn each_sections_atlas_lines_agree_with_the_lines_under_them() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_sets(&chain).expect("the sets read");
        let found = sets_sections(&lines);
        let total = chain.atlases.len();
        for (numbers, entries) in &found {
            assert!(
                !entries.is_empty(),
                "a section holds its labels over {name}"
            );
            for line in entries {
                assert_eq!(
                    figure(line, "first stood at atlas ", ","),
                    numbers[0],
                    "the first atlas line is the first atlas of its lines over {name}"
                );
                assert_eq!(
                    figure(line, "last stood at atlas ", ","),
                    numbers[numbers.len() - 1],
                    "the last atlas line is the last atlas of its lines over {name}"
                );
                assert_eq!(
                    figure(line, ", stands in ", " atlases"),
                    numbers.len(),
                    "the atlas lines count the standings of their lines over {name}"
                );
            }
            let closes = entries
                .iter()
                .all(|line| line.ends_with("stands at the close"));
            assert_eq!(
                numbers[numbers.len() - 1] == total,
                closes,
                "the last atlas line is the chain's last atlas exactly when its lines stand at the close over {name}"
            );
        }
    }
}

#[test]
fn every_line_is_the_standing_line_of_its_entry_and_the_sections_stand_in_order() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_sets(&chain).expect("the sets read");
        let found = sets_sections(&lines);
        let standing = render_standing(&chain).expect("the standing reads");
        let standing_lines = &standing[..standing.len() - 1];
        let place_of = |line: &String| {
            standing_lines
                .iter()
                .position(|held| held == line)
                .expect("a body line is a standing line of the reading")
        };
        let mut earliest: Vec<usize> = Vec::new();
        for (_, entries) in &found {
            let mut places: Vec<usize> = Vec::new();
            for line in entries {
                assert!(
                    standing_lines.contains(line),
                    "a body line is the standing line of its entry byte for byte over {name}"
                );
                places.push(place_of(line));
            }
            let mut ordered = places.clone();
            ordered.sort_unstable();
            assert_eq!(
                places, ordered,
                "the body lines stand in first standing order over {name}"
            );
            earliest.push(places[0]);
        }
        let mut ordered = earliest.clone();
        ordered.sort_unstable();
        assert_eq!(
            earliest, ordered,
            "the sections stand at their earliest label's place over {name}"
        );
    }
}

#[test]
fn the_closing_agrees_with_the_sections_and_the_standings() {
    let expected_singletons = [2usize, 4, 4];
    let expected_sums = [9usize, 8, 8];
    for (chain_index, name) in CHAINS.iter().enumerate() {
        let chain = load(name);
        let lines = render_sets(&chain).expect("the sets read");
        let found = sets_sections(&lines);
        let closing = lines.last().expect("a closing line");
        let standings = render_standings(&chain).expect("the standings read");
        let sets = standings_sections(&standings);
        let distinct: std::collections::BTreeSet<&Vec<usize>> =
            sets.iter().map(|(_, numbers)| numbers).collect();
        assert_eq!(
            figure(closing, "sets counts, sets ", ","),
            distinct.len(),
            "the sets figure is the count of the distinct sets over {name}"
        );
        let standing = render_standing(&chain).expect("the standing reads");
        let standing_closing = standing.last().expect("a closing line");
        assert_eq!(
            figure(closing, "entries ", ","),
            figure(standing_closing, "entries ", ","),
            "the entries figure is the standing reading's over {name}"
        );
        let singletons = found
            .iter()
            .filter(|(_, entries)| entries.len() == 1)
            .count();
        let lonely = sets
            .iter()
            .filter(|(_, numbers)| sets.iter().filter(|(_, held)| held == numbers).count() == 1)
            .count();
        assert_eq!(
            singletons, lonely,
            "the sections holding one label are the labels whose set no other label holds over {name}"
        );
        assert_eq!(
            singletons, expected_singletons[chain_index],
            "the singleton sections stand as counted over {name}"
        );
        let summed: usize = found
            .iter()
            .map(|(numbers, entries)| numbers.len() * entries.len())
            .sum();
        let standings_closing = standings.last().expect("a closing line");
        assert_eq!(
            summed,
            figure(standings_closing, "standings counts, standings ", ","),
            "each label's set counted once at its own section sums to the standings figure over {name}"
        );
        assert_eq!(summed, expected_sums[chain_index], "the sum over {name}");
        assert_eq!(
            figure_last(closing, "atlases "),
            chain.atlases.len(),
            "the atlases figure over {name}"
        );
    }
}

#[test]
fn labels_in_one_section_hold_the_same_company_apart_from_each_other() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_sets(&chain).expect("the sets read");
        let found = sets_sections(&lines);
        let company = render_company(&chain).expect("the company reads");
        let sections = company_sections(&company, &chain);
        let company_of = |label: &str| -> Vec<String> {
            sections
                .iter()
                .find(|(heading, _)| label_of(heading) == label)
                .expect("the label holds a company section")
                .1
                .iter()
                .map(|line| label_of(line))
                .collect()
        };
        for (_, entries) in &found {
            let labels: Vec<String> = entries.iter().map(|line| label_of(line)).collect();
            for first in &labels {
                for second in &labels {
                    if first == second {
                        continue;
                    }
                    let mut first_company = company_of(first);
                    first_company.retain(|held| held != second);
                    let mut second_company = company_of(second);
                    second_company.retain(|held| held != first);
                    assert_eq!(
                        first_company, second_company,
                        "{first} and {second} hold the same company apart from each other over {name}"
                    );
                    assert!(
                        company_of(first).contains(second),
                        "{second} stands in the company of {first} over {name}"
                    );
                }
            }
        }
    }
}

#[test]
fn the_widest_set_holds_its_entries_and_the_single_atlas_set_its_one() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", &["wide a", "wide b", "narrow"]),
            atlas("Second", &["wide a", "wide b"]),
            atlas("Third", &["wide a", "wide b"]),
        ],
    };
    let lines = render_sets(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "atlas 2",
            "atlas 3",
            "entry wide a, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry wide b, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "atlas 1",
            "entry narrow, first stood at atlas 1, last stood at atlas 1, stands in 1 atlases, does not stand at the close",
            "sets counts, sets 2, entries 3, atlases 3",
        ])
    );
}

#[test]
fn a_label_held_twice_in_one_atlas_names_that_atlas_once_in_its_set() {
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
    let lines = render_sets(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "atlas 2",
            "entry held twice, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, stands at the close",
            "entry companion, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, stands at the close",
            "sets counts, sets 1, entries 2, atlases 2",
        ])
    );
}

#[test]
fn a_lineage_holding_fewer_than_two_atlases_is_refused() {
    let chain = Lineage {
        atlases: vec![atlas("Only", &["kept"])],
    };
    assert_eq!(
        render_sets(&chain).expect_err("the short chain is refused"),
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
        render_sets(&chain).expect_err("the doubly broken label names the surrogate first"),
        "an entry label holds an unpaired surrogate"
    );
}
