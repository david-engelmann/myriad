//! Tests for MY-18's worked example, the standing of the
//! chain of the winter atlas, its spring successor, and
//! the summer successor of the spring atlas.
//!
//! The golden standing under examples is the output of
//! the real standing command over the standing lineage
//! file of Round 6, which does not move, and the counts
//! in its lines are hand counts stated in the contract.

mod common;

use common::*;
use myriad::atlas::{load_lineage_file, render_index, render_standing};
use std::path::PathBuf;

fn example(name: &str) -> PathBuf {
    repo_root().join("examples").join(name)
}

fn example_text(name: &str) -> String {
    std::fs::read_to_string(example(name)).expect("an example file")
}

#[test]
fn golden_standing_matches_the_command_output() {
    let outcome = run_cli(&[
        "standing",
        &example("winter-spring-summer-lineage.json").to_string_lossy(),
    ]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        example_text("winter-spring-summer-standing.txt")
    );
}

#[test]
fn golden_standing_matches_the_reading_of_the_loaded_chain() {
    let (lineage, defects) = load_lineage_file(&example("winter-spring-summer-lineage.json"));
    assert_eq!(defects, Vec::<String>::new());
    let lineage = lineage.expect("the example lineage loads");
    let lines = render_standing(&lineage).expect("the example chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert_eq!(text, example_text("winter-spring-summer-standing.txt"));
}

#[test]
fn golden_lines_match_the_contract_hand_counts() {
    let golden = example_text("winter-spring-summer-standing.txt");
    assert_eq!(
        golden,
        "entry ferry link, first stood at atlas 1, last stood at atlas 3, \
         stands in 3 atlases, stands at the close\n\
         entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, \
         stands in 3 atlases, stands at the close\n\
         entry harbour lights, first stood at atlas 1, last stood at atlas 3, \
         stands in 2 atlases, stands at the close\n\
         entry ice report, first stood at atlas 2, last stood at atlas 2, \
         stands in 1 atlases, does not stand at the close\n\
         standing counts, atlases 3, entries 4, \
         standing at every atlas 2, standing at the close 3\n"
    );
}

#[test]
fn the_example_close_count_equals_the_summer_index_count() {
    let (lineage, defects) = load_lineage_file(&example("winter-spring-summer-lineage.json"));
    assert_eq!(defects, Vec::<String>::new());
    let lineage = lineage.expect("the example lineage loads");
    let lines = render_standing(&lineage).expect("the example chain reads");
    let closing = lines.last().expect("a closing line");
    let mark = "standing at the close ";
    let rest = closing.rsplit(mark).next().expect("the close figure");
    let stated: usize = rest.parse().expect("a count");
    let summer = lineage.atlases.last().expect("a last atlas");
    let index = render_index(summer).expect("the summer atlas reads");
    let index_closing = index.last().expect("a closing line");
    let head = "atlas counts, entries ";
    let index_rest = index_closing.strip_prefix(head).expect("the counts head");
    let digits: String = index_rest
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect();
    let counted: usize = digits.parse().expect("a count");
    assert_eq!(stated, counted);
    assert_eq!(stated, 3);
}

#[test]
fn golden_holds_the_vocabulary_bar() {
    assert!(holds_vocabulary_bar(&example_text(
        "winter-spring-summer-standing.txt"
    )));
}
