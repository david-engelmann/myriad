//! Tests for MY-19 and MY-20, the absence reading.
//!
//! The reading takes a lineage loaded whole through the
//! MY-13 loader and reads the presence of every label in
//! every atlas of the chain in one pass. One line stands
//! for each absence run of the chain in the order the
//! labels first stand across the chain, an inside run
//! ending at a standing and a closing run closing with
//! the words to the close. Refusals of hand-built values
//! follow MY-13 and PORT-11.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_absence, render_returns, render_standing, Atlas, AtlasEntry, Lineage,
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

// The three atlases of the absence pins, the shapes of
// the standing corpus. The kept entry stands in every
// atlas, the gone entry departs at the first step and
// returns at the second, and the fresh entry arrives at
// the first step and departs at the second.

fn first() -> Atlas {
    atlas("First", vec![plain("kept"), plain("gone")])
}

fn second() -> Atlas {
    atlas("Second", vec![plain("kept"), plain("fresh")])
}

fn third() -> Atlas {
    atlas("Third", vec![plain("kept"), plain("gone")])
}

fn corpus() -> Lineage {
    Lineage {
        atlases: vec![first(), second(), third()],
    }
}

// The five atlas chain of the relation pins. The anchor
// stands everywhere, the rover stands at atlases 1, 3,
// and 5, the gone entry stands at atlas 1 alone, the
// fresh entry stands at atlas 2 alone, and the late
// entry stands at atlas 5 alone.

fn wide_chain() -> Lineage {
    Lineage {
        atlases: vec![
            atlas("One", vec![plain("rover"), plain("anchor"), plain("gone")]),
            atlas("Two", vec![plain("anchor"), plain("fresh")]),
            atlas("Three", vec![plain("rover"), plain("anchor")]),
            atlas("Four", vec![plain("anchor")]),
            atlas("Five", vec![plain("rover"), plain("anchor"), plain("late")]),
        ],
    }
}

/// The figure after a head in a line, read back out of
/// the rendered text so a relation fails when the
/// reading miscounts.
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

/// The figure after a head at the end of a line, read
/// back out of the rendered text.
fn figure_at_end(line: &str, head: &str) -> usize {
    line.rsplit(head)
        .next()
        .unwrap_or_else(|| panic!("the figure head {head} stands in the line {line}"))
        .parse()
        .expect("a count")
}

#[test]
fn the_absence_of_the_corpus_chain() {
    let lines = render_absence(&corpus()).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry gone, absent from atlas 2, for 1 atlases",
            "entry fresh, absent from atlas 3, for 1 atlases, to the close",
            "absence counts, absences 2, entries with absences 2, \
             absent at the close 1",
        ])
    );
}

#[test]
fn a_loaded_chain_renders_its_absence() {
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
    let lines = render_absence(&lineage).expect("the loaded chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry gone, absent from atlas 2, for 1 atlases",
            "absence counts, absences 1, entries with absences 1, \
             absent at the close 0",
        ])
    );
}

#[test]
fn a_two_atlas_chain_renders_one_closing_run() {
    let lineage = Lineage {
        atlases: vec![
            atlas("One", vec![plain("kept"), plain("gone")]),
            atlas("Two", vec![plain("kept")]),
        ],
    };
    let lines = render_absence(&lineage).expect("the pair reads");
    assert_eq!(
        lines,
        strings(&[
            "entry gone, absent from atlas 2, for 1 atlases, to the close",
            "absence counts, absences 1, entries with absences 1, \
             absent at the close 1",
        ])
    );
}

#[test]
fn a_chain_where_every_label_stands_everywhere_renders_zeros_alone() {
    let lineage = Lineage {
        atlases: vec![
            atlas("One", vec![plain("kept"), plain("also")]),
            atlas("Two", vec![plain("also"), plain("kept")]),
            atlas("Three", vec![plain("kept"), plain("also")]),
        ],
    };
    let lines = render_absence(&lineage).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&["absence counts, absences 0, entries with absences 0, \
                   absent at the close 0"])
    );
}

#[test]
fn a_label_with_two_inside_runs_stands_two_run_lines_and_returns_twice() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain("rover"), plain("anchor")]),
            atlas("Two", vec![plain("anchor")]),
            atlas("Three", vec![plain("rover"), plain("anchor")]),
            atlas("Four", vec![plain("anchor")]),
            atlas("Five", vec![plain("rover"), plain("anchor")]),
        ],
    };
    let lines = render_absence(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry rover, absent from atlas 2, for 1 atlases",
            "entry rover, absent from atlas 4, for 1 atlases",
            "absence counts, absences 2, entries with absences 1, \
             absent at the close 0",
        ])
    );
    let returns = render_returns(&chain).expect("the chain reads");
    assert_eq!(
        returns
            .iter()
            .filter(|line| line.starts_with("entry rover, returned"))
            .count(),
        2
    );
    assert_eq!(
        returns.last().expect("a closing line"),
        "returns counts, returns 2, entries returned 1"
    );
}

#[test]
fn a_late_first_standing_holds_no_run_before_it() {
    let lineage = Lineage {
        atlases: vec![
            atlas("One", vec![plain("kept")]),
            atlas("Two", vec![plain("kept")]),
            atlas("Three", vec![plain("kept"), plain("late")]),
            atlas("Four", vec![plain("kept"), plain("late")]),
        ],
    };
    let lines = render_absence(&lineage).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&["absence counts, absences 0, entries with absences 0, \
                   absent at the close 0"])
    );
}

#[test]
fn a_late_entry_absent_at_the_close_runs_from_the_close_alone() {
    // The late entry stands first at atlas 3 and not at
    // atlas 4, so its one run opens at atlas 4. A reading
    // that counted the atlases before its first standing
    // as absence would open the run at atlas 1.
    let lineage = Lineage {
        atlases: vec![
            atlas("One", vec![plain("kept")]),
            atlas("Two", vec![plain("kept")]),
            atlas("Three", vec![plain("kept"), plain("late")]),
            atlas("Four", vec![plain("kept")]),
        ],
    };
    let lines = render_absence(&lineage).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry late, absent from atlas 4, for 1 atlases, to the close",
            "absence counts, absences 1, entries with absences 1, \
             absent at the close 1",
        ])
    );
}

#[test]
fn a_run_of_three_atlases_counts_three() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain("rover")]),
            atlas("Two", vec![]),
            atlas("Three", vec![]),
            atlas("Four", vec![]),
            atlas("Five", vec![plain("rover")]),
        ],
    };
    let lines = render_absence(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry rover, absent from atlas 2, for 3 atlases",
            "absence counts, absences 1, entries with absences 1, \
             absent at the close 0",
        ])
    );
}

/// The absence lines of one chain, computed in the test
/// from the chain alone, in the forms MY-20 states.
fn naive_lines(chain: &Lineage) -> Vec<String> {
    let total = chain.atlases.len();
    let mut order: Vec<&PText> = Vec::new();
    for atlas in &chain.atlases {
        for entry in &atlas.entries {
            if !order.contains(&&entry.label) {
                order.push(&entry.label);
            }
        }
    }
    let stands = |label: &PText, index: usize| {
        chain.atlases[index]
            .entries
            .iter()
            .any(|entry| &entry.label == label)
    };
    let mut lines: Vec<String> = Vec::new();
    let mut runs = 0usize;
    let mut with_absences = 0usize;
    let mut absent_at_close = 0usize;
    for label in &order {
        let mut own = 0usize;
        let mut stood_before = false;
        let mut index = 0usize;
        while index < total {
            if stands(label, index) {
                stood_before = true;
                index += 1;
                continue;
            }
            // The label does not stand at this atlas. A run
            // opens only behind a standing.
            if !stood_before {
                index += 1;
                continue;
            }
            let start = index;
            while index < total && !stands(label, index) {
                index += 1;
            }
            runs += 1;
            own += 1;
            if index == total {
                absent_at_close += 1;
                lines.push(format!(
                    "entry {}, absent from atlas {}, for {} atlases, to the close",
                    label,
                    start + 1,
                    index - start
                ));
            } else {
                lines.push(format!(
                    "entry {}, absent from atlas {}, for {} atlases",
                    label,
                    start + 1,
                    index - start
                ));
            }
        }
        if own > 0 {
            with_absences += 1;
        }
    }
    lines.push(format!(
        "absence counts, absences {runs}, entries with absences {with_absences}, \
         absent at the close {absent_at_close}"
    ));
    lines
}

#[test]
fn every_line_matches_an_independent_count_from_the_chain() {
    let chain = wide_chain();
    let lines = render_absence(&chain).expect("the chain reads");
    assert_eq!(lines, naive_lines(&chain));
    // The hand reading of the same chain. The rover holds
    // two inside runs of one atlas each, the gone entry
    // holds a closing run of four atlases from atlas 2,
    // and the fresh entry holds a closing run of three
    // atlases from atlas 3.
    assert_eq!(
        lines,
        strings(&[
            "entry rover, absent from atlas 2, for 1 atlases",
            "entry rover, absent from atlas 4, for 1 atlases",
            "entry gone, absent from atlas 2, for 4 atlases, to the close",
            "entry fresh, absent from atlas 3, for 3 atlases, to the close",
            "absence counts, absences 4, entries with absences 3, \
             absent at the close 2",
        ])
    );
}

#[test]
fn the_inside_runs_equal_the_returns_of_the_same_chain() {
    for chain in [corpus(), wide_chain()] {
        let absence = render_absence(&chain).expect("the chain reads");
        let inside = absence
            .iter()
            .filter(|line| line.starts_with("entry ") && !line.ends_with("to the close"))
            .count();
        let returns = render_returns(&chain).expect("the chain reads");
        let closing = returns.last().expect("a closing line");
        let stated = figure(closing, "returns counts, returns ", ",");
        assert_eq!(inside, stated);
    }
}

#[test]
fn the_absent_at_the_close_relation_against_the_standing_reading() {
    let two_atlas = Lineage {
        atlases: vec![
            atlas("One", vec![plain("kept"), plain("gone")]),
            atlas("Two", vec![plain("kept")]),
        ],
    };
    for chain in [corpus(), wide_chain(), two_atlas] {
        let standing = render_standing(&chain).expect("the chain reads");
        let standing_closing = standing.last().expect("a closing line");
        let entries = figure(standing_closing, "entries ", ",");
        let at_close = figure_at_end(standing_closing, "standing at the close ");
        let absence = render_absence(&chain).expect("the chain reads");
        let absence_closing = absence.last().expect("a closing line");
        let absent = figure_at_end(absence_closing, "absent at the close ");
        assert_eq!(absent, entries - at_close);
    }
}

#[test]
fn the_span_relation_holds_for_every_label() {
    let chain = wide_chain();
    let standing = render_standing(&chain).expect("the chain reads");
    let absence = render_absence(&chain).expect("the chain reads");
    for line in standing.iter().filter(|line| line.starts_with("entry ")) {
        let label = line
            .strip_prefix("entry ")
            .expect("the label head")
            .split(',')
            .next()
            .expect("the label");
        let first = figure(line, "first stood at atlas ", ",");
        let last = figure(line, "last stood at atlas ", ",");
        let stood = figure(line, "stands in ", " atlases");
        let inside: usize = absence
            .iter()
            .filter(|run| {
                run.starts_with(&format!("entry {label}, absent from atlas "))
                    && !run.ends_with("to the close")
            })
            .map(|run| figure(run, ", for ", " atlases"))
            .sum();
        assert_eq!(last - first + 1, stood + inside, "label {label}");
    }
}

#[test]
fn a_label_cannot_mint_the_closing_line() {
    let forged = "absence counts, absences 9, entries with absences 9, \
                  absent at the close 9";
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain(forged)]),
            atlas("Two", vec![]),
            atlas("Three", vec![plain(forged)]),
        ],
    };
    let lines = render_absence(&chain).expect("the chain reads");
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("absence counts"))
            .count(),
        1
    );
    assert_eq!(
        lines[0],
        format!("entry {forged}, absent from atlas 2, for 1 atlases")
    );
    assert!(lines[0].starts_with("entry "));
}

#[test]
fn the_lines_stand_in_first_standing_order_not_close_order() {
    // The closing atlas holds the two labels in the other
    // order, so a reading that walked the chain backwards
    // or read the close first would swap the lines.
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain("wye"), plain("ex")]),
            atlas("Two", vec![]),
            atlas("Three", vec![plain("ex"), plain("wye")]),
        ],
    };
    let lines = render_absence(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry wye, absent from atlas 2, for 1 atlases",
            "entry ex, absent from atlas 2, for 1 atlases",
            "absence counts, absences 2, entries with absences 2, \
             absent at the close 0",
        ])
    );
}

#[test]
fn the_reading_is_a_pure_function_of_the_lineage() {
    let chain = corpus();
    let once = render_absence(&chain).expect("the chain reads");
    let twice = render_absence(&chain).expect("the chain reads");
    assert_eq!(once, twice);
}

#[test]
fn the_absence_reading_holds_the_vocabulary_bar() {
    let lines = render_absence(&wide_chain()).expect("the chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert!(holds_vocabulary_bar(&text));
}

#[test]
fn a_wide_chain_renders_its_absence_whole() {
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
    let lines = render_absence(&chain).expect("the wide chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry wide, absent from atlas 2, for 1 atlases",
            "absence counts, absences 1, entries with absences 1, \
             absent at the close 0",
        ])
    );
}

#[test]
fn a_hand_built_lineage_of_fewer_than_two_atlases_is_refused() {
    let one = Lineage {
        atlases: vec![first()],
    };
    assert_eq!(
        render_absence(&one).expect_err("a chain of one is refused"),
        "the lineage holds fewer than two atlases"
    );
    let none = Lineage { atlases: vec![] };
    assert_eq!(
        render_absence(&none).expect_err("an empty chain is refused"),
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
        render_absence(&chain).expect_err("the broken label is refused"),
        "an entry label holds an unpaired surrogate"
    );
}

#[test]
fn a_hand_built_surrogate_label_in_the_last_atlas_still_refuses() {
    let mut broken = third();
    broken.entries[1].label = bad();
    let chain = Lineage {
        atlases: vec![first(), second(), broken],
    };
    assert_eq!(
        render_absence(&chain).expect_err("the broken label is refused"),
        "an entry label holds an unpaired surrogate"
    );
}
