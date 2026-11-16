//! Tests for MY-32, the presence reading.
//!
//! The reading takes a lineage, reads the presence of
//! every label in the one presence pass MY-18 states,
//! and states one section for each atlas of the chain,
//! each section holding the standing lines of the
//! entries standing in that atlas, closing with the
//! presence counts. The pins here read the reading
//! over the shipped chains and over hand built chains,
//! and bind it to the standing, matched, arrivals, and
//! departures readings through the five relations
//! MY-32 states, each computed from the neighbouring
//! readings' own outputs.
//!
//! The discrimination split of these pins stands as
//! follows. Every pin in this file fails on the tree
//! before this slice by compile absence, because
//! render_presence stands nowhere on that tree, so no
//! pin here can pass there. Inside the slice, pin 1
//! kills a reading that sections by first standing, by
//! last standing, or by step, the three section shapes
//! already standing in the family. Pin 2 kills a
//! closing read from any path but the sections it
//! restates. Pin 3, the first relation, kills a section
//! that names a label where it does not stand or omits
//! it where it does. Pin 4, the second relation, kills
//! a membership read from first or last standings
//! alone, which intersects wrong at every step a label
//! stands through. Pin 5, the third relation, kills a
//! standing line read from any helper but the one
//! standing line helper, whose first and last figures
//! these counts rest on. Pin 6, the fourth relation,
//! kills a section line in a near form, and a
//! membership that drifts from the standing reading at
//! the close or through the whole chain. Pin 7, the
//! fifth relation, kills a closing that counts a
//! standing twice on one side of the sum. Pin 8 kills
//! a reading that requires a step, a return, or a
//! third atlas to state a membership. Pin 9 kills a
//! reading that skips an empty atlas, refuses it, or
//! counts its section from a neighbouring atlas. Pin
//! 10 kills a raw entry walk, the shape the departures
//! red team finding of Round 12 took, held from birth
//! here under the presence pass's set posture. The
//! refusal pins kill a reading born outside the
//! presence pass's refusal posture. No pin in this
//! file passes on the tree before this slice by
//! design.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_arrivals, render_departures, render_matched, render_presence,
    render_standing, Atlas, AtlasEntry, Lineage,
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

/// The sections of a presence reading, the closing set
/// aside, each section the entry lines standing under
/// its opening line.
fn sections(lines: &[String]) -> Vec<Vec<String>> {
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

/// The label an entry line names.
fn label_of(line: &str) -> String {
    line.strip_prefix("entry ")
        .expect("an entry line opens with the word entry")
        .split(',')
        .next()
        .expect("a label stands")
        .to_string()
}

#[test]
fn the_sections_name_each_atlas_membership_in_order() {
    let lines =
        render_presence(&load("winter-spring-summer-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "atlas 2",
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close",
            "atlas 3",
            "entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close",
            "entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "presence counts, atlas 1 standing 3, atlas 2 standing 3, atlas 3 standing 3, entries 4, atlases 3",
        ])
    );
}

#[test]
fn the_presence_closing_restates_its_sections() {
    let lines =
        render_presence(&load("winter-spring-summer-lineage.json")).expect("the chain reads");
    let closing = lines.last().expect("a closing line");
    let found = sections(&lines);
    for (index, section) in found.iter().enumerate() {
        assert_eq!(
            section.len(),
            figure(closing, &format!("atlas {} standing ", index + 1), ","),
            "the closing restates the count of section {}",
            index + 1
        );
    }
    let distinct: std::collections::BTreeSet<String> =
        found.iter().flatten().map(|line| label_of(line)).collect();
    assert_eq!(distinct.len(), figure(closing, "entries ", ","));
    assert_eq!(found.len(), figure_last(closing, "atlases "));
}

#[test]
fn each_label_stands_in_exactly_its_standings_sections() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_presence(&chain).expect("the presence reads");
        let found = sections(&lines);
        let labels: std::collections::BTreeSet<String> =
            found.iter().flatten().map(|line| label_of(line)).collect();
        for label in &labels {
            let named = found
                .iter()
                .filter(|section| {
                    section
                        .iter()
                        .any(|line| line.starts_with(&format!("entry {label},")))
                })
                .count();
            let line = found
                .iter()
                .flatten()
                .find(|line| line.starts_with(&format!("entry {label},")))
                .expect("the standing line of the label stands in a section");
            assert_eq!(
                named,
                figure(line, "stands in ", " atlases"),
                "the sections naming {label} are its standings over {name}"
            );
        }
    }
}

#[test]
fn neighbouring_sections_intersect_at_the_matched_counts() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_presence(&chain).expect("the presence reads");
        let found = sections(&lines);
        let matched = render_matched(&chain).expect("the matched reads");
        let matched_closing = matched.last().expect("a closing line");
        for step in 0..found.len() - 1 {
            let earlier: std::collections::BTreeSet<String> =
                found[step].iter().map(|line| label_of(line)).collect();
            let later: std::collections::BTreeSet<String> =
                found[step + 1].iter().map(|line| label_of(line)).collect();
            assert_eq!(
                earlier.intersection(&later).count(),
                figure(matched_closing, &format!("step {} matched ", step + 1), ","),
                "the sections of step {} intersect at its matched count over {name}",
                step + 1
            );
        }
    }
}

#[test]
fn the_sections_first_and_last_lines_restate_arrivals_and_departures() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_presence(&chain).expect("the presence reads");
        let found = sections(&lines);
        let arrivals = render_arrivals(&chain).expect("the arrivals read");
        let arrivals_closing = arrivals.last().expect("a closing line");
        let departures = render_departures(&chain).expect("the departures read");
        let departures_closing = departures.last().expect("a closing line");
        for (index, section) in found.iter().enumerate() {
            let atlas_no = index + 1;
            let firsts = section
                .iter()
                .filter(|line| line.contains(&format!("first stood at atlas {atlas_no},")))
                .count();
            assert_eq!(
                firsts,
                figure(
                    arrivals_closing,
                    &format!("atlas {atlas_no} arrivals "),
                    ","
                ),
                "the first standings of atlas {atlas_no} restate its arrivals over {name}"
            );
            // A departure is a last standing before the
            // close: an entry standing at the close
            // departs nowhere, and the departures
            // reading's last section always stands
            // empty. Before the last atlas the second
            // condition is implied by the first, so the
            // count is the design's count there, and at
            // the last atlas it is the empty count the
            // departures closing states.
            let lasts = section
                .iter()
                .filter(|line| {
                    line.contains(&format!("last stood at atlas {atlas_no},"))
                        && line.ends_with("does not stand at the close")
                })
                .count();
            assert_eq!(
                lasts,
                figure(
                    departures_closing,
                    &format!("atlas {atlas_no} departures "),
                    ","
                ),
                "the last standings of atlas {atlas_no} restate its departures over {name}"
            );
        }
    }
}

#[test]
fn the_section_lines_are_the_standing_lines() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_presence(&chain).expect("the presence reads");
        let found = sections(&lines);
        let standing = render_standing(&chain).expect("the standing reads");
        let standing_closing = standing.last().expect("a closing line");
        for line in found.iter().flatten() {
            assert!(
                standing.contains(line),
                "the section line stands in the standing reading over {name}: {line}"
            );
        }
        let distinct: std::collections::BTreeSet<String> =
            found.iter().flatten().map(|line| label_of(line)).collect();
        let everywhere = distinct
            .iter()
            .filter(|label| {
                found.iter().all(|section| {
                    section
                        .iter()
                        .any(|line| line.starts_with(&format!("entry {label},")))
                })
            })
            .count();
        assert_eq!(
            everywhere,
            figure(standing_closing, "standing at every atlas ", ","),
            "the labels in every section are the standing at every atlas over {name}"
        );
        assert_eq!(
            found.last().expect("a last section").len(),
            figure_last(standing_closing, "standing at the close "),
            "the last section names the standing at the close over {name}"
        );
    }
}

#[test]
fn the_closing_sums_both_ways_agree() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_presence(&chain).expect("the presence reads");
        let closing = lines.last().expect("a closing line");
        let found = sections(&lines);
        let by_atlas: usize = (1..=found.len())
            .map(|index| figure(closing, &format!("atlas {index} standing "), ","))
            .sum();
        let mut by_label: std::collections::BTreeMap<String, usize> =
            std::collections::BTreeMap::new();
        for line in found.iter().flatten() {
            by_label.insert(label_of(line), figure(line, "stands in ", " atlases"));
        }
        let summed: usize = by_label.values().sum();
        assert_eq!(by_atlas, summed, "the standings sum both ways over {name}");
        let entries = figure(closing, "entries ", ",");
        assert_eq!(entries, by_label.len(), "the entries agree over {name}");
        let standing = render_standing(&chain).expect("the standing reads");
        let standing_closing = standing.last().expect("a closing line");
        assert_eq!(
            entries,
            figure(standing_closing, "entries ", ","),
            "the entries figure is the standing reading's over {name}"
        );
    }
}

#[test]
fn a_two_atlas_chain_renders_its_presence() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", &["both", "first only"]),
            atlas("Second", &["both", "second only"]),
        ],
    };
    let lines = render_presence(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "entry both, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, stands at the close",
            "entry first only, first stood at atlas 1, last stood at atlas 1, stands in 1 atlases, does not stand at the close",
            "atlas 2",
            "entry both, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, stands at the close",
            "entry second only, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, stands at the close",
            "presence counts, atlas 1 standing 2, atlas 2 standing 2, entries 3, atlases 2",
        ])
    );
}

#[test]
fn an_empty_atlas_holds_an_empty_section() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", &["kept"]),
            atlas("Second", &[]),
            atlas("Third", &["kept"]),
        ],
    };
    let lines = render_presence(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "entry kept, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "atlas 2",
            "atlas 3",
            "entry kept, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close",
            "presence counts, atlas 1 standing 1, atlas 2 standing 0, atlas 3 standing 1, entries 1, atlases 3",
        ])
    );
}

#[test]
fn a_label_held_twice_in_one_atlas_stands_once_in_its_section() {
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
    let lines = render_presence(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "entry held twice, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, stands at the close",
            "atlas 2",
            "entry held twice, first stood at atlas 1, last stood at atlas 2, stands in 2 atlases, stands at the close",
            "presence counts, atlas 1 standing 1, atlas 2 standing 1, entries 1, atlases 2",
        ])
    );
}

#[test]
fn a_lineage_holding_fewer_than_two_atlases_is_refused() {
    let chain = Lineage {
        atlases: vec![atlas("Only", &["kept"])],
    };
    assert_eq!(
        render_presence(&chain).expect_err("the short chain is refused"),
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
        render_presence(&chain).expect_err("the surrogate label is refused"),
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
        render_presence(&chain).expect_err("the doubly broken label names the surrogate first"),
        "an entry label holds an unpaired surrogate"
    );
}

#[test]
fn a_label_holding_a_line_break_is_refused() {
    let chain = Lineage {
        atlases: vec![atlas("First", &["a\nb"]), atlas("Second", &["kept"])],
    };
    assert_eq!(
        render_presence(&chain).expect_err("the broken label is refused"),
        "an entry label holds a newline or a carriage return"
    );
}

/// A thin chain of n atlases built in process, atlas i
/// holding labels i and i plus one cyclically, so each
/// of the chain's labels stands in exactly two atlases.
fn thin_chain(n: usize) -> Lineage {
    let mut atlases = Vec::new();
    for i in 0..n {
        let first = format!("label-{}", i + 1);
        let second = format!("label-{}", (i + 1) % n + 1);
        let title = format!("atlas-{}", i + 1);
        atlases.push(atlas(&title, &[&first, &second]));
    }
    Lineage { atlases }
}

/// A window chain of one hundred and twenty atlases
/// built in process over thirty labels, each atlas
/// holding the five labels of a window turning by one
/// label per atlas, so each label stands in twenty
/// atlases.
fn window_chain() -> Lineage {
    let mut atlases = Vec::new();
    for i in 0..120usize {
        let labels: Vec<String> = (0..5)
            .map(|j| format!("label-{}", (i + j) % 30 + 1))
            .collect();
        let refs: Vec<&str> = labels.iter().map(String::as_str).collect();
        let title = format!("atlas-{}", i + 1);
        atlases.push(atlas(&title, &refs));
    }
    Lineage { atlases }
}

/// A paired chain of one hundred and twenty atlases
/// built in process over five labels, its odd atlases
/// holding label-2, label-1, and label-4 in that member
/// order and its even atlases holding label-3, label-5,
/// and label-2, so label-2 stands in every atlas and
/// each other label in sixty.
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

#[test]
fn the_thin_chain_presence_stands_as_counted() {
    let lines = render_presence(&thin_chain(302)).expect("the chain reads");
    assert_eq!(lines.len(), 907);
    let secs = sections(&lines);
    assert_eq!(secs.len(), 302);
    assert_eq!(
        secs[0],
        strings(&[
            "entry label-1, first stood at atlas 1, last stood at atlas 302, \
             stands in 2 atlases, stands at the close",
            "entry label-2, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, does not stand at the close",
        ])
    );
    assert_eq!(
        secs[151],
        strings(&[
            "entry label-152, first stood at atlas 151, last stood at atlas 152, \
             stands in 2 atlases, does not stand at the close",
            "entry label-153, first stood at atlas 152, last stood at atlas 153, \
             stands in 2 atlases, does not stand at the close",
        ])
    );
    assert_eq!(
        secs[301],
        strings(&[
            "entry label-302, first stood at atlas 301, last stood at atlas 302, \
             stands in 2 atlases, stands at the close",
            "entry label-1, first stood at atlas 1, last stood at atlas 302, \
             stands in 2 atlases, stands at the close",
        ])
    );
    let mut closing = String::from("presence counts");
    for i in 1..=302 {
        closing.push_str(&format!(", atlas {i} standing 2"));
    }
    closing.push_str(", entries 302, atlases 302");
    assert_eq!(lines[906], closing);
}

#[test]
fn the_window_chain_presence_stands_as_counted() {
    let lines = render_presence(&window_chain()).expect("the chain reads");
    assert_eq!(lines.len(), 721);
    let secs = sections(&lines);
    assert_eq!(secs.len(), 120);
    assert_eq!(
        secs[0],
        strings(&[
            "entry label-1, first stood at atlas 1, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
            "entry label-2, first stood at atlas 1, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
            "entry label-3, first stood at atlas 1, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
            "entry label-4, first stood at atlas 1, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
            "entry label-5, first stood at atlas 1, last stood at atlas 95, \
             stands in 20 atlases, does not stand at the close",
        ])
    );
    assert_eq!(
        secs[15],
        strings(&[
            "entry label-16, first stood at atlas 12, last stood at atlas 106, \
             stands in 20 atlases, does not stand at the close",
            "entry label-17, first stood at atlas 13, last stood at atlas 107, \
             stands in 20 atlases, does not stand at the close",
            "entry label-18, first stood at atlas 14, last stood at atlas 108, \
             stands in 20 atlases, does not stand at the close",
            "entry label-19, first stood at atlas 15, last stood at atlas 109, \
             stands in 20 atlases, does not stand at the close",
            "entry label-20, first stood at atlas 16, last stood at atlas 110, \
             stands in 20 atlases, does not stand at the close",
        ])
    );
    assert_eq!(
        secs[119],
        strings(&[
            "entry label-30, first stood at atlas 26, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
            "entry label-1, first stood at atlas 1, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
            "entry label-2, first stood at atlas 1, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
            "entry label-3, first stood at atlas 1, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
            "entry label-4, first stood at atlas 1, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
        ])
    );
    let mut closing = String::from("presence counts");
    for i in 1..=120 {
        closing.push_str(&format!(", atlas {i} standing 5"));
    }
    closing.push_str(", entries 30, atlases 120");
    assert_eq!(lines[720], closing);
}

#[test]
fn the_paired_chain_presence_stands_as_counted() {
    let lines = render_presence(&paired_chain()).expect("the chain reads");
    assert_eq!(lines.len(), 481);
    let secs = sections(&lines);
    assert_eq!(secs.len(), 120);
    assert_eq!(
        secs[0],
        strings(&[
            "entry label-2, first stood at atlas 1, last stood at atlas 120, \
             stands in 120 atlases, stands at the close",
            "entry label-1, first stood at atlas 1, last stood at atlas 119, \
             stands in 60 atlases, does not stand at the close",
            "entry label-4, first stood at atlas 1, last stood at atlas 119, \
             stands in 60 atlases, does not stand at the close",
        ])
    );
    assert_eq!(
        secs[1],
        strings(&[
            "entry label-3, first stood at atlas 2, last stood at atlas 120, \
             stands in 60 atlases, stands at the close",
            "entry label-5, first stood at atlas 2, last stood at atlas 120, \
             stands in 60 atlases, stands at the close",
            "entry label-2, first stood at atlas 1, last stood at atlas 120, \
             stands in 120 atlases, stands at the close",
        ])
    );
    assert_eq!(secs[119], secs[1]);
    let mut closing = String::from("presence counts");
    for i in 1..=120 {
        closing.push_str(&format!(", atlas {i} standing 3"));
    }
    closing.push_str(", entries 5, atlases 120");
    assert_eq!(lines[480], closing);
}

#[test]
fn the_sections_hold_the_standing_lines_in_entry_order() {
    let mut chains = vec![
        load("winter-spring-summer-lineage.json"),
        load("radio-investigation-lineage.json"),
        load("constructed-departures-lineage.json"),
        load("generated-chain-lineage.json"),
    ];
    chains.push(thin_chain(302));
    chains.push(window_chain());
    chains.push(paired_chain());
    for chain in &chains {
        let standing = render_standing(chain).expect("the chain reads");
        let mut by_label: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for line in &standing[..standing.len() - 1] {
            by_label.insert(label_of(line), line.clone());
        }
        let lines = render_presence(chain).expect("the chain reads");
        let secs = sections(&lines);
        assert_eq!(secs.len(), chain.atlases.len());
        let mut entry_lines = 0usize;
        for (index, atlas) in chain.atlases.iter().enumerate() {
            let mut expected: Vec<String> = Vec::new();
            let mut seen: Vec<String> = Vec::new();
            for entry in &atlas.entries {
                let label = entry.label.to_string();
                if !seen.contains(&label) {
                    seen.push(label.clone());
                    expected.push(
                        by_label
                            .get(&label)
                            .expect("every standing label holds a standing line")
                            .clone(),
                    );
                }
            }
            assert_eq!(secs[index], expected);
            entry_lines += secs[index].len();
        }
        let counted: usize = standing[..standing.len() - 1]
            .iter()
            .map(|line| figure(line, "stands in ", " atlases"))
            .sum();
        assert_eq!(entry_lines, counted);
        let closing = lines.last().expect("a closing stands");
        assert_eq!(figure(closing, ", entries ", ", atlases"), by_label.len());
        for (index, section) in secs.iter().enumerate() {
            let head = format!(", atlas {} standing ", index + 1);
            assert_eq!(figure(closing, &head, ","), section.len());
        }
    }
}
