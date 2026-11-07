//! Tests for MY-21's worked example, the investigation
//! log chain at examples/radio-investigation-lineage.json,
//! three atlases standing as the successive loggings of
//! one branching investigation of why the harbour radio
//! check fails on cold mornings.
//!
//! The three goldens under examples are the output of
//! the real standing, returns, and absence commands over
//! that file, and the counts in their lines are hand
//! counts stated in the contract. The example stands
//! beside the winter, spring, and summer chain, which
//! does not move.

mod common;

use common::*;
use myriad::atlas::{load_lineage_file, render_absence, render_returns, render_standing};
use std::path::PathBuf;

fn example(name: &str) -> PathBuf {
    repo_root().join("examples").join(name)
}

fn example_text(name: &str) -> String {
    std::fs::read_to_string(example(name)).expect("an example file")
}

/// The figure after a head in a line, read back out of
/// the rendered text.
fn figure(line: &str, head: &str, tail: &str) -> usize {
    let rest = line
        .split(head)
        .nth(1)
        .unwrap_or_else(|| panic!("the figure head {head} stands in the line {line}"));
    rest.split(tail)
        .next()
        .expect("the figure stands in the line")
        .parse()
        .expect("a count")
}

/// The figure after a head at the end of a line.
fn figure_at_end(line: &str, head: &str) -> usize {
    line.rsplit(head)
        .next()
        .unwrap_or_else(|| panic!("the figure head {head} stands in the line {line}"))
        .parse()
        .expect("a count")
}

#[test]
fn golden_standing_matches_the_command_output() {
    let outcome = run_cli(&[
        "standing",
        &example("radio-investigation-lineage.json").to_string_lossy(),
    ]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        example_text("radio-investigation-standing.txt")
    );
}

#[test]
fn golden_standing_matches_the_reading_of_the_loaded_chain() {
    let (lineage, defects) = load_lineage_file(&example("radio-investigation-lineage.json"));
    assert_eq!(defects, Vec::<String>::new());
    let lineage = lineage.expect("the example lineage loads");
    let lines = render_standing(&lineage).expect("the example chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert_eq!(text, example_text("radio-investigation-standing.txt"));
}

#[test]
fn golden_standing_lines_match_the_contract_hand_counts() {
    let golden = example_text("radio-investigation-standing.txt");
    assert_eq!(
        golden,
        "entry radio check, first stood at atlas 1, last stood at atlas 3, \
         stands in 3 atlases, stands at the close\n\
         entry battery drain, first stood at atlas 1, last stood at atlas 3, \
         stands in 2 atlases, stands at the close\n\
         entry aerial icing, first stood at atlas 2, last stood at atlas 2, \
         stands in 1 atlases, does not stand at the close\n\
         entry log gaps, first stood at atlas 2, last stood at atlas 3, \
         stands in 2 atlases, stands at the close\n\
         standing counts, atlases 3, entries 4, standing at every atlas 1, \
         standing at the close 3\n"
    );
}

#[test]
fn golden_returns_matches_the_command_output() {
    let outcome = run_cli(&[
        "returns",
        &example("radio-investigation-lineage.json").to_string_lossy(),
    ]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        example_text("radio-investigation-returns.txt")
    );
}

#[test]
fn golden_returns_matches_the_reading_of_the_loaded_chain() {
    let (lineage, defects) = load_lineage_file(&example("radio-investigation-lineage.json"));
    assert_eq!(defects, Vec::<String>::new());
    let lineage = lineage.expect("the example lineage loads");
    let lines = render_returns(&lineage).expect("the example chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert_eq!(text, example_text("radio-investigation-returns.txt"));
}

#[test]
fn golden_returns_lines_match_the_contract_hand_counts() {
    let golden = example_text("radio-investigation-returns.txt");
    assert_eq!(
        golden,
        "entry battery drain, returned at step 2, absent 1 atlases\n\
         returns counts, returns 1, entries returned 1\n"
    );
}

#[test]
fn golden_absence_matches_the_command_output() {
    let outcome = run_cli(&[
        "absence",
        &example("radio-investigation-lineage.json").to_string_lossy(),
    ]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        example_text("radio-investigation-absence.txt")
    );
}

#[test]
fn golden_absence_matches_the_reading_of_the_loaded_chain() {
    let (lineage, defects) = load_lineage_file(&example("radio-investigation-lineage.json"));
    assert_eq!(defects, Vec::<String>::new());
    let lineage = lineage.expect("the example lineage loads");
    let lines = render_absence(&lineage).expect("the example chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert_eq!(text, example_text("radio-investigation-absence.txt"));
}

#[test]
fn golden_absence_lines_match_the_contract_hand_counts() {
    let golden = example_text("radio-investigation-absence.txt");
    assert_eq!(
        golden,
        "entry battery drain, absent from atlas 2, for 1 atlases\n\
         entry aerial icing, absent from atlas 3, for 1 atlases, to the close\n\
         absence counts, absences 2, entries with absences 2, \
         absent at the close 1\n"
    );
}

#[test]
fn the_inside_runs_equal_the_returns_golden_count() {
    let absence = example_text("radio-investigation-absence.txt");
    let inside = absence
        .lines()
        .filter(|line| line.starts_with("entry ") && !line.ends_with("to the close"))
        .count();
    let returns = example_text("radio-investigation-returns.txt");
    let returns_closing = returns.lines().last().expect("a closing line");
    let stated = figure(returns_closing, "returns counts, returns ", ",");
    assert_eq!(inside, stated);
    assert_eq!(inside, 1);
}

#[test]
fn the_absent_at_the_close_matches_the_standing_golden() {
    let absence = example_text("radio-investigation-absence.txt");
    let absence_closing = absence.lines().last().expect("a closing line");
    let absent = figure_at_end(absence_closing, "absent at the close ");
    let standing = example_text("radio-investigation-standing.txt");
    let standing_closing = standing.lines().last().expect("a closing line");
    let entries = figure(standing_closing, "entries ", ",");
    let at_close = figure_at_end(standing_closing, "standing at the close ");
    assert_eq!(absent, entries - at_close);
    assert_eq!(absent, 1);
}

#[test]
fn the_span_relation_holds_over_the_goldens() {
    let standing = example_text("radio-investigation-standing.txt");
    let absence = example_text("radio-investigation-absence.txt");
    for line in standing.lines().filter(|line| line.starts_with("entry ")) {
        let label = line
            .strip_prefix("entry ")
            .expect("the label head")
            .split(',')
            .next()
            .expect("the label");
        let first = figure(line, "first stood at atlas ", ",");
        let last = figure(line, "last stood at atlas ", ",");
        let stood = figure(line, "stands in ", " atlases");
        let inside: usize = absence
            .lines()
            .filter(|run| {
                run.starts_with(&format!("entry {label}, absent from atlas "))
                    && !run.ends_with("to the close")
            })
            .map(|run| figure(run, ", for ", " atlases"))
            .sum();
        assert_eq!(last - first + 1, stood + inside, "label {label}");
    }
}

#[test]
fn goldens_hold_the_vocabulary_bar() {
    for name in [
        "radio-investigation-standing.txt",
        "radio-investigation-returns.txt",
        "radio-investigation-absence.txt",
    ] {
        assert!(holds_vocabulary_bar(&example_text(name)), "{name}");
    }
}
