//! Tests for MY-26, the departures examples.
//!
//! The shipped lineages print exactly their shipped
//! departures goldens under the library reading, and the
//! goldens' figures are the hand counts MY-26 states,
//! read against the shipped absence goldens of the same
//! chains, whose closing runs count the same departures
//! from the other side.

mod common;

use common::*;

use std::collections::BTreeMap;

use myriad::atlas::{load_lineage_text, render_departures, Lineage};

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

const CHAINS: [(&str, &str); 3] = [
    (
        "winter-spring-summer-lineage.json",
        "winter-spring-summer-departures.txt",
    ),
    (
        "radio-investigation-lineage.json",
        "radio-investigation-departures.txt",
    ),
    (
        "constructed-departures-lineage.json",
        "constructed-departures-departures.txt",
    ),
];

#[test]
fn each_shipped_lineage_prints_its_shipped_departures_golden() {
    for (lineage_name, golden_name) in CHAINS {
        let lineage = load(lineage_name);
        let lines = render_departures(&lineage).expect("the departures read");
        assert_eq!(lines, golden(golden_name), "the golden of {lineage_name}");
    }
}

#[test]
fn the_goldens_match_the_hand_counts_of_my26() {
    assert_eq!(
        golden("winter-spring-summer-departures.txt"),
        strings(&[
            "atlas 1",
            "atlas 2",
            "entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "atlas 3",
            "departures counts, atlas 1 departures 0, atlas 2 departures 1, atlas 3 departures 0, entries departed 1, entries 4, atlases 3",
        ])
    );
    assert_eq!(
        golden("radio-investigation-departures.txt"),
        strings(&[
            "atlas 1",
            "atlas 2",
            "entry aerial icing, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "atlas 3",
            "departures counts, atlas 1 departures 0, atlas 2 departures 1, atlas 3 departures 0, entries departed 1, entries 4, atlases 3",
        ])
    );
    assert_eq!(
        golden("constructed-departures-departures.txt"),
        strings(&[
            "atlas 1",
            "entry solo line, first stood at atlas 1, last stood at atlas 1, stands in 1 atlases, does not stand at the close",
            "atlas 2",
            "entry middle line, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, does not stand at the close",
            "atlas 3",
            "departures counts, atlas 1 departures 1, atlas 2 departures 1, atlas 3 departures 0, entries departed 2, entries 4, atlases 3",
        ])
    );
}

#[test]
fn the_departures_agree_with_the_shipped_absence_goldens() {
    for (lineage_name, golden_name, absence_name) in [
        (
            "winter-spring-summer-lineage.json",
            "winter-spring-summer-departures.txt",
            "winter-spring-summer-absence.txt",
        ),
        (
            "radio-investigation-lineage.json",
            "radio-investigation-departures.txt",
            "radio-investigation-absence.txt",
        ),
    ] {
        let _ = load(lineage_name);
        let departures = golden(golden_name);
        let absence = golden(absence_name);
        // The absence closing runs that run to the close,
        // grouped by the atlas they depart after.
        let mut grouped: BTreeMap<usize, usize> = BTreeMap::new();
        for line in &absence {
            if line.contains(" to the close") {
                let start: usize = line
                    .split("absent from atlas ")
                    .nth(1)
                    .expect("a run states its start")
                    .split([' ', ','])
                    .next()
                    .expect("a run states its start")
                    .parse()
                    .expect("a start parses");
                *grouped.entry(start - 1).or_insert(0) += 1;
            }
        }
        let closing = departures.last().expect("a closing line stands");
        let parts: Vec<&str> = closing.split(", ").collect();
        let mut total = 0usize;
        for part in &parts[1..] {
            if let Some(rest) = part.strip_prefix("atlas ") {
                let number: usize = rest
                    .split(' ')
                    .next()
                    .expect("an atlas number stands")
                    .parse()
                    .expect("an atlas number parses");
                let count: usize = rest
                    .rsplit(' ')
                    .next()
                    .expect("a count stands")
                    .parse()
                    .expect("a count parses");
                assert_eq!(
                    count,
                    grouped.get(&number).copied().unwrap_or(0),
                    "atlas {number} departures disagree with the absence golden"
                );
                total += count;
            } else if let Some(rest) = part.strip_prefix("entries departed ") {
                let departed: usize = rest.parse().expect("a count parses");
                assert_eq!(departed, total);
                let absent_at_close: usize = absence
                    .last()
                    .expect("a closing line stands")
                    .rsplit(", ")
                    .next()
                    .expect("a closing figure stands")
                    .trim_start_matches("absent at the close ")
                    .parse()
                    .expect("a figure parses");
                assert_eq!(departed, absent_at_close);
            }
        }
    }
}

#[test]
fn the_departures_goldens_hold_the_vocabulary_bar() {
    for (_, golden_name) in CHAINS {
        let lines = golden(golden_name);
        let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
        assert!(holds_vocabulary_bar(&text));
    }
}
