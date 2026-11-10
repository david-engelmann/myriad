//! Tests for MY-36, the company examples.
//!
//! The shipped lineages print exactly their shipped
//! company goldens under the library reading, and the
//! winter golden's lines are the hand counts MY-36
//! states, counted from the atlas files themselves.
//!
//! The discrimination split of these pins stands as
//! follows, pin by pin. Pin 17, the examples of the
//! repo parse and validate, passes on the tree before
//! this slice by design in the behaviour it pins,
//! because every loader it names and every shipped
//! example it reads stand on that tree unchanged, and
//! it guards the new files' shapes as its siblings
//! guard theirs. Pin 18, the four company goldens
//! match the readings over their lineages, fails on
//! that tree by execution, because the goldens stand
//! nowhere there and the reading is not implemented
//! there, and in this file by compile absence,
//! because render_company stands nowhere there.
//! Pin 19, the examples census, is stated in this
//! file in its own eleven name form, the golden
//! files of the chain readings over the four shipped
//! chains standing at 33 before the slice and 37
//! after it, the four company goldens the increment,
//! so it fails on that tree by assertion against its
//! named set.
//! Inside this file pins 17 and 19 share the file's
//! compile absence on that tree, because
//! render_company stands nowhere there, as the
//! pins of every reading file of the family do.

mod common;

use common::*;

use myriad::atlas::{load_atlas_text, load_lineage_text, render_company, Lineage};
use myriad::explorations::load_exploration_text;

fn load(name: &str) -> Lineage {
    let path = repo_root().join("examples").join(name);
    let text = std::fs::read_to_string(&path).expect("a shipped example reads");
    let (lineage, defects) = load_lineage_text(&text);
    assert!(
        defects.is_empty(),
        "a shipped lineage loads clean: {defects:?}"
    );
    lineage.expect("a shipped lineage value stands")
}

fn golden(name: &str) -> Vec<String> {
    let path = repo_root().join("examples").join(name);
    let text = std::fs::read_to_string(&path).expect("a shipped golden reads");
    text.lines().map(str::to_string).collect()
}

#[test]
fn the_examples_of_the_repo_parse_and_validate() {
    let dir = repo_root().join("examples");
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .expect("the examples directory reads")
        .map(|entry| {
            entry
                .expect("an examples entry reads")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name.ends_with(".json"))
        .collect();
    names.sort();
    assert!(!names.is_empty(), "the examples carry JSON files");
    for name in &names {
        let text = std::fs::read_to_string(dir.join(name)).expect("a shipped example reads");
        if text.contains("\"atlases\"") {
            let (value, defects) = load_lineage_text(&text);
            assert!(defects.is_empty(), "{name} loads clean: {defects:?}");
            assert!(value.is_some(), "{name} holds a lineage");
        } else if text.contains("\"entries\"") {
            let (value, defects) = load_atlas_text(&text);
            assert!(defects.is_empty(), "{name} loads clean: {defects:?}");
            assert!(value.is_some(), "{name} holds an atlas");
        } else {
            let (value, defects) = load_exploration_text(&text);
            assert!(defects.is_empty(), "{name} loads clean: {defects:?}");
            assert!(value.is_some(), "{name} holds an exploration");
        }
    }
}

#[test]
fn the_four_company_goldens_match_the_readings() {
    for (lineage_name, golden_name) in [
        (
            "winter-spring-summer-lineage.json",
            "winter-spring-summer-company.txt",
        ),
        (
            "radio-investigation-lineage.json",
            "radio-investigation-company.txt",
        ),
        (
            "constructed-departures-lineage.json",
            "constructed-departures-company.txt",
        ),
        (
            "generated-chain-lineage.json",
            "generated-chain-company.txt",
        ),
    ] {
        let lineage = load(lineage_name);
        let lines = render_company(&lineage).expect("the company reads");
        assert_eq!(lines, golden(golden_name), "the golden of {lineage_name}");
    }
    assert_eq!(
        golden("winter-spring-summer-company.txt"),
        strings(&[
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "company counts, company 10, entries 4, atlases 3",
        ])
    );
}

#[test]
fn the_examples_census_counts_the_chain_goldens() {
    const READINGS: [&str; 11] = [
        "lineage",
        "returns",
        "standing",
        "absence",
        "arrivals",
        "departures",
        "matched",
        "runs",
        "presence",
        "standings",
        "company",
    ];
    let dir = repo_root().join("examples");
    let count = std::fs::read_dir(&dir)
        .expect("the examples directory reads")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            READINGS
                .iter()
                .any(|reading| name.ends_with(&format!("-{reading}.txt")))
        })
        .count();
    assert_eq!(
        count, 37,
        "the golden files of the chain readings over the four shipped chains"
    );
}
