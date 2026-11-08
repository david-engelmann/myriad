//! Tests for MY-23 and MY-24, the arrivals reading.
//!
//! The reading takes a lineage, reads the presence of
//! every label in the one presence pass, and states one
//! section for each atlas of the chain, the entries
//! arriving at that atlas in the standing line form,
//! closing with the arrivals counts. The pins here read
//! the reading over hand built chains and over lineage
//! texts built as JSON, and bind its closings to the
//! standing reading and to the index reading.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_arrivals, render_index, render_standing, Atlas, AtlasEntry, Lineage,
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

#[test]
fn the_arrivals_reading_states_its_form() {
    let lines = render_arrivals(&form_chain()).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "entry ferry, first stood at atlas 1, last stood at atlas 3, \
             stands in 2 atlases, stands at the close",
            "entry lifeboat, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, does not stand at the close",
            "atlas 2",
            "entry ice, first stood at atlas 2, last stood at atlas 3, \
             stands in 2 atlases, stands at the close",
            "atlas 3",
            "arrivals counts, atlas 1 arrivals 2, atlas 2 arrivals 1, \
             atlas 3 arrivals 0, entries 3, atlases 3",
        ])
    );
}

#[test]
fn an_entry_standing_again_does_not_arrive_again() {
    // The ferry stands in the first and the third atlas
    // of the form chain and arrives only at the first,
    // its standing again the return the returns reading
    // names, and the third section stands empty.
    let lines = render_arrivals(&form_chain()).expect("the chain reads");
    let third = lines
        .iter()
        .position(|line| line == "atlas 3")
        .expect("the third section opens");
    assert!(lines[third + 1].starts_with("arrivals counts"));
    let ferry_lines = lines
        .iter()
        .filter(|line| line.starts_with("entry ferry,"))
        .count();
    assert_eq!(ferry_lines, 1);
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

#[test]
fn the_arrivals_sum_to_the_standing_entries() {
    let chain = form_chain();
    let arrivals = render_arrivals(&chain).expect("the chain reads");
    let closing = arrivals.last().expect("a closing line");
    let sum: usize = (1..=3)
        .map(|n| figure(closing, &format!("atlas {n} arrivals "), ","))
        .sum();
    let standing = render_standing(&chain).expect("the chain reads");
    let standing_closing = standing.last().expect("a closing line");
    let entries = figure(standing_closing, "entries ", ",");
    assert_eq!(sum, entries);
    assert_eq!(sum, 3);
}

#[test]
fn the_first_atlas_arrivals_equal_the_first_atlas_entries() {
    let chain = form_chain();
    let arrivals = render_arrivals(&chain).expect("the chain reads");
    let closing = arrivals.last().expect("a closing line");
    let first = figure(closing, "atlas 1 arrivals ", ",");
    let index = render_index(&chain.atlases[0]).expect("the atlas reads");
    let index_closing = index.last().expect("a closing line");
    let entries = figure(index_closing, "atlas counts, entries ", ",");
    assert_eq!(first, entries);
    assert_eq!(first, 2);
}

#[test]
fn the_labels_at_every_atlas_stand_in_the_first_section() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", &["harbour", "ferry"]),
            atlas("Two", &["harbour", "ice"]),
            atlas("Three", &["harbour"]),
        ],
    };
    let standing = render_standing(&chain).expect("the chain reads");
    let arrivals = render_arrivals(&chain).expect("the chain reads");
    let second = arrivals
        .iter()
        .position(|line| line == "atlas 2")
        .expect("the second section opens");
    let first_section = &arrivals[1..second];
    let at_every: Vec<&String> = standing
        .iter()
        .filter(|line| line.starts_with("entry ") && line.contains("stands in 3 atlases"))
        .collect();
    assert_eq!(at_every.len(), 1);
    for line in &at_every {
        assert!(first_section.contains(line), "line {line}");
    }
    assert!(at_every.len() <= first_section.len());
}

#[test]
fn a_section_where_nothing_arrives_stands_as_its_opening() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", &["a", "b"]),
            atlas("Two", &["b", "a"]),
            atlas("Three", &["c"]),
        ],
    };
    let lines = render_arrivals(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "entry a, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, does not stand at the close",
            "entry b, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, does not stand at the close",
            "atlas 2",
            "atlas 3",
            "entry c, first stood at atlas 3, last stood at atlas 3, \
             stands in 1 atlases, stands at the close",
            "arrivals counts, atlas 1 arrivals 2, atlas 2 arrivals 0, \
             atlas 3 arrivals 1, entries 3, atlases 3",
        ])
    );
}

#[test]
fn a_chain_of_two_renders_its_arrivals() {
    let chain = Lineage {
        atlases: vec![atlas("One", &["a"]), atlas("Two", &["a", "b"])],
    };
    let lines = render_arrivals(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "entry a, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close",
            "atlas 2",
            "entry b, first stood at atlas 2, last stood at atlas 2, \
             stands in 1 atlases, stands at the close",
            "arrivals counts, atlas 1 arrivals 1, atlas 2 arrivals 1, \
             entries 2, atlases 2",
        ])
    );
}

#[test]
fn fewer_than_two_atlases_draws_the_count_refusal() {
    let chain = Lineage {
        atlases: vec![atlas("One", &["a"])],
    };
    assert_eq!(
        render_arrivals(&chain),
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
        render_arrivals(&chain),
        Err("an entry label holds an unpaired surrogate".to_string())
    );
}

#[test]
fn a_label_holding_a_line_break_is_refused() {
    let chain = Lineage {
        atlases: vec![atlas("One", &["a"]), atlas("Two", &["a\nb"])],
    };
    assert_eq!(
        render_arrivals(&chain),
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
    let lines = render_arrivals(&lineage).expect("the chain reads");
    assert_eq!(
        lines.last().expect("a closing line"),
        "arrivals counts, atlas 1 arrivals 2, atlas 2 arrivals 0, \
         atlas 3 arrivals 1, entries 3, atlases 3"
    );
}

#[test]
fn a_member_defect_loads_no_chain_at_all() {
    let member = jobj(&[("entries", jarr(&[]))]);
    let text = jobj(&[("atlases", jarr(&[atlas_text("One", &["only"]), member]))]);
    let (found, defects) = load_lineage_text(&text);
    assert!(found.is_none());
    assert_eq!(defects, strings(&["atlas 1, missing field title"]));
}

#[test]
fn a_chain_past_a_thousand_branches_renders_whole() {
    let wide = |label: &str| -> Atlas {
        let branches: Vec<Branch> = (0..600)
            .map(|n| branch(&format!("b{n}"), None, "p", "open", None, None, None))
            .collect();
        Atlas {
            title: pt("T"),
            entries: vec![AtlasEntry {
                label: pt(label),
                exploration: Exploration {
                    question: pt("q"),
                    branches,
                },
            }],
        }
    };
    let chain = Lineage {
        atlases: vec![wide("one"), wide("two")],
    };
    let lines = render_arrivals(&chain).expect("the chain reads");
    assert_eq!(
        lines.last().expect("a closing line"),
        "arrivals counts, atlas 1 arrivals 1, atlas 2 arrivals 1, \
         entries 2, atlases 2"
    );
}

#[test]
fn the_reading_holds_the_vocabulary_bar() {
    let lines = render_arrivals(&form_chain()).expect("the chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert!(holds_vocabulary_bar(&text));
}
