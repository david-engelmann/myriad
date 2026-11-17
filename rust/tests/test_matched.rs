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
/// A thin chain of n atlases built in process, atlas i
/// holding labels i and i plus one cyclically, so each
/// of the chain's labels stands in exactly two atlases.
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
/// label per atlas, so each label stands in twenty
/// atlases.
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
/// and label-2, so label-2 stands in every atlas and
/// each other label in sixty.
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

/// The label an entry line names.
fn label_of(line: &str) -> String {
    line.strip_prefix("entry ")
        .expect("an entry line opens with the word entry")
        .split(',')
        .next()
        .expect("a label stands")
        .to_string()
}

#[test]
fn the_thin_chain_matched_stands_as_counted() {
    let lines = render_matched(&thin_chain(302)).expect("the chain reads");
    assert_eq!(lines.len(), 603);
    let secs = sections(&lines);
    assert_eq!(secs.len(), 301);
    for section in &secs {
        assert_eq!(section.len(), 2);
    }
    assert_eq!(
        secs[0],
        strings(&[
            "step 1",
            "entry label-2, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, does not stand at the close",
        ])
    );
    assert_eq!(
        secs[151],
        strings(&[
            "step 152",
            "entry label-153, first stood at atlas 152, last stood at atlas 153, \
             stands in 2 atlases, does not stand at the close",
        ])
    );
    assert_eq!(
        secs[300],
        strings(&[
            "step 301",
            "entry label-302, first stood at atlas 301, last stood at atlas 302, \
             stands in 2 atlases, stands at the close",
        ])
    );
    let mut closing = String::from("matched counts");
    for i in 1..=301 {
        closing.push_str(&format!(", step {i} matched 1"));
    }
    closing.push_str(", entries matched 301, entries 302, steps 301");
    assert_eq!(lines[602], closing);
}

#[test]
fn the_window_chain_matched_stands_as_counted() {
    let lines = render_matched(&window_chain()).expect("the chain reads");
    assert_eq!(lines.len(), 596);
    let secs = sections(&lines);
    assert_eq!(secs.len(), 119);
    for section in &secs {
        assert_eq!(section.len(), 5);
    }
    assert_eq!(
        secs[0],
        strings(&[
            "step 1",
            "entry label-2, first stood at atlas 1, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
            "entry label-3, first stood at atlas 1, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
            "entry label-4, first stood at atlas 1, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
            "entry label-5, first stood at atlas 1, last stood at atlas 95, \
             stands in 20 atlases, does not stand at the close",
        ])
    );
    assert_eq!(
        secs[93],
        strings(&[
            "step 94",
            "entry label-5, first stood at atlas 1, last stood at atlas 95, \
             stands in 20 atlases, does not stand at the close",
            "entry label-6, first stood at atlas 2, last stood at atlas 96, \
             stands in 20 atlases, does not stand at the close",
            "entry label-7, first stood at atlas 3, last stood at atlas 97, \
             stands in 20 atlases, does not stand at the close",
            "entry label-8, first stood at atlas 4, last stood at atlas 98, \
             stands in 20 atlases, does not stand at the close",
        ])
    );
    assert_eq!(
        secs[94],
        strings(&[
            "step 95",
            "entry label-6, first stood at atlas 2, last stood at atlas 96, \
             stands in 20 atlases, does not stand at the close",
            "entry label-7, first stood at atlas 3, last stood at atlas 97, \
             stands in 20 atlases, does not stand at the close",
            "entry label-8, first stood at atlas 4, last stood at atlas 98, \
             stands in 20 atlases, does not stand at the close",
            "entry label-9, first stood at atlas 5, last stood at atlas 99, \
             stands in 20 atlases, does not stand at the close",
        ])
    );
    assert_eq!(
        secs[118],
        strings(&[
            "step 119",
            "entry label-30, first stood at atlas 26, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
            "entry label-1, first stood at atlas 1, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
            "entry label-2, first stood at atlas 1, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
            "entry label-3, first stood at atlas 1, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
        ])
    );
    let mut closing = String::from("matched counts");
    for i in 1..=119 {
        closing.push_str(&format!(", step {i} matched 4"));
    }
    closing.push_str(", entries matched 30, entries 30, steps 119");
    assert_eq!(lines[595], closing);
}

#[test]
fn the_paired_chain_matched_stands_as_counted() {
    let lines = render_matched(&paired_chain()).expect("the chain reads");
    assert_eq!(lines.len(), 239);
    let secs = sections(&lines);
    assert_eq!(secs.len(), 119);
    let line = "entry label-2, first stood at atlas 1, last stood at atlas 120, \
                stands in 120 atlases, stands at the close";
    for (index, section) in secs.iter().enumerate() {
        assert_eq!(
            *section,
            vec![format!("step {}", index + 1), line.to_string()]
        );
    }
    let mut closing = String::from("matched counts");
    for i in 1..=119 {
        closing.push_str(&format!(", step {i} matched 1"));
    }
    closing.push_str(", entries matched 1, entries 5, steps 119");
    assert_eq!(lines[238], closing);
}

#[test]
fn the_sections_hold_the_matched_lines_in_entry_order() {
    let mut chains = vec![
        load("winter-spring-summer-lineage.json"),
        load("radio-investigation-lineage.json"),
        load("constructed-departures-lineage.json"),
        load("generated-chain-lineage.json"),
    ];
    chains.push(thin_chain(302));
    chains.push(window_chain());
    chains.push(paired_chain());
    for chain in &chains {
        let total = chain.atlases.len();
        let standing = render_standing(chain).expect("the chain reads");
        let mut by_label: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for line in &standing[..standing.len() - 1] {
            by_label.insert(label_of(line), line.clone());
        }
        let lines = render_matched(chain).expect("the chain reads");
        let secs = sections(&lines);
        assert_eq!(secs.len(), total - 1);
        let mut entry_lines = 0usize;
        let mut named: Vec<String> = Vec::new();
        for step in 0..total - 1 {
            let later: Vec<String> = chain.atlases[step + 1]
                .entries
                .iter()
                .map(|entry| entry.label.to_string())
                .collect();
            let mut expected: Vec<String> = vec![format!("step {}", step + 1)];
            let mut seen: Vec<String> = Vec::new();
            for entry in &chain.atlases[step].entries {
                let label = entry.label.to_string();
                if !seen.contains(&label) {
                    seen.push(label.clone());
                    if later.contains(&label) {
                        expected.push(
                            by_label
                                .get(&label)
                                .expect("every standing label holds a standing line")
                                .clone(),
                        );
                        if !named.contains(&label) {
                            named.push(label);
                        }
                    }
                }
            }
            assert_eq!(secs[step], expected);
            entry_lines += secs[step].len() - 1;
        }
        assert_eq!(lines.len(), total - 1 + entry_lines + 1);
        let closing = lines.last().expect("a closing stands");
        let counts = step_counts(closing, total - 1);
        for (index, section) in secs.iter().enumerate() {
            assert_eq!(counts[index], section.len() - 1);
        }
        assert_eq!(figure(closing, "entries matched ", ","), named.len());
        // The entries figure stands after the entries
        // matched figure, so it is read at the second
        // standing of its head.
        let entries: usize = closing
            .split(", entries ")
            .nth(2)
            .expect("the entries figure stands")
            .split(',')
            .next()
            .expect("a count stands")
            .parse()
            .expect("a count");
        assert_eq!(entries, by_label.len());
    }
}
