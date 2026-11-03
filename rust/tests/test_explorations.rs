//! Pins for the exploration loader, translated from
//! tests/test_explorations.py.
//!
//! Every defect phrase in MY-1 full form is pinned here
//! against the exact text the loader returns, and the
//! structural checks of the tree are pinned in both
//! directions, firing on the broken tree and staying silent
//! on the sound one.

mod common;

use myriad::explorations::{load_exploration_file, load_exploration_text};

use common::{
    branch, branch_json, branch_obj, defects_of, exploration_json, exploration_obj, jnull, jstr,
    loads, pt, temp_dir,
};

fn plain(branches: &[String]) -> String {
    exploration_obj(branches, &[], &[])
}

#[test]
fn a_clean_exploration_loads_and_keeps_only_the_named_fields() {
    let data = exploration_obj(
        &[
            branch_obj(&[("extra", jstr("ignored"))], &[]),
            branch_obj(
                &[
                    ("id", jstr("b")),
                    ("parent", jstr("a")),
                    ("state", jstr("taken")),
                    ("finding", jstr("found it")),
                ],
                &[],
            ),
            branch_obj(
                &[
                    ("id", jstr("c")),
                    ("parent", jstr("a")),
                    ("state", jstr("abandoned")),
                    ("reason", jstr("it died")),
                ],
                &[],
            ),
        ],
        &[("extra_top", jstr("ignored"))],
        &[],
    );
    let found = loads(&data);
    let expected = myriad::explorations::Exploration {
        question: pt("q"),
        branches: vec![
            branch("a", None, "p", "open", None, None, None),
            branch("b", Some("a"), "p", "taken", None, Some("found it"), None),
            branch(
                "c",
                Some("a"),
                "p",
                "abandoned",
                None,
                None,
                Some("it died"),
            ),
        ],
    };
    assert_eq!(found, expected);
}

#[test]
fn an_empty_branches_list_loads_clean() {
    let found = loads("{\"question\": \"q\", \"branches\": []}");
    assert_eq!(found.question, pt("q"));
    assert!(found.branches.is_empty());
}

#[test]
fn text_that_is_not_json_draws_the_not_valid_json_defect() {
    let (found, defects) = load_exploration_text("{not json");
    assert!(found.is_none());
    assert_eq!(defects, vec!["input is not valid JSON".to_string()]);
}

#[test]
fn input_nested_too_deeply_draws_the_not_valid_json_defect() {
    let text = format!("{}{}", "[".repeat(10000), "]".repeat(10000));
    let (found, defects) = load_exploration_text(&text);
    assert!(found.is_none());
    assert_eq!(defects, vec!["input is not valid JSON".to_string()]);
}

#[test]
fn json_that_is_not_an_object_draws_its_defect() {
    for text in ["[1, 2]", "\"a string\"", "42", "null", "true"] {
        let (found, defects) = load_exploration_text(text);
        assert!(found.is_none(), "{text}");
        assert_eq!(
            defects,
            vec!["input is not a JSON object".to_string()],
            "{text}"
        );
    }
}

#[test]
fn the_loader_never_raises_on_garbage() {
    let junk = [
        "",
        " ",
        "{}",
        "{\"question\": 1}",
        "{\"question\": \"q\", \"branches\": [null, true, [], {}, \"x\"]}",
        "{\"question\": \"q\", \"branches\": [{\"id\": [], \"parent\": {},\
         \"prompt\": null, \"state\": 9, \"finding\": [], \"reason\": {}}]}",
    ];
    for text in junk {
        let (found, defects) = load_exploration_text(text);
        assert!(found.is_none(), "{text}");
        assert!(!defects.is_empty(), "{text}");
    }
}

#[test]
fn a_file_that_cannot_be_read_draws_the_single_defect() {
    let dir = temp_dir("explorations-file");
    let (found, defects) = load_exploration_file(&dir.join("missing.json"));
    assert!(found.is_none());
    assert_eq!(defects, vec!["file cannot be read".to_string()]);
    let (found, defects) = load_exploration_file(&dir);
    assert!(found.is_none());
    assert_eq!(defects, vec!["file cannot be read".to_string()]);
}

#[test]
fn question_defects_are_named() {
    assert_eq!(
        defects_of("{\"branches\": []}"),
        vec!["missing field question".to_string()]
    );
    assert_eq!(
        defects_of(&exploration_obj(&[], &[("question", "7".to_string())], &[])),
        vec!["field question has the wrong type".to_string()]
    );
    assert_eq!(
        defects_of(&exploration_obj(&[], &[("question", jstr(""))], &[])),
        vec!["field question is empty".to_string()]
    );
}

#[test]
fn a_question_holding_an_unpaired_surrogate_is_named() {
    let text = "{\"question\": \"\\ud800x\", \"branches\": []}";
    let (found, defects) = load_exploration_text(text);
    assert!(found.is_none());
    assert_eq!(
        defects,
        vec!["field question holds an unpaired surrogate".to_string()]
    );
}

#[test]
fn branches_defects_are_named() {
    assert_eq!(
        defects_of("{\"question\": \"q\"}"),
        vec!["missing field branches".to_string()]
    );
    assert_eq!(
        defects_of(&exploration_obj(
            &[],
            &[("branches", "{\"a\": 1}".to_string())],
            &[]
        )),
        vec!["field branches has the wrong type".to_string()]
    );
    assert_eq!(
        defects_of(&exploration_obj(&[], &[("branches", jstr("abc"))], &[])),
        vec!["field branches has the wrong type".to_string()]
    );
}

#[test]
fn a_branch_that_is_not_an_object_is_named() {
    assert_eq!(
        defects_of(&plain(&["7".to_string()])),
        vec!["branch 0 is not an object".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[], &[]), jstr("x")])),
        vec!["branch 1 is not an object".to_string()]
    );
}

#[test]
fn id_defects_are_named() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[], &["id"])])),
        vec!["branch 0 is missing field id".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("id", "7".to_string())], &[])])),
        vec!["branch 0 field id has the wrong type".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("id", jstr(""))], &[])])),
        vec!["branch 0 field id is empty".to_string()]
    );
}

#[test]
fn parent_defects_are_named() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[], &["parent"])])),
        vec!["branch 0 is missing field parent".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("parent", "7".to_string())], &[])])),
        vec!["branch 0 field parent has the wrong type".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("parent", jstr(""))], &[])])),
        vec!["branch 0 field parent is empty".to_string()]
    );
}

#[test]
fn prompt_defects_are_named() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[], &["prompt"])])),
        vec!["branch 0 is missing field prompt".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("prompt", jnull())], &[])])),
        vec!["branch 0 field prompt has the wrong type".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("prompt", jstr(""))], &[])])),
        vec!["branch 0 field prompt is empty".to_string()]
    );
}

#[test]
fn a_prompt_holding_an_unpaired_surrogate_is_named() {
    let text = "{\"question\": \"q\", \"branches\": [{\"id\": \"a\", \"parent\": null, \
         \"prompt\": \"\\ud800x\", \"state\": \"open\"}]}";
    let (found, defects) = load_exploration_text(text);
    assert!(found.is_none());
    assert_eq!(
        defects,
        vec!["branch 0 field prompt holds an unpaired surrogate".to_string()]
    );
}

#[test]
fn state_defects_are_named() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[], &["state"])])),
        vec!["branch 0 is missing field state".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("state", "1".to_string())], &[])])),
        vec!["branch 0 field state has the wrong type".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("state", jstr("gone"))], &[])])),
        vec!["branch 0 field state is not a known state".to_string()]
    );
}

#[test]
fn a_taken_branch_without_a_finding_is_named() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("state", jstr("taken"))], &[])])),
        vec!["branch 0 is missing field finding".to_string()]
    );
}

#[test]
fn an_abandoned_branch_without_a_reason_is_named() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("state", jstr("abandoned"))], &[])])),
        vec!["branch 0 is missing field reason".to_string()]
    );
}

#[test]
fn finding_and_reason_values_are_checked() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(
            &[("state", jstr("taken")), ("finding", "3".to_string())],
            &[]
        )])),
        vec!["branch 0 field finding has the wrong type".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(
            &[("state", jstr("taken")), ("finding", jstr(""))],
            &[]
        )])),
        vec!["branch 0 field finding is empty".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(
            &[("state", jstr("abandoned")), ("reason", "[]".to_string())],
            &[]
        )])),
        vec!["branch 0 field reason has the wrong type".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(
            &[("state", jstr("abandoned")), ("reason", jstr(""))],
            &[]
        )])),
        vec!["branch 0 field reason is empty".to_string()]
    );
}

#[test]
fn a_finding_or_reason_in_a_forbidding_state_is_named() {
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("finding", jstr("f"))], &[])])),
        vec!["branch 0 field finding is not allowed in state open".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(&[("reason", jstr("r"))], &[])])),
        vec!["branch 0 field reason is not allowed in state open".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(
            &[
                ("state", jstr("taken")),
                ("finding", jstr("f")),
                ("reason", jstr("r")),
            ],
            &[]
        )])),
        vec!["branch 0 field reason is not allowed in state taken".to_string()]
    );
    assert_eq!(
        defects_of(&plain(&[branch_obj(
            &[
                ("state", jstr("abandoned")),
                ("reason", jstr("r")),
                ("finding", jstr("f")),
            ],
            &[]
        )])),
        vec!["branch 0 field finding is not allowed in state abandoned".to_string()]
    );
}

#[test]
fn one_input_draws_several_defects_in_one_pass() {
    let data = exploration_obj(
        &[
            branch_obj(&[("id", jstr(""))], &[]),
            branch_obj(
                &[
                    ("id", jstr("b")),
                    ("prompt", jstr("")),
                    ("state", jstr("gone")),
                ],
                &[],
            ),
        ],
        &[("question", jstr(""))],
        &[],
    );
    assert_eq!(
        defects_of(&data),
        common::strings(&[
            "field question is empty",
            "branch 0 field id is empty",
            "branch 1 field prompt is empty",
            "branch 1 field state is not a known state",
        ])
    );
}

#[test]
fn a_duplicate_id_is_named() {
    let data = plain(&[
        branch_obj(&[], &[]),
        branch_obj(&[("prompt", jstr("other"))], &[]),
    ]);
    assert_eq!(defects_of(&data), vec!["duplicate branch id a".to_string()]);
}

#[test]
fn a_parent_that_does_not_exist_is_named() {
    let data = plain(&[branch_obj(&[("parent", jstr("ghost"))], &[])]);
    assert_eq!(
        defects_of(&data),
        vec!["branch a parent ghost does not exist".to_string()]
    );
}

#[test]
fn a_parent_with_a_defective_field_still_exists() {
    let data = plain(&[
        branch_obj(&[("prompt", jstr(""))], &[]),
        branch_obj(
            &[
                ("id", jstr("b")),
                ("parent", jstr("a")),
                ("prompt", jstr("other")),
            ],
            &[],
        ),
    ]);
    assert_eq!(
        defects_of(&data),
        vec!["branch 0 field prompt is empty".to_string()]
    );
}

#[test]
fn a_self_parent_is_a_cycle() {
    let data = plain(&[branch_obj(&[("parent", jstr("a"))], &[])]);
    assert_eq!(
        defects_of(&data),
        vec!["branch a has a cycle in its parent chain".to_string()]
    );
}

#[test]
fn a_two_branch_cycle_names_both_branches() {
    let data = plain(&[
        branch_obj(&[("parent", jstr("b"))], &[]),
        branch_obj(
            &[
                ("id", jstr("b")),
                ("parent", jstr("a")),
                ("prompt", jstr("other")),
            ],
            &[],
        ),
    ]);
    assert_eq!(
        defects_of(&data),
        common::strings(&[
            "branch a has a cycle in its parent chain",
            "branch b has a cycle in its parent chain",
        ])
    );
}

#[test]
fn a_chain_leading_into_a_cycle_is_named() {
    let data = plain(&[
        branch_obj(
            &[
                ("id", jstr("a")),
                ("parent", jstr("b")),
                ("prompt", jstr("p1")),
            ],
            &[],
        ),
        branch_obj(
            &[
                ("id", jstr("b")),
                ("parent", jstr("c")),
                ("prompt", jstr("p2")),
            ],
            &[],
        ),
        branch_obj(
            &[
                ("id", jstr("c")),
                ("parent", jstr("b")),
                ("prompt", jstr("p3")),
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
    ]);
    assert_eq!(
        defects_of(&data),
        common::strings(&[
            "branch a has a cycle in its parent chain",
            "branch b has a cycle in its parent chain",
            "branch c has a cycle in its parent chain",
            "branch d has a cycle in its parent chain",
        ])
    );
}

#[test]
fn a_deep_sound_chain_draws_no_cycle_defect() {
    let data = plain(&[
        branch_obj(&[("id", jstr("a"))], &[]),
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
                ("parent", jstr("b")),
                ("prompt", jstr("p3")),
            ],
            &[],
        ),
        branch_obj(
            &[
                ("id", jstr("d")),
                ("parent", jstr("c")),
                ("prompt", jstr("p4")),
            ],
            &[],
        ),
    ]);
    let found = loads(&data);
    assert_eq!(found.branches.len(), 4);
}

#[test]
fn a_cycle_of_five_thousand_branches_names_every_branch() {
    let branches: Vec<String> = (0..5000)
        .map(|i| {
            branch_json(
                &format!("b{i}"),
                Some(&format!("b{}", (i + 1) % 5000)),
                &format!("p{i}"),
                "open",
            )
        })
        .collect();
    let text = exploration_json("q", &branches);
    let (found, defects) = load_exploration_text(&text);
    assert!(found.is_none());
    assert_eq!(defects.len(), 5000);
    assert_eq!(defects[0], "branch b0 has a cycle in its parent chain");
    assert_eq!(
        defects[4999],
        "branch b4999 has a cycle in its parent chain"
    );
}

#[test]
fn a_duplicate_prompt_among_open_branches_is_named() {
    let data = plain(&[
        branch_obj(&[("prompt", jstr("same"))], &[]),
        branch_obj(&[("id", jstr("b")), ("prompt", jstr("same"))], &[]),
    ]);
    assert_eq!(
        defects_of(&data),
        vec!["branch b repeats the prompt of open branch a".to_string()]
    );
}

#[test]
fn a_duplicate_prompt_stays_silent_across_states() {
    let cases: Vec<String> = vec![
        plain(&[
            branch_obj(&[("prompt", jstr("same"))], &[]),
            branch_obj(
                &[
                    ("id", jstr("b")),
                    ("prompt", jstr("same")),
                    ("state", jstr("abandoned")),
                    ("reason", jstr("r")),
                ],
                &[],
            ),
        ]),
        plain(&[
            branch_obj(&[("prompt", jstr("same"))], &[]),
            branch_obj(
                &[
                    ("id", jstr("b")),
                    ("prompt", jstr("same")),
                    ("state", jstr("taken")),
                    ("finding", jstr("f")),
                ],
                &[],
            ),
        ]),
        plain(&[
            branch_obj(
                &[
                    ("prompt", jstr("same")),
                    ("state", jstr("taken")),
                    ("finding", jstr("f")),
                ],
                &[],
            ),
            branch_obj(&[("id", jstr("b")), ("prompt", jstr("same"))], &[]),
        ]),
        plain(&[
            branch_obj(
                &[
                    ("prompt", jstr("same")),
                    ("state", jstr("taken")),
                    ("finding", jstr("f")),
                ],
                &[],
            ),
            branch_obj(
                &[
                    ("id", jstr("b")),
                    ("prompt", jstr("same")),
                    ("state", jstr("taken")),
                    ("finding", jstr("g")),
                ],
                &[],
            ),
        ]),
        plain(&[
            branch_obj(
                &[
                    ("prompt", jstr("same")),
                    ("state", jstr("abandoned")),
                    ("reason", jstr("r")),
                ],
                &[],
            ),
            branch_obj(
                &[
                    ("id", jstr("b")),
                    ("prompt", jstr("same")),
                    ("state", jstr("abandoned")),
                    ("reason", jstr("s")),
                ],
                &[],
            ),
        ]),
    ];
    for data in &cases {
        let found = loads(data);
        assert_eq!(found.branches.len(), 2);
    }
}
