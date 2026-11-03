//! Pins for the Round 2 example under examples, translated
//! from tests/test_examples_round2.py.
//!
//! The claimed winter ferry exploration is the Round 2
//! demo. It is the Round 1 exploration with claims grown
//! on some branches, held in a file of its own so the Round
//! 1 example and its golden map stand untouched, and its
//! golden map, claims listing, and census have to match
//! the renderings the code produces today byte for byte.

mod common;

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use myriad::explorations::{
    load_exploration_file, render_census, render_claims, render_map, Exploration,
};
use myriad::json::{parse_json, JsonValue};

use common::repo_root;

fn examples() -> PathBuf {
    repo_root().join("examples")
}

fn load_claims_example() -> Exploration {
    let (found, defects) = load_exploration_file(&examples().join("winter-ferry-claims.json"));
    assert_eq!(defects, Vec::<String>::new());
    found.expect("a clean load")
}

fn rendered_bytes(lines: &[String]) -> Vec<u8> {
    format!("{}\n", lines.join("\n")).into_bytes()
}

/// The claims example value with every claim field removed,
/// the shape the Python pin builds before its comparison.
fn stripped(value: &JsonValue) -> JsonValue {
    match value {
        JsonValue::Obj(_) => {
            let question = value.get("question").expect("a question").clone();
            let branches = match value.get("branches").expect("branches") {
                JsonValue::Arr(items) => JsonValue::Arr(
                    items
                        .iter()
                        .map(|item| match item {
                            JsonValue::Obj(fields) => JsonValue::Obj(
                                fields
                                    .iter()
                                    .filter(|(key, _)| !key.eq_str("claim"))
                                    .cloned()
                                    .collect(),
                            ),
                            other => other.clone(),
                        })
                        .collect(),
                ),
                other => other.clone(),
            };
            JsonValue::Obj(vec![
                (common::pt("question"), question),
                (common::pt("branches"), branches),
            ])
        }
        other => other.clone(),
    }
}

#[test]
fn the_claims_example_loads_clean_with_ten_claimed_branches() {
    let exploration = load_claims_example();
    let claimed: Vec<_> = exploration
        .branches
        .iter()
        .filter(|held| held.claim.is_some())
        .collect();
    assert_eq!(exploration.branches.len(), 26);
    assert_eq!(claimed.len(), 10);
    let labels: BTreeSet<String> = claimed
        .iter()
        .map(|held| held.claim.as_ref().expect("a claim").to_display_string())
        .collect();
    assert_eq!(
        labels,
        BTreeSet::from([
            "boat crew".to_string(),
            "harbour office".to_string(),
            "tide watch".to_string(),
        ])
    );
}

#[test]
fn the_claims_example_is_the_round_1_exploration_plus_claims() {
    let round_1_text =
        fs::read_to_string(examples().join("winter-ferry.json")).expect("the Round 1 example");
    let grown_text = fs::read_to_string(examples().join("winter-ferry-claims.json"))
        .expect("the claims example");
    let round_1 = parse_json(&round_1_text).expect("valid JSON");
    let grown = parse_json(&grown_text).expect("valid JSON");
    assert_eq!(stripped(&grown), round_1);
}

#[test]
fn the_claims_golden_map_is_byte_identical() {
    let exploration = load_claims_example();
    let golden = fs::read(examples().join("winter-ferry-claims-map.txt")).expect("the golden");
    assert_eq!(
        rendered_bytes(&render_map(&exploration).expect("a sound rendering")),
        golden
    );
}

#[test]
fn the_claims_golden_listing_is_byte_identical() {
    let exploration = load_claims_example();
    let golden = fs::read(examples().join("winter-ferry-claims-listing.txt")).expect("the golden");
    assert_eq!(
        rendered_bytes(&render_claims(&exploration).expect("a sound rendering")),
        golden
    );
}

#[test]
fn the_claims_golden_census_is_byte_identical() {
    let exploration = load_claims_example();
    let golden = fs::read(examples().join("winter-ferry-claims-census.txt")).expect("the golden");
    assert_eq!(
        rendered_bytes(&render_census(&exploration).expect("a sound rendering")),
        golden
    );
}

#[test]
fn the_round_1_example_lists_as_wholly_unclaimed() {
    let (found, defects) = load_exploration_file(&examples().join("winter-ferry.json"));
    assert_eq!(defects, Vec::<String>::new());
    let exploration = found.expect("a clean load");
    assert_eq!(
        render_claims(&exploration).expect("a sound rendering"),
        common::strings(&[
            "question, How does a small harbour town keep its ferry link alive through the winter",
            "unclaimed, branches 26, open 16, taken 6, abandoned 4",
        ])
    );
}
