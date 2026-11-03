//! Pins for the Round 4 example under examples, translated
//! from tests/test_examples_round4.py.
//!
//! The golden index of the winter atlas has to match the
//! index reading the code produces today byte for byte, and
//! the entry lines and closing counts are also pinned
//! against the counts counted by hand from the example
//! file, so the golden cannot drift with the code. The
//! winter atlas itself does not move in this round, and its
//! Round 3 golden still has to match it.

mod common;

use std::fs;
use std::path::PathBuf;

use myriad::atlas::{load_atlas_file, render_atlas, render_index, Atlas};

use common::{holds_vocabulary_bar, repo_root};

fn examples() -> PathBuf {
    repo_root().join("examples")
}

fn load_atlas_example() -> Atlas {
    let (found, defects) = load_atlas_file(&examples().join("winter-atlas.json"));
    assert_eq!(defects, Vec::<String>::new());
    found.expect("a clean load")
}

#[test]
fn the_index_golden_matches_the_reading_byte_for_byte() {
    let golden = fs::read_to_string(examples().join("winter-atlas-index.txt")).expect("the golden");
    let lines = render_index(&load_atlas_example()).expect("a sound rendering");
    assert_eq!(format!("{}\n", lines.join("\n")), golden);
}

#[test]
fn the_index_golden_lines_counted_by_hand() {
    let golden = fs::read_to_string(examples().join("winter-atlas-index.txt")).expect("the golden");
    let lines: Vec<&str> = golden.lines().collect();
    assert_eq!(
        lines,
        vec![
            "atlas, Winter questions for a small harbour",
            "entry ferry link, question, How does the harbour keep a ferry sailing through the winter, branches 6, open 3, taken 2, abandoned 1",
            "entry lifeboat cover, question, Who answers a call for help on the water in winter, branches 4, open 2, taken 1, abandoned 1",
            "entry harbour lights, question, Which harbour lights stay lit through the winter nights, branches 0, open 0, taken 0, abandoned 0",
            "atlas counts, entries 3, branches 10, open 5, taken 3, abandoned 2",
        ]
    );
}

#[test]
fn the_index_opens_and_closes_with_the_atlas_golden_lines() {
    let atlas = load_atlas_example();
    let atlas_golden = fs::read_to_string(examples().join("winter-atlas.txt")).expect("the golden");
    let atlas_lines: Vec<&str> = atlas_golden.lines().collect();
    let index = render_index(&atlas).expect("a sound rendering");
    assert_eq!(index[0], atlas_lines[0]);
    assert_eq!(index[index.len() - 1], atlas_lines[atlas_lines.len() - 1]);
}

#[test]
fn the_empty_exploration_carries_the_zero_counts_in_the_index() {
    let index = render_index(&load_atlas_example()).expect("a sound rendering");
    let lights: Vec<&String> = index
        .iter()
        .filter(|line| line.starts_with("entry harbour lights"))
        .collect();
    assert_eq!(
        lights,
        vec![&"entry harbour lights, question, Which harbour lights stay lit through the winter nights, branches 0, open 0, taken 0, abandoned 0".to_string()]
    );
}

#[test]
fn the_winter_atlas_still_matches_its_round_three_golden() {
    let golden = fs::read_to_string(examples().join("winter-atlas.txt")).expect("the golden");
    let lines = render_atlas(&load_atlas_example()).expect("a sound rendering");
    assert_eq!(format!("{}\n", lines.join("\n")), golden);
}

#[test]
fn the_index_golden_holds_to_the_vocabulary_bar() {
    let golden = fs::read_to_string(examples().join("winter-atlas-index.txt")).expect("the golden");
    assert!(holds_vocabulary_bar(&golden));
}

#[test]
fn the_index_reading_holds_to_the_vocabulary_bar() {
    let joined = render_index(&load_atlas_example())
        .expect("a sound rendering")
        .join("\n");
    assert!(holds_vocabulary_bar(&joined));
}
