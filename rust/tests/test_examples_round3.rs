//! Pins for the Round 3 example under examples, translated
//! from tests/test_examples_round3.py.
//!
//! The winter atlas holds three explorations of differing
//! shapes in one file, a tree carrying claims, a chain
//! carrying none, and a question with no branches at all,
//! and its golden rendering has to match the rendering the
//! code produces today byte for byte. The closing counts
//! are also pinned against the counts counted by hand from
//! the example file, so the golden cannot drift with the
//! code.

mod common;

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use myriad::atlas::{load_atlas_file, render_atlas, Atlas};

use common::{holds_vocabulary_bar, pt, repo_root};

fn examples() -> PathBuf {
    repo_root().join("examples")
}

fn load_atlas_example() -> Atlas {
    let (found, defects) = load_atlas_file(&examples().join("winter-atlas.json"));
    assert_eq!(defects, Vec::<String>::new());
    found.expect("a clean load")
}

#[test]
fn the_atlas_example_loads_clean_with_three_entries() {
    let atlas = load_atlas_example();
    assert_eq!(atlas.title, pt("Winter questions for a small harbour"));
    let labels: Vec<String> = atlas
        .entries
        .iter()
        .map(|entry| entry.label.to_display_string())
        .collect();
    assert_eq!(
        labels,
        vec!["ferry link", "lifeboat cover", "harbour lights"]
    );
}

#[test]
fn the_atlas_example_entries_differ_in_shape() {
    let atlas = load_atlas_example();
    let ferry = &atlas.entries[0].exploration;
    let lifeboat = &atlas.entries[1].exploration;
    let lights = &atlas.entries[2].exploration;
    let claimed: Vec<_> = ferry
        .branches
        .iter()
        .filter(|held| held.claim.is_some())
        .collect();
    assert_eq!(ferry.branches.len(), 6);
    assert_eq!(claimed.len(), 3);
    let labels: BTreeSet<String> = claimed
        .iter()
        .map(|held| held.claim.as_ref().expect("a claim").to_display_string())
        .collect();
    assert_eq!(
        labels,
        BTreeSet::from(["boat crew".to_string(), "harbour office".to_string()])
    );
    assert_eq!(lifeboat.branches.len(), 4);
    assert!(lifeboat.branches.iter().all(|held| held.claim.is_none()));
    let parents: Vec<Option<String>> = ["call", "launch", "sea", "back"]
        .iter()
        .map(|id| {
            lifeboat
                .branches
                .iter()
                .find(|held| held.id.eq_str(id))
                .expect("a chain branch")
                .parent
                .as_ref()
                .map(|parent| parent.to_display_string())
        })
        .collect();
    assert_eq!(
        parents,
        vec![
            None,
            Some("call".to_string()),
            Some("launch".to_string()),
            Some("sea".to_string()),
        ]
    );
    assert!(lights.branches.is_empty());
}

#[test]
fn the_atlas_golden_rendering_is_byte_identical() {
    let atlas = load_atlas_example();
    let golden = fs::read(examples().join("winter-atlas.txt")).expect("the golden");
    let rendered = format!(
        "{}\n",
        render_atlas(&atlas).expect("a sound rendering").join("\n")
    )
    .into_bytes();
    assert_eq!(rendered, golden);
}

#[test]
fn the_atlas_golden_closes_with_the_hand_counted_counts() {
    let golden = fs::read_to_string(examples().join("winter-atlas.txt")).expect("the golden");
    let closing = golden
        .trim_end_matches('\n')
        .lines()
        .last()
        .expect("a closing line");
    assert_eq!(
        closing,
        "atlas counts, entries 3, branches 10, open 5, taken 3, abandoned 2"
    );
}

#[test]
fn the_atlas_golden_carries_no_ranking_or_verdict_word() {
    let golden = fs::read_to_string(examples().join("winter-atlas.txt"))
        .expect("the golden")
        .to_lowercase();
    assert!(holds_vocabulary_bar(&golden));
}
