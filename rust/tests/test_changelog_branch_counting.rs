//! Tests for FIX-MY-20, the changelog branch counting
//! repair.
//!
//! The pins here run the changelog and lineage readings
//! over hand built values whose explorations hold one
//! branch id twice, the shapes the red team delta review
//! of Round 13 executed against changelog_branches. No
//! file can carry the shape, because the exploration
//! loader refuses a branch id carried by two branches
//! under the defect that names the duplicate branch id,
//! so every fixture stands hand built on the library
//! surface alone, and the pins assert the read once form
//! MY-11 states and FIX-MY-20 settles, one branch read
//! twice under one id and the changed count counting
//! each branch the reading names inside a matched entry
//! once.
//!
//! The discrimination split of these pins stands as
//! follows, pin by pin. Pins 21 to 25 fail on the tree
//! before this repair by assertion, over the shapes the
//! delta review executed. Pin 21, shape (a), an id
//! standing twice in the earlier exploration, each
//! standing differing in claim from the later standing:
//! two claim lines stand there and the changed figure
//! counts the id twice. Pin 22, shape (b), an id
//! standing twice in the later exploration, its first
//! standing identical to the earlier and its second
//! differing: the pairing stands at the last standing
//! there and the claim line stands. Pin 23, shape (c),
//! an arrived id standing twice in the later
//! exploration: two arrived lines stand there and the
//! figure counts it twice. Pin 24, shape (c) on its
//! other side, a departed id standing twice in the
//! earlier exploration: two departed lines stand there
//! and the figure counts it twice. Pin 25, the lineage
//! sums over a chain whose first step carries shape
//! (a): the lineage counts line states the branches
//! changed figure one higher there for the one
//! duplicated id. Pins 26 and 27 pass on the tree
//! before this repair by design. Pin 26 holds the
//! refusal posture over skipped standings, which the
//! atlas pass reads from every raw branch before any
//! walk on both trees, and it fails the moment a repair
//! narrows the pass or routes the refusals through the
//! walk alone. Pin 27 holds the no golden moves promise
//! of the amendment over unique ids, the changelog form
//! standing exactly as the unrepaired tree renders it,
//! and it fails if the repair moves any loader-shaped
//! value.

mod common;

use common::*;
use myriad::atlas::{render_changelog, render_lineage, Atlas, AtlasEntry, Lineage};
use myriad::explorations::{Branch, Exploration};
use myriad::pytext::PText;

fn atlas(title: &str, branches: Vec<Branch>) -> Atlas {
    Atlas {
        title: pt(title),
        entries: vec![AtlasEntry {
            label: pt("only"),
            exploration: Exploration {
                question: pt("q"),
                branches,
            },
        }],
    }
}

fn open(id: &str, claim: Option<&str>) -> Branch {
    branch(id, None, "p", "open", claim, None, None)
}

#[test]
fn an_earlier_id_standing_twice_is_read_at_its_first_standing() {
    let earlier = atlas("One", vec![open("b", Some("x")), open("b", Some("y"))]);
    let later = atlas("Two", vec![open("b", Some("z"))]);
    let lines = render_changelog(&earlier, &later).expect("the pair reads");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry only",
            "  branch b, claim changed, x to z",
            "changelog counts, entries arrived 0, departed 0, matched 1, branches changed 1",
        ])
    );
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.contains("claim changed"))
            .count(),
        1,
        "one claim line stands for the one id"
    );
}

#[test]
fn a_later_id_standing_twice_pairs_at_its_first_standing() {
    let earlier = atlas("One", vec![open("b", Some("x"))]);
    let later = atlas("Two", vec![open("b", Some("x")), open("b", Some("y"))]);
    let lines = render_changelog(&earlier, &later).expect("the pair reads");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry only, unchanged",
            "changelog counts, entries arrived 0, departed 0, matched 1, branches changed 0",
        ])
    );
}

#[test]
fn an_arrived_id_standing_twice_stands_once() {
    let earlier = atlas("One", vec![open("keep", None)]);
    let later = atlas(
        "Two",
        vec![open("keep", None), open("new", None), open("new", None)],
    );
    let lines = render_changelog(&earlier, &later).expect("the pair reads");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry only",
            "  branch new, arrived",
            "changelog counts, entries arrived 0, departed 0, matched 1, branches changed 1",
        ])
    );
}

#[test]
fn a_departed_id_standing_twice_stands_once() {
    let earlier = atlas(
        "One",
        vec![open("keep", None), open("gone", None), open("gone", None)],
    );
    let later = atlas("Two", vec![open("keep", None)]);
    let lines = render_changelog(&earlier, &later).expect("the pair reads");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry only",
            "  branch gone, departed",
            "changelog counts, entries arrived 0, departed 0, matched 1, branches changed 1",
        ])
    );
}

#[test]
fn the_lineage_sums_read_the_branch_figure_once() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![open("b", Some("x")), open("b", Some("y"))]),
            atlas("Two", vec![open("b", Some("z"))]),
            atlas("Three", vec![open("b", Some("z"))]),
        ],
    };
    let lines = render_lineage(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry only",
            "  branch b, claim changed, x to z",
            "changelog counts, entries arrived 0, departed 0, matched 1, branches changed 1",
            "earlier atlas, Two",
            "later atlas, Three",
            "entry only, unchanged",
            "changelog counts, entries arrived 0, departed 0, matched 1, branches changed 0",
            "lineage counts, steps 2, entries arrived 0, departed 0, matched 2, branches changed 1",
        ])
    );
}

#[test]
fn a_skipped_standings_fields_still_draw_their_refusals() {
    // A later standing of a duplicated id whose prompt
    // holds a line break draws the prompt's line break
    // refusal from the atlas pass, on the unrepaired
    // tree as on the repaired, because the pass reads
    // every raw branch before any walk on both.
    let earlier = atlas("One", vec![open("b", Some("x"))]);
    let later = atlas(
        "Two",
        vec![
            open("b", Some("x")),
            branch("b", None, "bad\nprompt", "open", Some("x"), None, None),
        ],
    );
    assert_eq!(
        render_changelog(&earlier, &later).expect_err("the broken prompt is refused"),
        "a branch prompt holds a newline or a carriage return"
    );
    // A later standing whose branch id holds an
    // unpaired surrogate draws the branch id refusal
    // from the same pass.
    let broken = Branch {
        id: PText(vec![0x62, 0xD800]),
        parent: None,
        prompt: pt("p"),
        state: myriad::explorations::State::Open,
        claim: None,
        finding: None,
        reason: None,
    };
    let later = atlas("Two", vec![open("b", Some("x")), broken]);
    assert_eq!(
        render_changelog(&earlier, &later).expect_err("the broken id is refused"),
        "a branch id holds an unpaired surrogate"
    );
}

#[test]
fn unique_ids_render_as_they_render_today() {
    let earlier = atlas("One", vec![open("stay", Some("x")), open("gone", None)]);
    let later = atlas("Two", vec![open("stay", Some("z")), open("new", None)]);
    let lines = render_changelog(&earlier, &later).expect("the pair reads");
    assert_eq!(
        lines,
        strings(&[
            "earlier atlas, One",
            "later atlas, Two",
            "entry only",
            "  branch stay, claim changed, x to z",
            "  branch gone, departed",
            "  branch new, arrived",
            "changelog counts, entries arrived 0, departed 0, matched 1, branches changed 3",
        ])
    );
}
