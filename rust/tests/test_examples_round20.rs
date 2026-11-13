//! Tests for MY-42, the fellows examples.
//!
//! The shipped lineages print exactly their shipped
//! fellows goldens under the library reading, and the
//! winter golden's lines are the hand counts MY-42
//! states, counted from the atlas files themselves.
//!
//! The discrimination split of these pins stands as
//! follows, pin by pin. Pin 17, the examples of the
//! repo parse and validate, passes on the tree before
//! this slice by design in the behaviour it pins,
//! because every loader it names and every shipped
//! example it reads stand on that tree unchanged, and
//! it guards the new files' shapes as its siblings
//! guard theirs. Pin 18, the four fellows goldens
//! match the readings over their lineages, fails on
//! that tree by execution, because the goldens stand
//! nowhere there and the reading is not implemented
//! there, and in this file by compile absence,
//! because render_fellows stands nowhere there.
//! Pin 19, the examples census, is stated in this
//! file in its own fourteen name form, the golden
//! files of the chain readings over the four shipped
//! chains standing at 45 before the slice and 49
//! after it, the four fellows goldens the increment,
//! so it fails on that tree by assertion against its
//! named set.
//! Inside this file pins 17 and 19 share the file's
//! compile absence on that tree, because
//! render_fellows stands nowhere there, as the
//! pins of every reading file of the family do.

mod common;

use common::*;

use myriad::atlas::{load_atlas_text, load_lineage_text, render_fellows, Lineage};
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
fn the_four_fellows_goldens_match_the_readings() {
    for (lineage_name, golden_name) in [
        (
            "winter-spring-summer-lineage.json",
            "winter-spring-summer-fellows.txt",
        ),
        (
            "radio-investigation-lineage.json",
            "radio-investigation-fellows.txt",
        ),
        (
            "constructed-departures-lineage.json",
            "constructed-departures-fellows.txt",
        ),
        (
            "generated-chain-lineage.json",
            "generated-chain-fellows.txt",
        ),
    ] {
        let lineage = load(lineage_name);
        let lines = render_fellows(&lineage).expect("the fellows read");
        assert_eq!(lines, golden(golden_name), "the golden of {lineage_name}");
    }
    assert_eq!(
        golden("winter-spring-summer-fellows.txt"),
        strings(&[
            "atlas 1",
            "atlas 2",
            "atlas 3",
            "atlas 2",
            "atlas 1",
            "atlas 3",
            "atlas 3",
            "atlas 1",
            "atlas 2",
            "fellows counts, fellows 6, atlases 3, entries 4",
        ])
    );
}

#[test]
fn the_examples_census_counts_the_chain_goldens() {
    const READINGS: [&str; 14] = [
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
        "sets",
        "rosters",
        "fellows",
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
        count, 49,
        "the golden files of the chain readings over the four shipped chains"
    );
}
