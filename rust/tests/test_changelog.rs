//! Tests for MY-11, the changelog reading of two atlases.
//!
//! The reading matches entries by label and branches by id
//! inside a matched entry, names what arrived, what
//! departed, and what changed, and closes with the counts.
//! Refusals of hand-built values follow PORT-11.

mod common;

use common::*;
use myriad::atlas::{render_changelog, Atlas, AtlasEntry};
use myriad::explorations::{Branch, Exploration};
use myriad::pytext::PText;

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

fn bad() -> PText {
    PText(vec![0x61, 0xD800])
}

fn rich_earlier() -> Atlas {
    atlas(
        "Earlier title",
        vec![
            entry(
                "alpha",
                "Question one",
                vec![
                    branch("a1", None, "Prompt a1", "open", None, None, None),
                    branch(
                        "a2",
                        None,
                        "Prompt a2",
                        "taken",
                        None,
                        Some("Finding old"),
                        None,
                    ),
                    branch("a3", Some("a1"), "Prompt a3", "open", None, None, None),
                    branch(
                        "a4",
                        None,
                        "Prompt a4",
                        "abandoned",
                        None,
                        None,
                        Some("Reason old"),
                    ),
                    branch("a5", Some("a2"), "Prompt a5", "open", None, None, None),
                    branch("a6", None, "Prompt a6", "open", Some("crew"), None, None),
                    branch("a7", Some("a6"), "Prompt a7", "open", None, None, None),
                ],
            ),
            entry(
                "beta",
                "Question two",
                vec![branch("b1", None, "Prompt b1", "open", None, None, None)],
            ),
            entry("gamma", "Question three", vec![]),
        ],
    )
}

fn rich_later() -> Atlas {
    atlas(
        "Later title",
        vec![
            entry(
                "alpha",
                "Question one revised",
                vec![
                    branch("a1", Some("a2"), "Prompt a1", "open", None, None, None),
                    branch(
                        "a2",
                        None,
                        "Prompt a2",
                        "taken",
                        None,
                        Some("Finding new"),
                        None,
                    ),
                    branch(
                        "a3",
                        Some("a1"),
                        "Prompt a3",
                        "abandoned",
                        None,
                        None,
                        Some("Reason fresh"),
                    ),
                    branch(
                        "a4",
                        None,
                        "Prompt a4",
                        "abandoned",
                        None,
                        None,
                        Some("Reason new"),
                    ),
                    branch(
                        "a5",
                        Some("a2"),
                        "Prompt a5 revised",
                        "open",
                        Some("crew"),
                        None,
                        None,
                    ),
                    branch("a6", None, "Prompt a6", "open", None, None, None),
                    branch("a8", Some("a1"), "Prompt a8", "open", None, None, None),
                ],
            ),
            entry(
                "beta",
                "Question two",
                vec![branch("b1", None, "Prompt b1", "open", None, None, None)],
            ),
            entry("delta", "Question four", vec![]),
        ],
    )
}

#[test]
fn rich_pair_renders_line_for_line() {
    let lines = render_changelog(&rich_earlier(), &rich_later()).expect("a sound pair");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, Earlier title",
            "later atlas, Later title",
            "entry alpha",
            "  earlier question, Question one",
            "  later question, Question one revised",
            "  branch a1, parent changed, root to a2",
            "  branch a2, finding changed, Finding old to Finding new",
            "  branch a3, state changed, open to abandoned",
            "  branch a4, reason changed, Reason old to Reason new",
            "  branch a5, prompt changed, Prompt a5 to Prompt a5 revised",
            "  branch a5, claim changed, unclaimed to crew",
            "  branch a6, claim changed, crew to unclaimed",
            "  branch a7, departed",
            "  branch a8, arrived",
            "entry beta, unchanged",
            "entry gamma, departed",
            "entry delta, arrived",
            "changelog counts, entries arrived 1, departed 1, matched 2, branches changed 8",
        ])
    );
}

#[test]
fn reading_holds_the_vocabulary_bar() {
    let lines = render_changelog(&rich_earlier(), &rich_later()).expect("a sound pair");
    assert!(holds_vocabulary_bar(&lines.join("\n")));
}

#[test]
fn entries_stand_in_earlier_order_then_arrivals_in_later_order() {
    let earlier = atlas(
        "One",
        vec![entry("second", "Q", vec![]), entry("first", "Q", vec![])],
    );
    let later = atlas(
        "Two",
        vec![
            entry("first", "Q", vec![]),
            entry("third", "Q", vec![]),
            entry("second", "Q", vec![]),
            entry("zeroth", "Q", vec![]),
        ],
    );
    let lines = render_changelog(&earlier, &later).expect("a sound pair");
    let entry_lines: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("entry "))
        .collect();
    assert_eq!(
        entry_lines,
        vec![
            "entry second, unchanged",
            "entry first, unchanged",
            "entry third, arrived",
            "entry zeroth, arrived",
        ]
    );
}

#[test]
fn branch_lines_stand_in_standing_order_then_arrivals_in_later_order() {
    let earlier = atlas(
        "One",
        vec![entry(
            "only",
            "Q",
            vec![
                branch("gone-two", None, "P", "open", None, None, None),
                branch("gone-one", None, "P", "open", None, None, None),
            ],
        )],
    );
    let later = atlas(
        "Two",
        vec![entry(
            "only",
            "Q",
            vec![
                branch("new-two", None, "P", "open", None, None, None),
                branch("new-one", None, "P", "open", None, None, None),
            ],
        )],
    );
    let lines = render_changelog(&earlier, &later).expect("a sound pair");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry only",
            "  branch gone-two, departed",
            "  branch gone-one, departed",
            "  branch new-two, arrived",
            "  branch new-one, arrived",
            "changelog counts, entries arrived 0, departed 0, matched 1, branches changed 4",
        ])
    );
}

#[test]
fn relabelled_entry_is_a_departure_and_an_arrival_never_a_match() {
    let branches = || vec![branch("x", None, "Prompt x", "open", None, None, None)];
    let earlier = atlas("One", vec![entry("old name", "Same question", branches())]);
    let later = atlas("Two", vec![entry("new name", "Same question", branches())]);
    let lines = render_changelog(&earlier, &later).expect("a sound pair");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry old name, departed",
            "entry new name, arrived",
            "changelog counts, entries arrived 1, departed 1, matched 0, branches changed 0",
        ])
    );
}

#[test]
fn branch_standing_in_another_entry_is_departed_and_arrived() {
    let earlier = atlas(
        "One",
        vec![
            entry(
                "home",
                "Q",
                vec![branch("rover", None, "Prompt r", "open", None, None, None)],
            ),
            entry("away", "Q", vec![]),
        ],
    );
    let later = atlas(
        "Two",
        vec![
            entry("home", "Q", vec![]),
            entry(
                "away",
                "Q",
                vec![branch("rover", None, "Prompt r", "open", None, None, None)],
            ),
        ],
    );
    let lines = render_changelog(&earlier, &later).expect("a sound pair");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry home",
            "  branch rover, departed",
            "entry away",
            "  branch rover, arrived",
            "changelog counts, entries arrived 0, departed 0, matched 2, branches changed 2",
        ])
    );
}

#[test]
fn parent_change_names_a_branch_parent_and_the_root_word() {
    let earlier = atlas(
        "One",
        vec![entry(
            "only",
            "Q",
            vec![
                branch("anchor", None, "P", "open", None, None, None),
                branch("lifted", Some("anchor"), "P", "open", None, None, None),
                branch("moved", Some("anchor"), "P", "open", None, None, None),
            ],
        )],
    );
    let later = atlas(
        "Two",
        vec![entry(
            "only",
            "Q",
            vec![
                branch("anchor", None, "P", "open", None, None, None),
                branch("lifted", None, "P", "open", None, None, None),
                branch("moved", Some("lifted"), "P", "open", None, None, None),
            ],
        )],
    );
    let lines = render_changelog(&earlier, &later).expect("a sound pair");
    assert!(lines.contains(&"  branch lifted, parent changed, anchor to root".to_string()));
    assert!(lines.contains(&"  branch moved, parent changed, anchor to lifted".to_string()));
}

#[test]
fn question_change_prints_both_questions_and_unchanged_prints_neither() {
    let earlier = atlas(
        "One",
        vec![
            entry("turned", "Old wording", vec![]),
            entry("steady", "Same wording", vec![]),
        ],
    );
    let later = atlas(
        "Two",
        vec![
            entry("turned", "New wording", vec![]),
            entry("steady", "Same wording", vec![]),
        ],
    );
    let lines = render_changelog(&earlier, &later).expect("a sound pair");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry turned",
            "  earlier question, Old wording",
            "  later question, New wording",
            "entry steady, unchanged",
            "changelog counts, entries arrived 0, departed 0, matched 2, branches changed 0",
        ])
    );
}

#[test]
fn state_change_draws_no_finding_or_reason_line_of_its_own() {
    let earlier = atlas(
        "One",
        vec![entry(
            "only",
            "Q",
            vec![
                branch("rises", None, "P", "open", None, None, None),
                branch("falls", None, "P", "taken", None, Some("Found text"), None),
            ],
        )],
    );
    let later = atlas(
        "Two",
        vec![entry(
            "only",
            "Q",
            vec![
                branch("rises", None, "P", "taken", None, Some("Fresh text"), None),
                branch(
                    "falls",
                    None,
                    "P",
                    "abandoned",
                    None,
                    None,
                    Some("Closed text"),
                ),
            ],
        )],
    );
    let lines = render_changelog(&earlier, &later).expect("a sound pair");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry only",
            "  branch rises, state changed, open to taken",
            "  branch falls, state changed, taken to abandoned",
            "changelog counts, entries arrived 0, departed 0, matched 1, branches changed 2",
        ])
    );
}

#[test]
fn several_changes_on_one_branch_count_the_branch_once() {
    let earlier = atlas(
        "One",
        vec![entry(
            "only",
            "Q",
            vec![branch("busy", None, "Old prompt", "open", None, None, None)],
        )],
    );
    let later = atlas(
        "Two",
        vec![entry(
            "only",
            "Q",
            vec![branch(
                "busy",
                None,
                "New prompt",
                "taken",
                Some("crew"),
                Some("Found text"),
                None,
            )],
        )],
    );
    let lines = render_changelog(&earlier, &later).expect("a sound pair");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry only",
            "  branch busy, state changed, open to taken",
            "  branch busy, prompt changed, Old prompt to New prompt",
            "  branch busy, claim changed, unclaimed to crew",
            "changelog counts, entries arrived 0, departed 0, matched 1, branches changed 1",
        ])
    );
}

#[test]
fn field_order_stands_pinned_for_every_field_that_can_stand_together() {
    let earlier = atlas(
        "One",
        vec![entry(
            "only",
            "Q",
            vec![
                branch(
                    "shifting",
                    Some("first parent"),
                    "Earlier prompt",
                    "open",
                    None,
                    None,
                    None,
                ),
                branch(
                    "standing",
                    Some("first parent"),
                    "Earlier standing prompt",
                    "taken",
                    Some("earlier claim"),
                    Some("Earlier finding"),
                    None,
                ),
            ],
        )],
    );
    let later = atlas(
        "Two",
        vec![entry(
            "only",
            "Q",
            vec![
                branch(
                    "shifting",
                    Some("second parent"),
                    "Later prompt",
                    "taken",
                    Some("later claim"),
                    Some("Later finding"),
                    None,
                ),
                branch(
                    "standing",
                    Some("second parent"),
                    "Later standing prompt",
                    "taken",
                    Some("later claim"),
                    Some("Later finding"),
                    None,
                ),
            ],
        )],
    );
    let lines = render_changelog(&earlier, &later).expect("a sound pair");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry only",
            "  branch shifting, state changed, open to taken",
            "  branch shifting, parent changed, first parent to second parent",
            "  branch shifting, prompt changed, Earlier prompt to Later prompt",
            "  branch shifting, claim changed, unclaimed to later claim",
            "  branch standing, parent changed, first parent to second parent",
            "  branch standing, prompt changed, Earlier standing prompt to Later standing prompt",
            "  branch standing, claim changed, earlier claim to later claim",
            "  branch standing, finding changed, Earlier finding to Later finding",
            "changelog counts, entries arrived 0, departed 0, matched 1, branches changed 2",
        ])
    );
}

#[test]
fn two_empty_atlases_render_the_three_lines() {
    let lines =
        render_changelog(&atlas("One", vec![]), &atlas("Two", vec![])).expect("a sound pair");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "changelog counts, entries arrived 0, departed 0, matched 0, branches changed 0",
        ])
    );
}

#[test]
fn empty_earlier_atlas_counts_every_later_entry_arrived() {
    let later = atlas(
        "Two",
        vec![
            entry(
                "fresh",
                "Q",
                vec![branch("x", None, "P", "open", None, None, None)],
            ),
            entry("newer", "Q", vec![]),
        ],
    );
    let lines = render_changelog(&atlas("One", vec![]), &later).expect("a sound pair");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry fresh, arrived",
            "entry newer, arrived",
            "changelog counts, entries arrived 2, departed 0, matched 0, branches changed 0",
        ])
    );
}

#[test]
fn changelog_over_a_loaded_pair_matches_the_hand_built_reading() {
    let open_x = branch_obj(
        &[
            ("id", jstr("x")),
            ("prompt", jstr("Prompt x")),
            ("state", jstr("open")),
        ],
        &[],
    );
    let taken_x = branch_obj(
        &[
            ("id", jstr("x")),
            ("prompt", jstr("Prompt x")),
            ("state", jstr("taken")),
            ("finding", jstr("Loaded finding")),
        ],
        &[],
    );
    let entry = |label: &str, branches: &[String]| {
        jobj(&[
            ("label", jstr(label)),
            ("exploration", exploration_json("Shared question", branches)),
        ])
    };
    let wrap = |title: &str, entries: &[String]| {
        jobj(&[("title", jstr(title)), ("entries", jarr(entries))])
    };
    let earlier = atlas_loads(&wrap("Loaded one", &[entry("only", &[open_x])]));
    let later = atlas_loads(&wrap("Loaded two", &[entry("only", &[taken_x])]));
    let lines = render_changelog(&earlier, &later).expect("a sound pair");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, Loaded one",
            "later atlas, Loaded two",
            "entry only",
            "  branch x, state changed, open to taken",
            "changelog counts, entries arrived 0, departed 0, matched 1, branches changed 1",
        ])
    );
}

#[test]
fn wide_matched_entry_renders_whole() {
    let build = |changed: bool| {
        let branches: Vec<Branch> = (0..1500)
            .map(|n| {
                let prompt = if changed && n == 1499 {
                    "Closing prompt revised"
                } else if n == 1499 {
                    "Closing prompt"
                } else {
                    "Standing prompt"
                };
                branch(&format!("b{n}"), None, prompt, "open", None, None, None)
            })
            .collect();
        atlas("Wide", vec![entry("only", "Q", branches)])
    };
    let lines = render_changelog(&build(false), &build(true)).expect("a sound pair");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, Wide",
            "later atlas, Wide",
            "entry only",
            "  branch b1499, prompt changed, Closing prompt to Closing prompt revised",
            "changelog counts, entries arrived 0, departed 0, matched 1, branches changed 1",
        ])
    );
}

#[test]
fn refuses_a_surrogate_earlier_title_before_anything_else() {
    let mut earlier = atlas("One", vec![]);
    earlier.title = bad();
    let later = atlas("Two", vec![entry("x", "Q", vec![])]);
    let mut later = later;
    later.entries[0].label = bad();
    assert_eq!(
        render_changelog(&earlier, &later),
        Err("the atlas title holds an unpaired surrogate".to_string())
    );
}

#[test]
fn refuses_a_surrogate_later_title() {
    let mut later = atlas("Two", vec![]);
    later.title = bad();
    assert_eq!(
        render_changelog(&atlas("One", vec![]), &later),
        Err("the atlas title holds an unpaired surrogate".to_string())
    );
}

#[test]
fn refuses_a_surrogate_entry_label() {
    let mut earlier = atlas("One", vec![entry("x", "Q", vec![])]);
    earlier.entries[0].label = bad();
    assert_eq!(
        render_changelog(&earlier, &atlas("Two", vec![])),
        Err("an entry label holds an unpaired surrogate".to_string())
    );
}

#[test]
fn refuses_a_surrogate_question() {
    let mut later = atlas("Two", vec![entry("x", "Q", vec![])]);
    later.entries[0].exploration.question = bad();
    let earlier = atlas("One", vec![entry("x", "Q", vec![])]);
    assert_eq!(
        render_changelog(&earlier, &later),
        Err("the question holds an unpaired surrogate".to_string())
    );
}

#[test]
fn refuses_a_surrogate_branch_id() {
    let mut earlier = atlas(
        "One",
        vec![entry(
            "x",
            "Q",
            vec![branch("b", None, "P", "open", None, None, None)],
        )],
    );
    earlier.entries[0].exploration.branches[0].id = bad();
    let later = atlas(
        "Two",
        vec![entry(
            "x",
            "Q",
            vec![branch("b", None, "P", "open", None, None, None)],
        )],
    );
    assert_eq!(
        render_changelog(&earlier, &later),
        Err("a branch id holds an unpaired surrogate".to_string())
    );
}

#[test]
fn refuses_a_surrogate_branch_parent() {
    let mut later = atlas(
        "Two",
        vec![entry(
            "x",
            "Q",
            vec![branch("b", Some("a"), "P", "open", None, None, None)],
        )],
    );
    later.entries[0].exploration.branches[0].parent = Some(bad());
    let earlier = atlas(
        "One",
        vec![entry(
            "x",
            "Q",
            vec![branch("b", None, "P", "open", None, None, None)],
        )],
    );
    assert_eq!(
        render_changelog(&earlier, &later),
        Err("a branch parent holds an unpaired surrogate".to_string())
    );
}

#[test]
fn refuses_a_surrogate_branch_prompt() {
    let mut earlier = atlas(
        "One",
        vec![entry(
            "x",
            "Q",
            vec![branch("b", None, "P", "open", None, None, None)],
        )],
    );
    earlier.entries[0].exploration.branches[0].prompt = bad();
    let later = atlas(
        "Two",
        vec![entry(
            "x",
            "Q",
            vec![branch("b", None, "P", "open", None, None, None)],
        )],
    );
    assert_eq!(
        render_changelog(&earlier, &later),
        Err("a branch prompt holds an unpaired surrogate".to_string())
    );
}

#[test]
fn refuses_a_surrogate_claim() {
    let mut later = atlas(
        "Two",
        vec![entry(
            "x",
            "Q",
            vec![branch("b", None, "P", "open", Some("crew"), None, None)],
        )],
    );
    later.entries[0].exploration.branches[0].claim = Some(bad());
    let earlier = atlas(
        "One",
        vec![entry(
            "x",
            "Q",
            vec![branch("b", None, "P", "open", None, None, None)],
        )],
    );
    assert_eq!(
        render_changelog(&earlier, &later),
        Err("a claim holds an unpaired surrogate".to_string())
    );
}

#[test]
fn refuses_a_surrogate_finding_under_a_standing_taken_state() {
    let mut later = atlas(
        "Two",
        vec![entry(
            "x",
            "Q",
            vec![branch(
                "b",
                None,
                "P",
                "taken",
                None,
                Some("Sound text"),
                None,
            )],
        )],
    );
    later.entries[0].exploration.branches[0].finding = Some(bad());
    let earlier = atlas(
        "One",
        vec![entry(
            "x",
            "Q",
            vec![branch(
                "b",
                None,
                "P",
                "taken",
                None,
                Some("Sound text"),
                None,
            )],
        )],
    );
    assert_eq!(
        render_changelog(&earlier, &later),
        Err("a finding holds an unpaired surrogate".to_string())
    );
}

#[test]
fn refuses_a_surrogate_reason_under_a_standing_abandoned_state() {
    let mut earlier = atlas(
        "One",
        vec![entry(
            "x",
            "Q",
            vec![branch(
                "b",
                None,
                "P",
                "abandoned",
                None,
                None,
                Some("Sound text"),
            )],
        )],
    );
    earlier.entries[0].exploration.branches[0].reason = Some(bad());
    let later = atlas(
        "Two",
        vec![entry(
            "x",
            "Q",
            vec![branch(
                "b",
                None,
                "P",
                "abandoned",
                None,
                None,
                Some("Sound text"),
            )],
        )],
    );
    assert_eq!(
        render_changelog(&earlier, &later),
        Err("a reason holds an unpaired surrogate".to_string())
    );
}

#[test]
fn refuses_a_taken_branch_holding_no_finding_under_a_standing_state() {
    let earlier = atlas(
        "One",
        vec![entry(
            "x",
            "Q",
            vec![branch("b", None, "P", "taken", None, None, None)],
        )],
    );
    let later = atlas(
        "Two",
        vec![entry(
            "x",
            "Q",
            vec![branch(
                "b",
                None,
                "P",
                "taken",
                None,
                Some("Found text"),
                None,
            )],
        )],
    );
    assert_eq!(
        render_changelog(&earlier, &later),
        Err("a taken branch holds no finding".to_string())
    );
}

#[test]
fn refuses_an_abandoned_branch_holding_no_reason_under_a_standing_state() {
    let earlier = atlas(
        "One",
        vec![entry(
            "x",
            "Q",
            vec![branch(
                "b",
                None,
                "P",
                "abandoned",
                None,
                None,
                Some("Closed text"),
            )],
        )],
    );
    let later = atlas(
        "Two",
        vec![entry(
            "x",
            "Q",
            vec![branch("b", None, "P", "abandoned", None, None, None)],
        )],
    );
    assert_eq!(
        render_changelog(&earlier, &later),
        Err("an abandoned branch holds no reason".to_string())
    );
}
