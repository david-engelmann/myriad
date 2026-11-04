//! Tests for MY-12's worked example, the winter atlas and
//! its stated successor.
//!
//! The golden changelog under examples is the output of
//! the real command over the two example atlases, and the
//! counts in its closing line are hand counts stated in
//! the contract.

mod common;

use common::*;
use myriad::atlas::{load_atlas_file, render_atlas, render_changelog, render_index};
use std::path::PathBuf;

fn example(name: &str) -> PathBuf {
    repo_root().join("examples").join(name)
}

fn example_text(name: &str) -> String {
    std::fs::read_to_string(example(name)).expect("an example file")
}

fn load_example(name: &str) -> myriad::atlas::Atlas {
    let (found, defects) = load_atlas_file(&example(name));
    assert_eq!(defects, Vec::<String>::new(), "example {name} loads clean");
    found.expect("an example atlas")
}

#[test]
fn golden_changelog_matches_the_command_output() {
    let outcome = run_cli(&[
        "changelog",
        &example("winter-atlas.json").to_string_lossy(),
        &example("spring-atlas.json").to_string_lossy(),
    ]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        example_text("winter-spring-changelog.txt")
    );
}

#[test]
fn golden_changelog_matches_the_reading_of_the_loaded_pair() {
    let earlier = load_example("winter-atlas.json");
    let later = load_example("spring-atlas.json");
    let lines = render_changelog(&earlier, &later).expect("the example pair reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert_eq!(text, example_text("winter-spring-changelog.txt"));
}

#[test]
fn golden_closing_counts_match_the_contract_hand_counts() {
    let golden = example_text("winter-spring-changelog.txt");
    let closing = golden.lines().last().expect("a closing line");
    assert_eq!(
        closing,
        "changelog counts, entries arrived 1, departed 1, matched 2, branches changed 8"
    );
}

#[test]
fn golden_holds_the_vocabulary_bar() {
    assert!(holds_vocabulary_bar(&example_text(
        "winter-spring-changelog.txt"
    )));
}

#[test]
fn successor_holds_an_arrived_a_departed_and_a_changed_entry() {
    let earlier = load_example("winter-atlas.json");
    let later = load_example("spring-atlas.json");
    let earlier_labels: Vec<String> = earlier
        .entries
        .iter()
        .map(|entry| entry.label.to_string())
        .collect();
    let later_labels: Vec<String> = later
        .entries
        .iter()
        .map(|entry| entry.label.to_string())
        .collect();
    assert!(later_labels
        .iter()
        .any(|label| !earlier_labels.contains(label)));
    assert!(earlier_labels
        .iter()
        .any(|label| !later_labels.contains(label)));
    let lines = render_changelog(&earlier, &later).expect("the example pair reads");
    assert!(lines.iter().any(|line| line.contains(", state changed, ")));
    assert!(lines.iter().any(|line| line.contains(", claim changed, ")));
}

fn ferry(atlas: &myriad::atlas::Atlas) -> &myriad::atlas::AtlasEntry {
    atlas
        .entries
        .iter()
        .find(|entry| entry.label.eq_str("ferry link"))
        .expect("the ferry entry stands in both atlases")
}

#[test]
fn state_changed_branch_reads_the_same_in_the_two_maps() {
    let earlier = load_example("winter-atlas.json");
    let later = load_example("spring-atlas.json");
    let earlier_map = myriad::explorations::render_map(&ferry(&earlier).exploration)
        .expect("the earlier map reads");
    let later_map =
        myriad::explorations::render_map(&ferry(&later).exploration).expect("the later map reads");
    assert!(earlier_map
        .iter()
        .any(|line| line == "berth (open) Where does the ferry tie up when the harbour ices over"));
    assert!(
        later_map
            .iter()
            .any(|line| line
                == "berth (taken) Where does the ferry tie up when the harbour ices over")
    );
    let lines = render_changelog(&earlier, &later).expect("the example pair reads");
    assert!(lines.contains(&"  branch berth, state changed, open to taken".to_string()));
}

#[test]
fn winter_atlas_readings_stay_byte_identical() {
    let winter = load_example("winter-atlas.json");
    let rendering = render_atlas(&winter).expect("the winter rendering reads");
    let text: String = rendering.iter().map(|line| format!("{line}\n")).collect();
    assert_eq!(text, example_text("winter-atlas.txt"));
    let index = render_index(&winter).expect("the winter index reads");
    let text: String = index.iter().map(|line| format!("{line}\n")).collect();
    assert_eq!(text, example_text("winter-atlas-index.txt"));
}
