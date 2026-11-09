//! Tests for FIX-MY-13, the chain fencing closure.
//!
//! The returns, standing, and absence readings read the
//! labels of a lineage in the one presence pass, and a
//! hand built lineage whose entry label holds a newline
//! or a carriage return draws the refusal an entry label
//! holds a newline or a carriage return in every one of
//! the three readings, in the reading order FIX-MY-13
//! states beside the surrogate refusal of PORT-11. No
//! file can carry such a label, because the MY-7 loader
//! refuses it by name, so every value here is built by
//! hand, as the finding these pins close was built.
//!
//! Round 13 adds one posture pin at the end of this
//! file, holding the judgement the Round 13 contract
//! section records on the second observation of the
//! Round 12 red team. The pin passes on the tree before
//! this slice by design, because the standing reading
//! there already renders a label carrying a literal
//! U+2028 with its separator bytes standing in the line,
//! and it fails by assertion the moment any widening of
//! the line break set lands without the version 2
//! amendment the contract names as its only door.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_absence, render_returns, render_standing, Atlas, AtlasEntry, Lineage,
};
use myriad::explorations::{Branch, Exploration};
use myriad::pytext::PText;

const BREAK_REFUSAL: &str = "an entry label holds a newline or a carriage return";
const SURROGATE_REFUSAL: &str = "an entry label holds an unpaired surrogate";

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

fn surrogate_label() -> PText {
    PText(vec![0x61, 0xD800])
}

fn broken_entry(label: PText) -> AtlasEntry {
    AtlasEntry {
        label,
        exploration: Exploration {
            question: pt("Q"),
            branches: vec![],
        },
    }
}

/// The one signature the three chain readings share.
type Reading = fn(&Lineage) -> Result<Vec<String>, String>;

/// The three chain readings under their one signature,
/// in the order the contract names them.
fn readings() -> [Reading; 3] {
    [render_returns, render_standing, render_absence]
}

fn assert_refused(chain: &Lineage, refusal: &str) {
    for render in readings() {
        assert_eq!(
            render(chain).expect_err("the broken label is refused"),
            refusal
        );
    }
}

#[test]
fn a_label_holding_a_line_feed_in_the_first_atlas_draws_the_refusal() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", vec![plain("a\nb"), plain("kept")]),
            atlas("Second", vec![plain("kept")]),
        ],
    };
    assert_refused(&chain, BREAK_REFUSAL);
}

#[test]
fn a_label_holding_a_carriage_return_draws_the_refusal() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", vec![plain("a\rb"), plain("kept")]),
            atlas("Second", vec![plain("kept")]),
        ],
    };
    assert_refused(&chain, BREAK_REFUSAL);
}

#[test]
fn a_line_break_first_standing_in_the_second_atlas_draws_the_refusal() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", vec![plain("kept")]),
            atlas("Second", vec![plain("kept"), plain("a\nb")]),
            atlas("Third", vec![plain("kept")]),
        ],
    };
    assert_refused(&chain, BREAK_REFUSAL);
}

#[test]
fn a_line_break_standing_only_in_the_last_atlas_draws_the_refusal() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", vec![plain("kept")]),
            atlas("Second", vec![plain("kept")]),
            atlas("Third", vec![plain("kept"), plain("a\nb")]),
        ],
    };
    assert_refused(&chain, BREAK_REFUSAL);
}

#[test]
fn a_line_break_standing_at_every_atlas_draws_the_refusal() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", vec![plain("a\nb"), plain("kept")]),
            atlas("Second", vec![plain("a\nb"), plain("kept")]),
        ],
    };
    assert_refused(&chain, BREAK_REFUSAL);
}

#[test]
fn the_forged_closing_label_draws_the_refusal() {
    // The shape the Round 9 red team executed against the
    // absence reading, a label carrying the text of the
    // reading's own closing line between two line feeds.
    let forged = "a\nabsence counts, absences 9, entries with absences 9, \
                  absent at the close 9\nb";
    let chain = Lineage {
        atlases: vec![
            atlas("First", vec![plain(forged), plain("kept")]),
            atlas("Second", vec![plain("kept")]),
        ],
    };
    assert_refused(&chain, BREAK_REFUSAL);
}

#[test]
fn a_label_holding_a_surrogate_and_a_break_draws_the_surrogate_refusal() {
    let both = PText(vec![0x61, 0xD800, 0x0A]);
    let chain = Lineage {
        atlases: vec![
            atlas("First", vec![broken_entry(both), plain("kept")]),
            atlas("Second", vec![plain("kept")]),
        ],
    };
    assert_refused(&chain, SURROGATE_REFUSAL);
}

#[test]
fn a_break_in_an_earlier_label_draws_before_a_surrogate_in_a_later_one() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", vec![plain("a\nb"), plain("kept")]),
            atlas(
                "Second",
                vec![broken_entry(surrogate_label()), plain("kept")],
            ),
        ],
    };
    assert_refused(&chain, BREAK_REFUSAL);
}

#[test]
fn a_surrogate_in_an_earlier_label_draws_before_a_break_in_a_later_one() {
    let chain = Lineage {
        atlases: vec![
            atlas(
                "First",
                vec![broken_entry(surrogate_label()), plain("kept")],
            ),
            atlas("Second", vec![plain("a\nb"), plain("kept")]),
        ],
    };
    assert_refused(&chain, SURROGATE_REFUSAL);
}

#[test]
fn a_short_lineage_holding_a_break_still_draws_the_fewer_than_two_refusal() {
    let chain = Lineage {
        atlases: vec![atlas("Only", vec![plain("a\nb")])],
    };
    assert_refused(&chain, "the lineage holds fewer than two atlases");
}

#[test]
fn a_clean_hand_built_lineage_renders_the_lines_it_rendered_before() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", vec![plain("kept"), plain("gone")]),
            atlas("Second", vec![plain("kept"), plain("fresh")]),
        ],
    };
    assert_eq!(
        render_returns(&chain).expect("the clean chain reads"),
        strings(&["returns counts, returns 0, entries returned 0"])
    );
    assert_eq!(
        render_standing(&chain).expect("the clean chain reads"),
        strings(&[
            "entry kept, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close",
            "entry gone, first stood at atlas 1, last stood at atlas 1, \
             stands in 1 atlases, does not stand at the close",
            "entry fresh, first stood at atlas 2, last stood at atlas 2, \
             stands in 1 atlases, stands at the close",
            "standing counts, atlases 2, entries 3, standing at every atlas 1, \
             standing at the close 2",
        ])
    );
    assert_eq!(
        render_absence(&chain).expect("the clean chain reads"),
        strings(&[
            "entry gone, absent from atlas 2, for 1 atlases, to the close",
            "absence counts, absences 1, entries with absences 1, \
             absent at the close 1",
        ])
    );
}

#[test]
fn a_line_separator_label_renders_with_its_bytes_standing() {
    // The posture pin of the Round 13 judgement, on the
    // reading side. The lineage loads whole through the
    // loader, the label carrying a literal U+2028
    // standing outside the contract's line break set,
    // and the standing reading renders the label with
    // its separator bytes standing raw in the one line,
    // the line count and the closing figures exact. The
    // pin passes on the tree before this slice by design
    // and fails the moment the set widens without a
    // version 2 amendment.
    let entry = |label: &str| {
        jobj(&[
            ("label", jstr(label)),
            ("exploration", exploration_json("Q", &[])),
        ])
    };
    let atlas = |title: &str, entries: &[String]| {
        jobj(&[("title", jstr(title)), ("entries", jarr(entries))])
    };
    let text = jobj(&[(
        "atlases",
        jarr(&[
            atlas("One", &[entry("a\u{2028}b"), entry("only")]),
            atlas("Two", &[entry("a\u{2028}b"), entry("only")]),
        ]),
    )]);
    let (found, defects) = load_lineage_text(&text);
    assert_eq!(defects, Vec::<String>::new());
    let chain = found.expect("the lineage loads whole");
    let lines = render_standing(&chain).expect("the standing reads");
    assert_eq!(
        lines,
        strings(&[
            "entry a\u{2028}b, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close",
            "entry only, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close",
            "standing counts, atlases 2, entries 2, standing at every atlas 2, \
             standing at the close 2",
        ])
    );
}
