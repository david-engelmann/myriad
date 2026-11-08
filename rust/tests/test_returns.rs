//! Tests for MY-15 and MY-16, the returns reading.
//!
//! The reading takes a lineage loaded whole through the
//! MY-13 loader and joins the presences of its entries
//! across the chain. An entry returns at a step when it
//! stands in the atlas the step leads to, does not stand
//! in the atlas the step leads from, and stands in at
//! least one earlier atlas, and one line stands for each
//! return in chain order and then in the order of the
//! atlas it returns in. Refusals of hand-built values
//! follow MY-13 and PORT-11.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_changelog, render_lineage, render_returns, Atlas, AtlasEntry, Lineage,
};
use myriad::explorations::{Branch, Exploration};
use myriad::pytext::PText;

fn atlas(title: &str, entries: Vec<AtlasEntry>) -> Atlas {
    Atlas {
        title: pt(title),
        entries,
    }
}

fn entry(label: &str, question: &str, branches: Vec<Branch>) -> AtlasEntry {
    AtlasEntry {
        label: pt(label),
        exploration: Exploration {
            question: pt(question),
            branches,
        },
    }
}

fn plain(label: &str) -> AtlasEntry {
    entry(label, "Q", vec![])
}

fn bad() -> PText {
    PText(vec![0x61, 0xD800])
}

fn atlas_json(title: &str, entries: &[String]) -> String {
    jobj(&[("title", jstr(title)), ("entries", jarr(entries))])
}

fn entry_json(label: &str, question: &str, branches: &[String]) -> String {
    jobj(&[
        ("label", jstr(label)),
        ("exploration", exploration_json(question, branches)),
    ])
}

fn lineage_json(atlases: &[String]) -> String {
    jobj(&[("atlases", jarr(atlases))])
}

fn lineage_loads(data_json: &str) -> Lineage {
    let (found, defects) = load_lineage_text(data_json);
    assert_eq!(defects, Vec::<String>::new());
    found.expect("a clean load holds a lineage")
}

// The three atlases of the return pins, sharing the
// shapes of the lineage corpus. The kept entry stands in
// every atlas, the gone entry departs at the first step
// and returns at the second, and the fresh entry arrives
// at the first step and departs at the second.

fn first() -> Atlas {
    atlas("First", vec![plain("kept"), plain("gone")])
}

fn second() -> Atlas {
    atlas("Second", vec![plain("kept"), plain("fresh")])
}

fn third() -> Atlas {
    atlas("Third", vec![plain("kept"), plain("gone")])
}

#[test]
fn a_chain_of_two_renders_its_closing_line_alone() {
    let lineage = Lineage {
        atlases: vec![first(), second()],
    };
    let lines = render_returns(&lineage).expect("the pair reads");
    assert_eq!(
        lines,
        strings(&["returns counts, returns 0, entries returned 0"])
    );
}

#[test]
fn a_loaded_chain_of_two_renders_its_closing_line_alone() {
    let text = lineage_json(&[
        atlas_json("One", &[entry_json("only", "Q", &[])]),
        atlas_json("Two", &[entry_json("only", "Q", &[])]),
    ]);
    let lineage = lineage_loads(&text);
    let lines = render_returns(&lineage).expect("the loaded pair reads");
    assert_eq!(
        lines,
        strings(&["returns counts, returns 0, entries returned 0"])
    );
}

#[test]
fn the_departed_entry_returns_at_the_second_step() {
    let lineage = Lineage {
        atlases: vec![first(), second(), third()],
    };
    let lines = render_returns(&lineage).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry gone, returned at step 2, absent 1 atlases",
            "returns counts, returns 1, entries returned 1",
        ])
    );
}

#[test]
fn a_loaded_chain_renders_its_return() {
    let one = atlas_json(
        "One",
        &[entry_json("kept", "Q", &[]), entry_json("gone", "Q", &[])],
    );
    let two = atlas_json("Two", &[entry_json("kept", "Q", &[])]);
    let three = atlas_json(
        "Three",
        &[entry_json("kept", "Q", &[]), entry_json("gone", "Q", &[])],
    );
    let lineage = lineage_loads(&lineage_json(&[one, two, three]));
    let lines = render_returns(&lineage).expect("the loaded chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry gone, returned at step 2, absent 1 atlases",
            "returns counts, returns 1, entries returned 1",
        ])
    );
}

#[test]
fn an_entry_can_return_twice_in_one_chain() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain("rover"), plain("anchor")]),
            atlas("Two", vec![plain("anchor")]),
            atlas("Three", vec![plain("rover"), plain("anchor")]),
            atlas("Four", vec![plain("anchor")]),
            atlas("Five", vec![plain("rover"), plain("anchor")]),
        ],
    };
    let lines = render_returns(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry rover, returned at step 2, absent 1 atlases",
            "entry rover, returned at step 4, absent 1 atlases",
            "returns counts, returns 2, entries returned 1",
        ])
    );
}

#[test]
fn an_entry_absent_for_two_atlases_counts_both_absences() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain("far")]),
            atlas("Two", vec![]),
            atlas("Three", vec![]),
            atlas("Four", vec![plain("far")]),
        ],
    };
    let lines = render_returns(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry far, returned at step 3, absent 2 atlases",
            "returns counts, returns 1, entries returned 1",
        ])
    );
}

#[test]
fn a_first_standing_at_the_last_atlas_is_an_arrival_and_never_a_return() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain("base")]),
            atlas("Two", vec![plain("base")]),
            atlas("Three", vec![plain("base"), plain("late")]),
        ],
    };
    let lines = render_returns(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&["returns counts, returns 0, entries returned 0"])
    );
    // The changelog of the step names the late entry
    // arrived, so the two readings agree on its standing.
    let changelog = render_changelog(&chain.atlases[1], &chain.atlases[2]).expect("the step reads");
    assert!(changelog.contains(&"entry late, arrived".to_string()));
}

#[test]
fn an_entry_standing_in_every_atlas_never_returns() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain("kept")]),
            atlas("Two", vec![plain("kept")]),
            atlas("Three", vec![plain("kept")]),
        ],
    };
    let lines = render_returns(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&["returns counts, returns 0, entries returned 0"])
    );
}

#[test]
fn the_returns_at_one_step_stand_in_the_order_of_the_atlas_they_return_in() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain("ex"), plain("wye")]),
            atlas("Two", vec![]),
            atlas("Three", vec![plain("wye"), plain("ex")]),
        ],
    };
    let lines = render_returns(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry wye, returned at step 2, absent 1 atlases",
            "entry ex, returned at step 2, absent 1 atlases",
            "returns counts, returns 2, entries returned 2",
        ])
    );
}

/// The arrived count of one step pair, read from the
/// changelog counts line the pair closes with.
fn arrived_of(earlier: &Atlas, later: &Atlas) -> usize {
    let lines = render_changelog(earlier, later).expect("the step reads");
    let closing = lines.last().expect("a closing line");
    let head = "changelog counts, entries arrived ";
    let rest = closing.strip_prefix(head).expect("the counts head");
    let digits: String = rest.chars().take_while(|ch| ch.is_ascii_digit()).collect();
    digits.parse().expect("a count")
}

/// The first arrivals of one step, counted in the test
/// from the label sets alone.
fn first_arrivals_of(chain: &Lineage, step: usize) -> usize {
    let later = &chain.atlases[step];
    let earlier = &chain.atlases[step - 1];
    later
        .entries
        .iter()
        .filter(|entry| {
            !earlier.entries.iter().any(|held| held.label == entry.label)
                && !chain.atlases[..step - 1]
                    .iter()
                    .any(|atlas| atlas.entries.iter().any(|held| held.label == entry.label))
        })
        .count()
}

/// The returns the reading states at one step.
fn returns_at(lines: &[String], step: usize) -> usize {
    let mark = format!(", returned at step {step},");
    lines.iter().filter(|line| line.contains(&mark)).count()
}

#[test]
fn each_steps_arrived_count_is_its_first_arrivals_plus_its_returns() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain("rover"), plain("anchor"), plain("gone")]),
            atlas("Two", vec![plain("anchor"), plain("fresh")]),
            atlas("Three", vec![plain("rover"), plain("anchor")]),
            atlas("Four", vec![plain("anchor")]),
            atlas("Five", vec![plain("rover"), plain("anchor"), plain("late")]),
        ],
    };
    let lines = render_returns(&chain).expect("the chain reads");
    let lineage = render_lineage(&chain).expect("the chain reads");
    let summed: usize = (0..chain.atlases.len() - 1)
        .map(|index| arrived_of(&chain.atlases[index], &chain.atlases[index + 1]))
        .sum();
    let closing = lineage.last().expect("a closing line");
    assert!(
        closing.contains(&format!("entries arrived {summed}")),
        "the lineage sums the step arrivals, {closing}"
    );
    for step in 1..chain.atlases.len() {
        let arrived = arrived_of(&chain.atlases[step - 1], &chain.atlases[step]);
        assert_eq!(
            arrived,
            first_arrivals_of(&chain, step) + returns_at(&lines, step),
            "step {step} decomposes"
        );
    }
    // The hand reading of the decomposition. Step one
    // arrives the fresh entry alone, a first arrival.
    // Step two arrives the rover, a return. Step three
    // arrives nothing. Step four arrives the rover again,
    // a return, and the late entry, a first arrival.
    assert_eq!(
        lines,
        strings(&[
            "entry rover, returned at step 2, absent 1 atlases",
            "entry rover, returned at step 4, absent 1 atlases",
            "returns counts, returns 2, entries returned 1",
        ])
    );
}

#[test]
fn the_returns_reading_holds_the_vocabulary_bar() {
    let chain = Lineage {
        atlases: vec![first(), second(), third()],
    };
    let lines = render_returns(&chain).expect("the chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert!(holds_vocabulary_bar(&text));
}

#[test]
fn a_wide_chain_renders_its_returns_whole() {
    let wide_branches: Vec<Branch> = (0..1200)
        .map(|index| {
            branch(
                &format!("b{index}"),
                None,
                "Prompt",
                "open",
                None,
                None,
                None,
            )
        })
        .collect();
    let wide = |title: &str| atlas(title, vec![entry("wide", "Q wide", wide_branches.clone())]);
    let empty = atlas("Empty", vec![]);
    let chain = Lineage {
        atlases: vec![wide("One"), empty, wide("Three")],
    };
    let lines = render_returns(&chain).expect("the wide chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry wide, returned at step 2, absent 1 atlases",
            "returns counts, returns 1, entries returned 1",
        ])
    );
}

#[test]
fn a_hand_built_lineage_of_fewer_than_two_atlases_is_refused() {
    let one = Lineage {
        atlases: vec![first()],
    };
    assert_eq!(
        render_returns(&one).expect_err("a chain of one is refused"),
        "the lineage holds fewer than two atlases"
    );
    let none = Lineage { atlases: vec![] };
    assert_eq!(
        render_returns(&none).expect_err("an empty chain is refused"),
        "the lineage holds fewer than two atlases"
    );
}

#[test]
fn a_hand_built_surrogate_label_names_its_refusal() {
    let mut broken = second();
    broken.entries[0].label = bad();
    let chain = Lineage {
        atlases: vec![first(), broken, third()],
    };
    assert_eq!(
        render_returns(&chain).expect_err("the broken label is refused"),
        "an entry label holds an unpaired surrogate"
    );
}

#[test]
fn a_hand_built_surrogate_label_in_an_atlas_no_return_reads_still_refuses() {
    let mut broken = third();
    broken.entries[1].label = bad();
    let chain = Lineage {
        atlases: vec![first(), second(), broken],
    };
    assert_eq!(
        render_returns(&chain).expect_err("the broken label is refused"),
        "an entry label holds an unpaired surrogate"
    );
}

#[test]
fn a_label_held_twice_in_the_atlas_it_returns_in_returns_once() {
    // FIX-MY-17. The loaders refuse a label carried by
    // two entries, so no loaded chain reaches this shape,
    // and the shape stands on the library surface alone.
    // Over it the reading reads the label once, at its
    // first standing in the atlas it returns in, and its
    // lines and its closing agree. On the unrepaired
    // reading the return line of x stands twice and the
    // closing states returns 2, entries returned 1, so
    // this pin fails there by assertion.
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain("x"), plain("y")]),
            atlas("Two", vec![plain("y")]),
            atlas("Three", vec![plain("x"), plain("x"), plain("y")]),
        ],
    };
    let lines = render_returns(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry x, returned at step 2, absent 1 atlases",
            "returns counts, returns 1, entries returned 1",
        ])
    );
}
