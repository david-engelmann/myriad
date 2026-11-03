//! Pins for the atlas loader and the atlas rendering, MY-7
//! and MY-8, translated from tests/test_atlas.py.
//!
//! Every defect wording is pinned exactly, including the
//! prefix an entry's defects stand under, because a reader
//! finds the malformed entry by the words the loader
//! prints. The rendering pins hold the atlas to the map and
//! the claims listing of its own entries, byte for byte,
//! and to the one counting path, because an atlas that
//! rendered or counted on its own would disagree with the
//! tool it stands beside.

mod common;

use std::collections::BTreeMap;

use myriad::atlas::{load_atlas_file, load_atlas_text, render_atlas, Atlas, AtlasEntry};
use myriad::explorations::{count_by_state, render_claims, render_map, Exploration};
use myriad::fork::fork_skeleton;
use myriad::pytext::PText;

use common::{
    atlas_defects_of, atlas_loads, branch, branch_json, exploration_json, holds_vocabulary_bar,
    jarr, jobj, jstr, pt, temp_dir,
};

/// The base exploration of the Python helpers as JSON text.
fn exploration_text() -> String {
    exploration_json("q", &[branch_json("a", None, "p", "open")])
}

/// One atlas as JSON text from raw title and entry values.
fn atlas_text(title: String, entries: &[String]) -> String {
    jobj(&[("title", title), ("entries", jarr(entries))])
}

/// One entry as JSON text from raw label and exploration
/// values.
fn entry_text(label: String, exploration: String) -> String {
    jobj(&[("label", label), ("exploration", exploration)])
}

fn two_entry_atlas() -> Atlas {
    let first = Exploration {
        question: pt("q one"),
        branches: vec![
            branch("a", None, "p", "taken", Some("crew"), Some("f"), None),
            branch("b", Some("a"), "p2", "open", None, None, None),
            branch("c", None, "p3", "abandoned", None, None, Some("r")),
        ],
    };
    let second = Exploration {
        question: pt("q two"),
        branches: vec![
            branch("d", None, "p4", "open", Some("crew"), None, None),
            branch(
                "e",
                Some("d"),
                "p5",
                "taken",
                Some("crew"),
                Some("f2"),
                None,
            ),
        ],
    };
    Atlas {
        title: pt("the title"),
        entries: vec![
            AtlasEntry {
                label: pt("first"),
                exploration: first,
            },
            AtlasEntry {
                label: pt("second"),
                exploration: second,
            },
        ],
    }
}

fn sections_of(lines: &[String]) -> BTreeMap<String, Vec<String>> {
    let mut sections: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut current: Option<String> = None;
    for line in &lines[1..lines.len() - 1] {
        if let Some(label) = line.strip_prefix("entry ") {
            current = Some(label.to_string());
            sections.insert(label.to_string(), Vec::new());
        } else if let Some(label) = &current {
            sections
                .get_mut(label)
                .expect("a standing section")
                .push(line.clone());
        }
    }
    sections
}

#[test]
fn a_clean_atlas_loads_with_only_the_named_fields() {
    let text = jobj(&[
        ("title", jstr("t")),
        ("extra", jstr("ignored")),
        (
            "entries",
            jarr(&[jobj(&[
                ("label", jstr("one")),
                ("extra", jstr("ignored")),
                (
                    "exploration",
                    jobj(&[
                        ("question", jstr("q")),
                        ("extra", jstr("ignored")),
                        (
                            "branches",
                            jarr(&[jobj(&[
                                ("id", jstr("a")),
                                ("parent", common::jnull()),
                                ("prompt", jstr("p")),
                                ("state", jstr("open")),
                                ("extra", jstr("ignored")),
                            ])]),
                        ),
                    ]),
                ),
            ])]),
        ),
    ]);
    let found = atlas_loads(&text);
    let expected = Atlas {
        title: pt("t"),
        entries: vec![AtlasEntry {
            label: pt("one"),
            exploration: Exploration {
                question: pt("q"),
                branches: vec![branch("a", None, "p", "open", None, None, None)],
            },
        }],
    };
    assert_eq!(found, expected);
}

#[test]
fn an_unreadable_file_draws_the_cannot_be_read_defect() {
    let dir = temp_dir("atlas-file");
    let (found, defects) = load_atlas_file(&dir.join("missing.json"));
    assert!(found.is_none());
    assert_eq!(defects, vec!["file cannot be read".to_string()]);
}

#[test]
fn text_that_is_not_json_draws_the_not_valid_json_defect() {
    let (found, defects) = load_atlas_text("this is not json");
    assert!(found.is_none());
    assert_eq!(defects, vec!["input is not valid JSON".to_string()]);
}

#[test]
fn input_nested_too_deeply_draws_the_not_valid_json_defect() {
    let text = format!("{}{}", "[".repeat(10000), "]".repeat(10000));
    let (found, defects) = load_atlas_text(&text);
    assert!(found.is_none());
    assert_eq!(defects, vec!["input is not valid JSON".to_string()]);
}

#[test]
fn json_that_is_not_an_object_draws_the_not_an_object_defect() {
    let (found, defects) = load_atlas_text("[1, 2]");
    assert!(found.is_none());
    assert_eq!(defects, vec!["input is not a JSON object".to_string()]);
}

#[test]
fn a_missing_title_is_named() {
    assert_eq!(
        atlas_defects_of("{\"entries\": []}"),
        vec!["missing field title".to_string()]
    );
}

#[test]
fn a_title_of_the_wrong_type_is_named() {
    let data = jobj(&[("title", "4".to_string()), ("entries", jarr(&[]))]);
    assert_eq!(
        atlas_defects_of(&data),
        vec!["field title has the wrong type".to_string()]
    );
}

#[test]
fn an_empty_title_is_named() {
    let data = atlas_text(jstr(""), &[]);
    assert_eq!(
        atlas_defects_of(&data),
        vec!["field title is empty".to_string()]
    );
}

#[test]
fn a_title_holding_an_unpaired_surrogate_is_named() {
    let text = "{\"title\": \"\\ud800x\", \"entries\": []}";
    let (found, defects) = load_atlas_text(text);
    assert!(found.is_none());
    assert_eq!(
        defects,
        vec!["field title holds an unpaired surrogate".to_string()]
    );
}

#[test]
fn a_title_holding_a_line_break_is_named() {
    for title in ["two\nlines", "two\rlines"] {
        let data = atlas_text(jstr(title), &[]);
        assert_eq!(
            atlas_defects_of(&data),
            vec!["field title holds a newline or a carriage return".to_string()]
        );
    }
}

#[test]
fn a_missing_entries_field_is_named() {
    assert_eq!(
        atlas_defects_of("{\"title\": \"t\"}"),
        vec!["missing field entries".to_string()]
    );
}

#[test]
fn an_entries_field_of_the_wrong_type_is_named() {
    let data = jobj(&[("title", jstr("t")), ("entries", jstr("x"))]);
    assert_eq!(
        atlas_defects_of(&data),
        vec!["field entries has the wrong type".to_string()]
    );
}

#[test]
fn an_empty_entries_list_loads_clean() {
    let found = atlas_loads(&atlas_text(jstr("t"), &[]));
    assert_eq!(
        found,
        Atlas {
            title: pt("t"),
            entries: vec![],
        }
    );
}

#[test]
fn an_entry_that_is_not_an_object_is_named_by_index() {
    let data = atlas_text(jstr("t"), &[jstr("nope")]);
    assert_eq!(
        atlas_defects_of(&data),
        vec!["entry 0 is not an object".to_string()]
    );
}

#[test]
fn a_missing_label_is_named_by_index() {
    let data = atlas_text(jstr("t"), &[jobj(&[("exploration", exploration_text())])]);
    assert_eq!(
        atlas_defects_of(&data),
        vec!["entry 0 is missing field label".to_string()]
    );
}

#[test]
fn a_label_of_the_wrong_type_is_named_by_index() {
    let data = atlas_text(
        jstr("t"),
        &[entry_text("4".to_string(), exploration_text())],
    );
    assert_eq!(
        atlas_defects_of(&data),
        vec!["entry 0 field label has the wrong type".to_string()]
    );
}

#[test]
fn an_empty_label_is_named_by_index() {
    let data = atlas_text(jstr("t"), &[entry_text(jstr(""), exploration_text())]);
    assert_eq!(
        atlas_defects_of(&data),
        vec!["entry 0 field label is empty".to_string()]
    );
}

#[test]
fn a_label_holding_an_unpaired_surrogate_is_named_by_index() {
    let text = "{\"title\": \"t\", \"entries\": [{\"label\": \"\\ud800x\", \
        \"exploration\": {\"question\": \"q\", \"branches\": [{\"id\": \"a\", \
        \"parent\": null, \"prompt\": \"p\", \"state\": \"open\"}]}}]}";
    assert_eq!(
        atlas_defects_of(text),
        vec!["entry 0 field label holds an unpaired surrogate".to_string()]
    );
}

#[test]
fn a_label_holding_a_line_break_is_named_by_index_only() {
    let label = "two\nlines";
    let data = atlas_text(jstr("t"), &[entry_text(jstr(label), exploration_text())]);
    let defects = atlas_defects_of(&data);
    assert_eq!(
        defects,
        vec!["entry 0 field label holds a newline or a carriage return".to_string()]
    );
    assert!(defects.iter().all(|defect| !defect.contains(label)));
    assert!(defects.iter().all(|defect| !defect.contains('\n')));
}

#[test]
fn a_missing_exploration_is_named_under_the_clean_label() {
    let data = atlas_text(jstr("t"), &[jobj(&[("label", jstr("ferry"))])]);
    assert_eq!(
        atlas_defects_of(&data),
        vec!["entry ferry is missing field exploration".to_string()]
    );
}

#[test]
fn a_missing_exploration_is_named_by_index_under_a_bad_label() {
    let data = atlas_text(jstr("t"), &[jobj(&[("label", jstr(""))])]);
    assert_eq!(
        atlas_defects_of(&data),
        common::strings(&[
            "entry 0 field label is empty",
            "entry 0 is missing field exploration",
        ])
    );
}

#[test]
fn an_exploration_that_is_not_an_object_is_named() {
    let data = atlas_text(jstr("t"), &[entry_text(jstr("ferry"), jstr("nope"))]);
    assert_eq!(
        atlas_defects_of(&data),
        vec!["entry ferry field exploration is not a JSON object".to_string()]
    );
}

#[test]
fn entry_defects_carry_the_label_and_the_my_1_wording() {
    let ferry = entry_text(jstr("ferry"), "{\"branches\": []}".to_string());
    let dock_exploration = exploration_json(
        "q",
        &[
            branch_json("a", None, "", "open"),
            branch_json("a", Some("z"), "p", "open"),
        ],
    );
    let dock = entry_text(jstr("dock"), dock_exploration);
    let data = atlas_text(jstr("t"), &[ferry, dock]);
    assert_eq!(
        atlas_defects_of(&data),
        common::strings(&[
            "entry ferry, missing field question",
            "entry dock, branch 0 field prompt is empty",
            "entry dock, duplicate branch id a",
            "entry dock, branch a parent z does not exist",
        ])
    );
}

#[test]
fn entry_defects_fall_back_to_the_index_under_a_bad_label() {
    let bad = entry_text(
        jstr("bad\nlabel"),
        exploration_json("", &[branch_json("a", None, "p", "open")]),
    );
    let data = atlas_text(jstr("t"), &[bad]);
    assert_eq!(
        atlas_defects_of(&data),
        common::strings(&[
            "entry 0 field label holds a newline or a carriage return",
            "entry 0, field question is empty",
        ])
    );
}

#[test]
fn a_repeated_label_names_the_label_and_both_positions() {
    let entry = entry_text(jstr("ferry"), exploration_text());
    let data = atlas_text(
        jstr("t"),
        &[
            entry.clone(),
            entry_text(jstr("dock"), exploration_text()),
            entry,
        ],
    );
    assert_eq!(
        atlas_defects_of(&data),
        vec!["duplicate entry label ferry, entries 0 and 2".to_string()]
    );
}

#[test]
fn a_label_carried_three_times_draws_two_defects() {
    let entry = entry_text(jstr("ferry"), exploration_text());
    let data = atlas_text(jstr("t"), &[entry.clone(), entry.clone(), entry]);
    assert_eq!(
        atlas_defects_of(&data),
        common::strings(&[
            "duplicate entry label ferry, entries 0 and 1",
            "duplicate entry label ferry, entries 0 and 2",
        ])
    );
}

#[test]
fn the_repeated_label_check_runs_over_a_malformed_entry() {
    let data = atlas_text(
        jstr("t"),
        &[
            entry_text(jstr("ferry"), exploration_text()),
            entry_text(
                jstr("ferry"),
                exploration_json("", &[branch_json("a", None, "p", "open")]),
            ),
        ],
    );
    assert_eq!(
        atlas_defects_of(&data),
        common::strings(&[
            "entry ferry, field question is empty",
            "duplicate entry label ferry, entries 0 and 1",
        ])
    );
}

#[test]
fn labels_that_drew_a_defect_draw_no_duplicate_defect() {
    let entry = entry_text(jstr("bad\nlabel"), exploration_text());
    let data = atlas_text(jstr("t"), &[entry.clone(), entry]);
    assert_eq!(
        atlas_defects_of(&data),
        common::strings(&[
            "entry 0 field label holds a newline or a carriage return",
            "entry 1 field label holds a newline or a carriage return",
        ])
    );
}

#[test]
fn one_bad_entry_fails_the_whole_atlas_and_naming_continues() {
    let data = atlas_text(
        jstr("t"),
        &[
            entry_text(jstr("sound"), exploration_text()),
            entry_text(
                jstr("broken"),
                exploration_json("", &[branch_json("a", None, "p", "open")]),
            ),
            entry_text(
                jstr("alsbroken"),
                jobj(&[("question", jstr("q")), ("branches", jstr("x"))]),
            ),
        ],
    );
    let (found, defects) = load_atlas_text(&data);
    assert!(found.is_none());
    assert_eq!(
        defects,
        common::strings(&[
            "entry broken, field question is empty",
            "entry alsbroken, field branches has the wrong type",
        ])
    );
}

#[test]
fn defects_stand_title_first_entries_next_duplicates_last() {
    let data = atlas_text(
        jstr(""),
        &[
            entry_text(
                jstr("ferry"),
                exploration_json("", &[branch_json("a", None, "p", "open")]),
            ),
            entry_text(jstr("ferry"), exploration_text()),
        ],
    );
    assert_eq!(
        atlas_defects_of(&data),
        common::strings(&[
            "field title is empty",
            "entry ferry, field question is empty",
            "duplicate entry label ferry, entries 0 and 1",
        ])
    );
}

#[test]
fn the_loader_never_raises_on_hostile_input() {
    let hostile = [
        "".to_string(),
        "null".to_string(),
        "4".to_string(),
        "\"text\"".to_string(),
        "[]".to_string(),
        "{}".to_string(),
        "{\"title\": null, \"entries\": null}".to_string(),
        "{\"title\": \"t\", \"entries\": [null, 4, [], {}]}".to_string(),
        "{\"title\": \"t\", \"entries\": [{\"label\": [], \"exploration\": []}, \
         {\"label\": \"x\", \"exploration\": {\"question\": [], \"branches\": {}}}]}"
            .to_string(),
        "{\"title\": \"t\", \"entries\": [{\"label\": \"x\", \"exploration\": \
         {\"question\": \"q\", \"branches\": [{\"id\": 1}]}}]}"
            .to_string(),
    ];
    for text in &hostile {
        let (found, defects) = load_atlas_text(text);
        assert!(found.is_none(), "{text}");
        assert!(!defects.is_empty(), "{text}");
    }
}

#[test]
fn the_rendering_shape_line_for_line() {
    assert_eq!(
        render_atlas(&two_entry_atlas()).expect("a sound rendering"),
        common::strings(&[
            "atlas, the title",
            "entry first",
            "question, q one",
            "a (taken) p",
            "  claimed by crew",
            "  finding, f",
            "  b (open) p2",
            "c (abandoned) p3",
            "  reason, r",
            "branches 3, open 1, taken 1, abandoned 1",
            "claim crew, branches 1, open 0, taken 1, abandoned 0",
            "unclaimed, branches 2, open 1, taken 0, abandoned 1",
            "entry second",
            "question, q two",
            "d (open) p4",
            "  claimed by crew",
            "  e (taken) p5",
            "    claimed by crew",
            "    finding, f2",
            "branches 2, open 1, taken 1, abandoned 0",
            "claim crew, branches 2, open 1, taken 1, abandoned 0",
            "atlas counts, entries 2, branches 5, open 2, taken 2, abandoned 1",
        ])
    );
}

#[test]
fn each_section_is_the_map_and_the_listing_of_its_exploration() {
    let atlas = two_entry_atlas();
    let sections = sections_of(&render_atlas(&atlas).expect("a sound rendering"));
    for entry in &atlas.entries {
        let section = &sections[&entry.label.to_display_string()];
        let mut expected = render_map(&entry.exploration).expect("a sound rendering");
        expected.extend(
            render_claims(&entry.exploration).expect("a sound rendering")[1..]
                .iter()
                .cloned(),
        );
        assert_eq!(section, &expected);
    }
}

#[test]
fn the_closing_counts_equal_the_sum_of_the_section_counts() {
    let atlas = two_entry_atlas();
    let lines = render_atlas(&atlas).expect("a sound rendering");
    let closing = lines.last().expect("a closing line");
    let mut total = myriad::explorations::Counts::default();
    let mut union: Vec<myriad::explorations::Branch> = Vec::new();
    for entry in &atlas.entries {
        let counts = count_by_state(&entry.exploration);
        total.total += counts.total;
        total.open += counts.open;
        total.taken += counts.taken;
        total.abandoned += counts.abandoned;
        union.extend(entry.exploration.branches.iter().cloned());
    }
    let by_path = myriad::explorations::count_branches(&union);
    assert_eq!(by_path, total);
    assert_eq!(
        closing,
        &format!(
            "atlas counts, entries 2, branches {}, open {}, taken {}, abandoned {}",
            total.total, total.open, total.taken, total.abandoned
        )
    );
}

#[test]
fn a_title_carrying_the_counts_text_cannot_mint_the_closing_line() {
    let rich_branch = |id: &str, prompt: &str, state: &str, extra: &[(&str, String)]| {
        let mut fields = vec![
            ("id", jstr(id)),
            ("parent", common::jnull()),
            ("prompt", jstr(prompt)),
            ("state", jstr(state)),
        ];
        fields.extend(extra.iter().cloned());
        jobj(&fields)
    };
    let first = exploration_json(
        "q",
        &[
            branch_json("a", None, "p", "open"),
            rich_branch("b", "p2", "taken", &[("finding", jstr("f"))]),
            rich_branch("c", "p3", "abandoned", &[("reason", jstr("r"))]),
        ],
    );
    let second = exploration_json(
        "q",
        &[
            branch_json("d", None, "p4", "open"),
            branch_json("e", None, "p5", "open"),
        ],
    );
    let data = atlas_text(
        jstr("entries 2, branches 5, open 3, taken 1, abandoned 1"),
        &[
            entry_text(jstr("first"), first),
            entry_text(jstr("second"), second),
        ],
    );
    let atlas = atlas_loads(&data);
    let lines = render_atlas(&atlas).expect("a sound rendering");
    assert_eq!(
        lines[0],
        "atlas, entries 2, branches 5, open 3, taken 1, abandoned 1"
    );
    assert_eq!(
        lines[lines.len() - 1],
        "atlas counts, entries 2, branches 5, open 3, taken 1, abandoned 1"
    );
    assert_ne!(lines[0], lines[lines.len() - 1]);
}

#[test]
fn sections_stand_in_atlas_order_regardless_of_size() {
    let make = |label: &str, prompts: Vec<String>| AtlasEntry {
        label: pt(label),
        exploration: fork_skeleton(
            &pt(&format!("q {label}")),
            &prompts.iter().map(|prompt| pt(prompt)).collect::<Vec<_>>(),
        )
        .expect("a sound skeleton"),
    };
    let atlas = Atlas {
        title: pt("t"),
        entries: vec![
            make("big", (0..30).map(|i| format!("prompt {i}")).collect()),
            make("small", vec!["only prompt".to_string()]),
            make("mid", (0..10).map(|i| format!("prompt {i}")).collect()),
        ],
    };
    let lines = render_atlas(&atlas).expect("a sound rendering");
    let order: Vec<&str> = lines
        .iter()
        .filter(|line| line.starts_with("entry "))
        .map(String::as_str)
        .collect();
    assert_eq!(order, vec!["entry big", "entry small", "entry mid"]);
}

#[test]
fn the_unclaimed_line_stands_only_where_an_unclaimed_branch_does() {
    let atlas = two_entry_atlas();
    let sections = sections_of(&render_atlas(&atlas).expect("a sound rendering"));
    assert!(sections["first"]
        .iter()
        .any(|line| line.starts_with("unclaimed")));
    assert!(!sections["second"]
        .iter()
        .any(|line| line.starts_with("unclaimed")));
    assert!(sections["second"]
        .iter()
        .any(|line| line.starts_with("claim ")));
}

#[test]
fn an_empty_atlas_renders_its_title_and_its_counts() {
    let found = atlas_loads(&base_atlas_empty());
    assert_eq!(
        render_atlas(&found).expect("a sound rendering"),
        common::strings(&[
            "atlas, t",
            "atlas counts, entries 0, branches 0, open 0, taken 0, abandoned 0",
        ])
    );
}

fn base_atlas_empty() -> String {
    atlas_text(jstr("t"), &[])
}

#[test]
fn an_entry_with_no_branches_renders_its_question_and_counts() {
    let data = atlas_text(
        jstr("t"),
        &[entry_text(
            jstr("bare"),
            "{\"question\": \"q\", \"branches\": []}".to_string(),
        )],
    );
    let found = atlas_loads(&data);
    assert_eq!(
        render_atlas(&found).expect("a sound rendering"),
        common::strings(&[
            "atlas, t",
            "entry bare",
            "question, q",
            "branches 0, open 0, taken 0, abandoned 0",
            "atlas counts, entries 1, branches 0, open 0, taken 0, abandoned 0",
        ])
    );
}

#[test]
fn the_rendering_carries_no_ranking_or_verdict_word() {
    let lines = render_atlas(&two_entry_atlas()).expect("a sound rendering");
    let text = lines.join("\n").to_lowercase();
    assert!(holds_vocabulary_bar(&text));
}

#[test]
fn the_rendering_is_pure_and_repeatable() {
    let atlas = two_entry_atlas();
    let before = atlas.clone();
    let first = render_atlas(&atlas).expect("a sound rendering");
    assert_eq!(render_atlas(&atlas).expect("a sound rendering"), first);
    assert_eq!(atlas, before);
}

#[test]
fn an_atlas_summing_past_a_thousand_branches_renders_whole() {
    let mut entries = Vec::new();
    for number in 0..3 {
        let prompts: Vec<String> = (0..400).map(|i| format!("prompt {number} {i}")).collect();
        let skeleton = fork_skeleton(
            &pt(&format!("question {number}")),
            &prompts.iter().map(|prompt| pt(prompt)).collect::<Vec<_>>(),
        )
        .expect("a sound skeleton");
        entries.push(AtlasEntry {
            label: pt(&format!("part {number}")),
            exploration: skeleton,
        });
    }
    let atlas = Atlas {
        title: pt("wide"),
        entries,
    };
    let lines = render_atlas(&atlas).expect("a sound rendering");
    assert_eq!(
        lines[lines.len() - 1],
        "atlas counts, entries 3, branches 1200, open 1200, taken 0, abandoned 0"
    );
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.as_str()
                == "atlas counts, entries 3, branches 1200, open 1200, taken 0, abandoned 0")
            .count(),
        1
    );
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("entry "))
            .count(),
        3
    );
}

#[test]
fn an_atlas_holding_a_deep_exploration_renders_whole() {
    let depth = 5000;
    let branches: Vec<String> = (0..depth)
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
    let data = atlas_text(
        jstr("deep"),
        &[entry_text(jstr("chain"), exploration_json("q", &branches))],
    );
    let found = atlas_loads(&data);
    let lines = render_atlas(&found).expect("a sound rendering");
    assert_eq!(
        lines[lines.len() - 1],
        format!("atlas counts, entries 1, branches {depth}, open {depth}, taken 0, abandoned 0")
    );
    let deepest = format!(
        "{}b{} (open) p{}",
        "  ".repeat(depth - 1),
        depth - 1,
        depth - 1
    );
    assert!(lines.iter().any(|line| line == &deepest));
}

// The PORT-11 pins. A hand-built atlas value that breaks
// the loader preconditions draws the named refusal, and a
// refusal inside an entry exploration stands in the atlas
// rendering whole.

/// A text holding an unpaired surrogate after one letter,
/// the hand-built shape the readings refuse by field.
fn surrogate_text() -> PText {
    PText(vec![0x61, 0xD800])
}

#[test]
fn a_surrogate_atlas_title_draws_the_refusal() {
    let atlas = Atlas {
        title: surrogate_text(),
        entries: vec![],
    };
    assert_eq!(
        render_atlas(&atlas).expect_err("a refusal"),
        "the atlas title holds an unpaired surrogate"
    );
}

#[test]
fn a_surrogate_entry_label_draws_the_refusal() {
    let mut atlas = two_entry_atlas();
    atlas.entries[0].label = surrogate_text();
    assert_eq!(
        render_atlas(&atlas).expect_err("a refusal"),
        "an entry label holds an unpaired surrogate"
    );
}

#[test]
fn a_taken_branch_holding_no_finding_draws_the_refusal_in_the_atlas() {
    let mut atlas = two_entry_atlas();
    atlas.entries[0].exploration.branches[0].finding = None;
    assert_eq!(
        render_atlas(&atlas).expect_err("a refusal"),
        "a taken branch holds no finding"
    );
}
