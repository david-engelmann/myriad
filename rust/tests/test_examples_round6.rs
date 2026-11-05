//! Tests for MY-14's worked example, the chain of the
//! winter atlas, its spring successor, and the summer
//! successor of the spring atlas.
//!
//! The golden lineage under examples is the output of the
//! real command over the lineage file, its first step
//! stands line for line as the golden changelog of Round
//! 5 stands, and the counts in its closing lines are hand
//! counts stated in the contract.

mod common;

use common::*;
use myriad::atlas::{load_atlas_file, load_lineage_file, render_lineage};
use std::path::PathBuf;

fn example(name: &str) -> PathBuf {
    repo_root().join("examples").join(name)
}

fn example_text(name: &str) -> String {
    std::fs::read_to_string(example(name)).expect("an example file")
}

#[test]
fn golden_lineage_matches_the_command_output() {
    let outcome = run_cli(&[
        "lineage",
        &example("winter-spring-summer-lineage.json").to_string_lossy(),
    ]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        example_text("winter-spring-summer-lineage.txt")
    );
}

#[test]
fn golden_lineage_matches_the_reading_of_the_loaded_chain() {
    let (lineage, defects) = load_lineage_file(&example("winter-spring-summer-lineage.json"));
    assert_eq!(defects, Vec::<String>::new());
    let lineage = lineage.expect("the example lineage loads");
    let lines = render_lineage(&lineage).expect("the example chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert_eq!(text, example_text("winter-spring-summer-lineage.txt"));
}

#[test]
fn the_embedded_atlases_are_the_example_atlases() {
    let (lineage, defects) = load_lineage_file(&example("winter-spring-summer-lineage.json"));
    assert_eq!(defects, Vec::<String>::new());
    let lineage = lineage.expect("the example lineage loads");
    for (index, name) in [
        "winter-atlas.json",
        "spring-atlas.json",
        "summer-atlas.json",
    ]
    .iter()
    .enumerate()
    {
        let (atlas, atlas_defects) = load_atlas_file(&example(name));
        assert_eq!(atlas_defects, Vec::<String>::new());
        assert_eq!(lineage.atlases[index], atlas.expect("an example atlas"));
    }
}

#[test]
fn the_first_step_of_the_golden_is_the_golden_changelog() {
    let golden = example_text("winter-spring-summer-lineage.txt");
    let changelog = example_text("winter-spring-changelog.txt");
    assert!(golden.starts_with(&changelog));
}

#[test]
fn the_second_step_of_the_golden_is_the_changelog_of_spring_and_summer() {
    let outcome = run_cli(&[
        "changelog",
        &example("spring-atlas.json").to_string_lossy(),
        &example("summer-atlas.json").to_string_lossy(),
    ]);
    assert_eq!(outcome.code, 0);
    let golden = example_text("winter-spring-summer-lineage.txt");
    let changelog = example_text("winter-spring-changelog.txt");
    let rest = &golden[changelog.len()..];
    let closing =
        "lineage counts, steps 2, entries arrived 2, departed 2, matched 4, branches changed 10\n";
    assert!(rest.ends_with(closing));
    assert_eq!(&rest[..rest.len() - closing.len()], stdout_text(&outcome));
}

#[test]
fn golden_closing_counts_match_the_contract_hand_counts() {
    let golden = example_text("winter-spring-summer-lineage.txt");
    let lines: Vec<&str> = golden.lines().collect();
    assert_eq!(
        lines.last().expect("a closing line"),
        &"lineage counts, steps 2, entries arrived 2, departed 2, matched 4, branches changed 10"
    );
    assert!(lines.contains(
        &"changelog counts, entries arrived 1, departed 1, matched 2, branches changed 2"
    ));
}

#[test]
fn the_departed_entry_returns_in_the_golden() {
    let golden = example_text("winter-spring-summer-lineage.txt");
    let lines: Vec<&str> = golden.lines().collect();
    let departed_at = lines
        .iter()
        .position(|line| *line == "entry harbour lights, departed")
        .expect("the departure stands in the golden");
    let arrived_at = lines
        .iter()
        .position(|line| *line == "entry harbour lights, arrived")
        .expect("the return stands in the golden");
    assert!(departed_at < arrived_at);
    assert!(!lines.contains(&"entry harbour lights"));
    assert!(!lines.contains(&"entry harbour lights, unchanged"));
}

#[test]
fn summer_carries_the_returned_entry_and_spring_does_not() {
    let (spring, defects) = load_atlas_file(&example("spring-atlas.json"));
    assert_eq!(defects, Vec::<String>::new());
    let spring = spring.expect("the spring atlas loads");
    assert!(!spring
        .entries
        .iter()
        .any(|entry| entry.label.eq_str("harbour lights")));
    let (summer, defects) = load_atlas_file(&example("summer-atlas.json"));
    assert_eq!(defects, Vec::<String>::new());
    let summer = summer.expect("the summer atlas loads");
    assert!(summer
        .entries
        .iter()
        .any(|entry| entry.label.eq_str("harbour lights")));
}

#[test]
fn golden_holds_the_vocabulary_bar() {
    assert!(holds_vocabulary_bar(&example_text(
        "winter-spring-summer-lineage.txt"
    )));
}
