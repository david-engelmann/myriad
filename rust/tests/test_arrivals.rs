//! Tests for MY-23 and MY-24, the arrivals reading.
//!
//! The reading takes a lineage, reads the presence of
//! every label in the one presence pass, and states one
//! section for each atlas of the chain, the entries
//! arriving at that atlas in the standing line form,
//! closing with the arrivals counts. The pins here read
//! the reading over hand built chains and over lineage
//! texts built as JSON, and bind its closings to the
//! standing reading and to the index reading.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_arrivals, render_index, render_standing, Atlas, AtlasEntry, Lineage,
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

#[test]
fn the_arrivals_reading_states_its_form() {
    let lines = render_arrivals(&form_chain()).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "entry ferry, first stood at atlas 1, last stood at atlas 3, \
             stands in 2 atlases, stands at the close",
            "entry lifeboat, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, does not stand at the close",
            "atlas 2",
            "entry ice, first stood at atlas 2, last stood at atlas 3, \
             stands in 2 atlases, stands at the close",
            "atlas 3",
            "arrivals counts, atlas 1 arrivals 2, atlas 2 arrivals 1, \
             atlas 3 arrivals 0, entries 3, atlases 3",
        ])
    );
}

#[test]
fn an_entry_standing_again_does_not_arrive_again() {
    // The ferry stands in the first and the third atlas
    // of the form chain and arrives only at the first,
    // its standing again the return the returns reading
    // names, and the third section stands empty.
    let lines = render_arrivals(&form_chain()).expect("the chain reads");
    let third = lines
        .iter()
        .position(|line| line == "atlas 3")
        .expect("the third section opens");
    assert!(lines[third + 1].starts_with("arrivals counts"));
    let ferry_lines = lines
        .iter()
        .filter(|line| line.starts_with("entry ferry,"))
        .count();
    assert_eq!(ferry_lines, 1);
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

#[test]
fn the_arrivals_sum_to_the_standing_entries() {
    let chain = form_chain();
    let arrivals = render_arrivals(&chain).expect("the chain reads");
    let closing = arrivals.last().expect("a closing line");
    let sum: usize = (1..=3)
        .map(|n| figure(closing, &format!("atlas {n} arrivals "), ","))
        .sum();
    let standing = render_standing(&chain).expect("the chain reads");
    let standing_closing = standing.last().expect("a closing line");
    let entries = figure(standing_closing, "entries ", ",");
    assert_eq!(sum, entries);
    assert_eq!(sum, 3);
}

#[test]
fn the_first_atlas_arrivals_equal_the_first_atlas_entries() {
    let chain = form_chain();
    let arrivals = render_arrivals(&chain).expect("the chain reads");
    let closing = arrivals.last().expect("a closing line");
    let first = figure(closing, "atlas 1 arrivals ", ",");
    let index = render_index(&chain.atlases[0]).expect("the atlas reads");
    let index_closing = index.last().expect("a closing line");
    let entries = figure(index_closing, "atlas counts, entries ", ",");
    assert_eq!(first, entries);
    assert_eq!(first, 2);
}

#[test]
fn the_labels_at_every_atlas_stand_in_the_first_section() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", &["harbour", "ferry"]),
            atlas("Two", &["harbour", "ice"]),
            atlas("Three", &["harbour"]),
        ],
    };
    let standing = render_standing(&chain).expect("the chain reads");
    let arrivals = render_arrivals(&chain).expect("the chain reads");
    let second = arrivals
        .iter()
        .position(|line| line == "atlas 2")
        .expect("the second section opens");
    let first_section = &arrivals[1..second];
    let at_every: Vec<&String> = standing
        .iter()
        .filter(|line| line.starts_with("entry ") && line.contains("stands in 3 atlases"))
        .collect();
    assert_eq!(at_every.len(), 1);
    for line in &at_every {
        assert!(first_section.contains(line), "line {line}");
    }
    assert!(at_every.len() <= first_section.len());
}

#[test]
fn a_section_where_nothing_arrives_stands_as_its_opening() {
    let chain = Lineage {
        atlases: vec![
            atlas("One", &["a", "b"]),
            atlas("Two", &["b", "a"]),
            atlas("Three", &["c"]),
        ],
    };
    let lines = render_arrivals(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "entry a, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, does not stand at the close",
            "entry b, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, does not stand at the close",
            "atlas 2",
            "atlas 3",
            "entry c, first stood at atlas 3, last stood at atlas 3, \
             stands in 1 atlases, stands at the close",
            "arrivals counts, atlas 1 arrivals 2, atlas 2 arrivals 0, \
             atlas 3 arrivals 1, entries 3, atlases 3",
        ])
    );
}

#[test]
fn a_chain_of_two_renders_its_arrivals() {
    let chain = Lineage {
        atlases: vec![atlas("One", &["a"]), atlas("Two", &["a", "b"])],
    };
    let lines = render_arrivals(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "entry a, first stood at atlas 1, last stood at atlas 2, \
             stands in 2 atlases, stands at the close",
            "atlas 2",
            "entry b, first stood at atlas 2, last stood at atlas 2, \
             stands in 1 atlases, stands at the close",
            "arrivals counts, atlas 1 arrivals 1, atlas 2 arrivals 1, \
             entries 2, atlases 2",
        ])
    );
}

#[test]
fn fewer_than_two_atlases_draws_the_count_refusal() {
    let chain = Lineage {
        atlases: vec![atlas("One", &["a"])],
    };
    assert_eq!(
        render_arrivals(&chain),
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
        render_arrivals(&chain),
        Err("an entry label holds an unpaired surrogate".to_string())
    );
}

#[test]
fn a_label_holding_a_line_break_is_refused() {
    let chain = Lineage {
        atlases: vec![atlas("One", &["a"]), atlas("Two", &["a\nb"])],
    };
    assert_eq!(
        render_arrivals(&chain),
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
    let lines = render_arrivals(&lineage).expect("the chain reads");
    assert_eq!(
        lines.last().expect("a closing line"),
        "arrivals counts, atlas 1 arrivals 2, atlas 2 arrivals 0, \
         atlas 3 arrivals 1, entries 3, atlases 3"
    );
}

#[test]
fn a_member_defect_loads_no_chain_at_all() {
    let member = jobj(&[("entries", jarr(&[]))]);
    let text = jobj(&[("atlases", jarr(&[atlas_text("One", &["only"]), member]))]);
    let (found, defects) = load_lineage_text(&text);
    assert!(found.is_none());
    assert_eq!(defects, strings(&["atlas 1, missing field title"]));
}

#[test]
fn a_chain_past_a_thousand_branches_renders_whole() {
    let wide = |label: &str| -> Atlas {
        let branches: Vec<Branch> = (0..600)
            .map(|n| branch(&format!("b{n}"), None, "p", "open", None, None, None))
            .collect();
        Atlas {
            title: pt("T"),
            entries: vec![AtlasEntry {
                label: pt(label),
                exploration: Exploration {
                    question: pt("q"),
                    branches,
                },
            }],
        }
    };
    let chain = Lineage {
        atlases: vec![wide("one"), wide("two")],
    };
    let lines = render_arrivals(&chain).expect("the chain reads");
    assert_eq!(
        lines.last().expect("a closing line"),
        "arrivals counts, atlas 1 arrivals 1, atlas 2 arrivals 1, \
         entries 2, atlases 2"
    );
}

#[test]
fn the_reading_holds_the_vocabulary_bar() {
    let lines = render_arrivals(&form_chain()).expect("the chain reads");
    let text: String = lines.iter().map(|line| format!("{line}\n")).collect();
    assert!(holds_vocabulary_bar(&text));
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

/// The entry lines of each section of an arrivals
/// rendering, in section order, the closing line read
/// as no section.
fn arr_sections(lines: &[String]) -> Vec<Vec<String>> {
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
fn the_thin_chain_arrivals_stand_as_counted() {
    let lines = render_arrivals(&thin_chain(302)).expect("the chain reads");
    assert_eq!(lines.len(), 605);
    let secs = arr_sections(&lines);
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
            "entry label-153, first stood at atlas 152, last stood at atlas 153, \
             stands in 2 atlases, does not stand at the close",
        ])
    );
    assert_eq!(secs[301], Vec::<String>::new());
    let mut closing = String::from("arrivals counts");
    for i in 1..=302 {
        let count = if i == 1 {
            2
        } else if i == 302 {
            0
        } else {
            1
        };
        closing.push_str(&format!(", atlas {i} arrivals {count}"));
    }
    closing.push_str(", entries 302, atlases 302");
    assert_eq!(lines[604], closing);
}

#[test]
fn the_window_chain_arrivals_stand_as_counted() {
    let lines = render_arrivals(&window_chain()).expect("the chain reads");
    assert_eq!(lines.len(), 151);
    let secs = arr_sections(&lines);
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
        secs[25],
        strings(&[
            "entry label-30, first stood at atlas 26, last stood at atlas 120, \
             stands in 20 atlases, stands at the close",
        ])
    );
    assert_eq!(secs[26], Vec::<String>::new());
    assert_eq!(secs[119], Vec::<String>::new());
    let mut closing = String::from("arrivals counts");
    for i in 1..=120 {
        let count = if i == 1 {
            5
        } else if i <= 26 {
            1
        } else {
            0
        };
        closing.push_str(&format!(", atlas {i} arrivals {count}"));
    }
    closing.push_str(", entries 30, atlases 120");
    assert_eq!(lines[150], closing);
}

#[test]
fn the_paired_chain_arrivals_stand_as_counted() {
    let lines = render_arrivals(&paired_chain()).expect("the chain reads");
    assert_eq!(lines.len(), 126);
    let secs = arr_sections(&lines);
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
        ])
    );
    assert_eq!(secs[119], Vec::<String>::new());
    let mut closing = String::from("arrivals counts");
    for i in 1..=120 {
        let count = if i == 1 {
            3
        } else if i == 2 {
            2
        } else {
            0
        };
        closing.push_str(&format!(", atlas {i} arrivals {count}"));
    }
    closing.push_str(", entries 5, atlases 120");
    assert_eq!(lines[125], closing);
}

#[test]
fn a_label_held_more_than_once_in_one_atlas_arrives_once() {
    // FIX-MY-17. A label held more than once in one
    // atlas arrives once, at its first standing in
    // that atlas. On a reading that takes the label
    // at every entry that carries it the first section
    // holds its line once per entry and the closing
    // counts it once per entry, so this pin fails
    // there by assertion.
    let twice = Lineage {
        atlases: vec![atlas("One", &["dup", "dup"]), atlas("Two", &["other"])],
    };
    let thrice = Lineage {
        atlases: vec![
            atlas("One", &["dup", "dup", "dup"]),
            atlas("Two", &["other"]),
        ],
    };
    for chain in [twice, thrice] {
        let lines = render_arrivals(&chain).expect("the chain reads");
        assert_eq!(
            lines,
            strings(&[
                "atlas 1",
                "entry dup, first stood at atlas 1, last stood at atlas 1, \
                 stands in 1 atlases, does not stand at the close",
                "atlas 2",
                "entry other, first stood at atlas 2, last stood at atlas 2, \
                 stands in 1 atlases, stands at the close",
                "arrivals counts, atlas 1 arrivals 1, atlas 2 arrivals 1, \
                 entries 2, atlases 2",
            ])
        );
    }
}

#[test]
fn the_sections_hold_the_arriving_lines_in_entry_order() {
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
        let lines = render_arrivals(chain).expect("the chain reads");
        let secs = arr_sections(&lines);
        assert_eq!(secs.len(), total);
        let mut entry_lines = 0usize;
        for (index, atlas) in chain.atlases.iter().enumerate() {
            let mut expected: Vec<String> = Vec::new();
            let mut seen: Vec<String> = Vec::new();
            for entry in &atlas.entries {
                let label = entry.label.to_string();
                if !seen.contains(&label) {
                    seen.push(label.clone());
                    let line = by_label
                        .get(&label)
                        .expect("every standing label holds a standing line");
                    if figure(line, ", first stood at atlas ", ",") == index + 1 {
                        expected.push(line.clone());
                    }
                }
            }
            assert_eq!(secs[index], expected);
            entry_lines += secs[index].len();
        }
        assert_eq!(lines.len(), total + entry_lines + 1);
        let closing = lines.last().expect("a closing stands");
        for (index, section) in secs.iter().enumerate() {
            let head = format!("atlas {} arrivals ", index + 1);
            assert_eq!(figure(closing, &head, ","), section.len());
        }
        assert_eq!(figure(closing, "entries ", ","), entry_lines);
        assert_eq!(entry_lines, by_label.len());
        let mut first_labels: Vec<String> = Vec::new();
        for entry in &chain.atlases[0].entries {
            let label = entry.label.to_string();
            if !first_labels.contains(&label) {
                first_labels.push(label);
            }
        }
        assert_eq!(
            figure(closing, "atlas 1 arrivals ", ","),
            first_labels.len()
        );
    }
}
