//! Tests for MY-30, the runs examples.
//!
//! The shipped lineages print exactly their shipped runs
//! goldens under the library reading, and each golden's
//! lines are the hand counts MY-30 states, counted from
//! the atlas files themselves. Each pin covers one chain,
//! the winter chain, the radio chain, and the constructed
//! chain.
//!
//! The discrimination split of these pins stands as
//! follows. All three pins fail on the tree before this
//! slice twice over, by compile absence, because
//! render_runs stands nowhere on that tree, and by
//! execution, because the runs goldens stand nowhere on
//! that tree either. No pin here passes there by design.

mod common;

use common::*;

use myriad::atlas::{load_lineage_text, render_runs, Lineage};

fn load(name: &str) -> Lineage {
    let path = repo_root().join("examples").join(name);
    let text = std::fs::read_to_string(&path).expect("a shipped example reads");
    let (lineage, defects) = load_lineage_text(&text);
    assert!(
        defects.is_empty(),
        "a shipped lineage loads clean: {defects:?}"
    );
    lineage.expect("a shipped lineage value stands")
}

fn golden(name: &str) -> Vec<String> {
    let path = repo_root().join("examples").join(name);
    let text = std::fs::read_to_string(&path).expect("a shipped golden reads");
    text.lines().map(str::to_string).collect()
}

#[test]
fn the_winter_chain_prints_its_runs_golden() {
    let lineage = load("winter-spring-summer-lineage.json");
    let lines = render_runs(&lineage).expect("the runs read");
    assert_eq!(lines, golden("winter-spring-summer-runs.txt"));
    assert_eq!(
        golden("winter-spring-summer-runs.txt"),
        strings(&[
            "entry ferry link, standing from atlas 1, for 3 atlases, to the close",
            "entry lifeboat cover, standing from atlas 1, for 3 atlases, to the close",
            "entry harbour lights, standing from atlas 1, for 1 atlases",
            "entry harbour lights, standing from atlas 3, for 1 atlases, to the close",
            "entry ice report, standing from atlas 2, for 1 atlases",
            "runs counts, runs 5, entries with runs 4, standing at the close 3",
        ])
    );
}

#[test]
fn the_radio_chain_prints_its_runs_golden() {
    let lineage = load("radio-investigation-lineage.json");
    let lines = render_runs(&lineage).expect("the runs read");
    assert_eq!(lines, golden("radio-investigation-runs.txt"));
    assert_eq!(
        golden("radio-investigation-runs.txt"),
        strings(&[
            "entry radio check, standing from atlas 1, for 3 atlases, to the close",
            "entry battery drain, standing from atlas 1, for 1 atlases",
            "entry battery drain, standing from atlas 3, for 1 atlases, to the close",
            "entry aerial icing, standing from atlas 2, for 1 atlases",
            "entry log gaps, standing from atlas 2, for 2 atlases, to the close",
            "runs counts, runs 5, entries with runs 4, standing at the close 3",
        ])
    );
}

#[test]
fn the_constructed_chain_prints_its_runs_golden() {
    let lineage = load("constructed-departures-lineage.json");
    let lines = render_runs(&lineage).expect("the runs read");
    assert_eq!(lines, golden("constructed-departures-runs.txt"));
    assert_eq!(
        golden("constructed-departures-runs.txt"),
        strings(&[
            "entry solo line, standing from atlas 1, for 1 atlases",
            "entry middle line, standing from atlas 1, for 2 atlases",
            "entry keeper line, standing from atlas 1, for 3 atlases, to the close",
            "entry late line, standing from atlas 2, for 2 atlases, to the close",
            "runs counts, runs 4, entries with runs 4, standing at the close 2",
        ])
    );
}
