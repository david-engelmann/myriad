//! Tests for MY-34, the standings reading.
//!
//! The reading takes a lineage, reads the presence of
//! every label in the one presence pass MY-18 states,
//! inverts the pass, and states one section for each
//! label of the chain, the section opening with the
//! entry's standing line and holding one bare atlas
//! line for each atlas the label stands in, closing
//! with the standings counts. The pins here read the
//! reading over the shipped chains and over hand built
//! chains, and bind it to the presence, standing,
//! runs, absence, and matched readings through the
//! five relations MY-34 states, each computed from
//! the neighbouring readings' own outputs.
//!
//! The discrimination split of these pins stands as
//! follows, pin by pin. Pins 1 to 11 fail on the tree
//! before this slice by execution, because
//! render_standings stands nowhere on that tree, so
//! the file does not compile there and no pin of 1 to
//! 11 can pass there. Pin 1, the winter golden byte
//! for byte, kills a reading that sections by atlas,
//! by step, or in any order but first standing. Pin
//! 2, the radio golden byte for byte, kills a section
//! order read from the last standing or from the
//! label text, and a battery drain section that
//! names the second atlas. Pin 3, the constructed
//! golden byte for byte, kills a closing read from
//! any path but the sections it restates. Pin 4, the
//! first relation, kills a section that names an
//! atlas the entry does not stand in or omits one it
//! stands in, read as the transpose against the
//! presence reading's own output. Pin 5, the second
//! relation, kills a heading taken from any
//! construction but the one standing line helper,
//! and a section whose atlas lines drift from its
//! heading's first atlas, last atlas, count of
//! atlases, or close words. Pin 6, the third
//! relation, kills a section whose consecutive
//! stretches are not the runs of the entry the runs
//! reading counts, in order, beginning, and length.
//! Pin 7, the fourth relation, kills a span read
//! that counts an atlas before the section's first
//! line or after its last line as absent, or that
//! loses an inside absence run. Pin 8, the fifth
//! relation's matched half, kills a section that
//! names one atlas of a step without the other where
//! the entry stands matched at that step, or both
//! where it does not. Pin 9, the fifth relation's
//! closing half, kills a closing that counts a
//! standing twice on one side of the sum, or states
//! an entries figure but the standing reading's.
//! Pin 10 kills a raw entry walk, the shape the
//! departures red team finding of Round 12 took,
//! held from birth here under the presence pass's
//! set posture, a label standing twice in one atlas
//! named once in its section and counted once in
//! the closing. Pin 11, the small span case, kills a
//! section that names the inside atlas of a gap or
//! drops a standing at either end of the span.
//! Pins 12 and 13 are the refusal battery, and the
//! design states their split in the open. The
//! refusals they pin, fewer than two atlases refused
//! and a label of one unpaired surrogate beside a
//! line break refused for the surrogate, stand in
//! the one presence pass the reading consumes
//! unchanged, so the behaviour the two pins pin
//! stands whole on the tree before this slice in
//! the pass the family's readings share, and the
//! pins pass there by design on that ground. Inside
//! this file the two pins share the file's compile
//! absence on that tree, because render_standings
//! stands nowhere there, as the pins of every
//! reading file of the family do.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_absence, render_matched, render_presence, render_runs,
    render_standing, render_standings, Atlas, AtlasEntry, Lineage,
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

/// The sections of a standings reading, the closing
/// set aside, each section its heading line and the
/// numbers of its atlas lines in order.
fn sections(lines: &[String]) -> Vec<(String, Vec<usize>)> {
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

/// The stretches of a list of atlas numbers, each the
/// first atlas of the stretch and its length.
fn stretches(numbers: &[usize]) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = Vec::new();
    for &number in numbers {
        match out.last_mut() {
            Some((start, length)) if *start + *length == number => *length += 1,
            _ => out.push((number, 1)),
        }
    }
    out
}

/// The run lines of one label in a runs or absence
/// reading, each the first atlas of the run and its
/// length, read from the reading's own lines.
fn runs_of(lines: &[String], label: &str, verb: &str) -> Vec<(usize, usize)> {
    lines
        .iter()
        .filter(|line| line.starts_with(&format!("entry {label},")))
        .map(|line| {
            (
                figure(line, &format!("{verb} atlas "), ","),
                figure(line, "for ", " atlases"),
            )
        })
        .collect()
}

#[test]
fn the_winter_sections_stand_as_counted() {
    let lines =
        render_standings(&load("winter-spring-summer-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "atlas 1",
            "atlas 2",
            "atlas 3",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "atlas 1",
            "atlas 2",
            "atlas 3",
            "entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "atlas 1",
            "atlas 3",
            "entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "atlas 2",
            "standings counts, standings 9, entries 4, atlases 3",
        ])
    );
}

#[test]
fn the_radio_sections_stand_as_counted() {
    let lines =
        render_standings(&load("radio-investigation-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry radio check, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "atlas 1",
            "atlas 2",
            "atlas 3",
            "entry battery drain, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "atlas 1",
            "atlas 3",
            "entry aerial icing, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "atlas 2",
            "entry log gaps, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "atlas 2",
            "atlas 3",
            "standings counts, standings 8, entries 4, atlases 3",
        ])
    );
}

#[test]
fn the_constructed_sections_stand_as_counted() {
    let lines =
        render_standings(&load("constructed-departures-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry solo line, first stood at atlas 1, last stood at atlas 1, stands in 1 atlases, does not stand at the close",
            "atlas 1",
            "entry middle line, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, does not stand at the close",
            "atlas 1",
            "atlas 2",
            "entry keeper line, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "atlas 1",
            "atlas 2",
            "atlas 3",
            "entry late line, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "atlas 2",
            "atlas 3",
            "standings counts, standings 8, entries 4, atlases 3",
        ])
    );
}

#[test]
fn each_section_is_the_transpose_of_the_presence_sections() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_standings(&chain).expect("the standings read");
        let found = sections(&lines);
        let presence = render_presence(&chain).expect("the presence reads");
        let by_atlas = presence_sections(&presence);
        for (heading, numbers) in &found {
            let label = label_of(heading);
            let expected: Vec<usize> = by_atlas
                .iter()
                .enumerate()
                .filter(|(_, section)| {
                    section
                        .iter()
                        .any(|line| line.starts_with(&format!("entry {label},")))
                })
                .map(|(index, _)| index + 1)
                .collect();
            assert_eq!(
                numbers, &expected,
                "the section of {label} transposes the presence sections over {name}"
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
fn each_section_agrees_with_its_heading() {
    for name in CHAINS {
        let chain = load(name);
        let total = chain.atlases.len();
        let lines = render_standings(&chain).expect("the standings read");
        for (heading, numbers) in sections(&lines) {
            assert_eq!(
                figure(&heading, "first stood at atlas ", ","),
                numbers[0],
                "the heading's first atlas is the section's first line over {name}"
            );
            assert_eq!(
                figure(&heading, "last stood at atlas ", ","),
                *numbers.last().expect("a section holds a line"),
                "the heading's last atlas is the section's last line over {name}"
            );
            assert_eq!(
                figure(&heading, "stands in ", " atlases"),
                numbers.len(),
                "the heading's count is the section's line count over {name}"
            );
            assert_eq!(
                heading.ends_with("stands at the close"),
                *numbers.last().expect("a section holds a line") == total,
                "the close words stand exactly at the last atlas over {name}"
            );
        }
    }
}

#[test]
fn the_section_stretches_are_the_runs_of_the_entry() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_standings(&chain).expect("the standings read");
        let runs = render_runs(&chain).expect("the runs read");
        for (heading, numbers) in sections(&lines) {
            let label = label_of(&heading);
            assert_eq!(
                stretches(&numbers),
                runs_of(&runs, &label, "standing from"),
                "the stretches of {label} are its runs over {name}"
            );
        }
    }
}

#[test]
fn the_span_gaps_are_the_inside_absence_runs() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_standings(&chain).expect("the standings read");
        let absence = render_absence(&chain).expect("the absence reads");
        for (heading, numbers) in sections(&lines) {
            let label = label_of(&heading);
            let first = numbers[0];
            let last = *numbers.last().expect("a section holds a line");
            let inside: Vec<(usize, usize)> = runs_of(&absence, &label, "absent from")
                .into_iter()
                .filter(|(start, length)| *start >= first && *start + *length - 1 <= last)
                .collect();
            let unnamed: Vec<usize> = (first..=last)
                .filter(|number| !numbers.contains(number))
                .collect();
            assert_eq!(
                inside,
                stretches(&unnamed),
                "the inside absence runs of {label} are its span gaps over {name}"
            );
            let lengths: usize = inside.iter().map(|(_, length)| length).sum();
            assert_eq!(
                lengths,
                (last - first + 1) - numbers.len(),
                "the span of {label} less its lines is its inside absence over {name}"
            );
        }
    }
    // The full span sections the design names hold no
    // inside run, and the one atlas span names it whole.
    let winter =
        render_standings(&load("winter-spring-summer-lineage.json")).expect("the chain reads");
    let winter_absence =
        render_absence(&load("winter-spring-summer-lineage.json")).expect("the absence reads");
    for label in ["ferry link", "lifeboat cover"] {
        let inside: Vec<(usize, usize)> = runs_of(&winter_absence, label, "absent from")
            .into_iter()
            .filter(|(start, length)| *start >= 1 && *start + *length - 1 <= 3)
            .collect();
        assert!(inside.is_empty(), "{label} holds no inside absence run");
    }
    let radio =
        render_standings(&load("radio-investigation-lineage.json")).expect("the chain reads");
    let keeper = sections(
        &render_standings(&load("constructed-departures-lineage.json")).expect("the chain reads"),
    )
    .into_iter()
    .find(|(heading, _)| label_of(heading) == "keeper line")
    .expect("the keeper section stands");
    assert_eq!(keeper.1, vec![1, 2, 3], "the keeper span stands whole");
    let ice = sections(&winter)
        .into_iter()
        .find(|(heading, _)| label_of(heading) == "ice report")
        .expect("the ice report section stands");
    assert_eq!(ice.1, vec![2], "the one atlas span names it whole");
    let radio_check = sections(&radio)
        .into_iter()
        .find(|(heading, _)| label_of(heading) == "radio check")
        .expect("the radio check section stands");
    assert_eq!(
        radio_check.1,
        vec![1, 2, 3],
        "the radio check span stands whole"
    );
}

#[test]
fn the_sections_naming_both_atlases_of_a_step_are_its_matched() {
    let expected_steps: [[usize; 2]; 3] = [[2, 2], [1, 2], [2, 2]];
    for (chain_index, name) in CHAINS.iter().enumerate() {
        let chain = load(name);
        let total = chain.atlases.len();
        let lines = render_standings(&chain).expect("the standings read");
        let found = sections(&lines);
        let matched = render_matched(&chain).expect("the matched reads");
        let matched_closing = matched.last().expect("a closing line");
        for step in 1..total {
            let both = found
                .iter()
                .filter(|(_, numbers)| numbers.contains(&step) && numbers.contains(&(step + 1)))
                .count();
            assert_eq!(
                both,
                figure(matched_closing, &format!("step {step} matched "), ","),
                "the sections naming both atlases of step {step} over {name}"
            );
            assert_eq!(
                both,
                expected_steps[chain_index][step - 1],
                "the counted matched figure of step {step} over {name}"
            );
        }
    }
}

#[test]
fn the_closing_sums_agree_by_entry_and_by_atlas() {
    let expected_standings = [9usize, 8, 8];
    for (chain_index, name) in CHAINS.iter().enumerate() {
        let chain = load(name);
        let lines = render_standings(&chain).expect("the standings read");
        let closing = lines.last().expect("a closing line");
        let standings = figure(closing, "standings counts, standings ", ",");
        assert_eq!(standings, expected_standings[chain_index], "over {name}");
        let presence = render_presence(&chain).expect("the presence reads");
        let presence_closing = presence.last().expect("a closing line");
        let by_atlas: usize = (1..=chain.atlases.len())
            .map(|index| figure(presence_closing, &format!("atlas {index} standing "), ","))
            .sum();
        assert_eq!(
            standings, by_atlas,
            "the standings counted by entry and by atlas agree over {name}"
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
    }
}

#[test]
fn a_label_held_twice_in_one_atlas_is_named_once_in_its_section() {
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
        ],
    };
    let chain = Lineage {
        atlases: vec![doubled, atlas("Second", &["held twice"])],
    };
    let lines = render_standings(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry held twice, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, stands at the close",
            "atlas 1",
            "atlas 2",
            "standings counts, standings 2, entries 1, atlases 2",
        ])
    );
}

#[test]
fn a_small_span_names_its_ends_and_no_line_for_its_gap() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", &["gap", "other"]),
            atlas("Second", &["other"]),
            atlas("Third", &["gap", "other"]),
        ],
    };
    let lines = render_standings(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry gap, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "atlas 1",
            "atlas 3",
            "entry other, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "atlas 1",
            "atlas 2",
            "atlas 3",
            "standings counts, standings 5, entries 2, atlases 3",
        ])
    );
    let absence = render_absence(&chain).expect("the absence reads");
    assert_eq!(
        runs_of(&absence, "gap", "absent from"),
        vec![(2, 1)],
        "the gap holds one inside absence run of length 1"
    );
}

#[test]
fn a_lineage_holding_fewer_than_two_atlases_is_refused() {
    let chain = Lineage {
        atlases: vec![atlas("Only", &["kept"])],
    };
    assert_eq!(
        render_standings(&chain).expect_err("the short chain is refused"),
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
        render_standings(&chain).expect_err("the doubly broken label names the surrogate first"),
        "an entry label holds an unpaired surrogate"
    );
}
