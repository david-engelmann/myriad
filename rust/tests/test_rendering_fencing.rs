//! Pins for FIX-MY-14, the rendering fencing closure.
//!
//! The pins hand each rendering a value no loader admits,
//! one whose printed field holds a line break, and read
//! the refusal the rendering returns. The pins were run
//! against the pre fix tree by the completion lane,
//! where sixteen of them failed as the renderings
//! printed the field raw, and the results stand
//! recorded in the Round 11 completion report.

mod common;

use common::{branch, pt};
use myriad::atlas::{
    render_atlas, render_changelog, render_index, render_lineage, Atlas, AtlasEntry, Lineage,
};
use myriad::explorations::{render_census, render_claims, render_map, Branch, Exploration};
use myriad::pytext::PText;

const QUESTION_BREAK: &str = "the question holds a newline or a carriage return";
const ID_BREAK: &str = "a branch id holds a newline or a carriage return";
const PARENT_BREAK: &str = "a branch parent holds a newline or a carriage return";
const PROMPT_BREAK: &str = "a branch prompt holds a newline or a carriage return";
const CLAIM_BREAK: &str = "a claim holds a newline or a carriage return";
const FINDING_BREAK: &str = "a finding holds a newline or a carriage return";
const REASON_BREAK: &str = "a reason holds a newline or a carriage return";
const TITLE_BREAK: &str = "the atlas title holds a newline or a carriage return";
const LABEL_BREAK: &str = "an entry label holds a newline or a carriage return";

fn exploration(question: &str, branches: Vec<Branch>) -> Exploration {
    Exploration {
        question: pt(question),
        branches,
    }
}

fn open(id: &str) -> Branch {
    branch(id, None, "p", "open", None, None, None)
}

/// Every rendering of the exploration family refuses the
/// value with the one refusal.
fn refuses_exploration(value: &Exploration, refusal: &str) {
    assert_eq!(render_map(value), Err(refusal.to_string()));
    assert_eq!(render_claims(value), Err(refusal.to_string()));
    assert_eq!(render_census(value), Err(refusal.to_string()));
}

#[test]
fn question_holding_a_line_feed_is_refused() {
    let value = exploration("a\nb", vec![open("a")]);
    refuses_exploration(&value, QUESTION_BREAK);
}

#[test]
fn question_holding_a_carriage_return_is_refused() {
    let value = exploration("a\rb", vec![open("a")]);
    refuses_exploration(&value, QUESTION_BREAK);
}

#[test]
fn branch_id_holding_a_line_break_is_refused() {
    let value = exploration(
        "q",
        vec![branch("a\nb", None, "p", "open", None, None, None)],
    );
    refuses_exploration(&value, ID_BREAK);
}

#[test]
fn branch_parent_holding_a_line_break_is_refused() {
    // No rendering of the exploration family prints a
    // parent, and the pass reads the field the value
    // holds, so the refusal stands in all three.
    let value = exploration(
        "q",
        vec![branch("b", Some("a\nb"), "p", "open", None, None, None)],
    );
    refuses_exploration(&value, PARENT_BREAK);
}

#[test]
fn branch_prompt_holding_a_line_break_is_refused() {
    let value = exploration(
        "q",
        vec![branch("a", None, "p\nq", "open", None, None, None)],
    );
    refuses_exploration(&value, PROMPT_BREAK);
}

#[test]
fn claim_holding_a_line_break_is_refused() {
    // The claim the Round 10 red team forged a counts
    // segment with, against the map.
    let forged = branch(
        "a",
        None,
        "p",
        "open",
        Some("shore\ncounts, claims 9"),
        None,
        None,
    );
    let value = exploration("q", vec![forged]);
    refuses_exploration(&value, CLAIM_BREAK);
}

#[test]
fn finding_holding_a_line_break_is_refused() {
    let value = exploration(
        "q",
        vec![branch("a", None, "p", "taken", None, Some("f\ng"), None)],
    );
    refuses_exploration(&value, FINDING_BREAK);
}

#[test]
fn reason_holding_a_line_break_is_refused() {
    let value = exploration(
        "q",
        vec![branch(
            "a",
            None,
            "p",
            "abandoned",
            None,
            None,
            Some("r\ns"),
        )],
    );
    refuses_exploration(&value, REASON_BREAK);
}

#[test]
fn the_question_stands_read_before_the_branches() {
    // A surrogate in the question and a break in a claim
    // draw the question refusal, because the pass reads
    // the question first, each field for a surrogate
    // before a break.
    let mut value = exploration(
        "q",
        vec![branch("a", None, "p", "open", Some("x\ny"), None, None)],
    );
    value.question = PText(vec![0x71, 0xD800]);
    refuses_exploration(&value, "the question holds an unpaired surrogate");
}

#[test]
fn branches_stand_read_in_the_order_they_stand() {
    // A break in the claim of the first branch and a
    // surrogate in the finding of the second draw the
    // first branch refusal.
    let mut value = exploration(
        "q",
        vec![
            branch("a", None, "p", "open", Some("x\ny"), None, None),
            branch("b", None, "p", "taken", None, None, None),
        ],
    );
    value.branches[1].finding = Some(PText(vec![0x66, 0xD800]));
    refuses_exploration(&value, CLAIM_BREAK);
    // A surrogate in the prompt of the first branch and a
    // break in the id of the second draw the first branch
    // refusal.
    let mut value = exploration(
        "q",
        vec![
            branch("a", None, "p", "open", None, None, None),
            branch("b\nc", None, "p", "open", None, None, None),
        ],
    );
    value.branches[0].prompt = PText(vec![0x70, 0xD800]);
    refuses_exploration(&value, "a branch prompt holds an unpaired surrogate");
}

#[test]
fn a_clean_exploration_renders_whole() {
    let value = exploration(
        "q",
        vec![
            branch("a", None, "p", "taken", Some("north"), Some("f"), None),
            branch("b", Some("a"), "p", "abandoned", None, None, Some("r")),
        ],
    );
    assert!(render_map(&value).is_ok());
    assert!(render_claims(&value).is_ok());
    assert!(render_census(&value).is_ok());
}

fn atlas(title: &str, entries: Vec<(&str, Exploration)>) -> Atlas {
    Atlas {
        title: pt(title),
        entries: entries
            .into_iter()
            .map(|(label, exploration)| AtlasEntry {
                label: pt(label),
                exploration,
            })
            .collect(),
    }
}

fn plain() -> Exploration {
    exploration("q", vec![open("a")])
}

/// Every rendering of the atlas family refuses the value
/// with the one refusal, the lineage through the
/// changelog it composes.
fn refuses_atlas(earlier: &Atlas, later: &Atlas, refusal: &str) {
    assert_eq!(render_atlas(earlier), Err(refusal.to_string()));
    assert_eq!(render_index(earlier), Err(refusal.to_string()));
    assert_eq!(render_changelog(earlier, later), Err(refusal.to_string()));
    let chain = Lineage {
        atlases: vec![earlier.clone(), later.clone()],
    };
    assert_eq!(render_lineage(&chain), Err(refusal.to_string()));
}

#[test]
fn title_holding_a_line_break_is_refused() {
    let earlier = atlas("a\nb", vec![("one", plain())]);
    let later = atlas("t", vec![("one", plain())]);
    refuses_atlas(&earlier, &later, TITLE_BREAK);
    // A break in the later title alone draws the same
    // refusal from the changelog and the lineage.
    let earlier = atlas("t", vec![("one", plain())]);
    let later = atlas("a\nb", vec![("one", plain())]);
    assert_eq!(
        render_changelog(&earlier, &later),
        Err(TITLE_BREAK.to_string())
    );
    let chain = Lineage {
        atlases: vec![earlier, later],
    };
    assert_eq!(render_lineage(&chain), Err(TITLE_BREAK.to_string()));
}

#[test]
fn entry_label_holding_a_line_break_is_refused() {
    // The label the Round 10 red team split the entry
    // lines of the atlas, index, and changelog readings
    // with, and of the lineage through the changelog.
    let earlier = atlas("t", vec![("a\nb", plain())]);
    let later = atlas("t", vec![("a\nb", plain())]);
    refuses_atlas(&earlier, &later, LABEL_BREAK);
}

#[test]
fn entry_question_holding_a_line_break_is_refused() {
    let broken = exploration("a\nb", vec![open("a")]);
    let earlier = atlas("t", vec![("one", broken.clone())]);
    let later = atlas("t", vec![("one", broken)]);
    refuses_atlas(&earlier, &later, QUESTION_BREAK);
}

#[test]
fn entry_claim_holding_a_line_break_is_refused() {
    let broken = exploration(
        "q",
        vec![branch("a", None, "p", "open", Some("x\ny"), None, None)],
    );
    let earlier = atlas("t", vec![("one", broken.clone())]);
    let later = atlas("t", vec![("one", broken)]);
    refuses_atlas(&earlier, &later, CLAIM_BREAK);
}

#[test]
fn entry_parent_holding_a_line_break_is_refused_by_the_changelog() {
    // The changelog prints a parent on its parent changed
    // line, and the atlas pass reads the field the value
    // holds, so the atlas readings refuse it as well.
    let broken = exploration(
        "q",
        vec![branch("b", Some("a\nb"), "p", "open", None, None, None)],
    );
    let earlier = atlas("t", vec![("one", broken.clone())]);
    let later = atlas("t", vec![("one", broken)]);
    refuses_atlas(&earlier, &later, PARENT_BREAK);
}

#[test]
fn entry_prompt_holding_a_line_break_is_refused_by_the_changelog() {
    let broken = exploration(
        "q",
        vec![branch("a", None, "p\nq", "open", None, None, None)],
    );
    let earlier = atlas("t", vec![("one", broken.clone())]);
    let later = atlas("t", vec![("one", broken)]);
    assert_eq!(
        render_changelog(&earlier, &later),
        Err(PROMPT_BREAK.to_string())
    );
    let chain = Lineage {
        atlases: vec![earlier.clone(), later],
    };
    assert_eq!(render_lineage(&chain), Err(PROMPT_BREAK.to_string()));
    assert_eq!(render_atlas(&earlier), Err(PROMPT_BREAK.to_string()));
}

#[test]
fn the_earlier_atlas_stands_read_before_the_later() {
    // A break in an earlier entry label and a break in
    // the later title draw the earlier refusal, because
    // the changelog reads its pass over the earlier atlas
    // and then over the later.
    let earlier = atlas("t", vec![("a\nb", plain())]);
    let later = atlas("a\nb", vec![("one", plain())]);
    assert_eq!(
        render_changelog(&earlier, &later),
        Err(LABEL_BREAK.to_string())
    );
    // A break standing only in a later entry label draws
    // the label refusal.
    let earlier = atlas("t", vec![("one", plain())]);
    let later = atlas("t", vec![("a\nb", plain())]);
    assert_eq!(
        render_changelog(&earlier, &later),
        Err(LABEL_BREAK.to_string())
    );
}

#[test]
fn a_clean_atlas_renders_whole() {
    let earlier = atlas("t", vec![("one", plain())]);
    let later = atlas("t", vec![("one", plain()), ("two", plain())]);
    assert!(render_atlas(&earlier).is_ok());
    assert!(render_index(&earlier).is_ok());
    assert!(render_changelog(&earlier, &later).is_ok());
    let chain = Lineage {
        atlases: vec![earlier, later],
    };
    assert!(render_lineage(&chain).is_ok());
}
