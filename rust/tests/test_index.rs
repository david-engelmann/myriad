//! Pins for the atlas index reading of MY-9, translated
//! from tests/test_index.py.
//!
//! The index is the atlas as a table of contents. Its first
//! line is the title line of the atlas rendering and its
//! last line is the atlas counts line of the atlas
//! rendering, and one line stands for each entry between
//! them, carrying the label, the question, and the counts
//! of that entry alone, read from the one counting path.

mod common;

use myriad::atlas::{render_atlas, render_index, Atlas, AtlasEntry};
use myriad::explorations::{render_census, Exploration};
use myriad::pytext::PText;

use common::{branch, holds_vocabulary_bar, pt};

fn entry(label: &str, question: &str, branches: Vec<myriad::explorations::Branch>) -> AtlasEntry {
    AtlasEntry {
        label: pt(label),
        exploration: Exploration {
            question: pt(question),
            branches,
        },
    }
}

fn sound() -> Atlas {
    Atlas {
        title: pt("the title"),
        entries: vec![
            entry(
                "first",
                "q one",
                vec![
                    branch(
                        "a",
                        None,
                        "prompt of a",
                        "taken",
                        Some("crew"),
                        Some("f"),
                        None,
                    ),
                    branch("b", Some("a"), "prompt of b", "open", None, None, None),
                ],
            ),
            entry("second", "q two", vec![]),
            entry(
                "third",
                "q three",
                vec![
                    branch("c", None, "prompt of c", "abandoned", None, None, Some("r")),
                    branch(
                        "d",
                        None,
                        "prompt of d",
                        "abandoned",
                        None,
                        None,
                        Some("r2"),
                    ),
                    branch("e", None, "prompt of e", "open", None, None, None),
                    branch("f", None, "prompt of f", "taken", None, Some("f2"), None),
                    branch("g", None, "prompt of g", "open", None, None, None),
                ],
            ),
        ],
    }
}

/// The four counts at the end of a line, in the order the
/// counts line states them.
fn closing_counts(line: &str) -> [usize; 4] {
    let start = line.rfind("branches ").expect("a counts tail") + "branches ".len();
    let tail = &line[start..];
    let parts: Vec<&str> = tail.split(", ").collect();
    let number = |part: &str, prefix: &str| -> usize {
        part.strip_prefix(prefix)
            .expect("a counts part")
            .parse()
            .expect("a count")
    };
    [
        parts[0].parse().expect("a count"),
        number(parts[1], "open "),
        number(parts[2], "taken "),
        number(parts[3], "abandoned "),
    ]
}

#[test]
fn the_index_of_a_sound_atlas_line_for_line() {
    assert_eq!(
        render_index(&sound()).expect("a sound rendering"),
        common::strings(&[
            "atlas, the title",
            "entry first, question, q one, branches 2, open 1, taken 1, abandoned 0",
            "entry second, question, q two, branches 0, open 0, taken 0, abandoned 0",
            "entry third, question, q three, branches 5, open 2, taken 1, abandoned 2",
            "atlas counts, entries 3, branches 7, open 3, taken 2, abandoned 2",
        ])
    );
}

#[test]
fn the_index_opens_and_closes_with_the_atlas_rendering_lines() {
    let atlas = sound();
    let rendering = render_atlas(&atlas).expect("a sound rendering");
    let index = render_index(&atlas).expect("a sound rendering");
    assert_eq!(index[0], rendering[0]);
    assert_eq!(index[index.len() - 1], rendering[rendering.len() - 1]);
}

#[test]
fn one_entry_line_stands_for_each_entry_in_file_order() {
    let atlas = Atlas {
        title: pt("t"),
        entries: vec![
            entry(
                "wide",
                "q wide",
                (0..9)
                    .map(|i| {
                        branch(
                            &format!("w{i}"),
                            None,
                            &format!("prompt of w{i}"),
                            "open",
                            None,
                            None,
                            None,
                        )
                    })
                    .collect(),
            ),
            entry(
                "narrow",
                "q narrow",
                vec![branch("n", None, "prompt of n", "open", None, None, None)],
            ),
            entry(
                "middle",
                "q middle",
                (0..4)
                    .map(|i| {
                        branch(
                            &format!("m{i}"),
                            None,
                            &format!("prompt of m{i}"),
                            "open",
                            None,
                            None,
                            None,
                        )
                    })
                    .collect(),
            ),
        ],
    };
    let index = render_index(&atlas).expect("a sound rendering");
    assert_eq!(index.len(), atlas.entries.len() + 2);
    let labels: Vec<&str> = index[1..index.len() - 1]
        .iter()
        .map(|line| line.split(", question,").next().expect("a label part"))
        .collect();
    assert_eq!(labels, vec!["entry wide", "entry narrow", "entry middle"]);
}

#[test]
fn each_entry_line_counts_its_own_entry_alone() {
    let atlas = sound();
    let index = render_index(&atlas).expect("a sound rendering");
    for (line, item) in index[1..index.len() - 1].iter().zip(atlas.entries.iter()) {
        let census = render_census(&item.exploration).expect("a sound rendering");
        let closing = census.last().expect("a closing line");
        assert!(line.ends_with(closing.as_str()));
    }
}

#[test]
fn the_entry_counts_sum_to_the_closing_counts() {
    let index = render_index(&sound()).expect("a sound rendering");
    let mut totals = [0usize; 4];
    for line in &index[1..index.len() - 1] {
        let counts = closing_counts(line);
        for place in 0..4 {
            totals[place] += counts[place];
        }
    }
    assert_eq!(closing_counts(&index[index.len() - 1]), totals);
}

#[test]
fn an_entry_with_no_branches_carries_the_zero_counts() {
    let atlas = Atlas {
        title: pt("t"),
        entries: vec![entry("empty", "q", vec![])],
    };
    assert_eq!(
        render_index(&atlas).expect("a sound rendering"),
        common::strings(&[
            "atlas, t",
            "entry empty, question, q, branches 0, open 0, taken 0, abandoned 0",
            "atlas counts, entries 1, branches 0, open 0, taken 0, abandoned 0",
        ])
    );
}

#[test]
fn an_atlas_with_no_entries_renders_two_lines() {
    let atlas = Atlas {
        title: pt("t"),
        entries: vec![],
    };
    assert_eq!(
        render_index(&atlas).expect("a sound rendering"),
        common::strings(&[
            "atlas, t",
            "atlas counts, entries 0, branches 0, open 0, taken 0, abandoned 0",
        ])
    );
}

#[test]
fn the_index_carries_no_branch_claim_or_prompt_text() {
    let atlas = Atlas {
        title: pt("t"),
        entries: vec![entry(
            "only",
            "the question",
            vec![branch(
                "a",
                None,
                "a distinctive prompt",
                "taken",
                Some("a distinctive claim"),
                Some("a distinctive finding"),
                None,
            )],
        )],
    };
    let lines = render_index(&atlas).expect("a sound rendering");
    assert_eq!(lines.len(), 3);
    let joined = lines.join("\n");
    assert!(!joined.contains("a distinctive prompt"));
    assert!(!joined.contains("a distinctive finding"));
    assert!(!joined.contains("a distinctive claim"));
    assert!(!joined.contains("claimed by"));
}

#[test]
fn the_index_of_a_wide_atlas_renders_whole() {
    let atlas = Atlas {
        title: pt("t"),
        entries: (0..3)
            .map(|i| {
                entry(
                    &format!("e{i}"),
                    &format!("q {i}"),
                    (0..400)
                        .map(|j| {
                            branch(
                                &format!("b{i}-{j}"),
                                None,
                                &format!("prompt of b{i}-{j}"),
                                "open",
                                None,
                                None,
                                None,
                            )
                        })
                        .collect(),
                )
            })
            .collect(),
    };
    let lines = render_index(&atlas).expect("a sound rendering");
    assert_eq!(lines.len(), 5);
    assert_eq!(
        lines[lines.len() - 1],
        "atlas counts, entries 3, branches 1200, open 1200, taken 0, abandoned 0"
    );
}

#[test]
fn the_index_reading_holds_to_the_vocabulary_bar() {
    let joined = render_index(&sound())
        .expect("a sound rendering")
        .join("\n");
    assert!(holds_vocabulary_bar(&joined));
}

// The PORT-11 pins. The index prints the title, the entry
// labels, and the entry questions, and a hand-built value
// holding an unpaired surrogate in one of those fields
// draws the named refusal.

/// A text holding an unpaired surrogate after one letter,
/// the hand-built shape the readings refuse by field.
fn surrogate_text() -> PText {
    PText(vec![0x61, 0xD800])
}

#[test]
fn a_surrogate_atlas_title_draws_the_refusal_in_the_index() {
    let atlas = Atlas {
        title: surrogate_text(),
        entries: vec![],
    };
    assert_eq!(
        render_index(&atlas).expect_err("a refusal"),
        "the atlas title holds an unpaired surrogate"
    );
}

#[test]
fn a_surrogate_entry_label_draws_the_refusal_in_the_index() {
    let mut atlas = sound();
    atlas.entries[0].label = surrogate_text();
    assert_eq!(
        render_index(&atlas).expect_err("a refusal"),
        "an entry label holds an unpaired surrogate"
    );
}

#[test]
fn a_surrogate_question_draws_the_refusal_in_the_index() {
    let mut atlas = sound();
    atlas.entries[0].exploration.question = surrogate_text();
    assert_eq!(
        render_index(&atlas).expect_err("a refusal"),
        "the question holds an unpaired surrogate"
    );
}
