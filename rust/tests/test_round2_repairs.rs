//! Pins for the Round 2 repair amendments, in two repair
//! passes, translated from tests/test_round2_repairs.py.
//!
//! FIX-MY-6 names a newline or a carriage return in every
//! text field whose value reaches a rendering, at the exact
//! defect text the amendment states. FIX-MY-7 first rendered
//! a claimed branch on the map with a bracketed tag before
//! the prompt, and FIX-MY-10 retired the tag for a claim
//! line of the branch's own, so the form pins below state
//! the claim line form.
//!
//! The second pass pins FIX-MY-8, FIX-MY-9, and FIX-MY-10.
//! FIX-MY-8 refuses a line break in the fork helper, FIX-MY-9
//! extends the line break rule to the branch id and the
//! branch parent, and FIX-MY-10 stands the claim on its own
//! line under the branch line.

mod common;

use myriad::explorations::{render_claims, render_map};
use myriad::fork::fork_skeleton;

use common::{branch_obj, defects_of, exploration_obj, jstr, loads, pt};

const LINE_BREAK: &str = "holds a newline or a carriage return";

fn plain(branches: &[String]) -> String {
    exploration_obj(branches, &[], &[])
}

fn defect(field: &str) -> String {
    format!("{field} {LINE_BREAK}")
}

#[test]
fn a_claim_holding_a_forged_counts_line_is_named() {
    let forged = "north\nbranches 99, open 99, taken 0, abandoned 0";
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("claim", jstr(forged))], &[])])),
        vec![defect("branch 0 field claim")]
    );
}

#[test]
fn a_claim_holding_a_carriage_return_is_named() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(
            &[("claim", jstr("north\rfork"))],
            &[]
        )])),
        vec![defect("branch 0 field claim")]
    );
}

#[test]
fn a_prompt_holding_a_newline_is_named() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(
            &[("prompt", jstr("first\nsecond"))],
            &[]
        )])),
        vec![defect("branch 0 field prompt")]
    );
}

#[test]
fn a_prompt_holding_a_carriage_return_is_named() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(
            &[("prompt", jstr("first\rsecond"))],
            &[]
        )])),
        vec![defect("branch 0 field prompt")]
    );
}

#[test]
fn a_question_holding_a_newline_is_named() {
    assert_eq!(
        defects_of(&exploration_obj(
            &[],
            &[("question", jstr("one\nquestion"))],
            &[]
        )),
        vec![defect("field question")]
    );
}

#[test]
fn a_question_holding_a_carriage_return_is_named() {
    assert_eq!(
        defects_of(&exploration_obj(
            &[],
            &[("question", jstr("one\rquestion"))],
            &[]
        )),
        vec![defect("field question")]
    );
}

#[test]
fn a_finding_holding_a_newline_is_named() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(
            &[
                ("state", jstr("taken")),
                ("finding", jstr("found\nbranches 1, open 0")),
            ],
            &[]
        )])),
        vec![defect("branch 0 field finding")]
    );
}

#[test]
fn a_reason_holding_a_newline_is_named() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(
            &[
                ("state", jstr("abandoned")),
                ("reason", jstr("it died\nquietly")),
            ],
            &[]
        )])),
        vec![defect("branch 0 field reason")]
    );
}

#[test]
fn the_loader_keeps_naming_after_a_line_break_defect() {
    assert_eq!(
        defects_of(&plain(&[
            branch_obj(&[("prompt", jstr("first\nsecond"))], &[]),
            branch_obj(
                &[
                    ("id", jstr("b")),
                    ("parent", jstr("a")),
                    ("prompt", jstr("p2")),
                    ("claim", jstr("north\nfork")),
                ],
                &[],
            ),
        ])),
        vec![
            defect("branch 0 field prompt"),
            defect("branch 1 field claim"),
        ]
    );
}

#[test]
fn fields_without_line_breaks_still_load_clean() {
    let found = loads(&plain(&[
        branch_obj(
            &[
                ("state", jstr("taken")),
                ("finding", jstr("found it, all of it")),
                ("claim", jstr("north fork")),
            ],
            &[],
        ),
        branch_obj(
            &[
                ("id", jstr("b")),
                ("parent", jstr("a")),
                ("prompt", jstr("p2 [draft]")),
            ],
            &[],
        ),
    ]));
    assert_eq!(found.branches[0].claim, Some(pt("north fork")));
}

#[test]
fn a_claimed_branch_renders_its_claim_on_its_own_line() {
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
fn the_retired_suffix_text_no_longer_reads_as_a_claim() {
    let found = loads(&plain(&[
        branch_obj(&[("prompt", jstr("route")), ("claim", jstr("route"))], &[]),
        branch_obj(
            &[
                ("id", jstr("b")),
                ("prompt", jstr("route, claimed by route")),
            ],
            &[],
        ),
    ]));
    assert_eq!(
        render_map(&found).expect("a sound rendering"),
        common::strings(&[
            "question, q",
            "a (open) route",
            "  claimed by route",
            "b (open) route, claimed by route",
            "branches 2, open 2, taken 0, abandoned 0",
        ])
    );
}

#[test]
fn the_listing_counts_the_suffix_prompt_as_unclaimed() {
    let found = loads(&plain(&[
        branch_obj(&[("prompt", jstr("route")), ("claim", jstr("route"))], &[]),
        branch_obj(
            &[
                ("id", jstr("b")),
                ("prompt", jstr("route, claimed by route")),
            ],
            &[],
        ),
    ]));
    assert_eq!(
        render_claims(&found).expect("a sound rendering"),
        common::strings(&[
            "question, q",
            "claim route, branches 1, open 1, taken 0, abandoned 0",
            "unclaimed, branches 1, open 1, taken 0, abandoned 0",
        ])
    );
}

#[test]
fn the_fork_helper_refuses_a_question_holding_a_newline() {
    assert_eq!(
        fork_skeleton(&pt("one\nquestion"), &[pt("one")]).expect_err("a refusal"),
        "the question holds a newline or a carriage return"
    );
}

#[test]
fn the_fork_helper_refuses_a_question_holding_a_carriage_return() {
    assert_eq!(
        fork_skeleton(&pt("one\rquestion"), &[pt("one")]).expect_err("a refusal"),
        "the question holds a newline or a carriage return"
    );
}

#[test]
fn the_fork_helper_refuses_a_prompt_holding_a_newline() {
    assert_eq!(
        fork_skeleton(&pt("q"), &[pt("one"), pt("two\nlines")]).expect_err("a refusal"),
        "each prompt must hold no newline or carriage return"
    );
}

#[test]
fn the_fork_helper_refuses_a_prompt_holding_a_carriage_return() {
    assert_eq!(
        fork_skeleton(&pt("q"), &[pt("one"), pt("two\rlines")]).expect_err("a refusal"),
        "each prompt must hold no newline or carriage return"
    );
}

#[test]
fn the_fork_helper_refuses_a_prompt_holding_a_crlf_pair() {
    assert_eq!(
        fork_skeleton(&pt("q"), &[pt("one"), pt("two\r\nlines")]).expect_err("a refusal"),
        "each prompt must hold no newline or carriage return"
    );
}

#[test]
fn an_id_holding_a_forged_counts_line_is_named() {
    let forged = "a\nbranches 99, open 99, taken 0, abandoned 0";
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("id", jstr(forged))], &[])])),
        vec![defect("branch 0 field id")]
    );
}

#[test]
fn a_parent_holding_a_carriage_return_is_named() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(
            &[("parent", jstr("root\rforged"))],
            &[]
        )])),
        vec![defect("branch 0 field parent")]
    );
}

#[test]
fn a_parent_holding_a_newline_draws_no_forged_defect_text() {
    let forged = "ghost\nbranches 99, open 99, taken 0, abandoned 0";
    let defects = defects_of(&plain(&[branch_obj(&[("parent", jstr(forged))], &[])]));
    assert_eq!(defects, vec![defect("branch 0 field parent")]);
}

#[test]
fn the_loader_keeps_naming_after_an_id_line_break_defect() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(
            &[("id", jstr("a\nb")), ("prompt", jstr("first\nsecond"))],
            &[],
        )])),
        vec![defect("branch 0 field id"), defect("branch 0 field prompt")]
    );
}

#[test]
fn a_bracketed_prompt_does_not_read_as_a_claim() {
    let found = loads(&plain(&[
        branch_obj(&[("prompt", jstr("route")), ("claim", jstr("north"))], &[]),
        branch_obj(&[("id", jstr("b")), ("prompt", jstr("[north] route"))], &[]),
    ]));
    assert_eq!(
        render_map(&found).expect("a sound rendering"),
        common::strings(&[
            "question, q",
            "a (open) route",
            "  claimed by north",
            "b (open) [north] route",
            "branches 2, open 2, taken 0, abandoned 0",
        ])
    );
}

#[test]
fn a_claim_label_holding_brackets_renders_on_its_claim_line() {
    let found = loads(&plain(&[
        branch_obj(&[("prompt", jstr("route")), ("claim", jstr("a] [b"))], &[]),
        branch_obj(
            &[
                ("id", jstr("b")),
                ("parent", jstr("a")),
                ("prompt", jstr("p2")),
                ("state", jstr("abandoned")),
                ("reason", jstr("it died")),
                ("claim", jstr("[north]")),
            ],
            &[],
        ),
    ]));
    assert_eq!(
        render_map(&found).expect("a sound rendering"),
        common::strings(&[
            "question, q",
            "a (open) route",
            "  claimed by a] [b",
            "  b (abandoned) p2",
            "    claimed by [north]",
            "    reason, it died",
            "branches 2, open 1, taken 0, abandoned 1",
        ])
    );
}
