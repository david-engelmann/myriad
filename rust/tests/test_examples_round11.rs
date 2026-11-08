//! Tests for MY-24's worked examples, the golden
//! arrivals under examples.
//!
//! The goldens are the output of the real arrivals
//! command over the two shipped lineage files, the
//! winter, spring, and summer chain of Round 6 and the
//! investigation log chain of MY-21, and the counts in
//! their lines are the hand counts stated in the
//! contract. Both chains hold an atlas at which no
//! entry arrives, the third atlas of each, so each
//! golden carries a section standing as its opening
//! line alone.

mod common;

use common::*;
use myriad::atlas::{load_lineage_file, render_arrivals};
use std::path::PathBuf;

fn example(name: &str) -> PathBuf {
    repo_root().join("examples").join(name)
}

fn example_text(name: &str) -> String {
    std::fs::read_to_string(example(name)).expect("an example file")
}

#[test]
fn golden_arrivals_matches_the_command_output() {
    let outcome = run_cli(&[
        "arrivals",
        &example("winter-spring-summer-lineage.json").to_string_lossy(),
    ]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        example_text("winter-spring-summer-arrivals.txt")
    );
}

#[test]
fn golden_arrivals_matches_the_reading_of_the_loaded_chain() {
    let (lineage, defects) = load_lineage_file(&example("winter-spring-summer-lineage.json"));
    assert_eq!(defects, Vec::<String>::new());
    let lineage = lineage.expect("the example lineage loads");
    let lines = render_arrivals(&lineage).expect("the example chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert_eq!(text, example_text("winter-spring-summer-arrivals.txt"));
}

#[test]
fn golden_arrivals_lines_match_the_contract_hand_counts() {
    let golden = example_text("winter-spring-summer-arrivals.txt");
    assert_eq!(
        golden,
        "atlas 1\n\
         entry ferry link, first stood at atlas 1, last stood at atlas 3, \
         stands in 3 atlases, stands at the close\n\
         entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, \
         stands in 3 atlases, stands at the close\n\
         entry harbour lights, first stood at atlas 1, last stood at atlas 3, \
         stands in 2 atlases, stands at the close\n\
         atlas 2\n\
         entry ice report, first stood at atlas 2, last stood at atlas 2, \
         stands in 1 atlases, does not stand at the close\n\
         atlas 3\n\
         arrivals counts, atlas 1 arrivals 3, atlas 2 arrivals 1, \
         atlas 3 arrivals 0, entries 4, atlases 3\n"
    );
}

#[test]
fn investigation_golden_arrivals_matches_the_command_output() {
    let outcome = run_cli(&[
        "arrivals",
        &example("radio-investigation-lineage.json").to_string_lossy(),
    ]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        example_text("radio-investigation-arrivals.txt")
    );
}

#[test]
fn investigation_golden_arrivals_matches_the_reading() {
    let (lineage, defects) = load_lineage_file(&example("radio-investigation-lineage.json"));
    assert_eq!(defects, Vec::<String>::new());
    let lineage = lineage.expect("the example lineage loads");
    let lines = render_arrivals(&lineage).expect("the example chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert_eq!(text, example_text("radio-investigation-arrivals.txt"));
}

#[test]
fn investigation_golden_lines_match_the_contract_hand_counts() {
    let golden = example_text("radio-investigation-arrivals.txt");
    assert_eq!(
        golden,
        "atlas 1\n\
         entry radio check, first stood at atlas 1, last stood at atlas 3, \
         stands in 3 atlases, stands at the close\n\
         entry battery drain, first stood at atlas 1, last stood at atlas 3, \
         stands in 2 atlases, stands at the close\n\
         atlas 2\n\
         entry aerial icing, first stood at atlas 2, last stood at atlas 2, \
         stands in 1 atlases, does not stand at the close\n\
         entry log gaps, first stood at atlas 2, last stood at atlas 3, \
         stands in 2 atlases, stands at the close\n\
         atlas 3\n\
         arrivals counts, atlas 1 arrivals 2, atlas 2 arrivals 2, \
         atlas 3 arrivals 0, entries 4, atlases 3\n"
    );
}

#[test]
fn the_arrivals_sum_matches_the_standing_golden() {
    for (arrivals_name, standing_name) in [
        (
            "winter-spring-summer-arrivals.txt",
            "winter-spring-summer-standing.txt",
        ),
        (
            "radio-investigation-arrivals.txt",
            "radio-investigation-standing.txt",
        ),
    ] {
        let arrivals = example_text(arrivals_name);
        let closing = arrivals.lines().last().expect("a closing line");
        let sum: usize = closing
            .split(" arrivals ")
            .skip(1)
            .map(|part| {
                part.split(',')
                    .next()
                    .expect("the figure stands in the segment")
                    .parse::<usize>()
                    .expect("a count")
            })
            .sum();
        let standing = example_text(standing_name);
        let standing_closing = standing.lines().last().expect("a closing line");
        let entries: usize = standing_closing
            .split("entries ")
            .nth(1)
            .expect("the entries figure stands in the line")
            .split(',')
            .next()
            .expect("the figure stands in the line")
            .parse()
            .expect("a count");
        assert_eq!(sum, entries, "{arrivals_name}");
        assert_eq!(sum, 4, "{arrivals_name}");
    }
}

#[test]
fn goldens_hold_the_vocabulary_bar() {
    for name in [
        "winter-spring-summer-arrivals.txt",
        "radio-investigation-arrivals.txt",
    ] {
        assert!(holds_vocabulary_bar(&example_text(name)), "{name}");
    }
}
