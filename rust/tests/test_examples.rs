//! Pins for the synthetic example under examples,
//! translated from tests/test_examples.py.
//!
//! The winter ferry exploration is the slice demo. It has
//! to load clean, carry tens of branches across all three
//! states, and hold its stored map byte identical to the
//! map the code renders today.

mod common;

use std::fs;
use std::path::PathBuf;

use myriad::explorations::{
    count_by_state, load_exploration_file, render_map, Counts, Exploration,
};

use common::{pt, repo_root};

fn examples() -> PathBuf {
    repo_root().join("examples")
}

fn load_example() -> Exploration {
    let (found, defects) = load_exploration_file(&examples().join("winter-ferry.json"));
    assert_eq!(defects, Vec::<String>::new());
    found.expect("a clean load")
}

#[test]
fn the_example_exploration_loads_clean() {
    let exploration = load_example();
    assert_eq!(
        exploration.question,
        pt("How does a small harbour town keep its ferry link alive through the winter")
    );
}

#[test]
fn the_example_carries_tens_of_branches_across_all_states() {
    let counts = count_by_state(&load_example());
    assert_eq!(
        counts,
        Counts {
            total: 26,
            open: 16,
            taken: 6,
            abandoned: 4,
        }
    );
    assert!((20..=99).contains(&counts.total));
}

#[test]
fn the_example_map_renders_the_whole_tree() {
    let exploration = load_example();
    let lines = render_map(&exploration).expect("a sound rendering");
    let branch_lines = lines
        .iter()
        .filter(|line| {
            exploration
                .branches
                .iter()
                .any(|held| line.contains(&format!("{} (", held.id.to_display_string())))
        })
        .count();
    assert_eq!(branch_lines, 26);
    assert!(lines[0].starts_with("question, "));
    assert_eq!(
        lines[lines.len() - 1],
        "branches 26, open 16, taken 6, abandoned 4"
    );
}

#[test]
fn the_golden_map_is_byte_identical() {
    let exploration = load_example();
    let rendered = format!(
        "{}\n",
        render_map(&exploration)
            .expect("a sound rendering")
            .join("\n")
    )
    .into_bytes();
    let golden = fs::read(examples().join("winter-ferry-map.txt")).expect("the golden map");
    assert_eq!(rendered, golden);
}
