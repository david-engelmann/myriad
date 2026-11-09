//! Tests for MY-28, the matched reading.
//!
//! The reading takes a lineage, reads the presence of
//! every label in the one presence pass FIX-MY-13 states,
//! and states one section for each step of the chain, the
//! entries standing on both sides of the step named in
//! the standing line form in the order of the atlas the
//! step leads from, closing with the matched counts. The
//! pins here read the reading over the shipped chains and
//! over hand built chains, and bind its closings to the
//! lineage reading's own figures, to its per-step
//! changelog counts, and to the standing reading's lines.
//!
//! The discrimination split of these pins stands as
//! follows. Every pin in this file fails on the tree
//! before this slice by compile absence, because
//! render_matched stands nowhere on that tree, so no pin
//! here can pass there. Inside the slice, pin 4 computes
//! its figures through the lineage reading's per-step
//! changelog counts lines and not through the departures
//! closing, guarding the false identity the design of
//! this round caught, under which a chain whose entry
//! stands in the first and third atlases and not the
//! second would read as matched somewhere. Pin 7 isolates
//! that shape on its own. Pin 8 carries assertions that
//! fail against any walk that reads the raw entries of an
//! atlas rather than its distinct labels. No pin in this
//! file passes on the tree before this slice by design.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_lineage, render_matched, render_standing, Atlas, AtlasEntry, Lineage,
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

/// The sections of a matched reading, each opening line
/// with the entry lines under it, the closing line set
/// aside.
fn sections(lines: &[String]) -> Vec<Vec<String>> {
    let mut found: Vec<Vec<String>> = Vec::new();
    for line in lines {
        if line.starts_with("step ") && !line.contains("matched") {
            found.push(vec![line.clone()]);
        } else if line.starts_with("matched counts") {
            break;
        } else if let Some(section) = found.last_mut() {
            section.push(line.clone());
        }
    }
    found
}

/// The per-step matched counts of a matched reading's
/// closing line, in step order.
fn step_counts(closing: &str, steps: usize) -> Vec<usize> {
    (1..=steps)
        .map(|step| figure(closing, &format!("step {step} matched "), ","))
        .collect()
}

#[test]
fn the_matched_reading_states_its_form_over_the_winter_chain() {
    let lines =
        render_matched(&load("winter-spring-summer-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "step 1",
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, \
             stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, \
             stands in 3 atlases, stands at the close",
            "step 2",
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, \
             stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, \
             stands in 3 atlases, stands at the close",
            "matched counts, step 1 matched 2, step 2 matched 2, \
             entries matched 2, entries 4, steps 2",
        ])
    );
}

#[test]
fn the_matched_closing_restates_its_sections() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_matched(&chain).expect("the chain reads");
        let closing = lines.last().expect("a closing line stands");
        let steps = chain.atlases.len() - 1;
        let counts = step_counts(closing, steps);
        let found = sections(&lines);
        assert_eq!(found.len(), steps, "one section per step of {name}");
        let mut distinct: Vec<String> = Vec::new();
        for (index, section) in found.iter().enumerate() {
            assert_eq!(
                section.len() - 1,
                counts[index],
                "the closing restates the section of step {} of {name}",
                index + 1
            );
            for line in &section[1..] {
                let label = line
                    .strip_prefix("entry ")
                    .expect("an entry line stands")
                    .split(',')
                    .next()
                    .expect("a label stands")
                    .to_string();
                if !distinct.contains(&label) {
                    distinct.push(label);
                }
            }
        }
        let entries_matched = figure(closing, "entries matched ", ",");
        assert_eq!(
            entries_matched,
            distinct.len(),
            "the entries matched figure names the distinct labels of {name}"
        );
        let standing = render_standing(&chain).expect("the chain reads");
        let standing_closing = standing.last().expect("a closing line stands");
        let at_every = figure(standing_closing, "standing at every atlas ", ",");
        assert!(
            entries_matched >= at_every,
            "the entries matched of {name} stand no smaller than the standing at every atlas"
        );
    }
}

#[test]
fn the_matched_counts_sum_to_the_lineage_matched_figure() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_matched(&chain).expect("the chain reads");
        let closing = lines.last().expect("a closing line stands");
        let steps = chain.atlases.len() - 1;
        let sum: usize = step_counts(closing, steps).iter().sum();
        let lineage = render_lineage(&chain).expect("the chain reads");
        let lineage_closing = lineage.last().expect("a closing line stands");
        let matched = figure(lineage_closing, "matched ", ",");
        assert_eq!(
            sum, matched,
            "the steps of {name} sum to the lineage figure"
        );
    }
}

#[test]
fn the_matched_counts_close_each_step_against_the_changelog() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_matched(&chain).expect("the chain reads");
        let closing = lines.last().expect("a closing line stands");
        let steps = chain.atlases.len() - 1;
        let counts = step_counts(closing, steps);
        let lineage = render_lineage(&chain).expect("the chain reads");
        let changelogs: Vec<&String> = lineage
            .iter()
            .filter(|line| line.starts_with("changelog counts,"))
            .collect();
        assert_eq!(changelogs.len(), steps, "one changelog per step of {name}");
        for step in 0..steps {
            let arrived = figure(changelogs[step], "entries arrived ", ",");
            let departed = figure(changelogs[step], "departed ", ",");
            assert_eq!(
                counts[step] + departed,
                chain.atlases[step].entries.len(),
                "step {} of {name} closes against the atlas it leads from",
                step + 1
            );
            assert_eq!(
                counts[step] + arrived,
                chain.atlases[step + 1].entries.len(),
                "step {} of {name} closes against the atlas it leads to",
                step + 1
            );
        }
    }
}

#[test]
fn every_matched_line_is_a_standing_line() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_matched(&chain).expect("the chain reads");
        let standing = render_standing(&chain).expect("the chain reads");
        for section in sections(&lines) {
            for line in &section[1..] {
                assert!(
                    standing.contains(line),
                    "the matched line of {name} stands in the standing reading: {line}"
                );
            }
        }
    }
}

#[test]
fn the_entries_at_every_atlas_and_at_one_atlas_are_bound() {
    let chain = load("winter-spring-summer-lineage.json");
    let lines = render_matched(&chain).expect("the chain reads");
    let found = sections(&lines);
    let standing = render_standing(&chain).expect("the chain reads");
    let total = chain.atlases.len();
    let mut at_every = 0usize;
    let mut at_one = 0usize;
    for line in &standing {
        if !line.starts_with("entry ") {
            continue;
        }
        let stood = figure(line, "stands in ", " atlases");
        if stood == total {
            at_every += 1;
            for section in &found {
                assert!(
                    section.contains(line),
                    "an entry at every atlas stands in every section: {line}"
                );
            }
        } else if stood == 1 {
            at_one += 1;
            for section in &found {
                assert!(
                    !section.contains(line),
                    "an entry at one atlas stands in no section: {line}"
                );
            }
        }
    }
    let standing_closing = standing.last().expect("a closing line stands");
    assert_eq!(
        at_every,
        figure(standing_closing, "standing at every atlas ", ","),
        "the bound reads the standing closing's own figure"
    );
    assert!(at_one > 0, "the winter chain holds an entry at one atlas");
}

#[test]
fn an_entry_in_non_neighbouring_atlases_is_matched_at_no_step() {
    // The harbour lights trajectory of the winter chain,
    // isolated. The label stands in the first and third
    // atlases and not the second, so it departs nowhere
    // and is matched at no step either, and a relation
    // computed through the departures closing would read
    // it as matched somewhere.
    let chain = Lineage {
        atlases: vec![
            atlas("One", &["harbour"]),
            atlas("Two", &[]),
            atlas("Three", &["harbour"]),
        ],
    };
    let lines = render_matched(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "step 1",
            "step 2",
            "matched counts, step 1 matched 0, step 2 matched 0, \
             entries matched 0, entries 1, steps 2",
        ])
    );
}

#[test]
fn a_label_held_twice_in_one_atlas_is_matched_once() {
    // A raw entry walk over the first atlas would name
    // the repeated label twice in the section and count
    // it twice in the closing. The reading reads the
    // label once, at its first standing, from the pass's
    // per-atlas sets.
    let chain = Lineage {
        atlases: vec![
            atlas("One", &["dup", "dup", "kept"]),
            atlas("Two", &["dup", "kept"]),
        ],
    };
    let lines = render_matched(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "step 1",
            "entry dup, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close",
            "entry kept, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close",
            "matched counts, step 1 matched 2, entries matched 2, \
             entries 2, steps 1",
        ])
    );
}

#[test]
fn a_chain_of_two_renders_one_section() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", &["alpha", "beta"]),
            atlas("Two", &["beta", "gamma"]),
        ],
    };
    let lines = render_matched(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "step 1",
            "entry beta, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close",
            "matched counts, step 1 matched 1, entries matched 1, \
             entries 3, steps 1",
        ])
    );
}

#[test]
fn the_matched_reading_refuses_in_the_family_form() {
    let one = Lineage {
        atlases: vec![atlas("One", &["alpha"])],
    };
    assert_eq!(
        render_matched(&one),
        Err("the lineage holds fewer than two atlases".to_string())
    );

    let surrogate = Lineage {
        atlases: vec![
            Atlas {
                title: pt("One"),
                entries: vec![AtlasEntry {
                    label: PText(vec![0x61, 0xD800]),
                    exploration: expl(),
                }],
            },
            atlas("Two", &["alpha"]),
        ],
    };
    assert_eq!(
        render_matched(&surrogate),
        Err("an entry label holds an unpaired surrogate".to_string())
    );

    // A label holding both an unpaired surrogate and a
    // line break names the surrogate first, as the
    // presence pass reads each label.
    let both = Lineage {
        atlases: vec![
            Atlas {
                title: pt("One"),
                entries: vec![AtlasEntry {
                    label: PText(vec![0xD800, 0x0A]),
                    exploration: expl(),
                }],
            },
            atlas("Two", &["alpha"]),
        ],
    };
    assert_eq!(
        render_matched(&both),
        Err("an entry label holds an unpaired surrogate".to_string())
    );

    let breaking = Lineage {
        atlases: vec![atlas("One", &["a\nb"]), atlas("Two", &["alpha"])],
    };
    assert_eq!(
        render_matched(&breaking),
        Err("an entry label holds a newline or a carriage return".to_string())
    );
}
