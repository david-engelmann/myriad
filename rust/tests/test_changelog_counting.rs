//! Tests for FIX-MY-19, the changelog counting repair.
//!
//! The pins here run the changelog, lineage, and
//! matched readings over hand built lineages whose
//! atlases hold one label twice, the shapes the red
//! team of Round 13 executed as its fixtures A, C, D,
//! E, and F. No file can carry the shape, because the
//! chain loader refuses an atlas holding one label
//! twice under the defect that names the duplicate
//! entry label, so every fixture stands hand built on
//! the library surface alone, and the pins assert the
//! first and second relations of MY-28 in the read
//! once form FIX-MY-19 settles, the changelog counts
//! and the lineage counts line agreeing with the
//! matched reading's own figures.
//!
//! The discrimination split of these pins stands as
//! follows, pin by pin. Every pin in this file fails
//! on the tree before this repair by assertion, and
//! the failing assertion of each pin is the one its
//! own paragraph names. The matched reading's lines
//! inside these pins hold on that tree too, because
//! the matched reading was born holding the read once
//! posture, and no pin here claims them as its
//! discrimination.
//!
//! Pin 1, fixture A, a label held twice in the
//! earlier atlas beside one held once, each standing
//! in the later atlas. It fails pre-fix on its
//! lineage lines, which carry the repeated label's
//! section twice, and on the first relation, the
//! changelog counting the label as matched once for
//! each entry that carries it. Its second relation
//! sums hold on both trees in the read once form, and
//! its pairing assertion, that the section reads the
//! earlier atlas's first standing of the label,
//! stands in the same lines.
//!
//! Pin 2, fixture F, a label held twice on both
//! sides of the step. It fails pre-fix on its lineage
//! lines and on the first relation, as pin 1 does.
//!
//! Pin 3, fixture E, a chain of three atlases whose
//! middle atlas holds a label twice and whose first
//! atlas holds its departing label twice. It fails
//! pre-fix on its lineage lines, on the first
//! relation read against the lineage counts line,
//! and on the second relation's first half at the
//! first step, the departed figure counting the
//! departing label once for each entry that carries
//! it.
//!
//! Pin 4, fixture C, a label held twice in the later
//! atlas and not in the earlier. It fails pre-fix on
//! its lineage lines and on the second relation's
//! second half, the arrived figure counting the
//! label once for each entry that carries it.
//!
//! Pin 5, fixture D, a label held twice in the later
//! atlas under two different questions. It fails
//! pre-fix on its lineage lines alone, the section
//! pairing the matched label with its last standing
//! in the later atlas on that tree and with its
//! first standing under the repair. Its relation
//! sums hold on both trees in the read once form.
//!
//! No pin in this file passes on the tree before
//! this repair by design. The evidence that loader
//! accepted values stand unmoved is by design
//! evidence and stands elsewhere, in the standing
//! suite's goldens over the shipped chains, which
//! pass unchanged on both trees and which this file
//! does not claim as discrimination.

mod common;

use std::collections::HashSet;

use common::*;
use myriad::atlas::{render_changelog, render_lineage, render_matched, Atlas, AtlasEntry, Lineage};
use myriad::explorations::Exploration;

fn expl(question: &str) -> Exploration {
    Exploration {
        question: pt(question),
        branches: vec![branch("a", None, "p", "open", None, None, None)],
    }
}

fn atlas(title: &str, entries: &[(&str, &str)]) -> Atlas {
    Atlas {
        title: pt(title),
        entries: entries
            .iter()
            .map(|(label, question)| AtlasEntry {
                label: pt(label),
                exploration: expl(question),
            })
            .collect(),
    }
}

/// The labels of one atlas read once, the count the
/// second relation of MY-28 closes against under
/// FIX-MY-19.
fn labels_read_once(atlas: &Atlas) -> usize {
    atlas
        .entries
        .iter()
        .map(|entry| &entry.label)
        .collect::<HashSet<_>>()
        .len()
}

/// The figure after a head in a line, read back out
/// of the rendered text.
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

/// The counts one changelog counts line states, in
/// the order arrived, departed, matched, the step
/// counted from zero.
fn changelog_counts(lines: &[String], step: usize) -> (usize, usize, usize) {
    let found: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("changelog counts,"))
        .collect();
    let line = found[step];
    (
        figure(line, "entries arrived ", ","),
        figure(line, "departed ", ","),
        figure(line, "matched ", ","),
    )
}

/// The per-step matched counts of a matched reading's
/// closing line, in step order.
fn step_counts(closing: &str, steps: usize) -> Vec<usize> {
    (1..=steps)
        .map(|step| figure(closing, &format!("step {step} matched "), ","))
        .collect()
}

#[test]
fn a_label_held_twice_in_the_earlier_atlas_is_matched_once() {
    // Fixture A. The first standing of alpha carries
    // the question first and its second standing the
    // question the later atlas carries, so the one
    // section the repair prints reads the first
    // standing, and the raw walk prints both.
    let chain = Lineage {
        atlases: vec![
            atlas("One", &[("alpha", "first"), ("alpha", "q"), ("beta", "q")]),
            atlas("Two", &[("alpha", "q"), ("beta", "q")]),
        ],
    };
    let lineage = render_lineage(&chain).expect("the chain reads");
    assert_eq!(
        lineage,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry alpha",
            "  earlier question, first",
            "  later question, q",
            "entry beta, unchanged",
            "changelog counts, entries arrived 0, departed 0, \
             matched 2, branches changed 0",
        ])
    );
    let pair = render_changelog(&chain.atlases[0], &chain.atlases[1]).expect("the pair reads");
    assert_eq!(
        pair, lineage,
        "a chain of two renders its one changelog whole"
    );
    let matched = render_matched(&chain).expect("the chain reads");
    assert_eq!(
        matched,
        strings(&[
            "step 1",
            "entry alpha, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close",
            "entry beta, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close",
            "matched counts, step 1 matched 2, entries matched 2, \
             entries 2, steps 1",
        ])
    );
    let closing = matched.last().expect("a closing line stands");
    let counts = step_counts(closing, 1);
    let (arrived, departed, stated) = changelog_counts(&lineage, 0);
    assert_eq!(
        counts.iter().sum::<usize>(),
        stated,
        "the first relation, the steps sum to the changelog matched figure"
    );
    assert_eq!(
        counts[0] + departed,
        labels_read_once(&chain.atlases[0]),
        "the second relation, matched and departed close the earlier atlas"
    );
    assert_eq!(
        counts[0] + arrived,
        labels_read_once(&chain.atlases[1]),
        "the second relation, matched and arrived close the later atlas"
    );
}

#[test]
fn a_label_held_twice_on_both_sides_is_matched_once() {
    // Fixture F. Alpha stands twice in each atlas, and
    // the step matches it once.
    let chain = Lineage {
        atlases: vec![
            atlas("One", &[("alpha", "q"), ("alpha", "q")]),
            atlas("Two", &[("alpha", "q"), ("alpha", "q")]),
        ],
    };
    let lineage = render_lineage(&chain).expect("the chain reads");
    assert_eq!(
        lineage,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry alpha, unchanged",
            "changelog counts, entries arrived 0, departed 0, \
             matched 1, branches changed 0",
        ])
    );
    let matched = render_matched(&chain).expect("the chain reads");
    assert_eq!(
        matched,
        strings(&[
            "step 1",
            "entry alpha, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close",
            "matched counts, step 1 matched 1, entries matched 1, \
             entries 1, steps 1",
        ])
    );
    let closing = matched.last().expect("a closing line stands");
    let counts = step_counts(closing, 1);
    let (_, _, stated) = changelog_counts(&lineage, 0);
    assert_eq!(
        counts.iter().sum::<usize>(),
        stated,
        "the first relation, the steps sum to the changelog matched figure"
    );
}

#[test]
fn the_lineage_counts_line_sums_the_read_once_counts() {
    // Fixture E. The middle atlas holds dup twice and
    // the last holds it once, and the first atlas holds
    // its departing label early twice, so both the
    // matched sum and the first step's departed figure
    // discriminate in the one chain.
    let chain = Lineage {
        atlases: vec![
            atlas("One", &[("early", "q"), ("early", "q")]),
            atlas("Two", &[("dup", "q"), ("dup", "q")]),
            atlas("Three", &[("dup", "q")]),
        ],
    };
    let lineage = render_lineage(&chain).expect("the chain reads");
    assert_eq!(
        lineage,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry early, departed",
            "entry dup, arrived",
            "changelog counts, entries arrived 1, departed 1, \
             matched 0, branches changed 0",
            "earlier atlas, Two",
            "later atlas, Three",
            "entry dup, unchanged",
            "changelog counts, entries arrived 0, departed 0, \
             matched 1, branches changed 0",
            "lineage counts, steps 2, entries arrived 1, departed 1, \
             matched 1, branches changed 0",
        ])
    );
    let matched = render_matched(&chain).expect("the chain reads");
    assert_eq!(
        matched,
        strings(&[
            "step 1",
            "step 2",
            "entry dup, first stood at atlas 2, last stood at atlas 3, \
             stands in 2 atlases, stands at the close",
            "matched counts, step 1 matched 0, step 2 matched 1, \
             entries matched 1, entries 2, steps 2",
        ])
    );
    let closing = matched.last().expect("a closing line stands");
    let counts = step_counts(closing, 2);
    let lineage_closing = lineage.last().expect("a closing line stands");
    assert_eq!(
        counts.iter().sum::<usize>(),
        figure(lineage_closing, "matched ", ","),
        "the first relation, the steps sum to the lineage matched figure"
    );
    let (arrived, departed, _) = changelog_counts(&lineage, 0);
    assert_eq!(
        counts[0] + departed,
        labels_read_once(&chain.atlases[0]),
        "the second relation at step 1, matched and departed close \
         the earlier atlas"
    );
    assert_eq!(
        counts[0] + arrived,
        labels_read_once(&chain.atlases[1]),
        "the second relation at step 1, matched and arrived close \
         the later atlas"
    );
    let (arrived, departed, _) = changelog_counts(&lineage, 1);
    assert_eq!(
        counts[1] + departed,
        labels_read_once(&chain.atlases[1]),
        "the second relation at step 2, matched and departed close \
         the earlier atlas"
    );
    assert_eq!(
        counts[1] + arrived,
        labels_read_once(&chain.atlases[2]),
        "the second relation at step 2, matched and arrived close \
         the later atlas"
    );
}

#[test]
fn a_label_held_twice_in_the_later_atlas_arrives_once() {
    // Fixture C. Zed stands twice in the later atlas
    // and not in the earlier, and it arrives once.
    let chain = Lineage {
        atlases: vec![
            atlas("One", &[("kept", "q")]),
            atlas("Two", &[("kept", "q"), ("zed", "q"), ("zed", "q")]),
        ],
    };
    let lineage = render_lineage(&chain).expect("the chain reads");
    assert_eq!(
        lineage,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry kept, unchanged",
            "entry zed, arrived",
            "changelog counts, entries arrived 1, departed 0, \
             matched 1, branches changed 0",
        ])
    );
    let matched = render_matched(&chain).expect("the chain reads");
    assert_eq!(
        matched,
        strings(&[
            "step 1",
            "entry kept, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close",
            "matched counts, step 1 matched 1, entries matched 1, \
             entries 2, steps 1",
        ])
    );
    let closing = matched.last().expect("a closing line stands");
    let counts = step_counts(closing, 1);
    let (arrived, _, _) = changelog_counts(&lineage, 0);
    assert_eq!(
        counts[0] + arrived,
        labels_read_once(&chain.atlases[1]),
        "the second relation, matched and arrived close the later atlas"
    );
}

#[test]
fn a_matched_label_pairs_its_first_standing_in_the_later_atlas() {
    // Fixture D. Alpha stands twice in the later atlas,
    // its first standing carrying the earlier question
    // and its second a different one, so the section
    // the repair prints reads the first standing and
    // stands unchanged, and the raw map pairs the last.
    let chain = Lineage {
        atlases: vec![
            atlas("One", &[("alpha", "q"), ("beta", "q")]),
            atlas("Two", &[("alpha", "q"), ("beta", "q"), ("alpha", "later")]),
        ],
    };
    let lineage = render_lineage(&chain).expect("the chain reads");
    assert_eq!(
        lineage,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry alpha, unchanged",
            "entry beta, unchanged",
            "changelog counts, entries arrived 0, departed 0, \
             matched 2, branches changed 0",
        ])
    );
    let matched = render_matched(&chain).expect("the chain reads");
    assert_eq!(
        matched,
        strings(&[
            "step 1",
            "entry alpha, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close",
            "entry beta, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close",
            "matched counts, step 1 matched 2, entries matched 2, \
             entries 2, steps 1",
        ])
    );
    let closing = matched.last().expect("a closing line stands");
    let counts = step_counts(closing, 1);
    let (arrived, departed, stated) = changelog_counts(&lineage, 0);
    assert_eq!(
        counts.iter().sum::<usize>(),
        stated,
        "the first relation, the steps sum to the changelog matched figure"
    );
    assert_eq!(
        counts[0] + departed,
        labels_read_once(&chain.atlases[0]),
        "the second relation, matched and departed close the earlier atlas"
    );
    assert_eq!(
        counts[0] + arrived,
        labels_read_once(&chain.atlases[1]),
        "the second relation, matched and arrived close the later atlas"
    );
}
