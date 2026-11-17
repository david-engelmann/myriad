//! Tests for MY-26, the departures reading.
//!
//! The reading takes a lineage, reads the presence of
//! every label in the one presence pass, and states one
//! section for each atlas of the chain, the entries that
//! stand in no later atlas named after the atlas they
//! were last seen in, in the standing line form, closing
//! with the departures counts. The pins here read the
//! reading over hand built chains and over lineage texts
//! built as JSON, and bind its closings to the absence
//! reading's closing runs and to the standing reading's
//! own lines.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_absence, render_departures, render_standing, Atlas, AtlasEntry,
    Lineage,
};
use myriad::explorations::{Branch, Exploration};
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

/// The example chain of the reading's form, three
/// atlases over ferry, lifeboat, and ice.
fn form_chain() -> Lineage {
    Lineage {
        atlases: vec![
            atlas("One", &["ferry", "lifeboat"]),
            atlas("Two", &["lifeboat", "ice"]),
            atlas("Three", &["ice", "ferry"]),
        ],
    }
}

/// A chain where an entry leaves, returns, and leaves
/// again, departing once, after its last standing.
fn return_chain() -> Lineage {
    Lineage {
        atlases: vec![
            atlas("One", &["alpha", "beta"]),
            atlas("Two", &["beta"]),
            atlas("Three", &["alpha", "beta"]),
            atlas("Four", &["beta"]),
        ],
    }
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

/// The departures after each atlas, read out of the
/// absence reading's own closing runs: a run that starts
/// at atlas N and runs to the close departs after atlas
/// N - 1.
fn closing_run_departures(chain: &Lineage) -> Vec<usize> {
    let absence = render_absence(chain).expect("the chain reads");
    let mut grouped = vec![0usize; chain.atlases.len()];
    for line in &absence {
        if line.contains(", to the close") {
            let start = figure(line, "absent from atlas ", ",");
            grouped[start - 2] += 1;
        }
    }
    grouped
}

#[test]
fn the_departures_reading_states_its_form() {
    let lines = render_departures(&form_chain()).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "atlas 2",
            "entry lifeboat, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, does not stand at the close",
            "atlas 3",
            "departures counts, atlas 1 departures 0, atlas 2 departures 1, \
             atlas 3 departures 0, entries departed 1, entries 3, atlases 3",
        ])
    );
}

#[test]
fn an_entry_that_leaves_returns_and_leaves_again_departs_once() {
    // Alpha stands absent through the second atlas and
    // stands again in the third, so the absence inside
    // the chain is not a departure, and alpha departs
    // after the third atlas alone.
    let lines = render_departures(&return_chain()).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "atlas 2",
            "atlas 3",
            "entry alpha, first stood at atlas 1, last stood at atlas 3, \
             stands in 2 atlases, does not stand at the close",
            "atlas 4",
            "departures counts, atlas 1 departures 0, atlas 2 departures 0, \
             atlas 3 departures 1, atlas 4 departures 0, entries departed 1, \
             entries 2, atlases 4",
        ])
    );
}

#[test]
fn the_last_atlas_always_departs_nothing() {
    for chain in [form_chain(), return_chain()] {
        let lines = render_departures(&chain).expect("the chain reads");
        let closing = lines.last().expect("a closing line");
        let total = chain.atlases.len();
        let last = figure(closing, &format!("atlas {total} departures "), ",");
        assert_eq!(last, 0);
        // The last section header stands directly above
        // the counts line, with no entry under it.
        assert_eq!(lines[lines.len() - 2], format!("atlas {total}"));
    }
}

#[test]
fn the_per_atlas_departures_agree_with_the_absence_closing_runs() {
    for chain in [form_chain(), return_chain()] {
        let lines = render_departures(&chain).expect("the chain reads");
        let closing = lines.last().expect("a closing line");
        let grouped = closing_run_departures(&chain);
        let mut sum = 0usize;
        for (index, expected) in grouped.iter().enumerate() {
            let count = figure(closing, &format!("atlas {} departures ", index + 1), ",");
            assert_eq!(count, *expected, "atlas {} departures", index + 1);
            sum += count;
        }
        let absence = render_absence(&chain).expect("the chain reads");
        let absence_closing = absence.last().expect("a closing line");
        let at_close: usize = absence_closing
            .split("absent at the close ")
            .nth(1)
            .expect("the closing figure stands")
            .parse()
            .expect("a count");
        assert_eq!(figure(closing, "entries departed ", ","), at_close);
        assert_eq!(sum, at_close);
    }
}

#[test]
fn every_named_entry_line_is_the_standing_line_of_an_entry_absent_at_the_close() {
    for chain in [form_chain(), return_chain()] {
        let standing = render_standing(&chain).expect("the chain reads");
        let lines = render_departures(&chain).expect("the chain reads");
        let mut section = 0usize;
        for line in &lines {
            if let Some(number) = line.strip_prefix("atlas ") {
                section = number.parse().expect("a section number");
            } else if line.starts_with("entry ") {
                assert!(standing.contains(line), "the line stands: {line}");
                assert!(line.ends_with("does not stand at the close"));
                let last = figure(line, ", last stood at atlas ", ",");
                assert_eq!(last, section, "the entry departs after its last standing");
            }
        }
    }
}

#[test]
fn fewer_than_two_atlases_draws_the_count_refusal() {
    let chain = Lineage {
        atlases: vec![atlas("One", &["a"])],
    };
    assert_eq!(
        render_departures(&chain),
        Err("the lineage holds fewer than two atlases".to_string())
    );
}

#[test]
fn a_label_holding_an_unpaired_surrogate_is_refused() {
    let mut broken = atlas("Two", &["a"]);
    broken.entries[0].label = PText(vec![0x61, 0xD800]);
    let chain = Lineage {
        atlases: vec![atlas("One", &["a"]), broken],
    };
    assert_eq!(
        render_departures(&chain),
        Err("an entry label holds an unpaired surrogate".to_string())
    );
}

#[test]
fn a_label_holding_a_line_break_is_refused() {
    let chain = Lineage {
        atlases: vec![atlas("One", &["a"]), atlas("Two", &["a\nb"])],
    };
    assert_eq!(
        render_departures(&chain),
        Err("an entry label holds a newline or a carriage return".to_string())
    );
}

fn atlas_text(title: &str, labels: &[&str]) -> String {
    let entries: Vec<String> = labels
        .iter()
        .map(|label| {
            jobj(&[
                ("label", jstr(label)),
                ("exploration", exploration_json("Q", &[])),
            ])
        })
        .collect();
    jobj(&[("title", jstr(title)), ("entries", jarr(&entries))])
}

#[test]
fn a_loaded_chain_renders_the_reading_of_its_labels() {
    let text = jobj(&[(
        "atlases",
        jarr(&[
            atlas_text("One", &["kept", "gone"]),
            atlas_text("Two", &["kept"]),
            atlas_text("Three", &["kept", "gone", "new"]),
        ]),
    )]);
    let (found, defects) = load_lineage_text(&text);
    assert_eq!(defects, Vec::<String>::new());
    let lineage = found.expect("the chain loads");
    let lines = render_departures(&lineage).expect("the chain reads");
    // Gone returns in the third atlas and stands at the
    // close, so nothing departs at all.
    assert_eq!(
        lines.last().expect("a closing line"),
        "departures counts, atlas 1 departures 0, atlas 2 departures 0, \
         atlas 3 departures 0, entries departed 0, entries 3, atlases 3"
    );
}

#[test]
fn a_chain_past_a_thousand_branches_renders_whole() {
    let wide = |labels: &[&str]| -> Atlas {
        Atlas {
            title: pt("T"),
            entries: labels
                .iter()
                .map(|label| AtlasEntry {
                    label: pt(label),
                    exploration: Exploration {
                        question: pt("q"),
                        branches: (0..40)
                            .map(|n| branch(&format!("b{n}"), None, "p", "open", None, None, None))
                            .collect::<Vec<Branch>>(),
                    },
                })
                .collect(),
        }
    };
    let labels: Vec<String> = (0..30).map(|i| format!("entry {i}")).collect();
    let refs: Vec<&str> = labels.iter().map(String::as_str).collect();
    let chain = Lineage {
        atlases: vec![wide(&refs), wide(&["entry 0"])],
    };
    let lines = render_departures(&chain).expect("the chain reads");
    assert_eq!(lines.len(), 1 + 29 + 1 + 1);
    assert_eq!(
        lines.last().expect("a closing line"),
        "departures counts, atlas 1 departures 29, atlas 2 departures 0, \
         entries departed 29, entries 30, atlases 2"
    );
}

#[test]
fn the_reading_holds_the_vocabulary_bar() {
    let lines = render_departures(&form_chain()).expect("the chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert!(holds_vocabulary_bar(&text));
}

#[test]
fn a_label_held_twice_in_one_atlas_departs_once() {
    // FIX-MY-17. The loaders refuse a label carried by
    // two entries, so no loaded chain reaches this shape,
    // and the shape stands on the library surface alone.
    // Over it the reading reads the label once, at its
    // first standing in the atlas, and the departures
    // taken together equal the count the absence reading
    // states as absent at the close over the same value.
    // On the unrepaired reading the standing line of dup
    // stands twice in the first section and the closing
    // states entries departed 3, entries 3, so this pin
    // fails there by assertion.
    let chain = Lineage {
        atlases: vec![
            atlas("One", &["dup", "dup", "solo"]),
            atlas("Two", &["other"]),
        ],
    };
    let lines = render_departures(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "entry dup, first stood at atlas 1, last stood at atlas 1, \
             stands in 1 atlases, does not stand at the close",
            "entry solo, first stood at atlas 1, last stood at atlas 1, \
             stands in 1 atlases, does not stand at the close",
            "atlas 2",
            "departures counts, atlas 1 departures 2, atlas 2 departures 0, \
             entries departed 2, entries 3, atlases 2",
        ])
    );
    let absence = render_absence(&chain).expect("the chain reads");
    let absence_closing = absence.last().expect("a closing line");
    let at_close: usize = absence_closing
        .split("absent at the close ")
        .nth(1)
        .expect("the closing figure stands")
        .parse()
        .expect("a count");
    let closing = lines.last().expect("a closing line");
    assert_eq!(figure(closing, "entries departed ", ","), at_close);
}

#[test]
fn a_label_held_three_times_in_one_atlas_departs_once() {
    // FIX-MY-17. The strongest form of the shape above.
    // On the unrepaired reading the closing states
    // entries departed 3, entries 2, a count of departed
    // entries larger than the count of entries itself,
    // so this pin fails there by assertion.
    let chain = Lineage {
        atlases: vec![
            atlas("One", &["dup", "dup", "dup"]),
            atlas("Two", &["other"]),
        ],
    };
    let lines = render_departures(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "entry dup, first stood at atlas 1, last stood at atlas 1, \
             stands in 1 atlases, does not stand at the close",
            "atlas 2",
            "departures counts, atlas 1 departures 1, atlas 2 departures 0, \
             entries departed 1, entries 2, atlases 2",
        ])
    );
    let absence = render_absence(&chain).expect("the chain reads");
    let absence_closing = absence.last().expect("a closing line");
    let at_close: usize = absence_closing
        .split("absent at the close ")
        .nth(1)
        .expect("the closing figure stands")
        .parse()
        .expect("a count");
    let closing = lines.last().expect("a closing line");
    assert_eq!(figure(closing, "entries departed ", ","), at_close);
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

/// The entry lines of each section of a departures
/// rendering, in section order, the closing line read
/// as no section.
fn dep_sections(lines: &[String]) -> Vec<Vec<String>> {
    let mut out: Vec<Vec<String>> = Vec::new();
    for line in lines {
        if line.starts_with("atlas ") && !line.contains(',') {
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

#[test]
fn the_thin_chain_departures_stand_as_counted() {
    let lines = render_departures(&thin_chain(302)).expect("the chain reads");
    assert_eq!(lines.len(), 603);
    let secs = dep_sections(&lines);
    assert_eq!(secs.len(), 302);
    assert_eq!(secs[0], Vec::<String>::new());
    assert_eq!(
        secs[1],
        strings(&[
            "entry label-2, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, does not stand at the close",
        ])
    );
    assert_eq!(
        secs[151],
        strings(&[
            "entry label-152, first stood at atlas 151, last stood at atlas 152, \
             stands in 2 atlases, does not stand at the close",
        ])
    );
    assert_eq!(secs[301], Vec::<String>::new());
    let mut closing = String::from("departures counts");
    for i in 1..=302 {
        let count = if i == 1 || i == 302 { 0 } else { 1 };
        closing.push_str(&format!(", atlas {i} departures {count}"));
    }
    closing.push_str(", entries departed 300, entries 302, atlases 302");
    assert_eq!(lines[602], closing);
}

#[test]
fn the_window_chain_departures_stand_as_counted() {
    let lines = render_departures(&window_chain()).expect("the chain reads");
    assert_eq!(lines.len(), 146);
    let secs = dep_sections(&lines);
    assert_eq!(secs.len(), 120);
    assert_eq!(secs[0], Vec::<String>::new());
    assert_eq!(
        secs[94],
        strings(&[
            "entry label-5, first stood at atlas 1, last stood at atlas 95, \
             stands in 20 atlases, does not stand at the close",
        ])
    );
    assert_eq!(
        secs[95],
        strings(&[
            "entry label-6, first stood at atlas 2, last stood at atlas 96, \
             stands in 20 atlases, does not stand at the close",
        ])
    );
    assert_eq!(secs[119], Vec::<String>::new());
    let mut closing = String::from("departures counts");
    for i in 1..=120 {
        let count = if (95..=119).contains(&i) { 1 } else { 0 };
        closing.push_str(&format!(", atlas {i} departures {count}"));
    }
    closing.push_str(", entries departed 25, entries 30, atlases 120");
    assert_eq!(lines[145], closing);
}

#[test]
fn the_paired_chain_departures_stand_as_counted() {
    let lines = render_departures(&paired_chain()).expect("the chain reads");
    assert_eq!(lines.len(), 123);
    let secs = dep_sections(&lines);
    assert_eq!(secs.len(), 120);
    assert_eq!(secs[0], Vec::<String>::new());
    assert_eq!(
        secs[118],
        strings(&[
            "entry label-1, first stood at atlas 1, last stood at atlas 119, \
             stands in 60 atlases, does not stand at the close",
            "entry label-4, first stood at atlas 1, last stood at atlas 119, \
             stands in 60 atlases, does not stand at the close",
        ])
    );
    assert_eq!(secs[119], Vec::<String>::new());
    let mut closing = String::from("departures counts");
    for i in 1..=120 {
        let count = if i == 119 { 2 } else { 0 };
        closing.push_str(&format!(", atlas {i} departures {count}"));
    }
    closing.push_str(", entries departed 2, entries 5, atlases 120");
    assert_eq!(lines[122], closing);
}

#[test]
fn the_sections_hold_the_departing_lines_in_entry_order() {
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
        let total = chain.atlases.len();
        let standing = render_standing(chain).expect("the chain reads");
        let mut by_label: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        for line in &standing[..standing.len() - 1] {
            by_label.insert(label_of(line), line.clone());
        }
        let lines = render_departures(chain).expect("the chain reads");
        let secs = dep_sections(&lines);
        assert_eq!(secs.len(), total);
        let mut entry_lines = 0usize;
        for (index, atlas) in chain.atlases.iter().enumerate() {
            let mut expected: Vec<String> = Vec::new();
            if index + 1 < total {
                let mut seen: Vec<String> = Vec::new();
                for entry in &atlas.entries {
                    let label = entry.label.to_string();
                    if !seen.contains(&label) {
                        seen.push(label.clone());
                        let line = by_label
                            .get(&label)
                            .expect("every standing label holds a standing line");
                        if figure(line, ", last stood at atlas ", ",") == index + 1 {
                            expected.push(line.clone());
                        }
                    }
                }
            }
            assert_eq!(secs[index], expected);
            entry_lines += secs[index].len();
        }
        assert_eq!(lines.len(), total + entry_lines + 1);
        let closing = lines.last().expect("a closing stands");
        for (index, section) in secs.iter().enumerate() {
            let head = format!("atlas {} departures ", index + 1);
            assert_eq!(figure(closing, &head, ","), section.len());
        }
        assert_eq!(figure(closing, "entries departed ", ","), entry_lines);
        let entries: usize = closing
            .split("entries departed ")
            .nth(1)
            .expect("the departed figure stands")
            .split(", entries ")
            .nth(1)
            .expect("the entries figure stands")
            .split(", atlases")
            .next()
            .expect("the entries figure stands")
            .parse()
            .expect("a count");
        assert_eq!(entries, by_label.len());
        let absence = render_absence(chain).expect("the chain reads");
        let absence_closing = absence.last().expect("a closing line");
        let at_close: usize = absence_closing
            .split("absent at the close ")
            .nth(1)
            .expect("the closing figure stands")
            .parse()
            .expect("a count");
        assert_eq!(entry_lines, at_close);
    }
}
