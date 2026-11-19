//! Tests for MY-19 and MY-20, the absence reading.
//!
//! The reading takes a lineage loaded whole through the
//! MY-13 loader and reads the presence of every label in
//! every atlas of the chain in one pass. One line stands
//! for each absence run of the chain in the order the
//! labels first stand across the chain, an inside run
//! ending at a standing and a closing run closing with
//! the words to the close. Refusals of hand-built values
//! follow MY-13 and PORT-11.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_absence, render_returns, render_standing, Atlas, AtlasEntry, Lineage,
};
use myriad::explorations::{Branch, Exploration};
use myriad::pytext::PText;

fn atlas(title: &str, entries: Vec<AtlasEntry>) -> Atlas {
    Atlas {
        title: pt(title),
        entries,
    }
}

fn entry(label: &str, question: &str, branches: Vec<Branch>) -> AtlasEntry {
    AtlasEntry {
        label: pt(label),
        exploration: Exploration {
            question: pt(question),
            branches,
        },
    }
}

fn plain(label: &str) -> AtlasEntry {
    entry(label, "Q", vec![])
}

fn bad() -> PText {
    PText(vec![0x61, 0xD800])
}

fn atlas_json(title: &str, entries: &[String]) -> String {
    jobj(&[("title", jstr(title)), ("entries", jarr(entries))])
}

fn entry_json(label: &str, question: &str, branches: &[String]) -> String {
    jobj(&[
        ("label", jstr(label)),
        ("exploration", exploration_json(question, branches)),
    ])
}

fn lineage_json(atlases: &[String]) -> String {
    jobj(&[("atlases", jarr(atlases))])
}

fn lineage_loads(data_json: &str) -> Lineage {
    let (found, defects) = load_lineage_text(data_json);
    assert_eq!(defects, Vec::<String>::new());
    found.expect("a clean load holds a lineage")
}

// The three atlases of the absence pins, the shapes of
// the standing corpus. The kept entry stands in every
// atlas, the gone entry departs at the first step and
// returns at the second, and the fresh entry arrives at
// the first step and departs at the second.

fn first() -> Atlas {
    atlas("First", vec![plain("kept"), plain("gone")])
}

fn second() -> Atlas {
    atlas("Second", vec![plain("kept"), plain("fresh")])
}

fn third() -> Atlas {
    atlas("Third", vec![plain("kept"), plain("gone")])
}

fn corpus() -> Lineage {
    Lineage {
        atlases: vec![first(), second(), third()],
    }
}

// The five atlas chain of the relation pins. The anchor
// stands everywhere, the rover stands at atlases 1, 3,
// and 5, the gone entry stands at atlas 1 alone, the
// fresh entry stands at atlas 2 alone, and the late
// entry stands at atlas 5 alone.

fn wide_chain() -> Lineage {
    Lineage {
        atlases: vec![
            atlas("One", vec![plain("rover"), plain("anchor"), plain("gone")]),
            atlas("Two", vec![plain("anchor"), plain("fresh")]),
            atlas("Three", vec![plain("rover"), plain("anchor")]),
            atlas("Four", vec![plain("anchor")]),
            atlas("Five", vec![plain("rover"), plain("anchor"), plain("late")]),
        ],
    }
}

/// The figure after a head in a line, read back out of
/// the rendered text so a relation fails when the
/// reading miscounts.
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

/// The figure after a head at the end of a line, read
/// back out of the rendered text.
fn figure_at_end(line: &str, head: &str) -> usize {
    line.rsplit(head)
        .next()
        .unwrap_or_else(|| panic!("the figure head {head} stands in the line {line}"))
        .parse()
        .expect("a count")
}

#[test]
fn the_absence_of_the_corpus_chain() {
    let lines = render_absence(&corpus()).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry gone, absent from atlas 2, for 1 atlases",
            "entry fresh, absent from atlas 3, for 1 atlases, to the close",
            "absence counts, absences 2, entries with absences 2, \
             absent at the close 1",
        ])
    );
}

#[test]
fn a_loaded_chain_renders_its_absence() {
    let one = atlas_json(
        "One",
        &[entry_json("kept", "Q", &[]), entry_json("gone", "Q", &[])],
    );
    let two = atlas_json("Two", &[entry_json("kept", "Q", &[])]);
    let three = atlas_json(
        "Three",
        &[entry_json("kept", "Q", &[]), entry_json("gone", "Q", &[])],
    );
    let lineage = lineage_loads(&lineage_json(&[one, two, three]));
    let lines = render_absence(&lineage).expect("the loaded chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry gone, absent from atlas 2, for 1 atlases",
            "absence counts, absences 1, entries with absences 1, \
             absent at the close 0",
        ])
    );
}

#[test]
fn a_two_atlas_chain_renders_one_closing_run() {
    let lineage = Lineage {
        atlases: vec![
            atlas("One", vec![plain("kept"), plain("gone")]),
            atlas("Two", vec![plain("kept")]),
        ],
    };
    let lines = render_absence(&lineage).expect("the pair reads");
    assert_eq!(
        lines,
        strings(&[
            "entry gone, absent from atlas 2, for 1 atlases, to the close",
            "absence counts, absences 1, entries with absences 1, \
             absent at the close 1",
        ])
    );
}

#[test]
fn a_chain_where_every_label_stands_everywhere_renders_zeros_alone() {
    let lineage = Lineage {
        atlases: vec![
            atlas("One", vec![plain("kept"), plain("also")]),
            atlas("Two", vec![plain("also"), plain("kept")]),
            atlas("Three", vec![plain("kept"), plain("also")]),
        ],
    };
    let lines = render_absence(&lineage).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&["absence counts, absences 0, entries with absences 0, \
                   absent at the close 0"])
    );
}

#[test]
fn a_label_with_two_inside_runs_stands_two_run_lines_and_returns_twice() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain("rover"), plain("anchor")]),
            atlas("Two", vec![plain("anchor")]),
            atlas("Three", vec![plain("rover"), plain("anchor")]),
            atlas("Four", vec![plain("anchor")]),
            atlas("Five", vec![plain("rover"), plain("anchor")]),
        ],
    };
    let lines = render_absence(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry rover, absent from atlas 2, for 1 atlases",
            "entry rover, absent from atlas 4, for 1 atlases",
            "absence counts, absences 2, entries with absences 1, \
             absent at the close 0",
        ])
    );
    let returns = render_returns(&chain).expect("the chain reads");
    assert_eq!(
        returns
            .iter()
            .filter(|line| line.starts_with("entry rover, returned"))
            .count(),
        2
    );
    assert_eq!(
        returns.last().expect("a closing line"),
        "returns counts, returns 2, entries returned 1"
    );
}

#[test]
fn a_late_first_standing_holds_no_run_before_it() {
    let lineage = Lineage {
        atlases: vec![
            atlas("One", vec![plain("kept")]),
            atlas("Two", vec![plain("kept")]),
            atlas("Three", vec![plain("kept"), plain("late")]),
            atlas("Four", vec![plain("kept"), plain("late")]),
        ],
    };
    let lines = render_absence(&lineage).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&["absence counts, absences 0, entries with absences 0, \
                   absent at the close 0"])
    );
}

#[test]
fn a_late_entry_absent_at_the_close_runs_from_the_close_alone() {
    // The late entry stands first at atlas 3 and not at
    // atlas 4, so its one run opens at atlas 4. A reading
    // that counted the atlases before its first standing
    // as absence would open the run at atlas 1.
    let lineage = Lineage {
        atlases: vec![
            atlas("One", vec![plain("kept")]),
            atlas("Two", vec![plain("kept")]),
            atlas("Three", vec![plain("kept"), plain("late")]),
            atlas("Four", vec![plain("kept")]),
        ],
    };
    let lines = render_absence(&lineage).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry late, absent from atlas 4, for 1 atlases, to the close",
            "absence counts, absences 1, entries with absences 1, \
             absent at the close 1",
        ])
    );
}

#[test]
fn a_run_of_three_atlases_counts_three() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain("rover")]),
            atlas("Two", vec![]),
            atlas("Three", vec![]),
            atlas("Four", vec![]),
            atlas("Five", vec![plain("rover")]),
        ],
    };
    let lines = render_absence(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry rover, absent from atlas 2, for 3 atlases",
            "absence counts, absences 1, entries with absences 1, \
             absent at the close 0",
        ])
    );
}

/// The absence lines of one chain, computed in the test
/// from the chain alone, in the forms MY-20 states.
fn naive_lines(chain: &Lineage) -> Vec<String> {
    let total = chain.atlases.len();
    let mut order: Vec<&PText> = Vec::new();
    for atlas in &chain.atlases {
        for entry in &atlas.entries {
            if !order.contains(&&entry.label) {
                order.push(&entry.label);
            }
        }
    }
    let stands = |label: &PText, index: usize| {
        chain.atlases[index]
            .entries
            .iter()
            .any(|entry| &entry.label == label)
    };
    let mut lines: Vec<String> = Vec::new();
    let mut runs = 0usize;
    let mut with_absences = 0usize;
    let mut absent_at_close = 0usize;
    for label in &order {
        let mut own = 0usize;
        let mut stood_before = false;
        let mut index = 0usize;
        while index < total {
            if stands(label, index) {
                stood_before = true;
                index += 1;
                continue;
            }
            // The label does not stand at this atlas. A run
            // opens only behind a standing.
            if !stood_before {
                index += 1;
                continue;
            }
            let start = index;
            while index < total && !stands(label, index) {
                index += 1;
            }
            runs += 1;
            own += 1;
            if index == total {
                absent_at_close += 1;
                lines.push(format!(
                    "entry {}, absent from atlas {}, for {} atlases, to the close",
                    label,
                    start + 1,
                    index - start
                ));
            } else {
                lines.push(format!(
                    "entry {}, absent from atlas {}, for {} atlases",
                    label,
                    start + 1,
                    index - start
                ));
            }
        }
        if own > 0 {
            with_absences += 1;
        }
    }
    lines.push(format!(
        "absence counts, absences {runs}, entries with absences {with_absences}, \
         absent at the close {absent_at_close}"
    ));
    lines
}

#[test]
fn every_line_matches_an_independent_count_from_the_chain() {
    let chain = wide_chain();
    let lines = render_absence(&chain).expect("the chain reads");
    assert_eq!(lines, naive_lines(&chain));
    // The hand reading of the same chain. The rover holds
    // two inside runs of one atlas each, the gone entry
    // holds a closing run of four atlases from atlas 2,
    // and the fresh entry holds a closing run of three
    // atlases from atlas 3.
    assert_eq!(
        lines,
        strings(&[
            "entry rover, absent from atlas 2, for 1 atlases",
            "entry rover, absent from atlas 4, for 1 atlases",
            "entry gone, absent from atlas 2, for 4 atlases, to the close",
            "entry fresh, absent from atlas 3, for 3 atlases, to the close",
            "absence counts, absences 4, entries with absences 3, \
             absent at the close 2",
        ])
    );
}

#[test]
fn the_inside_runs_equal_the_returns_of_the_same_chain() {
    for chain in [corpus(), wide_chain()] {
        let absence = render_absence(&chain).expect("the chain reads");
        let inside = absence
            .iter()
            .filter(|line| line.starts_with("entry ") && !line.ends_with("to the close"))
            .count();
        let returns = render_returns(&chain).expect("the chain reads");
        let closing = returns.last().expect("a closing line");
        let stated = figure(closing, "returns counts, returns ", ",");
        assert_eq!(inside, stated);
    }
}

#[test]
fn the_absent_at_the_close_relation_against_the_standing_reading() {
    let two_atlas = Lineage {
        atlases: vec![
            atlas("One", vec![plain("kept"), plain("gone")]),
            atlas("Two", vec![plain("kept")]),
        ],
    };
    for chain in [corpus(), wide_chain(), two_atlas] {
        let standing = render_standing(&chain).expect("the chain reads");
        let standing_closing = standing.last().expect("a closing line");
        let entries = figure(standing_closing, "entries ", ",");
        let at_close = figure_at_end(standing_closing, "standing at the close ");
        let absence = render_absence(&chain).expect("the chain reads");
        let absence_closing = absence.last().expect("a closing line");
        let absent = figure_at_end(absence_closing, "absent at the close ");
        assert_eq!(absent, entries - at_close);
    }
}

#[test]
fn the_span_relation_holds_for_every_label() {
    let chain = wide_chain();
    let standing = render_standing(&chain).expect("the chain reads");
    let absence = render_absence(&chain).expect("the chain reads");
    for line in standing.iter().filter(|line| line.starts_with("entry ")) {
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
            .iter()
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
fn a_label_cannot_mint_the_closing_line() {
    let forged = "absence counts, absences 9, entries with absences 9, \
                  absent at the close 9";
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain(forged)]),
            atlas("Two", vec![]),
            atlas("Three", vec![plain(forged)]),
        ],
    };
    let lines = render_absence(&chain).expect("the chain reads");
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.starts_with("absence counts"))
            .count(),
        1
    );
    assert_eq!(
        lines[0],
        format!("entry {forged}, absent from atlas 2, for 1 atlases")
    );
    assert!(lines[0].starts_with("entry "));
}

#[test]
fn the_lines_stand_in_first_standing_order_not_close_order() {
    // The closing atlas holds the two labels in the other
    // order, so a reading that walked the chain backwards
    // or read the close first would swap the lines.
    let chain = Lineage {
        atlases: vec![
            atlas("One", vec![plain("wye"), plain("ex")]),
            atlas("Two", vec![]),
            atlas("Three", vec![plain("ex"), plain("wye")]),
        ],
    };
    let lines = render_absence(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry wye, absent from atlas 2, for 1 atlases",
            "entry ex, absent from atlas 2, for 1 atlases",
            "absence counts, absences 2, entries with absences 2, \
             absent at the close 0",
        ])
    );
}

#[test]
fn the_reading_is_a_pure_function_of_the_lineage() {
    let chain = corpus();
    let once = render_absence(&chain).expect("the chain reads");
    let twice = render_absence(&chain).expect("the chain reads");
    assert_eq!(once, twice);
}

#[test]
fn the_absence_reading_holds_the_vocabulary_bar() {
    let lines = render_absence(&wide_chain()).expect("the chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert!(holds_vocabulary_bar(&text));
}

#[test]
fn a_wide_chain_renders_its_absence_whole() {
    let wide_branches: Vec<Branch> = (0..1200)
        .map(|index| {
            branch(
                &format!("b{index}"),
                None,
                "Prompt",
                "open",
                None,
                None,
                None,
            )
        })
        .collect();
    let wide = |title: &str| atlas(title, vec![entry("wide", "Q wide", wide_branches.clone())]);
    let empty = atlas("Empty", vec![]);
    let chain = Lineage {
        atlases: vec![wide("One"), empty, wide("Three")],
    };
    let lines = render_absence(&chain).expect("the wide chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry wide, absent from atlas 2, for 1 atlases",
            "absence counts, absences 1, entries with absences 1, \
             absent at the close 0",
        ])
    );
}

#[test]
fn a_hand_built_lineage_of_fewer_than_two_atlases_is_refused() {
    let one = Lineage {
        atlases: vec![first()],
    };
    assert_eq!(
        render_absence(&one).expect_err("a chain of one is refused"),
        "the lineage holds fewer than two atlases"
    );
    let none = Lineage { atlases: vec![] };
    assert_eq!(
        render_absence(&none).expect_err("an empty chain is refused"),
        "the lineage holds fewer than two atlases"
    );
}

#[test]
fn a_hand_built_surrogate_label_names_its_refusal() {
    let mut broken = second();
    broken.entries[0].label = bad();
    let chain = Lineage {
        atlases: vec![first(), broken, third()],
    };
    assert_eq!(
        render_absence(&chain).expect_err("the broken label is refused"),
        "an entry label holds an unpaired surrogate"
    );
}

#[test]
fn a_hand_built_surrogate_label_in_the_last_atlas_still_refuses() {
    let mut broken = third();
    broken.entries[1].label = bad();
    let chain = Lineage {
        atlases: vec![first(), second(), broken],
    };
    assert_eq!(
        render_absence(&chain).expect_err("the broken label is refused"),
        "an entry label holds an unpaired surrogate"
    );
}

fn thin_chain(n: usize) -> Lineage {
    let mut atlases = Vec::new();
    for i in 0..n {
        let first = format!("label-{}", i + 1);
        let second = format!("label-{}", (i + 1) % n + 1);
        let title = format!("atlas-{}", i + 1);
        atlases.push(atlas(&title, vec![plain(&first), plain(&second)]));
    }
    Lineage { atlases }
}

fn window_chain() -> Lineage {
    let mut atlases = Vec::new();
    for i in 0..120usize {
        let labels: Vec<String> = (0..5)
            .map(|j| format!("label-{}", (i + j) % 30 + 1))
            .collect();
        let entries: Vec<AtlasEntry> = labels.iter().map(|label| plain(label)).collect();
        let title = format!("atlas-{}", i + 1);
        atlases.push(atlas(&title, entries));
    }
    Lineage { atlases }
}

fn paired_chain() -> Lineage {
    let mut atlases = Vec::new();
    for i in 0..120 {
        let title = format!("atlas-{}", i + 1);
        if i % 2 == 0 {
            atlases.push(atlas(
                &title,
                vec![plain("label-2"), plain("label-1"), plain("label-4")],
            ));
        } else {
            atlases.push(atlas(
                &title,
                vec![plain("label-3"), plain("label-5"), plain("label-2")],
            ));
        }
    }
    Lineage { atlases }
}

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

/// The label an entry line names.
fn label_of(line: &str) -> String {
    line.strip_prefix("entry ")
        .expect("an entry line opens with the word entry")
        .split(',')
        .next()
        .expect("a label stands")
        .to_string()
}

/// The sections of a presence reading, each the entry
/// lines standing under its opening line.
fn presence_sections(lines: &[String]) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = Vec::new();
    for line in lines {
        if line.starts_with("atlas ") && !line.contains("standing") {
            out.push(Vec::new());
        } else if line.starts_with("entry ") {
            out.last_mut()
                .expect("an entry line stands inside a section")
                .push(line.clone());
        }
    }
    out
}

/// The absence of the thin chain states one run for
/// every label but the label standing at the close,
/// the label standing in the first and last atlases
/// holding the one inside run of the chain after the
/// whole chain's absence and every other label but
/// one holding one closing run, the counts of the
/// closing standing as the count of the run lines,
/// of the labels holding one, and of the labels
/// holding a closing run.
#[test]
fn the_thin_chain_absence_stands_as_counted() {
    let lines = render_absence(&thin_chain(302)).expect("the chain reads");
    assert_eq!(lines.len(), 302);
    assert_eq!(
        lines[0], "entry label-1, absent from atlas 2, for 300 atlases",
        "the one inside run of the chain"
    );
    assert_eq!(
        lines[1], "entry label-2, absent from atlas 3, for 300 atlases, to the close",
        "the first closing run of the chain"
    );
    assert_eq!(
        lines[300], "entry label-301, absent from atlas 302, for 1 atlases, to the close",
        "the last run line of the chain"
    );
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.ends_with("to the close"))
            .count(),
        300,
        "every run but the inside run and the closing line reaches the close"
    );
    assert!(
        !lines
            .iter()
            .any(|line| line.starts_with("entry label-302,")),
        "the label standing at the close holds no run"
    );
    assert_eq!(
        lines[301],
        "absence counts, absences 301, entries with absences 301, absent at the close 300"
    );
}

/// The absence of the window chain states one run
/// for each gap between the runs of five standings
/// of a label and one closing run for each label
/// standing short of the close, every inside run
/// standing at the twenty-five atlases of the gap,
/// the counts of the closing standing as the count
/// of the run lines, of the labels holding one, and
/// of the labels holding a closing run.
#[test]
fn the_window_chain_absence_stands_as_counted() {
    let lines = render_absence(&window_chain()).expect("the chain reads");
    assert_eq!(lines.len(), 120);
    assert_eq!(
        lines[0], "entry label-1, absent from atlas 2, for 25 atlases",
        "the first run of the chain"
    );
    assert_eq!(
        lines[1], "entry label-1, absent from atlas 32, for 25 atlases",
        "the second run of the chain"
    );
    assert_eq!(
        lines[2], "entry label-1, absent from atlas 62, for 25 atlases",
        "the third run of the chain"
    );
    assert_eq!(
        lines[118], "entry label-30, absent from atlas 91, for 25 atlases",
        "the last run line of the chain"
    );
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.ends_with("to the close"))
            .count(),
        25,
        "the labels standing short of the close hold one closing run each"
    );
    assert!(
        lines[..119]
            .iter()
            .filter(|line| !line.ends_with("to the close"))
            .all(|line| line.ends_with("for 25 atlases")),
        "every inside run stands at the gap of the window"
    );
    assert_eq!(
        lines[119],
        "absence counts, absences 119, entries with absences 30, absent at the close 25"
    );
}

/// The absence of the paired chain states one run
/// of one atlas for each gap between the standings
/// of the alternating labels and one closing run
/// for each label standing in the first roster, the
/// label standing in every atlas holding no run, the
/// counts of the closing standing as the count of
/// the run lines, of the labels holding one, and of
/// the labels holding a closing run.
#[test]
fn the_paired_chain_absence_stands_as_counted() {
    let lines = render_absence(&paired_chain()).expect("the chain reads");
    assert_eq!(lines.len(), 239);
    assert_eq!(
        lines[0], "entry label-1, absent from atlas 2, for 1 atlases",
        "the first run of the chain"
    );
    assert_eq!(
        lines[1], "entry label-1, absent from atlas 4, for 1 atlases",
        "the second run of the chain"
    );
    assert_eq!(
        lines[237], "entry label-5, absent from atlas 119, for 1 atlases",
        "the last run line of the chain"
    );
    assert!(
        lines[..238]
            .iter()
            .all(|line| line.contains(", for 1 atlases")),
        "every run of the chain stands at one atlas"
    );
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.ends_with("to the close"))
            .count(),
        2,
        "the two labels of the first roster hold the closing runs"
    );
    assert!(
        !lines.iter().any(|line| line.starts_with("entry label-2,")),
        "the label standing in every atlas holds no run"
    );
    assert_eq!(
        lines[238],
        "absence counts, absences 238, entries with absences 4, absent at the close 2"
    );
}

/// The absence runs of a chain stand bound to the
/// presence reading's sections over the same chain,
/// a run standing in the gap between two standings
/// of a label and in the tail after its last
/// standing, its first atlas standing one past the
/// earlier standing and its count standing as the
/// atlases between the standings less the standing
/// it ends at, the closing's absences standing as
/// the count of the run lines, its entries with
/// absences as the count of the labels holding a
/// run, and its absent at the close as the count of
/// the labels whose last standing stands short of
/// the last atlas.
#[test]
fn the_absence_runs_stand_bound_to_the_presence_sections() {
    let chains = vec![
        load("winter-spring-summer-lineage.json"),
        load("radio-investigation-lineage.json"),
        load("constructed-departures-lineage.json"),
        load("generated-chain-lineage.json"),
        thin_chain(302),
        window_chain(),
        paired_chain(),
    ];
    for chain in &chains {
        let lines = render_absence(chain).expect("the chain reads");
        let presence = myriad::atlas::render_presence(chain).expect("the chain reads");
        let named = presence_sections(&presence);
        assert_eq!(named.len(), chain.atlases.len());
        let mut order: Vec<String> = Vec::new();
        let mut standings: Vec<Vec<usize>> = Vec::new();
        for (index, section) in named.iter().enumerate() {
            for line in section {
                let label = label_of(line);
                match order.iter().position(|held| held == &label) {
                    Some(at) => standings[at].push(index + 1),
                    None => {
                        order.push(label);
                        standings.push(vec![index + 1]);
                    }
                }
            }
        }
        let total = chain.atlases.len();
        let mut expected: Vec<String> = Vec::new();
        let mut with_runs = 0usize;
        let mut at_close = 0usize;
        for (at, list) in standings.iter().enumerate() {
            let label = &order[at];
            let mut label_runs = 0usize;
            for pair in list.windows(2) {
                if pair[1] - pair[0] > 1 {
                    label_runs += 1;
                    expected.push(format!(
                        "entry {label}, absent from atlas {}, for {} atlases",
                        pair[0] + 1,
                        pair[1] - pair[0] - 1
                    ));
                }
            }
            let last = list[list.len() - 1];
            if last < total {
                label_runs += 1;
                at_close += 1;
                expected.push(format!(
                    "entry {label}, absent from atlas {}, for {} atlases, to the close",
                    last + 1,
                    total - last
                ));
            }
            if label_runs > 0 {
                with_runs += 1;
            }
        }
        expected.push(format!(
            "absence counts, absences {}, entries with absences {}, absent at the close {}",
            expected.len(),
            with_runs,
            at_close
        ));
        assert_eq!(lines, expected);
    }
}
