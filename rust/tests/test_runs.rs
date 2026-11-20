//! Tests for MY-30, the runs reading.
//!
//! The reading takes a lineage, reads the presence of
//! every label in the one presence pass FIX-MY-13 states,
//! and states one line for each standing run of the chain,
//! a run of one or more atlases in a row in which the
//! label stands, closing with the runs counts. The pins
//! here read the reading over the shipped chains and over
//! hand built chains, and bind its closings to the
//! standing reading's figures, to the returns reading's
//! figure, and to the absence reading's figures, each
//! computed from the neighbouring readings' own outputs.
//!
//! The discrimination split of these pins stands as
//! follows. Every pin in this file fails on the tree
//! before this slice by compile absence, because
//! render_runs stands nowhere on that tree, so no pin
//! here can pass there. Inside the slice, pin 1 kills a
//! reading that orders its lines by atlas of run start
//! alone or by last standing. Pin 2 kills a closing read
//! from any path but the lines it restates. Pin 3, the
//! first relation, kills a first run counted as a return
//! or a return opening no run. Pin 4, the second
//! relation, kills runs that overlap, skip an atlas, or
//! merge across an absence. Pin 5, the third relation,
//! kills a closing run of absence counted as an inside
//! run in this reading. Pin 6, the fourth relation,
//! kills a run split at a step boundary for a label
//! standing through it. Pin 7, the fifth relation, kills
//! runs anchored to arrivals or departures rather than
//! to standings. Pin 8 kills a reading that requires a
//! return or a third atlas to state a run. Pin 9 kills
//! an implementation that reasons from the span between
//! first and last standings. Pin 10 kills a raw-entry
//! walk, the shape the departures red team finding of
//! Round 12 took, held from birth here under the
//! presence pass's set posture. The refusal pins kill a
//! reading born outside the presence pass's refusal
//! posture. No pin in this file passes on the tree
//! before this slice by design.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_absence, render_returns, render_runs, render_standing, Atlas,
    AtlasEntry, Lineage,
};
use myriad::explorations::Exploration;
use myriad::pytext::PText;

fn expl() -> Exploration {
    Exploration {
        question: pt("q"),
        branches: vec![branch("a", None, "p", "open", None, None, None)],
    }
}

fn atlas(title: &str, labels: &[&str]) -> Atlas {
    Atlas {
        title: pt(title),
        entries: labels
            .iter()
            .map(|label| AtlasEntry {
                label: pt(label),
                exploration: expl(),
            })
            .collect(),
    }
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

const CHAINS: [&str; 3] = [
    "winter-spring-summer-lineage.json",
    "radio-investigation-lineage.json",
    "constructed-departures-lineage.json",
];

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

/// The figure after a head that ends the line.
fn figure_last(line: &str, head: &str) -> usize {
    line.split(head)
        .nth(1)
        .unwrap_or_else(|| panic!("the figure head {head} stands in the line {line}"))
        .parse()
        .expect("a count")
}

/// One run line read back into its label, its first
/// atlas, and its count of atlases.
fn run_parts(line: &str) -> (String, usize, usize) {
    let rest = line
        .strip_prefix("entry ")
        .expect("a run line opens with the word entry");
    let (label, tail) = rest
        .split_once(", standing from atlas ")
        .expect("a run line names its first atlas");
    let start: usize = tail
        .split(',')
        .next()
        .expect("the first atlas stands")
        .parse()
        .expect("a count");
    let count = figure(line, ", for ", " atlases");
    (label.to_string(), start, count)
}

/// The run lines of a runs reading, the closing set
/// aside, grouped by label in the order they stand.
fn runs_by_label(lines: &[String]) -> Vec<(String, usize, usize, bool)> {
    lines
        .iter()
        .filter(|line| line.starts_with("entry "))
        .map(|line| {
            let (label, start, count) = run_parts(line);
            (label, start, count, line.ends_with("to the close"))
        })
        .collect()
}

/// The standing figures of one label, read from the
/// standing reading's line for it: its first atlas, its
/// last atlas, and the count of atlases it stands in.
fn standing_figures(lines: &[String], label: &str) -> (usize, usize, usize) {
    let line = lines
        .iter()
        .find(|line| line.starts_with(&format!("entry {label},")))
        .unwrap_or_else(|| panic!("the standing line of {label} stands"));
    (
        figure(line, "first stood at atlas ", ","),
        figure(line, "last stood at atlas ", ","),
        figure(line, "stands in ", " atlases"),
    )
}

#[test]
fn the_run_lines_stand_in_first_standing_order() {
    let lines = render_runs(&load("winter-spring-summer-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
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
fn the_runs_closing_restates_its_lines() {
    let lines = render_runs(&load("winter-spring-summer-lineage.json")).expect("the chain reads");
    let closing = lines.last().expect("a closing line");
    let runs = runs_by_label(&lines);
    assert_eq!(runs.len(), figure(closing, "runs counts, runs ", ","));
    let distinct: std::collections::BTreeSet<&str> =
        runs.iter().map(|(label, _, _, _)| label.as_str()).collect();
    assert_eq!(distinct.len(), figure(closing, "entries with runs ", ","));
    assert_eq!(
        runs.iter().filter(|(_, _, _, closes)| *closes).count(),
        figure_last(closing, "standing at the close ")
    );
}

#[test]
fn the_runs_count_is_the_entries_and_the_returns() {
    for name in CHAINS {
        let chain = load(name);
        let runs = render_runs(&chain).expect("the runs read");
        let closing = runs.last().expect("a closing line");
        let standing = render_standing(&chain).expect("the standing reads");
        let standing_closing = standing.last().expect("a closing line");
        let returns = render_returns(&chain).expect("the returns read");
        let returns_closing = returns.last().expect("a closing line");
        assert_eq!(
            figure(closing, "runs counts, runs ", ","),
            figure(standing_closing, "entries ", ",")
                + figure(returns_closing, "returns counts, returns ", ","),
            "the first relation holds over {name}"
        );
    }
}

#[test]
fn each_labels_runs_partition_its_standings() {
    for name in CHAINS {
        let chain = load(name);
        let runs = render_runs(&chain).expect("the runs read");
        let standing = render_standing(&chain).expect("the standing reads");
        let mut sums: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
        for (label, _, count, _) in runs_by_label(&runs) {
            *sums.entry(label).or_insert(0) += count;
        }
        for (label, sum) in &sums {
            let (_, _, stands_in) = standing_figures(&standing, label);
            assert_eq!(
                *sum, stands_in,
                "the runs of {label} partition its standings over {name}"
            );
        }
    }
}

#[test]
fn the_runs_count_reads_off_the_absence_figures() {
    for name in CHAINS {
        let chain = load(name);
        let runs = render_runs(&chain).expect("the runs read");
        let closing = runs.last().expect("a closing line");
        let absence = render_absence(&chain).expect("the absence reads");
        let absence_closing = absence.last().expect("a closing line");
        let standing = render_standing(&chain).expect("the standing reads");
        let standing_closing = standing.last().expect("a closing line");
        assert_eq!(
            figure(closing, "runs counts, runs ", ","),
            figure(absence_closing, "absence counts, absences ", ",")
                - figure_last(absence_closing, "absent at the close ")
                + figure(standing_closing, "entries ", ","),
            "the third relation holds over {name}"
        );
    }
}

#[test]
fn standing_at_every_atlas_holds_one_run_to_the_close() {
    for name in [
        "winter-spring-summer-lineage.json",
        "radio-investigation-lineage.json",
    ] {
        let chain = load(name);
        let total = chain.atlases.len();
        let runs = render_runs(&chain).expect("the runs read");
        let closing = runs.last().expect("a closing line");
        let standing = render_standing(&chain).expect("the standing reads");
        let parsed = runs_by_label(&runs);
        for line in standing.iter().filter(|line| line.starts_with("entry ")) {
            let label = line
                .strip_prefix("entry ")
                .expect("an entry line stands")
                .split(',')
                .next()
                .expect("a label stands");
            let (_, _, stands_in) = standing_figures(&standing, label);
            if stands_in == total {
                let mine: Vec<&(String, usize, usize, bool)> = parsed
                    .iter()
                    .filter(|(held, _, _, _)| held == label)
                    .collect();
                assert_eq!(mine.len(), 1, "{label} holds one run over {name}");
                assert_eq!(mine[0].2, total, "{label} runs the whole chain");
                assert!(mine[0].3, "{label} closes its run over {name}");
            }
        }
        assert_eq!(
            parsed.iter().filter(|(_, _, _, closes)| *closes).count(),
            figure_last(closing, "standing at the close "),
            "the closing runs count holds over {name}"
        );
    }
}

#[test]
fn the_first_and_last_runs_end_where_the_standing_line_names() {
    for name in CHAINS {
        let chain = load(name);
        let runs = render_runs(&chain).expect("the runs read");
        let standing = render_standing(&chain).expect("the standing reads");
        let parsed = runs_by_label(&runs);
        let mut labels: Vec<&str> = Vec::new();
        for (label, _, _, _) in &parsed {
            if !labels.contains(&label.as_str()) {
                labels.push(label.as_str());
            }
        }
        for label in labels {
            let mine: Vec<&(String, usize, usize, bool)> = parsed
                .iter()
                .filter(|(held, _, _, _)| held == label)
                .collect();
            let (first, last, _) = standing_figures(&standing, label);
            assert_eq!(
                mine.first().expect("a first run").1,
                first,
                "the first run of {label} opens where it first stands over {name}"
            );
            let last_run = mine.last().expect("a last run");
            assert_eq!(
                last_run.1 + last_run.2 - 1,
                last,
                "the last run of {label} ends where it last stands over {name}"
            );
        }
    }
}

#[test]
fn a_two_atlas_chain_renders_its_runs() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", &["both", "first only"]),
            atlas("Second", &["both"]),
        ],
    };
    let lines = render_runs(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry both, standing from atlas 1, for 2 atlases, to the close",
            "entry first only, standing from atlas 1, for 1 atlases",
            "runs counts, runs 2, entries with runs 2, standing at the close 1",
        ])
    );
}

#[test]
fn an_entry_standing_in_non_neighbouring_atlases_holds_two_runs() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", &["gap label"]),
            atlas("Second", &["other label"]),
            atlas("Third", &["gap label", "other label"]),
        ],
    };
    let lines = render_runs(&chain).expect("the chain reads");
    let mine: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("entry gap label,"))
        .collect();
    assert_eq!(
        mine,
        [
            "entry gap label, standing from atlas 1, for 1 atlases",
            "entry gap label, standing from atlas 3, for 1 atlases, to the close",
        ]
    );
    assert!(
        !lines
            .iter()
            .any(|line| line.starts_with("entry gap label,") && line.contains("for 2 atlases")),
        "no run spans the absence"
    );
}

#[test]
fn a_label_held_twice_in_one_atlas_runs_once_through_it() {
    let doubled = Atlas {
        title: pt("First"),
        entries: vec![
            AtlasEntry {
                label: pt("held twice"),
                exploration: expl(),
            },
            AtlasEntry {
                label: pt("held twice"),
                exploration: expl(),
            },
        ],
    };
    let chain = Lineage {
        atlases: vec![doubled, atlas("Second", &["held twice"])],
    };
    let lines = render_runs(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "entry held twice, standing from atlas 1, for 2 atlases, to the close",
            "runs counts, runs 1, entries with runs 1, standing at the close 1",
        ])
    );
}

#[test]
fn a_lineage_holding_fewer_than_two_atlases_is_refused() {
    let chain = Lineage {
        atlases: vec![atlas("Only", &["kept"])],
    };
    assert_eq!(
        render_runs(&chain).expect_err("the short chain is refused"),
        "the lineage holds fewer than two atlases"
    );
}

#[test]
fn a_label_holding_an_unpaired_surrogate_is_refused_first() {
    let broken = Atlas {
        title: pt("First"),
        entries: vec![AtlasEntry {
            label: PText(vec![0x61, 0xD800]),
            exploration: expl(),
        }],
    };
    let chain = Lineage {
        atlases: vec![broken, atlas("Second", &["kept"])],
    };
    assert_eq!(
        render_runs(&chain).expect_err("the surrogate label is refused"),
        "an entry label holds an unpaired surrogate"
    );
    let both = Atlas {
        title: pt("First"),
        entries: vec![AtlasEntry {
            label: PText(vec![0x61, 0xD800, 0x0A]),
            exploration: expl(),
        }],
    };
    let chain = Lineage {
        atlases: vec![both, atlas("Second", &["kept"])],
    };
    assert_eq!(
        render_runs(&chain).expect_err("the doubly broken label names the surrogate first"),
        "an entry label holds an unpaired surrogate"
    );
}

#[test]
fn a_label_holding_a_line_break_is_refused() {
    let chain = Lineage {
        atlases: vec![atlas("First", &["a\nb"]), atlas("Second", &["kept"])],
    };
    assert_eq!(
        render_runs(&chain).expect_err("the broken label is refused"),
        "an entry label holds a newline or a carriage return"
    );
}
fn thin_chain(n: usize) -> Lineage {
    let mut atlases = Vec::new();
    for i in 0..n {
        let first = format!("label-{}", i + 1);
        let second = format!("label-{}", (i + 1) % n + 1);
        let title = format!("atlas-{}", i + 1);
        atlases.push(atlas(&title, &[first.as_str(), second.as_str()]));
    }
    Lineage { atlases }
}

fn window_chain() -> Lineage {
    let mut atlases = Vec::new();
    for i in 0..120usize {
        let labels: Vec<String> = (0..5)
            .map(|j| format!("label-{}", (i + j) % 30 + 1))
            .collect();
        let refs: Vec<&str> = labels.iter().map(|label| label.as_str()).collect();
        let title = format!("atlas-{}", i + 1);
        atlases.push(atlas(&title, &refs));
    }
    Lineage { atlases }
}

fn paired_chain() -> Lineage {
    let mut atlases = Vec::new();
    for i in 0..120 {
        let title = format!("atlas-{}", i + 1);
        if i % 2 == 0 {
            atlases.push(atlas(&title, &["label-2", "label-1", "label-4"]));
        } else {
            atlases.push(atlas(&title, &["label-3", "label-5", "label-2"]));
        }
    }
    Lineage { atlases }
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

/// The runs of the thin chain stand one for every
/// label of the chain and one more, the first label
/// holding two runs of one atlas at the two ends of
/// the chain, every other label holding one run of
/// two atlases, and the runs of the first and last
/// labels reaching the close, the counts of the
/// closing standing as the count of the run lines,
/// of the labels holding one, and of the labels
/// standing at the close.
#[test]
fn the_thin_chain_runs_stand_as_counted() {
    let lines = render_runs(&thin_chain(302)).expect("the chain reads");
    assert_eq!(lines.len(), 304);
    assert_eq!(
        lines[0], "entry label-1, standing from atlas 1, for 1 atlases",
        "the first run of the chain"
    );
    assert_eq!(
        lines[1], "entry label-1, standing from atlas 302, for 1 atlases, to the close",
        "the second run of the chain, at the close"
    );
    assert_eq!(
        lines[2], "entry label-2, standing from atlas 1, for 2 atlases",
        "the first run of two atlases"
    );
    assert_eq!(
        lines[302], "entry label-302, standing from atlas 301, for 2 atlases, to the close",
        "the last run line of the chain"
    );
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.ends_with("to the close"))
            .count(),
        2,
        "the two labels standing in the last atlas hold the closing runs"
    );
    assert_eq!(
        lines[303],
        "runs counts, runs 303, entries with runs 302, standing at the close 2"
    );
}

/// The runs of the window chain stand one for every
/// block of standings of the chain, the four labels
/// whose blocks wrap the period's turn holding five
/// runs each and every other label four, the first
/// label's first run standing at one atlas alone and
/// its later runs at the five atlases of its blocks,
/// the counts of the closing standing as the count
/// of the run lines, of the labels holding one, and
/// of the labels standing at the close.
#[test]
fn the_window_chain_runs_stand_as_counted() {
    let lines = render_runs(&window_chain()).expect("the chain reads");
    assert_eq!(lines.len(), 125);
    assert_eq!(
        lines[0], "entry label-1, standing from atlas 1, for 1 atlases",
        "the first run of the chain"
    );
    assert_eq!(
        lines[1], "entry label-1, standing from atlas 27, for 5 atlases",
        "the second run of the chain"
    );
    assert_eq!(
        lines[2], "entry label-1, standing from atlas 57, for 5 atlases",
        "the third run of the chain"
    );
    assert_eq!(
        lines[123], "entry label-30, standing from atlas 116, for 5 atlases, to the close",
        "the last run line of the chain"
    );
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.ends_with("to the close"))
            .count(),
        5,
        "the five labels standing in the last atlas hold the closing runs"
    );
    assert_eq!(
        lines[124],
        "runs counts, runs 124, entries with runs 30, standing at the close 5"
    );
}

/// The runs of the paired chain stand one for every
/// standing of the four labels that stand in every
/// second atlas and one for the label standing in
/// every atlas, its run standing the whole chain to
/// the close, every other run standing at one atlas,
/// the counts of the closing standing as the count
/// of the run lines, of the labels holding one, and
/// of the labels standing at the close.
#[test]
fn the_paired_chain_runs_stand_as_counted() {
    let lines = render_runs(&paired_chain()).expect("the chain reads");
    assert_eq!(lines.len(), 242);
    assert_eq!(
        lines[0], "entry label-2, standing from atlas 1, for 120 atlases, to the close",
        "the run of the label standing in every atlas"
    );
    assert_eq!(
        lines[1], "entry label-1, standing from atlas 1, for 1 atlases",
        "the first run of one atlas"
    );
    assert_eq!(
        lines[240], "entry label-5, standing from atlas 120, for 1 atlases, to the close",
        "the last run line of the chain"
    );
    assert!(
        lines[1..241]
            .iter()
            .all(|line| line.contains(", for 1 atlases")),
        "every run but the full run stands at one atlas"
    );
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.ends_with("to the close"))
            .count(),
        3,
        "the three labels standing in the last atlas hold the closing runs"
    );
    assert_eq!(
        lines[241],
        "runs counts, runs 241, entries with runs 5, standing at the close 3"
    );
}

/// The runs of a chain stand bound to the presence
/// reading's sections over the same chain, a run
/// standing as a stretch of atlases in a row in the
/// list of its label, its first atlas standing as
/// the first atlas of the stretch and its count
/// standing as the atlases of the stretch, the
/// closing's runs figure standing as the count of
/// the run lines and its entries with runs figure
/// standing as the count of the labels holding a
/// run and its standing at the close figure standing
/// as the count of the labels whose last standing
/// stands at the last atlas.
#[test]
fn the_runs_stand_bound_to_the_presence_sections() {
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
        let lines = render_runs(chain).expect("the chain reads");
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
            let mut run_start = 0usize;
            for (index, pair) in list.windows(2).enumerate() {
                if pair[1] > pair[0] + 1 {
                    label_runs += 1;
                    expected.push(format!(
                        "entry {label}, standing from atlas {}, for {} atlases",
                        list[run_start],
                        pair[0] - list[run_start] + 1
                    ));
                    run_start = index + 1;
                }
            }
            let last = list[list.len() - 1];
            label_runs += 1;
            if last == total {
                at_close += 1;
                expected.push(format!(
                    "entry {label}, standing from atlas {}, for {} atlases, to the close",
                    list[run_start],
                    last - list[run_start] + 1
                ));
            } else {
                expected.push(format!(
                    "entry {label}, standing from atlas {}, for {} atlases",
                    list[run_start],
                    last - list[run_start] + 1
                ));
            }
            if label_runs > 0 {
                with_runs += 1;
            }
        }
        expected.push(format!(
            "runs counts, runs {}, entries with runs {}, standing at the close {}",
            expected.len(),
            with_runs,
            at_close
        ));
        assert_eq!(lines, expected);
    }
}
