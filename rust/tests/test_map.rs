//! Pins for the map rendering and the counts, translated
//! from tests/test_map.py.
//!
//! The rendering is pinned line for line on a tree whose
//! list order and tree order disagree, so the walk order
//! the contract states is the only order that passes. The
//! Python corpus hands the raw tree to the renderer, so the
//! Rust corpus builds the same tree as values.

mod common;

use myriad::explorations::{
    count_by_state, load_exploration_text, render_map, Counts, Exploration,
};
use myriad::pytext::PText;

use common::{branch, branch_json, exploration_json, pt};

fn tree() -> Exploration {
    Exploration {
        question: pt("How does the town keep its ferry?"),
        branches: vec![
            branch(
                "child",
                Some("vessel"),
                "What fuel",
                "open",
                None,
                None,
                None,
            ),
            branch(
                "vessel",
                None,
                "What vessel",
                "taken",
                None,
                Some("The spare boat serves"),
                None,
            ),
            branch("crew", None, "Who crews it", "open", None, None, None),
            branch(
                "old",
                Some("vessel"),
                "What sank",
                "abandoned",
                None,
                None,
                Some("Settled by the survey"),
            ),
            branch(
                "grand",
                Some("child"),
                "What dosing",
                "open",
                None,
                None,
                None,
            ),
        ],
    }
}

#[test]
fn the_map_renders_line_for_line() {
    assert_eq!(
        render_map(&tree()).expect("a sound rendering"),
        common::strings(&[
            "question, How does the town keep its ferry?",
            "vessel (taken) What vessel",
            "  finding, The spare boat serves",
            "  child (open) What fuel",
            "    grand (open) What dosing",
            "  old (abandoned) What sank",
            "    reason, Settled by the survey",
            "crew (open) Who crews it",
            "branches 5, open 3, taken 1, abandoned 1",
        ])
    );
}

#[test]
fn roots_stand_in_list_order_and_children_with_them() {
    let lines = render_map(&tree()).expect("a sound rendering");
    let position = |wanted: &str| lines.iter().position(|line| line == wanted).unwrap();
    assert!(position("vessel (taken) What vessel") < position("crew (open) Who crews it"));
    assert!(position("  child (open) What fuel") < position("  old (abandoned) What sank"));
}

#[test]
fn the_same_exploration_renders_byte_identical_lines() {
    let first = render_map(&tree()).expect("a sound rendering");
    let second = render_map(&tree()).expect("a sound rendering");
    assert_eq!(first, second);
    assert_eq!(
        first.join("\n").into_bytes(),
        second.join("\n").into_bytes()
    );
}

#[test]
fn counts_are_counted_from_the_tree() {
    assert_eq!(
        count_by_state(&tree()),
        Counts {
            total: 5,
            open: 3,
            taken: 1,
            abandoned: 1,
        }
    );
}

#[test]
fn the_summary_line_reads_the_counts() {
    let exploration = tree();
    let counts = count_by_state(&exploration);
    let lines = render_map(&exploration).expect("a sound rendering");
    let summary = lines.last().expect("a closing line");
    assert_eq!(
        summary,
        &format!(
            "branches {}, open {}, taken {}, abandoned {}",
            counts.total, counts.open, counts.taken, counts.abandoned
        )
    );
}

#[test]
fn an_empty_tree_renders_the_question_and_zero_counts() {
    let exploration = Exploration {
        question: pt("q"),
        branches: vec![],
    };
    assert_eq!(
        render_map(&exploration).expect("a sound rendering"),
        common::strings(&["question, q", "branches 0, open 0, taken 0, abandoned 0"])
    );
}

#[test]
fn a_chain_of_five_thousand_renders_in_full() {
    let branches: Vec<String> = (0..5000)
        .map(|i| {
            let parent = if i == 0 {
                None
            } else {
                Some(format!("b{}", i - 1))
            };
            branch_json(
                &format!("b{i}"),
                parent.as_deref(),
                &format!("p{i}"),
                "open",
            )
        })
        .collect();
    let (found, defects) = load_exploration_text(&exploration_json("q", &branches));
    assert_eq!(defects, Vec::<String>::new());
    let exploration = found.expect("a clean load");
    let lines = render_map(&exploration).expect("a sound rendering");
    assert_eq!(lines.len(), 5002);
    assert_eq!(lines[0], "question, q");
    assert_eq!(lines[1], "b0 (open) p0");
    assert_eq!(
        lines[lines.len() - 2],
        format!("{}b4999 (open) p4999", "  ".repeat(4999))
    );
    assert_eq!(
        lines[lines.len() - 1],
        "branches 5000, open 5000, taken 0, abandoned 0"
    );
}

#[test]
fn open_branches_carry_no_extra_line() {
    let exploration = Exploration {
        question: pt("q"),
        branches: vec![branch("a", None, "p", "open", None, None, None)],
    };
    assert_eq!(
        render_map(&exploration).expect("a sound rendering"),
        common::strings(&[
            "question, q",
            "a (open) p",
            "branches 1, open 1, taken 0, abandoned 0",
        ])
    );
}

// The PORT-11 pins. The finding and reason lines key on the
// branch state alone, and a hand-built value that breaks the
// loader preconditions draws the named refusal.

/// A text holding an unpaired surrogate after one letter,
/// the hand-built shape the readings refuse by field.
fn surrogate_text() -> PText {
    PText(vec![0x61, 0xD800])
}

#[test]
fn the_map_keys_the_finding_and_reason_lines_on_state_alone() {
    let exploration = Exploration {
        question: pt("q"),
        branches: vec![
            branch(
                "t",
                None,
                "p",
                "taken",
                None,
                Some("found it"),
                Some("set aside"),
            ),
            branch(
                "a",
                None,
                "p2",
                "abandoned",
                None,
                Some("found it"),
                Some("set aside"),
            ),
            branch(
                "o",
                None,
                "p3",
                "open",
                None,
                Some("found it"),
                Some("set aside"),
            ),
        ],
    };
    assert_eq!(
        render_map(&exploration).expect("a sound rendering"),
        common::strings(&[
            "question, q",
            "t (taken) p",
            "  finding, found it",
            "a (abandoned) p2",
            "  reason, set aside",
            "o (open) p3",
            "branches 3, open 1, taken 1, abandoned 1",
        ])
    );
}

#[test]
fn a_taken_branch_holding_no_finding_draws_the_refusal() {
    let exploration = Exploration {
        question: pt("q"),
        branches: vec![branch("t", None, "p", "taken", None, None, None)],
    };
    assert_eq!(
        render_map(&exploration).expect_err("a refusal"),
        "a taken branch holds no finding"
    );
}

#[test]
fn an_abandoned_branch_holding_no_reason_draws_the_refusal() {
    let exploration = Exploration {
        question: pt("q"),
        branches: vec![branch("a", None, "p", "abandoned", None, None, None)],
    };
    assert_eq!(
        render_map(&exploration).expect_err("a refusal"),
        "an abandoned branch holds no reason"
    );
}

#[test]
fn a_surrogate_question_draws_the_refusal_in_the_map() {
    let exploration = Exploration {
        question: surrogate_text(),
        branches: vec![],
    };
    assert_eq!(
        render_map(&exploration).expect_err("a refusal"),
        "the question holds an unpaired surrogate"
    );
}

#[test]
fn a_surrogate_branch_id_draws_the_refusal_in_the_map() {
    let mut held = branch("a", None, "p", "open", None, None, None);
    held.id = surrogate_text();
    let exploration = Exploration {
        question: pt("q"),
        branches: vec![held],
    };
    assert_eq!(
        render_map(&exploration).expect_err("a refusal"),
        "a branch id holds an unpaired surrogate"
    );
}

#[test]
fn a_surrogate_prompt_draws_the_refusal_in_the_map() {
    let mut held = branch("a", None, "p", "open", None, None, None);
    held.prompt = surrogate_text();
    let exploration = Exploration {
        question: pt("q"),
        branches: vec![held],
    };
    assert_eq!(
        render_map(&exploration).expect_err("a refusal"),
        "a branch prompt holds an unpaired surrogate"
    );
}

#[test]
fn a_surrogate_claim_draws_the_refusal_in_the_map() {
    let mut held = branch("a", None, "p", "open", None, None, None);
    held.claim = Some(surrogate_text());
    let exploration = Exploration {
        question: pt("q"),
        branches: vec![held],
    };
    assert_eq!(
        render_map(&exploration).expect_err("a refusal"),
        "a claim holds an unpaired surrogate"
    );
}

#[test]
fn a_surrogate_finding_draws_the_refusal_in_the_map() {
    let mut held = branch("a", None, "p", "taken", None, Some("found it"), None);
    held.finding = Some(surrogate_text());
    let exploration = Exploration {
        question: pt("q"),
        branches: vec![held],
    };
    assert_eq!(
        render_map(&exploration).expect_err("a refusal"),
        "a finding holds an unpaired surrogate"
    );
}

#[test]
fn a_surrogate_reason_draws_the_refusal_in_the_map() {
    let mut held = branch("a", None, "p", "abandoned", None, None, Some("set aside"));
    held.reason = Some(surrogate_text());
    let exploration = Exploration {
        question: pt("q"),
        branches: vec![held],
    };
    assert_eq!(
        render_map(&exploration).expect_err("a refusal"),
        "a reason holds an unpaired surrogate"
    );
}
