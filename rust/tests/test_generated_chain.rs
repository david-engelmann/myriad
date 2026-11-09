//! Tests for the generated fixture family of Round 12.
//!
//! The shipped examples are miniatures, the largest
//! exploration holding 26 branches at depth at most 4
//! and both chains holding exactly 3 atlases. This
//! family pins behaviour at the shapes the audit of
//! 2026-11-08 named: branches per exploration 40 to
//! 120, depth 3 to 8, a claim rate near the claims
//! example's 10 of 26, a chain of 8 loggings, and
//! per-step entry churn near a quarter, entries
//! arriving, departing, and returning across steps. A
//! seeded generator below produces the chain
//! deterministically, and every word of its content is
//! synthetic and written for this repo. The chain
//! ships at examples/generated-chain-lineage.json with
//! goldens for the five chain readings, and the pins
//! are the closing relations among the five readings
//! over the generated chain, computed from the
//! readings' own outputs, with a hand count nowhere,
//! so a wrong count fails even where a golden was
//! regenerated carelessly.
//!
//! Round 13 extends the family by addition. The
//! matched golden joins the five at
//! examples/generated-chain-matched.txt, and three pins
//! below read the matched reading over the generated
//! chain and two measured bands over its fields. The
//! discrimination split of the three stands as follows.
//! The matched golden pin and the matched relations pin
//! fail on the tree before this slice by compile
//! absence, because render_matched stands nowhere on
//! that tree. The band pin passes on that tree by
//! design, because the chain and its loader stand
//! unchanged there, and its failure mode is a drifted
//! regeneration of the chain.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_absence, render_arrivals, render_departures, render_index,
    render_lineage, render_matched, render_returns, render_standing, Lineage,
};

// ----------------------------------------------------------------
// The generator. A SplitMix64 stream under one stated
// seed drives every draw, so the chain the test
// regenerates is the chain that ships, byte for byte.

const SEED: u64 = 0x2026_1108;

struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next_u64() % bound as u64) as usize
    }
}

/// A minimal JSON value for the generator's own
/// assembly, printed in the shape of the shipped
/// examples by the printer below.
enum J {
    Null,
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(&'static str, J)>),
}

fn jstr(text: &str) -> J {
    J::Str(text.to_string())
}

fn print_json(value: &J, indent: usize, out: &mut String) {
    let pad = "  ".repeat(indent);
    let inner = "  ".repeat(indent + 1);
    match value {
        J::Null => out.push_str("null"),
        J::Str(text) => {
            out.push('"');
            for ch in text.chars() {
                match ch {
                    '"' => out.push_str("\\\""),
                    '\\' => out.push_str("\\\\"),
                    '\n' => out.push_str("\\n"),
                    '\r' => out.push_str("\\r"),
                    '\t' => out.push_str("\\t"),
                    _ => out.push(ch),
                }
            }
            out.push('"');
        }
        J::Arr(items) => {
            if items.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push_str("[\n");
            for (index, item) in items.iter().enumerate() {
                out.push_str(&inner);
                print_json(item, indent + 1, out);
                if index + 1 < items.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            out.push_str(&pad);
            out.push(']');
        }
        J::Obj(pairs) => {
            if pairs.is_empty() {
                out.push_str("{}");
                return;
            }
            out.push_str("{\n");
            for (index, (key, item)) in pairs.iter().enumerate() {
                out.push_str(&inner);
                print_json(&J::Str((*key).to_string()), indent + 1, out);
                out.push_str(": ");
                print_json(item, indent + 1, out);
                if index + 1 < pairs.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            out.push_str(&pad);
            out.push('}');
        }
    }
}

const LABELS: [&str; 22] = [
    "ferry link",
    "ice report",
    "harbour lights",
    "lifeboat cover",
    "tide table",
    "fuel store",
    "crew rota",
    "signal mast",
    "buoy line",
    "pier lights",
    "fog bell",
    "anchor watch",
    "cargo shed",
    "pilot boat",
    "rescue drill",
    "weather glass",
    "mooring plan",
    "ballast note",
    "channel marks",
    "storm ledger",
    "salt store",
    "beacon list",
];

const QUESTIONS: [&str; 22] = [
    "How does the harbour keep a ferry sailing through the winter",
    "When does the ice survey read the harbour ice",
    "Who keeps the harbour light list through the winter nights",
    "Who answers a call for help on the water in winter",
    "Where does the tide table stand for the spring tides",
    "Where is the winter diesel stored ashore",
    "Who crews the boat when the summer crew leaves",
    "Which mast carries the harbour signal flags",
    "Where do the channel buoys lie after the ice moves",
    "Do the pier lights meet the night sailing rules",
    "When does the fog bell ring on the pier head",
    "Who stands the anchor watch on a storm night",
    "What does the cargo shed hold over the winter",
    "Who pilots the mail boat through the ice channel",
    "When does the rescue crew drill on the winter water",
    "How does the weather glass read a coming frost",
    "Where do the boats moor when the harbour ices over",
    "How is the ballast logged on the freight runs",
    "Which marks bound the dredged channel",
    "Where does the storm ledger record the winter gales",
    "Where does the salt for the pier paths sit",
    "Which beacons stand lit along the harbour wall",
];

const OPENERS: [&str; 8] = [
    "Where does",
    "How does",
    "When does",
    "Who keeps",
    "What covers",
    "Which watch checks",
    "Why does",
    "What does",
];

const SUBJECTS: [&str; 20] = [
    "the ferry crew",
    "the harbour office",
    "the night watch",
    "the boat crew",
    "the survey crew",
    "the pier keeper",
    "the relief crew",
    "the town depot",
    "the freight boat",
    "the light keeper",
    "the ice watch",
    "the harbour master",
    "the winter rota",
    "the signal watch",
    "the cargo crew",
    "the pilot",
    "the rescue crew",
    "the store keeper",
    "the tide reader",
    "the beacon watch",
];

const TAILS: [&str; 5] = [
    "in the cold weeks",
    "before the first ice",
    "on the night sailings",
    "through the thaw",
    "at the spring tides",
];

const FINDINGS: [&str; 14] = [
    "The gauge reads forty centimetres at the pier",
    "Two lights fall short and are already on the works list",
    "The spare vessel is fitted before November",
    "The ledger names the ferry account in full",
    "The rota stands in the harbour office window",
    "The freight boat already covers Thursdays",
    "The store holds two heated tanks behind the office",
    "The survey is booked for the second week",
    "The bell rings from the pier head shelter",
    "The channel marks stand where the chart puts them",
    "The crew of four covers sickness by the swap rule",
    "The grant window opens for bids in September",
    "The ballast note stands signed in the log",
    "The mooring plan holds through the ice",
];

const REASONS: [&str; 10] = [
    "The roof failed its survey in October",
    "The bylaw settles the hours already",
    "The audit ruled it out after last winter",
    "The survey settled the counts last year",
    "The work is booked for spring and cannot move",
    "The licence does not allow the swap",
    "The store was emptied before the frost",
    "The chart already names the marks",
    "The rota leaves no watch to spare",
    "The grant terms bar the match",
];

const CLAIMS: [&str; 4] = ["harbour office", "boat crew", "survey crew", "town depot"];

fn gen_exploration(rng: &mut Rng, question: &str) -> J {
    let target = 40 + rng.below(81);
    let roots = 8 + rng.below(7);
    let mut parents: Vec<Option<usize>> = Vec::new();
    let mut depths: Vec<usize> = Vec::new();
    for _ in 0..roots {
        parents.push(None);
        depths.push(1);
    }
    // A forced chain, so every exploration reaches
    // depth 3 whatever the draws say.
    parents.push(Some(0));
    depths.push(2);
    let chain_parent = parents.len() - 1;
    parents.push(Some(chain_parent));
    depths.push(3);
    while parents.len() < target {
        let pick = rng.below(parents.len());
        if depths[pick] < 8 {
            parents.push(Some(pick));
            depths.push(depths[pick] + 1);
        }
    }
    let count = parents.len();
    // Prompt combinations drawn without replacement,
    // so no exploration repeats a prompt.
    let combos_total = OPENERS.len() * SUBJECTS.len() * TAILS.len();
    let mut combos: Vec<usize> = (0..combos_total).collect();
    for index in 0..count {
        let swap = index + rng.below(combos_total - index);
        combos.swap(index, swap);
    }
    let mut branches = Vec::new();
    for (index, parent) in parents.iter().enumerate() {
        let combo = combos[index];
        let prompt = format!(
            "{} {} {}",
            OPENERS[combo % OPENERS.len()],
            SUBJECTS[(combo / OPENERS.len()) % SUBJECTS.len()],
            TAILS[combo / (OPENERS.len() * SUBJECTS.len())]
        );
        let state_draw = rng.below(100);
        let (state, extra) = if state_draw < 55 {
            ("open", None)
        } else if state_draw < 82 {
            (
                "taken",
                Some(("finding", FINDINGS[rng.below(FINDINGS.len())])),
            )
        } else {
            (
                "abandoned",
                Some(("reason", REASONS[rng.below(REASONS.len())])),
            )
        };
        let mut pairs: Vec<(&'static str, J)> = vec![
            ("id", jstr(&format!("b{index}"))),
            (
                "parent",
                match parent {
                    None => J::Null,
                    Some(parent) => jstr(&format!("b{parent}")),
                },
            ),
            ("prompt", jstr(&prompt)),
            ("state", jstr(state)),
        ];
        if rng.below(26) < 10 {
            pairs.push(("claim", jstr(CLAIMS[rng.below(CLAIMS.len())])));
        }
        if let Some((key, text)) = extra {
            pairs.push((key, jstr(text)));
        }
        branches.push(J::Obj(pairs));
    }
    J::Obj(vec![
        ("question", jstr(question)),
        ("branches", J::Arr(branches)),
    ])
}

fn generate_chain_text() -> String {
    let mut rng = Rng(SEED);
    let mut standing = vec![false; LABELS.len()];
    for slot in standing.iter_mut().take(10) {
        *slot = true;
    }
    let mut atlases = Vec::new();
    for step in 0..8 {
        let entries: Vec<J> = LABELS
            .iter()
            .enumerate()
            .filter(|(index, _)| standing[*index])
            .map(|(index, label)| {
                J::Obj(vec![
                    ("label", jstr(label)),
                    ("exploration", gen_exploration(&mut rng, QUESTIONS[index])),
                ])
            })
            .collect();
        atlases.push(J::Obj(vec![
            (
                "title",
                jstr(&format!("A generated harbour chain, logging {}", step + 1)),
            ),
            ("entries", J::Arr(entries)),
        ]));
        if step < 7 {
            for slot in standing.iter_mut() {
                let draw = rng.below(4);
                *slot = if *slot { draw != 0 } else { draw == 0 };
            }
        }
    }
    let root = J::Obj(vec![("atlases", J::Arr(atlases))]);
    let mut out = String::new();
    print_json(&root, 0, &mut out);
    out.push('\n');
    out
}

// ----------------------------------------------------------------
// The pins.

fn shipped_text() -> String {
    let path = repo_root().join("examples/generated-chain-lineage.json");
    std::fs::read_to_string(&path).expect("the shipped chain reads")
}

fn shipped_chain() -> Lineage {
    let (lineage, defects) = load_lineage_text(&shipped_text());
    assert!(
        defects.is_empty(),
        "the shipped chain loads clean: {defects:?}"
    );
    lineage.expect("the shipped chain value stands")
}

fn golden(name: &str) -> Vec<String> {
    let path = repo_root().join("examples").join(name);
    let text = std::fs::read_to_string(&path).expect("a shipped golden reads");
    text.lines().map(str::to_string).collect()
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

/// The figure after a head that ends the line.
fn figure_last(line: &str, head: &str) -> usize {
    line.split(head)
        .nth(1)
        .unwrap_or_else(|| panic!("the figure head {head} stands in the line {line}"))
        .parse()
        .expect("a count")
}

#[test]
fn the_shipped_chain_is_the_generated_chain() {
    assert_eq!(generate_chain_text(), shipped_text());
}

#[test]
fn the_generated_chain_holds_its_stated_shape() {
    let chain = shipped_chain();
    assert_eq!(chain.atlases.len(), 8);
    for atlas in &chain.atlases {
        for entry in &atlas.entries {
            let branches = &entry.exploration.branches;
            assert!(
                (40..=120).contains(&branches.len()),
                "an exploration holds 40 to 120 branches, found {}",
                branches.len()
            );
            // Depth, walked over the parent ids.
            let mut depth_of: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();
            let mut max_depth = 0usize;
            for branch in branches {
                let depth = match &branch.parent {
                    None => 1,
                    Some(parent) => depth_of[&parent.to_display_string()] + 1,
                };
                depth_of.insert(branch.id.to_display_string(), depth);
                max_depth = max_depth.max(depth);
            }
            assert!(
                (3..=8).contains(&max_depth),
                "an exploration reaches depth 3 to 8, found {max_depth}"
            );
        }
    }
}

#[test]
fn the_five_goldens_match_the_readings_of_the_generated_chain() {
    let chain = shipped_chain();
    assert_eq!(
        render_standing(&chain).expect("the standing reads"),
        golden("generated-chain-standing.txt")
    );
    assert_eq!(
        render_returns(&chain).expect("the returns read"),
        golden("generated-chain-returns.txt")
    );
    assert_eq!(
        render_absence(&chain).expect("the absence reads"),
        golden("generated-chain-absence.txt")
    );
    assert_eq!(
        render_arrivals(&chain).expect("the arrivals read"),
        golden("generated-chain-arrivals.txt")
    );
    assert_eq!(
        render_departures(&chain).expect("the departures read"),
        golden("generated-chain-departures.txt")
    );
}

#[test]
fn the_arrivals_sum_to_the_distinct_entries() {
    let chain = shipped_chain();
    let arrivals = render_arrivals(&chain).expect("the arrivals read");
    let closing = arrivals.last().expect("a closing line");
    let mut sum = 0usize;
    for index in 1..=chain.atlases.len() {
        sum += figure(closing, &format!("atlas {index} arrivals "), ",");
    }
    let standing = render_standing(&chain).expect("the standing reads");
    let standing_closing = standing.last().expect("a closing line");
    assert_eq!(sum, figure(closing, "entries ", ","));
    assert_eq!(sum, figure(standing_closing, "entries ", ","));
}

#[test]
fn the_departures_sum_to_the_absent_at_the_close() {
    let chain = shipped_chain();
    let departures = render_departures(&chain).expect("the departures read");
    let closing = departures.last().expect("a closing line");
    let mut sum = 0usize;
    for index in 1..=chain.atlases.len() {
        sum += figure(closing, &format!("atlas {index} departures "), ",");
    }
    let absence = render_absence(&chain).expect("the absence reads");
    let absence_closing = absence.last().expect("a closing line");
    let at_close: usize = absence_closing
        .split("absent at the close ")
        .nth(1)
        .expect("the closing figure stands")
        .parse()
        .expect("a count");
    assert_eq!(sum, figure(closing, "entries departed ", ","));
    assert_eq!(sum, at_close);
}

#[test]
fn the_inside_runs_equal_the_returns() {
    let chain = shipped_chain();
    let absence = render_absence(&chain).expect("the absence reads");
    let inside = absence
        .iter()
        .filter(|line| line.contains("absent from atlas") && !line.contains("to the close"))
        .count();
    let returns = render_returns(&chain).expect("the returns read");
    let closing = returns.last().expect("a closing line");
    assert_eq!(inside, figure(closing, "returns counts, returns ", ","));
}

#[test]
fn the_entries_standing_at_the_close_equal_the_last_atlases_entries() {
    let chain = shipped_chain();
    let standing = render_standing(&chain).expect("the standing reads");
    let closing = standing.last().expect("a closing line");
    let at_close = figure_last(closing, "standing at the close ");
    let last = chain.atlases.last().expect("a last atlas stands");
    let index = render_index(last).expect("the index reads");
    let index_closing = index.last().expect("a closing line");
    assert_eq!(
        at_close,
        figure(index_closing, "atlas counts, entries ", ",")
    );
    assert_eq!(at_close, last.entries.len());
}

#[test]
fn each_readings_closing_restates_its_own_sections() {
    let chain = shipped_chain();
    // Standing: the closing counts the entry lines, the
    // lines that end standing at the close, and the
    // lines that stand in every atlas.
    let standing = render_standing(&chain).expect("the standing reads");
    let closing = standing.last().expect("a closing line");
    let entries: Vec<&String> = standing
        .iter()
        .filter(|line| line.starts_with("entry "))
        .collect();
    assert_eq!(entries.len(), figure(closing, "entries ", ","));
    assert_eq!(
        entries
            .iter()
            .filter(|line| line.ends_with("stands at the close"))
            .count(),
        figure_last(closing, "standing at the close ")
    );
    assert_eq!(
        entries
            .iter()
            .filter(|line| line.contains(&format!("stands in {} atlases", chain.atlases.len())))
            .count(),
        figure(closing, "standing at every atlas ", ",")
    );
    // Returns: the closing counts the return lines and
    // the distinct labels among them.
    let returns = render_returns(&chain).expect("the returns read");
    let closing = returns.last().expect("a closing line");
    let lines: Vec<&String> = returns
        .iter()
        .filter(|line| line.starts_with("entry "))
        .collect();
    assert_eq!(
        lines.len(),
        figure(closing, "returns counts, returns ", ",")
    );
    let distinct: std::collections::BTreeSet<&str> = lines
        .iter()
        .map(|line| line.split(',').next().expect("a label stands"))
        .collect();
    assert_eq!(distinct.len(), figure_last(closing, "entries returned "));
    // Absence: the closing counts the run lines and the
    // distinct labels among them.
    let absence = render_absence(&chain).expect("the absence reads");
    let closing = absence.last().expect("a closing line");
    let runs: Vec<&String> = absence
        .iter()
        .filter(|line| line.contains("absent from atlas"))
        .collect();
    assert_eq!(
        runs.len(),
        figure(closing, "absence counts, absences ", ",")
    );
    let distinct: std::collections::BTreeSet<&str> = runs
        .iter()
        .map(|line| line.split(',').next().expect("a label stands"))
        .collect();
    assert_eq!(
        distinct.len(),
        figure(closing, "entries with absences ", ",")
    );
    // Arrivals and departures: each closing restates
    // the count of entry lines under each section.
    let arrivals = render_arrivals(&chain).expect("the arrivals read");
    let closing = arrivals.last().expect("a closing line");
    let mut section = 0usize;
    let mut counts = vec![0usize; chain.atlases.len()];
    for line in &arrivals {
        if let Some(number) = line.strip_prefix("atlas ") {
            section = number.parse().expect("a section number");
        } else if line.starts_with("entry ") {
            counts[section - 1] += 1;
        }
    }
    for (index, count) in counts.iter().enumerate() {
        assert_eq!(
            *count,
            figure(closing, &format!("atlas {} arrivals ", index + 1), ",")
        );
    }
    let departures = render_departures(&chain).expect("the departures read");
    let closing = departures.last().expect("a closing line");
    let mut section = 0usize;
    let mut counts = vec![0usize; chain.atlases.len()];
    for line in &departures {
        if let Some(number) = line.strip_prefix("atlas ") {
            section = number.parse().expect("a section number");
        } else if line.starts_with("entry ") {
            counts[section - 1] += 1;
        }
    }
    for (index, count) in counts.iter().enumerate() {
        assert_eq!(
            *count,
            figure(closing, &format!("atlas {} departures ", index + 1), ",")
        );
    }
}

#[test]
fn the_matched_golden_matches_the_reading_of_the_generated_chain() {
    let chain = shipped_chain();
    assert_eq!(
        render_matched(&chain).expect("the matched reads"),
        golden("generated-chain-matched.txt")
    );
}

#[test]
fn the_matched_relations_hold_over_the_generated_chain() {
    let chain = shipped_chain();
    let matched = render_matched(&chain).expect("the matched reads");
    let closing = matched.last().expect("a closing line");
    let steps = chain.atlases.len() - 1;
    let counts: Vec<usize> = (1..=steps)
        .map(|step| figure(closing, &format!("step {step} matched "), ","))
        .collect();
    // The first relation: the per-step counts sum to the
    // lineage reading's own matched figure.
    let lineage = render_lineage(&chain).expect("the lineage reads");
    let lineage_closing = lineage.last().expect("a closing line");
    let sum: usize = counts.iter().sum();
    assert_eq!(sum, figure(lineage_closing, "matched ", ","));
    // The second relation, through the lineage reading's
    // per-step changelog counts lines: matched and
    // departed close the atlas a step leads from, and
    // matched and arrived close the atlas it leads to.
    let changelogs: Vec<&String> = lineage
        .iter()
        .filter(|line| line.starts_with("changelog counts,"))
        .collect();
    assert_eq!(changelogs.len(), steps);
    for step in 0..steps {
        let arrived = figure(changelogs[step], "entries arrived ", ",");
        let departed = figure(changelogs[step], "departed ", ",");
        assert_eq!(counts[step] + departed, chain.atlases[step].entries.len());
        assert_eq!(
            counts[step] + arrived,
            chain.atlases[step + 1].entries.len()
        );
    }
    // The fifth relation: the entries matched figure
    // names the distinct labels across the sections and
    // stands no smaller than the standing reading's
    // standing at every atlas figure.
    let labels: std::collections::BTreeSet<&str> = matched
        .iter()
        .filter(|line| line.starts_with("entry "))
        .map(|line| {
            line.strip_prefix("entry ")
                .expect("an entry line stands")
                .split(',')
                .next()
                .expect("a label stands")
        })
        .collect();
    let entries_matched = figure(closing, "entries matched ", ",");
    assert_eq!(entries_matched, labels.len());
    let standing = render_standing(&chain).expect("the standing reads");
    let standing_closing = standing.last().expect("a closing line");
    assert!(
        entries_matched >= figure(standing_closing, "standing at every atlas ", ","),
        "the entries matched stand no smaller than the standing at every atlas"
    );
}

#[test]
fn the_measured_bands_hold_over_the_generated_chain() {
    let chain = shipped_chain();
    // The claim rate, branches carrying a claim over
    // branches in all, computed from the loaded chain's
    // own fields, stands in the band 0.30 to 0.45.
    let mut branches = 0usize;
    let mut claimed = 0usize;
    for atlas in &chain.atlases {
        for entry in &atlas.entries {
            for branch in &entry.exploration.branches {
                branches += 1;
                if branch.claim.is_some() {
                    claimed += 1;
                }
            }
        }
    }
    let claim_rate = claimed as f64 / branches as f64;
    assert!(
        (0.30..=0.45).contains(&claim_rate),
        "the claim rate stands in its band, found {claim_rate}"
    );
    // The mean per-step churn, the share of each step's
    // earlier atlas labels not standing in its later
    // atlas, averaged over the steps, stands in the band
    // 0.15 to 0.35.
    let label_sets: Vec<std::collections::HashSet<String>> = chain
        .atlases
        .iter()
        .map(|atlas| {
            atlas
                .entries
                .iter()
                .map(|entry| entry.label.to_display_string())
                .collect()
        })
        .collect();
    let mut churn_total = 0.0f64;
    for step in 0..chain.atlases.len() - 1 {
        let earlier = &label_sets[step];
        let later = &label_sets[step + 1];
        let gone = earlier
            .iter()
            .filter(|label| !later.contains(*label))
            .count();
        churn_total += gone as f64 / earlier.len() as f64;
    }
    let mean_churn = churn_total / (chain.atlases.len() - 1) as f64;
    assert!(
        (0.15..=0.35).contains(&mean_churn),
        "the mean per-step churn stands in its band, found {mean_churn}"
    );
}
