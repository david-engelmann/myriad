//! Tests for MY-16's worked example, the returns of the
//! chain of the winter atlas, its spring successor, and
//! the summer successor of the spring atlas.
//!
//! The golden returns under examples is the output of the
//! real returns command over the standing lineage file of
//! Round 6, which does not move, and the counts in its
//! lines are hand counts stated in the contract.

mod common;

use common::*;
use myriad::atlas::{load_lineage_file, render_changelog, render_returns};
use std::path::PathBuf;

fn example(name: &str) -> PathBuf {
    repo_root().join("examples").join(name)
}

fn example_text(name: &str) -> String {
    std::fs::read_to_string(example(name)).expect("an example file")
}

#[test]
fn golden_returns_matches_the_command_output() {
    let outcome = run_cli(&[
        "returns",
        &example("winter-spring-summer-lineage.json").to_string_lossy(),
    ]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        example_text("winter-spring-summer-returns.txt")
    );
}

#[test]
fn golden_returns_matches_the_reading_of_the_loaded_chain() {
    let (lineage, defects) = load_lineage_file(&example("winter-spring-summer-lineage.json"));
    assert_eq!(defects, Vec::<String>::new());
    let lineage = lineage.expect("the example lineage loads");
    let lines = render_returns(&lineage).expect("the example chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert_eq!(text, example_text("winter-spring-summer-returns.txt"));
}

#[test]
fn golden_lines_match_the_contract_hand_counts() {
    let golden = example_text("winter-spring-summer-returns.txt");
    assert_eq!(
        golden,
        "entry harbour lights, returned at step 2, absent 1 atlases\n\
         returns counts, returns 1, entries returned 1\n"
    );
}

#[test]
fn each_step_of_the_example_decomposes_into_first_arrivals_and_returns() {
    let (lineage, defects) = load_lineage_file(&example("winter-spring-summer-lineage.json"));
    assert_eq!(defects, Vec::<String>::new());
    let lineage = lineage.expect("the example lineage loads");
    let returns = render_returns(&lineage).expect("the example chain reads");
    for step in 1..lineage.atlases.len() {
        let changelog = render_changelog(&lineage.atlases[step - 1], &lineage.atlases[step])
            .expect("the step reads");
        let closing = changelog.last().expect("a closing line");
        let head = "changelog counts, entries arrived ";
        let rest = closing.strip_prefix(head).expect("the counts head");
        let digits: String = rest.chars().take_while(|ch| ch.is_ascii_digit()).collect();
        let arrived: usize = digits.parse().expect("a count");
        let later = &lineage.atlases[step];
        let earlier = &lineage.atlases[step - 1];
        let first = later
            .entries
            .iter()
            .filter(|entry| {
                !earlier.entries.iter().any(|held| held.label == entry.label)
                    && !lineage.atlases[..step - 1]
                        .iter()
                        .any(|atlas| atlas.entries.iter().any(|held| held.label == entry.label))
            })
            .count();
        let mark = format!(", returned at step {step},");
        let stated = returns.iter().filter(|line| line.contains(&mark)).count();
        assert_eq!(arrived, first + stated, "step {step} decomposes");
    }
    // The hand reading of the decomposition. Step one
    // arrives the ice report, a first arrival, and states
    // no return. Step two arrives the harbour lights, a
    // return, and no first arrival.
    let step_one = render_changelog(&lineage.atlases[0], &lineage.atlases[1]).expect("step one");
    assert!(step_one.contains(&"entry ice report, arrived".to_string()));
    let step_two = render_changelog(&lineage.atlases[1], &lineage.atlases[2]).expect("step two");
    assert!(step_two.contains(&"entry harbour lights, arrived".to_string()));
}

#[test]
fn golden_holds_the_vocabulary_bar() {
    assert!(holds_vocabulary_bar(&example_text(
        "winter-spring-summer-returns.txt"
    )));
}
