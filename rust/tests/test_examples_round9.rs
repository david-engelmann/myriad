//! Tests for MY-20's worked example, the absence of the
//! chain of the winter atlas, its spring successor, and
//! the summer successor of the spring atlas.
//!
//! The golden absence under examples is the output of
//! the real absence command over the standing lineage
//! file of Round 6, which does not move, and the counts
//! in its lines are hand counts stated in the contract.

mod common;

use common::*;
use myriad::atlas::{load_lineage_file, render_absence};
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
fn golden_absence_matches_the_command_output() {
    let outcome = run_cli(&[
        "absence",
        &example("winter-spring-summer-lineage.json").to_string_lossy(),
    ]);
    assert_eq!(outcome.code, 0);
    assert_eq!(
        stdout_text(&outcome),
        example_text("winter-spring-summer-absence.txt")
    );
}

#[test]
fn golden_absence_matches_the_reading_of_the_loaded_chain() {
    let (lineage, defects) = load_lineage_file(&example("winter-spring-summer-lineage.json"));
    assert_eq!(defects, Vec::<String>::new());
    let lineage = lineage.expect("the example lineage loads");
    let lines = render_absence(&lineage).expect("the example chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert_eq!(text, example_text("winter-spring-summer-absence.txt"));
}

#[test]
fn golden_lines_match_the_contract_hand_counts() {
    let golden = example_text("winter-spring-summer-absence.txt");
    assert_eq!(
        golden,
        "entry harbour lights, absent from atlas 2, for 1 atlases\n\
         entry ice report, absent from atlas 3, for 1 atlases, to the close\n\
         absence counts, absences 2, entries with absences 2, \
         absent at the close 1\n"
    );
}

#[test]
fn the_inside_runs_equal_the_returns_golden_count() {
    let absence = example_text("winter-spring-summer-absence.txt");
    let inside = absence
        .lines()
        .filter(|line| line.starts_with("entry ") && !line.ends_with("to the close"))
        .count();
    let returns = example_text("winter-spring-summer-returns.txt");
    let returns_closing = returns.lines().last().expect("a closing line");
    let stated = figure(returns_closing, "returns counts, returns ", ",");
    assert_eq!(inside, stated);
    assert_eq!(inside, 1);
}

#[test]
fn the_absent_at_the_close_matches_the_standing_golden() {
    let absence = example_text("winter-spring-summer-absence.txt");
    let absence_closing = absence.lines().last().expect("a closing line");
    let absent = figure_at_end(absence_closing, "absent at the close ");
    let standing = example_text("winter-spring-summer-standing.txt");
    let standing_closing = standing.lines().last().expect("a closing line");
    let entries = figure(standing_closing, "entries ", ",");
    let at_close = figure_at_end(standing_closing, "standing at the close ");
    assert_eq!(absent, entries - at_close);
    assert_eq!(absent, 1);
}

#[test]
fn the_span_relation_holds_over_the_goldens() {
    let standing = example_text("winter-spring-summer-standing.txt");
    let absence = example_text("winter-spring-summer-absence.txt");
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
fn golden_holds_the_vocabulary_bar() {
    assert!(holds_vocabulary_bar(&example_text(
        "winter-spring-summer-absence.txt"
    )));
}
