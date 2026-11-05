//! Tests for MY-13, the lineage loader and reading.
//!
//! The loader validates each member atlas through the
//! MY-7 validation and names every defect under the
//! position of its atlas, and the reading is the
//! changelogs of the steps of the chain composed, a chain
//! of two rendering the changelog of the pair byte for
//! byte and a longer chain closing with the summed
//! lineage counts. Refusals of hand-built values follow
//! MY-11 and PORT-11.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_file, load_lineage_text, render_changelog, render_lineage, Atlas, AtlasEntry,
    Lineage,
};
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

fn atlas_json(title: &str, entries: &[String]) -> String {
    jobj(&[("title", jstr(title)), ("entries", jarr(entries))])
}

fn entry_json(label: &str, question: &str, branches: &[String]) -> String {
    jobj(&[
        ("label", jstr(label)),
        ("exploration", exploration_json(question, branches)),
    ])
}

fn lineage_json(atlases: &[String]) -> String {
    jobj(&[("atlases", jarr(atlases))])
}

fn plain_atlas_json(title: &str) -> String {
    atlas_json(title, &[entry_json("only", "Q", &[])])
}

fn lineage_loads(data_json: &str) -> Lineage {
    let (found, defects) = load_lineage_text(data_json);
    assert_eq!(defects, Vec::<String>::new());
    found.expect("a clean load holds a lineage")
}

fn lineage_defects_of(data_json: &str) -> Vec<String> {
    let (found, defects) = load_lineage_text(data_json);
    assert!(found.is_none(), "expected defects, found a clean load");
    defects
}

// The three atlases of the composed reading pins. The
// kept entry changes at the first step and stands still
// at the second, the gone entry departs at the first
// step and returns at the second, and the fresh entry
// arrives at the first step and departs at the second.

fn first() -> Atlas {
    atlas(
        "First",
        vec![
            entry(
                "kept",
                "Q kept",
                vec![branch("k1", None, "Prompt k1", "open", None, None, None)],
            ),
            entry(
                "gone",
                "Q gone",
                vec![branch("g1", None, "Prompt g1", "open", None, None, None)],
            ),
        ],
    )
}

fn second() -> Atlas {
    atlas(
        "Second",
        vec![
            entry(
                "kept",
                "Q kept",
                vec![branch(
                    "k1",
                    None,
                    "Prompt k1",
                    "taken",
                    None,
                    Some("Finding k1"),
                    None,
                )],
            ),
            entry(
                "fresh",
                "Q fresh",
                vec![branch("f1", None, "Prompt f1", "open", None, None, None)],
            ),
        ],
    )
}

fn third() -> Atlas {
    atlas(
        "Third",
        vec![
            entry(
                "kept",
                "Q kept",
                vec![branch(
                    "k1",
                    None,
                    "Prompt k1",
                    "taken",
                    None,
                    Some("Finding k1"),
                    None,
                )],
            ),
            entry(
                "gone",
                "Q gone returns",
                vec![
                    branch("g1", None, "Prompt g1", "open", None, None, None),
                    branch("g2", None, "Prompt g2", "open", None, None, None),
                ],
            ),
        ],
    )
}

#[test]
fn a_chain_of_two_renders_the_changelog_of_the_pair() {
    let lineage = Lineage {
        atlases: vec![first(), second()],
    };
    let lines = render_lineage(&lineage).expect("the pair reads");
    let changelog = render_changelog(&first(), &second()).expect("the pair reads");
    assert_eq!(lines, changelog);
    assert_eq!(
        lines.last().map(String::as_str),
        Some("changelog counts, entries arrived 1, departed 1, matched 1, branches changed 1")
    );
}

#[test]
fn a_loaded_chain_of_two_renders_the_changelog_of_the_pair() {
    let text = lineage_json(&[plain_atlas_json("One"), plain_atlas_json("Two")]);
    let lineage = lineage_loads(&text);
    let lines = render_lineage(&lineage).expect("the loaded pair reads");
    assert_eq!(
        lines,
        render_changelog(&lineage.atlases[0], &lineage.atlases[1]).expect("the pair reads")
    );
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry only, unchanged",
            "changelog counts, entries arrived 0, departed 0, matched 1, branches changed 0",
        ])
    );
}

#[test]
fn a_chain_of_three_composes_its_steps_and_sums_the_counts() {
    let lineage = Lineage {
        atlases: vec![first(), second(), third()],
    };
    let lines = render_lineage(&lineage).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, First",
            "later atlas, Second",
            "entry kept",
            "  branch k1, state changed, open to taken",
            "entry gone, departed",
            "entry fresh, arrived",
            "changelog counts, entries arrived 1, departed 1, matched 1, branches changed 1",
            "earlier atlas, Second",
            "later atlas, Third",
            "entry kept, unchanged",
            "entry fresh, departed",
            "entry gone, arrived",
            "changelog counts, entries arrived 1, departed 1, matched 1, branches changed 0",
            "lineage counts, steps 2, entries arrived 2, departed 2, matched 2, branches changed 1",
        ])
    );
}

#[test]
fn the_departed_entry_returns_as_arrived_and_never_matched() {
    let lineage = Lineage {
        atlases: vec![first(), second(), third()],
    };
    let lines = render_lineage(&lineage).expect("the chain reads");
    let departed_at = lines
        .iter()
        .position(|line| line == "entry gone, departed")
        .expect("the departure stands");
    let arrived_at = lines
        .iter()
        .position(|line| line == "entry gone, arrived")
        .expect("the return stands");
    assert!(departed_at < arrived_at);
    assert!(!lines.iter().any(|line| line == "entry gone"));
    assert!(!lines
        .iter()
        .any(|line| line == "entry gone, unchanged" || line.starts_with("  branch g")));
}

#[test]
fn a_chain_of_four_sums_three_steps() {
    let lineage = Lineage {
        atlases: vec![first(), second(), third(), third()],
    };
    let lines = render_lineage(&lineage).expect("the chain reads");
    assert_eq!(
        lines.last().map(String::as_str),
        Some(
            "lineage counts, steps 3, entries arrived 2, departed 2, matched 4, branches changed 1"
        )
    );
    let step_heads: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("earlier atlas, "))
        .collect();
    assert_eq!(step_heads.len(), 3);
}

#[test]
fn the_lineage_reading_holds_the_vocabulary_bar() {
    let lineage = Lineage {
        atlases: vec![first(), second(), third()],
    };
    let lines = render_lineage(&lineage).expect("the chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert!(holds_vocabulary_bar(&text));
}

#[test]
fn a_wide_chain_renders_its_lineage_whole() {
    let wide_branches: Vec<Branch> = (0..1200)
        .map(|index| {
            branch(
                &format!("b{index}"),
                None,
                "Prompt",
                "open",
                None,
                None,
                None,
            )
        })
        .collect();
    let wide = atlas("Wide", vec![entry("wide", "Q wide", wide_branches)]);
    let lineage = Lineage {
        atlases: vec![wide.clone(), wide.clone(), wide],
    };
    let lines = render_lineage(&lineage).expect("the wide chain reads");
    assert_eq!(
        lines.last().map(String::as_str),
        Some(
            "lineage counts, steps 2, entries arrived 0, departed 0, matched 2, branches changed 0"
        )
    );
}

#[test]
fn text_that_is_not_json_draws_the_not_valid_defect() {
    assert_eq!(
        lineage_defects_of("not json at all"),
        strings(&["input is not valid JSON"])
    );
}

#[test]
fn json_that_is_not_an_object_draws_the_not_object_defect() {
    assert_eq!(
        lineage_defects_of("[1, 2]"),
        strings(&["input is not a JSON object"])
    );
}

#[test]
fn a_missing_atlases_field_draws_missing_field_atlases() {
    assert_eq!(
        lineage_defects_of(&jobj(&[("title", jstr("Chain"))])),
        strings(&["missing field atlases"])
    );
}

#[test]
fn an_atlases_field_of_the_wrong_kind_draws_the_wrong_type() {
    let text = jobj(&[("atlases", jstr("nope"))]);
    assert_eq!(
        lineage_defects_of(&text),
        strings(&["field atlases has the wrong type"])
    );
}

#[test]
fn an_empty_chain_and_a_chain_of_one_draw_fewer_than_two() {
    assert_eq!(
        lineage_defects_of(&lineage_json(&[])),
        strings(&["field atlases holds fewer than two atlases"])
    );
    assert_eq!(
        lineage_defects_of(&lineage_json(&[plain_atlas_json("One")])),
        strings(&["field atlases holds fewer than two atlases"])
    );
}

#[test]
fn a_member_defect_stands_under_the_index_of_its_atlas() {
    let member = jobj(&[("entries", jarr(&[]))]);
    let text = lineage_json(&[plain_atlas_json("One"), member]);
    assert_eq!(
        lineage_defects_of(&text),
        strings(&["atlas 1, missing field title"])
    );
}

#[test]
fn a_member_entry_defect_carries_the_atlas_and_entry_prefixes() {
    let bad_branch = branch_json("a", None, "p", "sideways");
    let member = atlas_json("One", &[entry_json("only", "Q", &[bad_branch])]);
    let text = lineage_json(&[member, plain_atlas_json("Two")]);
    assert_eq!(
        lineage_defects_of(&text),
        strings(&["atlas 0, entry only, branch 0 field state is not a known state"])
    );
}

#[test]
fn a_member_that_is_not_an_object_draws_the_shape_wording_under_its_index() {
    let text = lineage_json(&[plain_atlas_json("One"), jstr("nope")]);
    assert_eq!(
        lineage_defects_of(&text),
        strings(&["atlas 1, input is not a JSON object"])
    );
}

#[test]
fn the_field_defect_stands_before_the_member_defects() {
    let member = jobj(&[("entries", jarr(&[]))]);
    let text = lineage_json(&[member]);
    assert_eq!(
        lineage_defects_of(&text),
        strings(&[
            "field atlases holds fewer than two atlases",
            "atlas 0, missing field title",
        ])
    );
}

#[test]
fn member_defects_stand_in_member_order() {
    let bad_one = jobj(&[("entries", jarr(&[]))]);
    let bad_two = jobj(&[("title", jstr("Three"))]);
    let text = lineage_json(&[bad_one, plain_atlas_json("Two"), bad_two]);
    assert_eq!(
        lineage_defects_of(&text),
        strings(&[
            "atlas 0, missing field title",
            "atlas 2, missing field entries",
        ])
    );
}

#[test]
fn a_malformed_member_returns_no_lineage() {
    let text = lineage_json(&[
        plain_atlas_json("One"),
        jobj(&[("entries", jarr(&[]))]),
        plain_atlas_json("Three"),
    ]);
    let (found, defects) = load_lineage_text(&text);
    assert!(found.is_none());
    assert_eq!(defects, strings(&["atlas 1, missing field title"]));
}

#[test]
fn fields_the_contract_does_not_name_are_ignored() {
    let mut member = plain_atlas_json("One");
    member = member.replace("\"title\"", "\"extra\": 1, \"title\"");
    let text = lineage_json(&[member, plain_atlas_json("Two")])
        .replace("\"atlases\"", "\"note\": \"a chain\", \"atlases\"");
    let lineage = lineage_loads(&text);
    assert_eq!(lineage.atlases.len(), 2);
    assert_eq!(lineage.atlases[0].title.to_string(), "One");
}

#[test]
fn an_unreadable_lineage_file_draws_file_cannot_be_read() {
    let missing = temp_dir("lineage-missing").join("gone.json");
    let (found, defects) = load_lineage_file(&missing);
    assert!(found.is_none());
    assert_eq!(defects, strings(&["file cannot be read"]));
}

#[test]
fn a_hand_built_lineage_of_fewer_than_two_atlases_is_refused() {
    let one = Lineage {
        atlases: vec![first()],
    };
    assert_eq!(
        render_lineage(&one).expect_err("a chain of one is refused"),
        "the lineage holds fewer than two atlases"
    );
    let none = Lineage { atlases: vec![] };
    assert_eq!(
        render_lineage(&none).expect_err("an empty chain is refused"),
        "the lineage holds fewer than two atlases"
    );
}

#[test]
fn a_hand_built_surrogate_in_the_last_atlas_names_its_refusal() {
    let mut broken = third();
    broken.title = bad();
    let lineage = Lineage {
        atlases: vec![first(), second(), broken],
    };
    assert_eq!(
        render_lineage(&lineage).expect_err("the broken step is refused"),
        "the atlas title holds an unpaired surrogate"
    );
}

#[test]
fn a_hand_built_surrogate_in_a_later_step_names_that_step() {
    let mut broken_second = second();
    broken_second.entries[0].exploration.branches[0].prompt = bad();
    let lineage = Lineage {
        atlases: vec![first(), broken_second, third()],
    };
    assert_eq!(
        render_lineage(&lineage).expect_err("the broken branch is refused"),
        "a branch prompt holds an unpaired surrogate"
    );
}
