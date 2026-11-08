//! Tests for MY-26, the departures reading.
//!
//! The reading takes a lineage, reads the presence of
//! every label in the one presence pass, and states one
//! section for each atlas of the chain, the entries that
//! stand in no later atlas named after the atlas they
//! were last seen in, in the standing line form, closing
//! with the departures counts. The pins here read the
//! reading over hand built chains and over lineage texts
//! built as JSON, and bind its closings to the absence
//! reading's closing runs and to the standing reading's
//! own lines.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_absence, render_departures, render_standing, Atlas, AtlasEntry,
    Lineage,
};
use myriad::explorations::{Branch, Exploration};
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

/// The example chain of the reading's form, three
/// atlases over ferry, lifeboat, and ice.
fn form_chain() -> Lineage {
    Lineage {
        atlases: vec![
            atlas("One", &["ferry", "lifeboat"]),
            atlas("Two", &["lifeboat", "ice"]),
            atlas("Three", &["ice", "ferry"]),
        ],
    }
}

/// A chain where an entry leaves, returns, and leaves
/// again, departing once, after its last standing.
fn return_chain() -> Lineage {
    Lineage {
        atlases: vec![
            atlas("One", &["alpha", "beta"]),
            atlas("Two", &["beta"]),
            atlas("Three", &["alpha", "beta"]),
            atlas("Four", &["beta"]),
        ],
    }
}

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

/// The departures after each atlas, read out of the
/// absence reading's own closing runs: a run that starts
/// at atlas N and runs to the close departs after atlas
/// N - 1.
fn closing_run_departures(chain: &Lineage) -> Vec<usize> {
    let absence = render_absence(chain).expect("the chain reads");
    let mut grouped = vec![0usize; chain.atlases.len()];
    for line in &absence {
        if line.contains(", to the close") {
            let start = figure(line, "absent from atlas ", ",");
            grouped[start - 2] += 1;
        }
    }
    grouped
}

#[test]
fn the_departures_reading_states_its_form() {
    let lines = render_departures(&form_chain()).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "atlas 2",
            "entry lifeboat, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, does not stand at the close",
            "atlas 3",
            "departures counts, atlas 1 departures 0, atlas 2 departures 1, \
             atlas 3 departures 0, entries departed 1, entries 3, atlases 3",
        ])
    );
}

#[test]
fn an_entry_that_leaves_returns_and_leaves_again_departs_once() {
    // Alpha stands absent through the second atlas and
    // stands again in the third, so the absence inside
    // the chain is not a departure, and alpha departs
    // after the third atlas alone.
    let lines = render_departures(&return_chain()).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "atlas 2",
            "atlas 3",
            "entry alpha, first stood at atlas 1, last stood at atlas 3, \
             stands in 2 atlases, does not stand at the close",
            "atlas 4",
            "departures counts, atlas 1 departures 0, atlas 2 departures 0, \
             atlas 3 departures 1, atlas 4 departures 0, entries departed 1, \
             entries 2, atlases 4",
        ])
    );
}

#[test]
fn the_last_atlas_always_departs_nothing() {
    for chain in [form_chain(), return_chain()] {
        let lines = render_departures(&chain).expect("the chain reads");
        let closing = lines.last().expect("a closing line");
        let total = chain.atlases.len();
        let last = figure(closing, &format!("atlas {total} departures "), ",");
        assert_eq!(last, 0);
        // The last section header stands directly above
        // the counts line, with no entry under it.
        assert_eq!(lines[lines.len() - 2], format!("atlas {total}"));
    }
}

#[test]
fn the_per_atlas_departures_agree_with_the_absence_closing_runs() {
    for chain in [form_chain(), return_chain()] {
        let lines = render_departures(&chain).expect("the chain reads");
        let closing = lines.last().expect("a closing line");
        let grouped = closing_run_departures(&chain);
        let mut sum = 0usize;
        for (index, expected) in grouped.iter().enumerate() {
            let count = figure(closing, &format!("atlas {} departures ", index + 1), ",");
            assert_eq!(count, *expected, "atlas {} departures", index + 1);
            sum += count;
        }
        let absence = render_absence(&chain).expect("the chain reads");
        let absence_closing = absence.last().expect("a closing line");
        let at_close: usize = absence_closing
            .split("absent at the close ")
            .nth(1)
            .expect("the closing figure stands")
            .parse()
            .expect("a count");
        assert_eq!(figure(closing, "entries departed ", ","), at_close);
        assert_eq!(sum, at_close);
    }
}

#[test]
fn every_named_entry_line_is_the_standing_line_of_an_entry_absent_at_the_close() {
    for chain in [form_chain(), return_chain()] {
        let standing = render_standing(&chain).expect("the chain reads");
        let lines = render_departures(&chain).expect("the chain reads");
        let mut section = 0usize;
        for line in &lines {
            if let Some(number) = line.strip_prefix("atlas ") {
                section = number.parse().expect("a section number");
            } else if line.starts_with("entry ") {
                assert!(standing.contains(line), "the line stands: {line}");
                assert!(line.ends_with("does not stand at the close"));
                let last = figure(line, ", last stood at atlas ", ",");
                assert_eq!(last, section, "the entry departs after its last standing");
            }
        }
    }
}

#[test]
fn fewer_than_two_atlases_draws_the_count_refusal() {
    let chain = Lineage {
        atlases: vec![atlas("One", &["a"])],
    };
    assert_eq!(
        render_departures(&chain),
        Err("the lineage holds fewer than two atlases".to_string())
    );
}

#[test]
fn a_label_holding_an_unpaired_surrogate_is_refused() {
    let mut broken = atlas("Two", &["a"]);
    broken.entries[0].label = PText(vec![0x61, 0xD800]);
    let chain = Lineage {
        atlases: vec![atlas("One", &["a"]), broken],
    };
    assert_eq!(
        render_departures(&chain),
        Err("an entry label holds an unpaired surrogate".to_string())
    );
}

#[test]
fn a_label_holding_a_line_break_is_refused() {
    let chain = Lineage {
        atlases: vec![atlas("One", &["a"]), atlas("Two", &["a\nb"])],
    };
    assert_eq!(
        render_departures(&chain),
        Err("an entry label holds a newline or a carriage return".to_string())
    );
}

fn atlas_text(title: &str, labels: &[&str]) -> String {
    let entries: Vec<String> = labels
        .iter()
        .map(|label| {
            jobj(&[
                ("label", jstr(label)),
                ("exploration", exploration_json("Q", &[])),
            ])
        })
        .collect();
    jobj(&[("title", jstr(title)), ("entries", jarr(&entries))])
}

#[test]
fn a_loaded_chain_renders_the_reading_of_its_labels() {
    let text = jobj(&[(
        "atlases",
        jarr(&[
            atlas_text("One", &["kept", "gone"]),
            atlas_text("Two", &["kept"]),
            atlas_text("Three", &["kept", "gone", "new"]),
        ]),
    )]);
    let (found, defects) = load_lineage_text(&text);
    assert_eq!(defects, Vec::<String>::new());
    let lineage = found.expect("the chain loads");
    let lines = render_departures(&lineage).expect("the chain reads");
    // Gone returns in the third atlas and stands at the
    // close, so nothing departs at all.
    assert_eq!(
        lines.last().expect("a closing line"),
        "departures counts, atlas 1 departures 0, atlas 2 departures 0, \
         atlas 3 departures 0, entries departed 0, entries 3, atlases 3"
    );
}

#[test]
fn a_chain_past_a_thousand_branches_renders_whole() {
    let wide = |labels: &[&str]| -> Atlas {
        Atlas {
            title: pt("T"),
            entries: labels
                .iter()
                .map(|label| AtlasEntry {
                    label: pt(label),
                    exploration: Exploration {
                        question: pt("q"),
                        branches: (0..40)
                            .map(|n| branch(&format!("b{n}"), None, "p", "open", None, None, None))
                            .collect::<Vec<Branch>>(),
                    },
                })
                .collect(),
        }
    };
    let labels: Vec<String> = (0..30).map(|i| format!("entry {i}")).collect();
    let refs: Vec<&str> = labels.iter().map(String::as_str).collect();
    let chain = Lineage {
        atlases: vec![wide(&refs), wide(&["entry 0"])],
    };
    let lines = render_departures(&chain).expect("the chain reads");
    assert_eq!(lines.len(), 1 + 29 + 1 + 1);
    assert_eq!(
        lines.last().expect("a closing line"),
        "departures counts, atlas 1 departures 29, atlas 2 departures 0, \
         entries departed 29, entries 30, atlases 2"
    );
}

#[test]
fn the_reading_holds_the_vocabulary_bar() {
    let lines = render_departures(&form_chain()).expect("the chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert!(holds_vocabulary_bar(&text));
}

#[test]
fn a_label_held_twice_in_one_atlas_departs_once() {
    // FIX-MY-17. The loaders refuse a label carried by
    // two entries, so no loaded chain reaches this shape,
    // and the shape stands on the library surface alone.
    // Over it the reading reads the label once, at its
    // first standing in the atlas, and the departures
    // taken together equal the count the absence reading
    // states as absent at the close over the same value.
    // On the unrepaired reading the standing line of dup
    // stands twice in the first section and the closing
    // states entries departed 3, entries 3, so this pin
    // fails there by assertion.
    let chain = Lineage {
        atlases: vec![
            atlas("One", &["dup", "dup", "solo"]),
            atlas("Two", &["other"]),
        ],
    };
    let lines = render_departures(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "entry dup, first stood at atlas 1, last stood at atlas 1, \
             stands in 1 atlases, does not stand at the close",
            "entry solo, first stood at atlas 1, last stood at atlas 1, \
             stands in 1 atlases, does not stand at the close",
            "atlas 2",
            "departures counts, atlas 1 departures 2, atlas 2 departures 0, \
             entries departed 2, entries 3, atlases 2",
        ])
    );
    let absence = render_absence(&chain).expect("the chain reads");
    let absence_closing = absence.last().expect("a closing line");
    let at_close: usize = absence_closing
        .split("absent at the close ")
        .nth(1)
        .expect("the closing figure stands")
        .parse()
        .expect("a count");
    let closing = lines.last().expect("a closing line");
    assert_eq!(figure(closing, "entries departed ", ","), at_close);
}

#[test]
fn a_label_held_three_times_in_one_atlas_departs_once() {
    // FIX-MY-17. The strongest form of the shape above.
    // On the unrepaired reading the closing states
    // entries departed 3, entries 2, a count of departed
    // entries larger than the count of entries itself,
    // so this pin fails there by assertion.
    let chain = Lineage {
        atlases: vec![
            atlas("One", &["dup", "dup", "dup"]),
            atlas("Two", &["other"]),
        ],
    };
    let lines = render_departures(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "entry dup, first stood at atlas 1, last stood at atlas 1, \
             stands in 1 atlases, does not stand at the close",
            "atlas 2",
            "departures counts, atlas 1 departures 1, atlas 2 departures 0, \
             entries departed 1, entries 2, atlases 2",
        ])
    );
    let absence = render_absence(&chain).expect("the chain reads");
    let absence_closing = absence.last().expect("a closing line");
    let at_close: usize = absence_closing
        .split("absent at the close ")
        .nth(1)
        .expect("the closing figure stands")
        .parse()
        .expect("a count");
    let closing = lines.last().expect("a closing line");
    assert_eq!(figure(closing, "entries departed ", ","), at_close);
}
