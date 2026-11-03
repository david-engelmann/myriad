//! Pins for the fork helper, translated from
//! tests/test_fork.py.
//!
//! The skeleton is pinned against the MY-1 loader and the
//! MY-2 map, so the shape a live forker would emit is held
//! by the same checks every other exploration passes.
//!
//! The Python corpus also pins that a question of the wrong
//! type and a prompt of the wrong type raise. The Rust
//! helper takes text values, so those two inputs cannot be
//! built at all. The refusal stands by construction in the
//! typed surface, and the remaining unsound inputs pin the
//! exact refusal texts the helper returns.

mod common;

use myriad::explorations::{count_by_state, load_exploration_text, render_map, Counts};
use myriad::fork::fork_skeleton;
use myriad::pytext::PText;

use common::{branch, pt};

/// Serialize a skeleton back to JSON text the way
/// json.dumps writes it, so the loader reads it whole.
fn skeleton_text(skeleton: &myriad::explorations::Exploration) -> String {
    let branches: Vec<String> = skeleton
        .branches
        .iter()
        .map(|held| {
            let parent = match &held.parent {
                None => "null".to_string(),
                Some(parent) => common::jstr(&parent.to_display_string()),
            };
            format!(
                "{{\"id\": {}, \"parent\": {}, \"prompt\": {}, \"state\": \"open\"}}",
                common::jstr(&held.id.to_display_string()),
                parent,
                common::jstr(&held.prompt.to_display_string()),
            )
        })
        .collect();
    format!(
        "{{\"question\": {}, \"branches\": [{}]}}",
        common::jstr(&skeleton.question.to_display_string()),
        branches.join(", ")
    )
}

#[test]
fn the_skeleton_holds_one_open_root_per_prompt() {
    let skeleton =
        fork_skeleton(&pt("q"), &[pt("one"), pt("two"), pt("three")]).expect("a sound skeleton");
    let expected = myriad::explorations::Exploration {
        question: pt("q"),
        branches: vec![
            branch("b01", None, "one", "open", None, None, None),
            branch("b02", None, "two", "open", None, None, None),
            branch("b03", None, "three", "open", None, None, None),
        ],
    };
    assert_eq!(skeleton, expected);
}

#[test]
fn the_skeleton_loads_clean_and_renders() {
    let skeleton = fork_skeleton(&pt("q"), &[pt("one"), pt("two")]).expect("a sound skeleton");
    let (found, defects) = load_exploration_text(&skeleton_text(&skeleton));
    assert_eq!(defects, Vec::<String>::new());
    let found = found.expect("a clean load");
    assert_eq!(found, skeleton);
    assert_eq!(
        render_map(&found).expect("a sound rendering"),
        common::strings(&[
            "question, q",
            "b01 (open) one",
            "b02 (open) two",
            "branches 2, open 2, taken 0, abandoned 0",
        ])
    );
}

#[test]
fn the_skeleton_forks_tens_of_branches() {
    let prompts: Vec<PText> = (1..=30)
        .map(|n| pt(&format!("branch number {n}")))
        .collect();
    let skeleton = fork_skeleton(&pt("q"), &prompts).expect("a sound skeleton");
    assert_eq!(
        count_by_state(&skeleton),
        Counts {
            total: 30,
            open: 30,
            taken: 0,
            abandoned: 0,
        }
    );
    assert_eq!(
        skeleton.branches.last().expect("a last branch").id,
        pt("b30")
    );
}

#[test]
fn an_empty_prompt_list_builds_an_empty_skeleton() {
    let skeleton = fork_skeleton(&pt("q"), &[]).expect("a sound skeleton");
    let (found, defects) = load_exploration_text(&skeleton_text(&skeleton));
    assert_eq!(defects, Vec::<String>::new());
    let found = found.expect("a clean load");
    assert_eq!(found.question, pt("q"));
    assert!(found.branches.is_empty());
}

#[test]
fn a_question_holding_an_unpaired_surrogate_raises() {
    let question = PText(vec![0xD800, 'x' as u32]);
    let error = fork_skeleton(&question, &[pt("one")]).expect_err("a refusal");
    assert_eq!(error, "the question holds an unpaired surrogate");
}

#[test]
fn a_prompt_holding_an_unpaired_surrogate_raises() {
    let bad = PText(vec![
        'b' as u32, 'a' as u32, 'd' as u32, ' ' as u32, 0xD800, 'x' as u32,
    ]);
    let error = fork_skeleton(&pt("q"), &[pt("one"), bad]).expect_err("a refusal");
    assert_eq!(error, "each prompt must hold no unpaired surrogate");
}

#[test]
fn unsound_inputs_raise() {
    assert_eq!(
        fork_skeleton(&pt(""), &[pt("one")]).expect_err("a refusal"),
        "the question must be a non empty string"
    );
    assert_eq!(
        fork_skeleton(&pt("q"), &[pt("")]).expect_err("a refusal"),
        "each prompt must be a non empty string"
    );
    assert_eq!(
        fork_skeleton(&pt("q"), &[pt("same"), pt("same")]).expect_err("a refusal"),
        "two prompts are the same, same"
    );
}
