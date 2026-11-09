//! Tests for MY-28, the matched examples.
//!
//! The shipped lineages print exactly their shipped
//! matched goldens under the library reading, and each
//! golden's lines are the hand counts MY-28 states,
//! counted from the atlas files themselves. Each pin
//! covers one chain, the winter chain, the radio chain,
//! and the constructed chain.
//!
//! The discrimination split of these pins stands as
//! follows. All three pins fail on the tree before this
//! slice twice over, by compile absence, because
//! render_matched stands nowhere on that tree, and by
//! execution, because the matched goldens stand nowhere
//! on that tree either. No pin here passes there by
//! design.

mod common;

use common::*;

use myriad::atlas::{load_lineage_text, render_matched, Lineage};

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
fn the_winter_chain_prints_its_matched_golden() {
    let lineage = load("winter-spring-summer-lineage.json");
    let lines = render_matched(&lineage).expect("the matched reads");
    assert_eq!(lines, golden("winter-spring-summer-matched.txt"));
    assert_eq!(
        golden("winter-spring-summer-matched.txt"),
        strings(&[
            "step 1",
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "step 2",
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "matched counts, step 1 matched 2, step 2 matched 2, entries matched 2, entries 4, steps 2",
        ])
    );
}

#[test]
fn the_radio_chain_prints_its_matched_golden() {
    let lineage = load("radio-investigation-lineage.json");
    let lines = render_matched(&lineage).expect("the matched reads");
    assert_eq!(lines, golden("radio-investigation-matched.txt"));
    assert_eq!(
        golden("radio-investigation-matched.txt"),
        strings(&[
            "step 1",
            "entry radio check, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "step 2",
            "entry radio check, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry log gaps, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "matched counts, step 1 matched 1, step 2 matched 2, entries matched 2, entries 4, steps 2",
        ])
    );
}

#[test]
fn the_constructed_chain_prints_its_matched_golden() {
    let lineage = load("constructed-departures-lineage.json");
    let lines = render_matched(&lineage).expect("the matched reads");
    assert_eq!(lines, golden("constructed-departures-matched.txt"));
    assert_eq!(
        golden("constructed-departures-matched.txt"),
        strings(&[
            "step 1",
            "entry middle line, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, does not stand at the close",
            "entry keeper line, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "step 2",
            "entry keeper line, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry late line, first stood at atlas 2, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "matched counts, step 1 matched 2, step 2 matched 2, entries matched 3, entries 4, steps 2",
        ])
    );
}
