//! Tests for MY-17 and MY-18, the standing reading.
//!
//! The reading takes a lineage loaded whole through the
//! MY-13 loader and reads the presence of every label in
//! every atlas of the chain in one pass. One line stands
//! for each label in the order the labels first stand
//! across the chain, naming the first atlas the entry
//! stands in, the last atlas it stands in, the count of
//! atlases it stands in, and whether it stands at the
//! close. Refusals of hand-built values follow MY-13 and
//! PORT-11.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_index, render_returns, render_standing, Atlas, AtlasEntry, Lineage,
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

// The three atlases of the standing pins, the shapes of
// the returns corpus. The kept entry stands in every
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

#[test]
fn the_standing_of_the_corpus_chain() {
    let lineage = Lineage {
        atlases: vec![first(), second(), third()],
    };
    let lines = render_standing(&lineage).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry kept, first stood at atlas 1, last stood at atlas 3, \
             stands in 3 atlases, stands at the close",
            "entry gone, first stood at atlas 1, last stood at atlas 3, \
             stands in 2 atlases, stands at the close",
            "entry fresh, first stood at atlas 2, last stood at atlas 2, \
             stands in 1 atlases, does not stand at the close",
            "standing counts, atlases 3, entries 3, \
             standing at every atlas 1, standing at the close 2",
        ])
    );
}

#[test]
fn a_loaded_chain_renders_its_standing() {
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
    let lines = render_standing(&lineage).expect("the loaded chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry kept, first stood at atlas 1, last stood at atlas 3, \
             stands in 3 atlases, stands at the close",
            "entry gone, first stood at atlas 1, last stood at atlas 3, \
             stands in 2 atlases, stands at the close",
            "standing counts, atlases 3, entries 2, \
             standing at every atlas 1, standing at the close 2",
        ])
    );
}

#[test]
fn a_two_atlas_chain_where_one_entry_departs_and_none_returns() {
    let lineage = Lineage {
        atlases: vec![
            atlas("One", vec![plain("kept"), plain("gone")]),
            atlas("Two", vec![plain("kept")]),
        ],
    };
    let lines = render_standing(&lineage).expect("the pair reads");
    assert_eq!(
        lines,
        strings(&[
            "entry kept, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close",
            "entry gone, first stood at atlas 1, last stood at atlas 1, \
             stands in 1 atlases, does not stand at the close",
            "standing counts, atlases 2, entries 2, \
             standing at every atlas 1, standing at the close 1",
        ])
    );
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
    let lines = render_standing(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry wye, first stood at atlas 1, last stood at atlas 3, \
             stands in 2 atlases, stands at the close",
            "entry ex, first stood at atlas 1, last stood at atlas 3, \
             stands in 2 atlases, stands at the close",
            "standing counts, atlases 3, entries 2, \
             standing at every atlas 0, standing at the close 2",
        ])
    );
}

/// The standing of one label, computed in the test from
/// the chain alone, in the line form MY-18 states.
fn naive_line(chain: &Lineage, label: &PText) -> String {
    let mut stood_at: Vec<usize> = Vec::new();
    for (index, atlas) in chain.atlases.iter().enumerate() {
        if atlas.entries.iter().any(|entry| &entry.label == label) {
            stood_at.push(index + 1);
        }
    }
    let first = *stood_at.first().expect("the label stands somewhere");
    let last = *stood_at.last().expect("the label stands somewhere");
    let closes = last == chain.atlases.len();
    format!(
        "entry {}, first stood at atlas {}, last stood at atlas {}, \
         stands in {} atlases, {}",
        label,
        first,
        last,
        stood_at.len(),
        if closes {
            "stands at the close"
        } else {
            "does not stand at the close"
        }
    )
}

#[test]
fn every_line_matches_an_independent_count_from_the_chain() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain("rover"), plain("anchor"), plain("gone")]),
            atlas("Two", vec![plain("anchor"), plain("fresh")]),
            atlas("Three", vec![plain("rover"), plain("anchor")]),
            atlas("Four", vec![plain("anchor")]),
            atlas("Five", vec![plain("rover"), plain("anchor"), plain("late")]),
        ],
    };
    let lines = render_standing(&chain).expect("the chain reads");
    let mut labels: Vec<&PText> = Vec::new();
    for atlas in &chain.atlases {
        for entry in &atlas.entries {
            if !labels.contains(&&entry.label) {
                labels.push(&entry.label);
            }
        }
    }
    let mut expected: Vec<String> = labels
        .iter()
        .map(|label| naive_line(&chain, label))
        .collect();
    let at_every = labels
        .iter()
        .copied()
        .filter(|label| {
            chain
                .atlases
                .iter()
                .all(|atlas| atlas.entries.iter().any(|entry| entry.label == **label))
        })
        .count();
    let at_close = chain.atlases.last().expect("a last atlas").entries.len();
    expected.push(format!(
        "standing counts, atlases {}, entries {}, \
         standing at every atlas {}, standing at the close {}",
        chain.atlases.len(),
        labels.len(),
        at_every,
        at_close
    ));
    assert_eq!(lines, expected);
    // The hand reading of the same chain. The anchor
    // stands everywhere, the rover stands at atlases 1, 3,
    // and 5, the gone entry departs at the first step and
    // never stands again, the fresh entry stands at atlas
    // 2 alone, and the late entry stands at the close
    // alone.
    assert_eq!(
        lines,
        strings(&[
            "entry rover, first stood at atlas 1, last stood at atlas 5, \
             stands in 3 atlases, stands at the close",
            "entry anchor, first stood at atlas 1, last stood at atlas 5, \
             stands in 5 atlases, stands at the close",
            "entry gone, first stood at atlas 1, last stood at atlas 1, \
             stands in 1 atlases, does not stand at the close",
            "entry fresh, first stood at atlas 2, last stood at atlas 2, \
             stands in 1 atlases, does not stand at the close",
            "entry late, first stood at atlas 5, last stood at atlas 5, \
             stands in 1 atlases, stands at the close",
            "standing counts, atlases 5, entries 5, \
             standing at every atlas 1, standing at the close 3",
        ])
    );
}

#[test]
fn an_entry_standing_at_every_atlas_counts_the_whole_chain_and_closes() {
    let chain = Lineage {
        atlases: vec![first(), second(), third()],
    };
    let lines = render_standing(&chain).expect("the chain reads");
    let closing = lines.last().expect("a closing line");
    let head = "standing counts, atlases ";
    let rest = closing.strip_prefix(head).expect("the counts head");
    let digits: String = rest.chars().take_while(|ch| ch.is_ascii_digit()).collect();
    let atlases: usize = digits.parse().expect("a count");
    assert_eq!(atlases, 3);
    let kept = lines
        .iter()
        .find(|line| line.starts_with("entry kept,"))
        .expect("the kept line");
    assert!(kept.contains(&format!("stands in {atlases} atlases")));
    assert!(kept.ends_with("stands at the close"));
}

#[test]
fn an_entry_that_returns_stands_below_the_span_of_its_standing() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain("rover"), plain("anchor")]),
            atlas("Two", vec![plain("anchor")]),
            atlas("Three", vec![plain("rover"), plain("anchor")]),
            atlas("Four", vec![plain("anchor")]),
            atlas("Five", vec![plain("rover"), plain("anchor")]),
        ],
    };
    // The returns reading states the rover returns, so
    // the relation MY-18 states binds on this chain.
    let returns = render_returns(&chain).expect("the chain reads");
    assert!(returns
        .iter()
        .any(|line| line.starts_with("entry rover, returned")));
    let lines = render_standing(&chain).expect("the chain reads");
    let rover = lines
        .iter()
        .find(|line| line.starts_with("entry rover,"))
        .expect("the rover line");
    assert_eq!(
        rover,
        "entry rover, first stood at atlas 1, last stood at atlas 5, \
         stands in 3 atlases, stands at the close"
    );
    // The span from the first atlas to the last atlas
    // taken together is five atlases, and the rover
    // stands in three of them, below the span. The
    // figures are read back out of the rendered line,
    // so the relation fails if the reading miscounts.
    let figure = |head: &str, tail: &str| -> usize {
        let rest = rover
            .split(head)
            .nth(1)
            .expect("the figure head stands in the line");
        rest.split(tail)
            .next()
            .expect("the figure stands in the line")
            .parse()
            .expect("a count")
    };
    let first = figure("first stood at atlas ", ",");
    let last = figure("last stood at atlas ", ",");
    let stood = figure("stands in ", " atlases");
    let span = last - first + 1;
    assert!(stood < span);
}

#[test]
fn the_close_count_equals_the_index_entry_count_of_the_last_atlas() {
    let chain = Lineage {
        atlases: vec![first(), second(), third()],
    };
    let lines = render_standing(&chain).expect("the chain reads");
    let closing = lines.last().expect("a closing line");
    let mark = "standing at the close ";
    let rest = closing
        .rsplit(mark)
        .next()
        .expect("the close figure stands in the closing line");
    let stated: usize = rest.parse().expect("a count");
    let index = render_index(&chain.atlases[2]).expect("the last atlas reads");
    let index_closing = index.last().expect("a closing line");
    let head = "atlas counts, entries ";
    let index_rest = index_closing.strip_prefix(head).expect("the counts head");
    let digits: String = index_rest
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect();
    let counted: usize = digits.parse().expect("a count");
    assert_eq!(stated, counted);
    assert_eq!(stated, 2);
}

#[test]
fn a_label_cannot_mint_the_closing_line() {
    let forged = "standing counts, atlases 9, entries 9, \
                  standing at every atlas 9, standing at the close 9";
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain(forged)]),
            atlas("Two", vec![plain(forged)]),
        ],
    };
    let lines = render_standing(&chain).expect("the chain reads");
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("standing counts"))
            .count(),
        1
    );
    assert_eq!(
        lines[0],
        format!(
            "entry {forged}, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close"
        )
    );
    assert!(lines[0].starts_with("entry "));
}

#[test]
fn the_reading_is_a_pure_function_of_the_lineage() {
    let chain = Lineage {
        atlases: vec![first(), second(), third()],
    };
    let once = render_standing(&chain).expect("the chain reads");
    let twice = render_standing(&chain).expect("the chain reads");
    assert_eq!(once, twice);
}

#[test]
fn the_standing_reading_holds_the_vocabulary_bar() {
    let chain = Lineage {
        atlases: vec![first(), second(), third()],
    };
    let lines = render_standing(&chain).expect("the chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert!(holds_vocabulary_bar(&text));
}

#[test]
fn a_wide_chain_renders_its_standing_whole() {
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
    let lines = render_standing(&chain).expect("the wide chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry wide, first stood at atlas 1, last stood at atlas 3, \
             stands in 2 atlases, stands at the close",
            "standing counts, atlases 3, entries 1, \
             standing at every atlas 0, standing at the close 1",
        ])
    );
}

#[test]
fn a_hand_built_lineage_of_fewer_than_two_atlases_is_refused() {
    let one = Lineage {
        atlases: vec![first()],
    };
    assert_eq!(
        render_standing(&one).expect_err("a chain of one is refused"),
        "the lineage holds fewer than two atlases"
    );
    let none = Lineage { atlases: vec![] };
    assert_eq!(
        render_standing(&none).expect_err("an empty chain is refused"),
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
        render_standing(&chain).expect_err("the broken label is refused"),
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
        render_standing(&chain).expect_err("the broken label is refused"),
        "an entry label holds an unpaired surrogate"
    );
}
