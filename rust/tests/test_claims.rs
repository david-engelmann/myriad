//! Pins for the dibs claim in MY-5 full form, translated
//! from tests/test_claims.py.
//!
//! Every defect phrase the claim field can draw is pinned
//! at its exact text, and the claims listing is pinned line
//! for line on a tree whose labels appear out of
//! alphabetical order, so the ordering the contract states
//! is the only order that passes.

mod common;

use myriad::explorations::{render_claims, render_map, Exploration};
use myriad::fork::fork_skeleton;
use myriad::pytext::PText;

use common::{branch, branch_obj, defects_of, exploration_obj, jstr, loads, pt};

fn plain(branches: &[String]) -> String {
    exploration_obj(branches, &[], &[])
}

#[test]
fn a_claimed_branch_loads_and_carries_its_claim() {
    let found = loads(&plain(&[
        branch_obj(&[("claim", jstr("north fork"))], &[]),
        branch_obj(
            &[
                ("id", jstr("b")),
                ("parent", jstr("a")),
                ("state", jstr("taken")),
                ("finding", jstr("found it")),
                ("claim", jstr("north fork")),
            ],
            &[],
        ),
        branch_obj(
            &[
                ("id", jstr("c")),
                ("parent", jstr("a")),
                ("state", jstr("abandoned")),
                ("reason", jstr("it died")),
                ("claim", jstr("south fork")),
            ],
            &[],
        ),
        branch_obj(
            &[
                ("id", jstr("d")),
                ("parent", jstr("a")),
                ("prompt", jstr("p4")),
            ],
            &[],
        ),
    ]));
    assert_eq!(found.branches[0].claim, Some(pt("north fork")));
    assert_eq!(found.branches[1].claim, Some(pt("north fork")));
    assert_eq!(found.branches[2].claim, Some(pt("south fork")));
    assert_eq!(found.branches[3].claim, None);
}

#[test]
fn a_claim_of_the_wrong_type_is_named() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("claim", "7".to_string())], &[])])),
        vec!["branch 0 field claim has the wrong type".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("claim", common::jnull())], &[])])),
        vec!["branch 0 field claim has the wrong type".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[
            branch_obj(&[], &[]),
            branch_obj(
                &[
                    ("id", jstr("b")),
                    ("parent", jstr("a")),
                    ("prompt", jstr("p2")),
                    ("claim", "[\"north fork\"]".to_string()),
                ],
                &[],
            ),
        ])),
        vec!["branch 1 field claim has the wrong type".to_string()]
    );
}

#[test]
fn an_empty_claim_is_named() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("claim", jstr(""))], &[])])),
        vec!["branch 0 field claim is empty".to_string()]
    );
}

#[test]
fn a_claim_holding_an_unpaired_surrogate_is_named() {
    let text = "{\"question\": \"q\", \"branches\": [{\"id\": \"a\", \"parent\": null, \
        \"prompt\": \"p\", \"state\": \"open\", \"claim\": \"bad \\ud800 label\"}]}";
    assert_eq!(
        defects_of(text),
        vec!["branch 0 field claim holds an unpaired surrogate".to_string()]
    );
}

#[test]
fn a_claim_defect_does_not_stop_the_naming() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(
            &[
                ("state", jstr("open")),
                ("finding", jstr("found it")),
                ("claim", jstr("")),
            ],
            &[],
        )])),
        common::strings(&[
            "branch 0 field claim is empty",
            "branch 0 field finding is not allowed in state open",
        ])
    );
}

#[test]
fn a_claim_is_allowed_in_every_state() {
    let found = loads(&plain(&[
        branch_obj(&[("claim", jstr("north fork"))], &[]),
        branch_obj(
            &[
                ("id", jstr("b")),
                ("parent", jstr("a")),
                ("state", jstr("taken")),
                ("finding", jstr("found it")),
                ("claim", jstr("north fork")),
            ],
            &[],
        ),
        branch_obj(
            &[
                ("id", jstr("c")),
                ("parent", jstr("a")),
                ("state", jstr("abandoned")),
                ("reason", jstr("it died")),
                ("claim", jstr("north fork")),
            ],
            &[],
        ),
    ]));
    let claims: Vec<_> = found
        .branches
        .iter()
        .map(|held| held.claim.clone())
        .collect();
    assert_eq!(
        claims,
        vec![
            Some(pt("north fork")),
            Some(pt("north fork")),
            Some(pt("north fork")),
        ]
    );
}

#[test]
fn the_loader_output_differs_only_by_the_claim() {
    let bare = loads(&plain(&[branch_obj(&[], &[])]));
    let claimed = loads(&plain(&[branch_obj(&[("claim", jstr("north fork"))], &[])]));
    let mut widened = bare.branches[0].clone();
    widened.claim = Some(pt("north fork"));
    assert_eq!(claimed.branches[0], widened);
    assert_eq!(claimed.question, bare.question);
}

#[test]
fn the_map_names_the_claim_on_a_line_of_its_own() {
    let found = loads(&plain(&[
        branch_obj(
            &[
                ("state", jstr("taken")),
                ("finding", jstr("found it")),
                ("claim", jstr("north fork")),
            ],
            &[],
        ),
        branch_obj(
            &[
                ("id", jstr("b")),
                ("parent", jstr("a")),
                ("prompt", jstr("p2")),
            ],
            &[],
        ),
    ]));
    assert_eq!(
        render_map(&found).expect("a sound rendering"),
        common::strings(&[
            "question, q",
            "a (taken) p",
            "  claimed by north fork",
            "  finding, found it",
            "  b (open) p2",
            "branches 2, open 1, taken 1, abandoned 0",
        ])
    );
}

#[test]
fn the_map_of_a_claim_free_exploration_keeps_its_round_1_lines() {
    let found = loads(&plain(&[
        branch_obj(
            &[("state", jstr("taken")), ("finding", jstr("found it"))],
            &[],
        ),
        branch_obj(
            &[
                ("id", jstr("b")),
                ("parent", jstr("a")),
                ("prompt", jstr("p2")),
            ],
            &[],
        ),
        branch_obj(
            &[
                ("id", jstr("c")),
                ("prompt", jstr("p3")),
                ("state", jstr("abandoned")),
                ("reason", jstr("it died")),
            ],
            &[],
        ),
    ]));
    assert_eq!(
        render_map(&found).expect("a sound rendering"),
        common::strings(&[
            "question, q",
            "a (taken) p",
            "  finding, found it",
            "  b (open) p2",
            "c (abandoned) p3",
            "  reason, it died",
            "branches 3, open 1, taken 1, abandoned 1",
        ])
    );
}

fn listing_tree() -> String {
    plain(&[
        branch_obj(
            &[
                ("state", jstr("taken")),
                ("finding", jstr("found it")),
                ("claim", jstr("tide watch")),
            ],
            &[],
        ),
        branch_obj(
            &[
                ("id", jstr("b")),
                ("parent", jstr("a")),
                ("prompt", jstr("p2")),
                ("claim", jstr("boat crew")),
            ],
            &[],
        ),
        branch_obj(
            &[
                ("id", jstr("c")),
                ("prompt", jstr("p3")),
                ("state", jstr("abandoned")),
                ("reason", jstr("it died")),
                ("claim", jstr("tide watch")),
            ],
            &[],
        ),
        branch_obj(
            &[
                ("id", jstr("d")),
                ("prompt", jstr("p4")),
                ("state", jstr("taken")),
                ("finding", jstr("found more")),
                ("claim", jstr("harbour office")),
            ],
            &[],
        ),
        branch_obj(&[("id", jstr("e")), ("prompt", jstr("p5"))], &[]),
        branch_obj(
            &[
                ("id", jstr("f")),
                ("prompt", jstr("p6")),
                ("state", jstr("taken")),
                ("finding", jstr("found last")),
            ],
            &[],
        ),
    ])
}

#[test]
fn the_claims_listing_stands_in_alphabetical_label_order() {
    let found = loads(&listing_tree());
    assert_eq!(
        render_claims(&found).expect("a sound rendering"),
        common::strings(&[
            "question, q",
            "claim boat crew, branches 1, open 1, taken 0, abandoned 0",
            "claim harbour office, branches 1, open 0, taken 1, abandoned 0",
            "claim tide watch, branches 2, open 0, taken 1, abandoned 1",
            "unclaimed, branches 2, open 1, taken 1, abandoned 0",
        ])
    );
}

#[test]
fn the_unclaimed_line_stands_only_when_a_branch_is_unclaimed() {
    let found = loads(&plain(&[
        branch_obj(&[("claim", jstr("north fork"))], &[]),
        branch_obj(
            &[
                ("id", jstr("b")),
                ("parent", jstr("a")),
                ("prompt", jstr("p2")),
                ("claim", jstr("north fork")),
            ],
            &[],
        ),
    ]));
    assert_eq!(
        render_claims(&found).expect("a sound rendering"),
        common::strings(&[
            "question, q",
            "claim north fork, branches 2, open 2, taken 0, abandoned 0",
        ])
    );
}

#[test]
fn the_listing_of_an_empty_exploration_is_the_question_line() {
    let found = loads("{\"question\": \"q\", \"branches\": []}");
    assert_eq!(
        render_claims(&found).expect("a sound rendering"),
        common::strings(&["question, q"])
    );
}

#[test]
fn a_fork_skeleton_lists_as_unclaimed_branches() {
    let skeleton =
        fork_skeleton(&pt("q"), &[pt("p1"), pt("p2"), pt("p3")]).expect("a sound skeleton");
    assert!(skeleton.branches.iter().all(|held| held.claim.is_none()));
    assert_eq!(
        render_claims(&skeleton).expect("a sound rendering"),
        common::strings(&[
            "question, q",
            "unclaimed, branches 3, open 3, taken 0, abandoned 0",
        ])
    );
}

// The PORT-11 pins. The claims listing prints the question
// and the claim labels, and a hand-built value holding an
// unpaired surrogate in one of those fields draws the
// named refusal.

/// A text holding an unpaired surrogate after one letter,
/// the hand-built shape the readings refuse by field.
fn surrogate_text() -> PText {
    PText(vec![0x61, 0xD800])
}

#[test]
fn a_surrogate_question_draws_the_refusal_in_the_claims_listing() {
    let found = Exploration {
        question: surrogate_text(),
        branches: vec![],
    };
    assert_eq!(
        render_claims(&found).expect_err("a refusal"),
        "the question holds an unpaired surrogate"
    );
}

#[test]
fn a_surrogate_claim_draws_the_refusal_in_the_claims_listing() {
    let mut held = branch("a", None, "p", "open", None, None, None);
    held.claim = Some(surrogate_text());
    let found = Exploration {
        question: pt("q"),
        branches: vec![held],
    };
    assert_eq!(
        render_claims(&found).expect_err("a refusal"),
        "a claim holds an unpaired surrogate"
    );
}
