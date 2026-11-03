//! Pins for the territory census in MY-6 full form,
//! translated from tests/test_census.py.
//!
//! The census is pinned line for line on a tree whose
//! branches list interleaves the states and whose claim
//! labels appear out of alphabetical order, so the section
//! order, the grouping order, and the unclaimed heading
//! the contract states are the only readings that pass.

mod common;

use myriad::explorations::{render_census, render_map, Exploration};
use myriad::pytext::PText;

use common::{branch, loads, pt};

fn census_tree() -> Exploration {
    Exploration {
        question: pt("q"),
        branches: vec![
            branch("o1", None, "first open", "open", None, None, None),
            branch(
                "t1",
                None,
                "p",
                "taken",
                Some("tide watch"),
                Some("found one"),
                None,
            ),
            branch(
                "t2",
                None,
                "p",
                "taken",
                Some("boat crew"),
                Some("found two"),
                None,
            ),
            branch(
                "t3",
                None,
                "p",
                "taken",
                Some("tide watch"),
                Some("found three"),
                None,
            ),
            branch("t4", None, "p", "taken", None, Some("found four"), None),
            branch(
                "a1",
                None,
                "p",
                "abandoned",
                Some("boat crew"),
                None,
                Some("died one"),
            ),
            branch("a2", None, "p", "abandoned", None, None, Some("died two")),
            branch(
                "o2",
                None,
                "second open",
                "open",
                Some("tide watch"),
                None,
                None,
            ),
        ],
    }
}

#[test]
fn the_census_renders_line_for_line() {
    assert_eq!(
        render_census(&census_tree()).expect("a sound rendering"),
        common::strings(&[
            "question, q",
            "taken",
            "  claim boat crew",
            "    t2, finding, found two",
            "  claim tide watch",
            "    t1, finding, found one",
            "    t3, finding, found three",
            "  unclaimed",
            "    t4, finding, found four",
            "abandoned",
            "  claim boat crew",
            "    a1, reason, died one",
            "  unclaimed",
            "    a2, reason, died two",
            "open",
            "  o1, prompt, first open",
            "  o2, prompt, second open",
            "branches 8, open 2, taken 4, abandoned 2",
        ])
    );
}

#[test]
fn the_open_section_ignores_claims_and_stands_last() {
    let lines = render_census(&census_tree()).expect("a sound rendering");
    let position = |wanted: &str| lines.iter().position(|line| line == wanted).unwrap();
    assert!(position("open") > position("abandoned"));
    let open_lines = &lines[position("open") + 1..lines.len() - 1];
    assert_eq!(
        open_lines,
        common::strings(&["  o1, prompt, first open", "  o2, prompt, second open"]).as_slice()
    );
}

#[test]
fn a_section_stands_only_when_its_state_is_held() {
    let found = loads(
        "{\"question\": \"q\", \"branches\": [{\"id\": \"a\", \"parent\": null, \
         \"prompt\": \"p\", \"state\": \"taken\", \"finding\": \"found it\"}]}",
    );
    assert_eq!(
        render_census(&found).expect("a sound rendering"),
        common::strings(&[
            "question, q",
            "taken",
            "  unclaimed",
            "    a, finding, found it",
            "branches 1, open 0, taken 1, abandoned 0",
        ])
    );
}

#[test]
fn the_census_of_an_empty_exploration_has_no_section() {
    let found = loads("{\"question\": \"q\", \"branches\": []}");
    assert_eq!(
        render_census(&found).expect("a sound rendering"),
        common::strings(&["question, q", "branches 0, open 0, taken 0, abandoned 0"])
    );
}

#[test]
fn the_census_counts_line_matches_the_map_counts_line() {
    let found = census_tree();
    let census = render_census(&found).expect("a sound rendering");
    let map = render_map(&found).expect("a sound rendering");
    assert_eq!(census.last(), map.last());
    assert_eq!(
        census.last().map(String::as_str),
        Some("branches 8, open 2, taken 4, abandoned 2")
    );
}

#[test]
fn the_census_carries_a_finding_in_full() {
    let long_finding = "The dosing already on the shelf covers both boats, \
        the spare included, through the coldest month on record";
    let found = Exploration {
        question: pt("q"),
        branches: vec![branch(
            "a",
            None,
            "p",
            "taken",
            Some("tide watch"),
            Some(long_finding),
            None,
        )],
    };
    assert_eq!(
        render_census(&found).expect("a sound rendering"),
        vec![
            "question, q".to_string(),
            "taken".to_string(),
            "  claim tide watch".to_string(),
            format!("    a, finding, {long_finding}"),
            "branches 1, open 0, taken 1, abandoned 0".to_string(),
        ]
    );
}

// The PORT-11 pins. The census reads the closed value the
// branch state names, and a hand-built value that breaks
// the loader preconditions draws the named refusal instead
// of failing at the process level.

/// A text holding an unpaired surrogate after one letter,
/// the hand-built shape the readings refuse by field.
fn surrogate_text() -> PText {
    PText(vec![0x61, 0xD800])
}

#[test]
fn a_taken_branch_holding_no_finding_draws_the_refusal_in_the_census() {
    let found = Exploration {
        question: pt("q"),
        branches: vec![branch("t", None, "p", "taken", None, None, None)],
    };
    assert_eq!(
        render_census(&found).expect_err("a refusal"),
        "a taken branch holds no finding"
    );
}

#[test]
fn an_abandoned_branch_holding_no_reason_draws_the_refusal_in_the_census() {
    let found = Exploration {
        question: pt("q"),
        branches: vec![branch("a", None, "p", "abandoned", None, None, None)],
    };
    assert_eq!(
        render_census(&found).expect_err("a refusal"),
        "an abandoned branch holds no reason"
    );
}

#[test]
fn the_census_reads_the_value_the_state_names() {
    let found = Exploration {
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
        ],
    };
    assert_eq!(
        render_census(&found).expect("a sound rendering"),
        common::strings(&[
            "question, q",
            "taken",
            "  unclaimed",
            "    t, finding, found it",
            "abandoned",
            "  unclaimed",
            "    a, reason, set aside",
            "branches 2, open 0, taken 1, abandoned 1",
        ])
    );
}

#[test]
fn a_surrogate_question_draws_the_refusal_in_the_census() {
    let found = Exploration {
        question: surrogate_text(),
        branches: vec![],
    };
    assert_eq!(
        render_census(&found).expect_err("a refusal"),
        "the question holds an unpaired surrogate"
    );
}

#[test]
fn a_surrogate_claim_draws_the_refusal_in_the_census() {
    let mut held = branch("t", None, "p", "taken", None, Some("found it"), None);
    held.claim = Some(surrogate_text());
    let found = Exploration {
        question: pt("q"),
        branches: vec![held],
    };
    assert_eq!(
        render_census(&found).expect_err("a refusal"),
        "a claim holds an unpaired surrogate"
    );
}

#[test]
fn a_surrogate_finding_draws_the_refusal_in_the_census() {
    let mut held = branch("t", None, "p", "taken", None, Some("found it"), None);
    held.finding = Some(surrogate_text());
    let found = Exploration {
        question: pt("q"),
        branches: vec![held],
    };
    assert_eq!(
        render_census(&found).expect_err("a refusal"),
        "a finding holds an unpaired surrogate"
    );
}

#[test]
fn a_surrogate_reason_draws_the_refusal_in_the_census() {
    let mut held = branch("a", None, "p", "abandoned", None, None, Some("set aside"));
    held.reason = Some(surrogate_text());
    let found = Exploration {
        question: pt("q"),
        branches: vec![held],
    };
    assert_eq!(
        render_census(&found).expect_err("a refusal"),
        "a reason holds an unpaired surrogate"
    );
}

#[test]
fn a_surrogate_open_prompt_draws_the_refusal_in_the_census() {
    let mut held = branch("o", None, "p", "open", None, None, None);
    held.prompt = surrogate_text();
    let found = Exploration {
        question: pt("q"),
        branches: vec![held],
    };
    assert_eq!(
        render_census(&found).expect_err("a refusal"),
        "a branch prompt holds an unpaired surrogate"
    );
}

#[test]
fn a_surrogate_open_id_draws_the_refusal_in_the_census() {
    let mut held = branch("o", None, "p", "open", None, None, None);
    held.id = surrogate_text();
    let found = Exploration {
        question: pt("q"),
        branches: vec![held],
    };
    assert_eq!(
        render_census(&found).expect_err("a refusal"),
        "a branch id holds an unpaired surrogate"
    );
}
