//! Tests for MY-42, the fellows reading.
//!
//! The reading takes a lineage, reads the presence of
//! every label in the one presence pass MY-18 states,
//! gathers the atlases standing with each atlas from
//! the one pass, walking the pass entry by entry in
//! the order of first standing, each entry standing
//! its atlases with each other, and states one
//! section for each atlas of the chain, the section
//! opening with the bare atlas line of its own atlas
//! and holding one bare atlas line for each atlas
//! standing with it, in chain order, closing with the
//! fellows counts. The pins here read the reading
//! over the shipped chains and over hand built
//! chains, and bind it to the presence, standings,
//! rosters, and matched readings through the five
//! relations MY-42 states, each computed from the
//! neighbouring readings' own outputs.
//!
//! The mutations the plan names against the reading
//! stand here once, as exact edits to the built
//! reading, and the pins below cite them by number.
//! MF-1, step fellows: the gathering names for each
//! atlas the atlases adjacent to it in the chain, its
//! predecessor and its successor, whether or not they
//! share an entry, and no other atlas. MF-2, roster
//! fellows: the gathering names for each atlas the
//! atlases holding its own roster exactly, the
//! rosters gathering standing in place of the
//! standing together gathering, an atlas sharing an
//! entry with another whose roster differs standing
//! with none of them. MF-3, containment fellows: the
//! gathering names for each atlas the atlases whose
//! rosters contain its own roster or whose rosters
//! its own roster contains, and no other atlas. MF-4,
//! the pairs closing: the closing's fellows figure
//! counts the unordered pairs of atlases standing
//! with each other, each pair counted once from one
//! end, reading pairs in place of fellows. MF-5, self
//! standing: each atlas stands among its own fellows,
//! its own atlas line standing in its section in
//! chain order among its fellows lines. MF-6, the two
//! entry threshold: the gathering names for each
//! atlas the atlases sharing at least two entries
//! with it, two atlases sharing exactly one entry
//! standing apart.
//!
//! The discrimination split of these pins stands as
//! follows, pin by pin. Pins 1 to 11 fail on the tree
//! before this slice by execution, because
//! render_fellows stands nowhere on that tree, so
//! the file does not compile there and no pin of 1 to
//! 11 can pass there. Pin 1, the winter golden byte
//! for byte, fails under MF-1, the first atlas losing
//! the third, its only fellows standing at its
//! successor alone; under MF-2, the second atlas
//! standing with none, its roster differing from both
//! its fellows' rosters; under MF-3, on the same
//! parting, the second atlas's roster neither
//! containing another's nor standing inside
//! another's; under MF-4, the closing reading fellows
//! 3, pin 1 standing as MF-4's designated killer; and
//! under MF-5. Under MF-6 the pin passes, and the
//! posture is stated here at design time from the
//! computed sharing counts, not found later: every
//! winter pair shares at least two entries, the
//! counts standing at two, three, and two. Pin 2, the
//! radio golden byte for byte, fails under all six
//! edits: under MF-1 the first atlas loses the third
//! as on the winter chain; under MF-2 every section
//! stands empty of fellows, the radio rosters all
//! differing; under MF-3 the first and third atlases
//! stand together, the first atlas's roster standing
//! inside the third's, and the second atlas stands
//! parted from both; under MF-4 the closing reading
//! fellows 3; under MF-5 each section gaining its own
//! line; and under MF-6 the first and second atlases
//! part, their pair sharing exactly one entry, the
//! radio check. Pin 3, the constructed golden byte
//! for byte, fails under all six edits: under MF-1
//! the first atlas loses the third and the third the
//! first; under MF-2 every section stands empty, the
//! constructed rosters all differing; under MF-3 the
//! second and third atlases stand together, the
//! third's roster standing inside the second's, and
//! the first stands with none; under MF-4 the closing
//! reading fellows 3; under MF-5 each section gaining
//! its own line; and under MF-6 the first and third
//! atlases part, their pair sharing exactly one
//! entry, the keeper line. Pin 4, the transpose
//! relation, read against the presence reading's own
//! output, fails under MF-1, MF-2, MF-3, MF-5, and
//! MF-6, each edit parting or joining a pair against
//! the presence sections on at least one of its
//! chains, MF-6 on the radio and constructed chains,
//! and it passes under MF-4, the edit moving no
//! section. Pin 5, the symmetry relation, fails under
//! MF-4, the closing reading half the line count, and
//! under MF-5, the self lines standing without
//! mirrors and the figure reading nine on the winter
//! chain against twice the three pairs, and it passes
//! under MF-1, MF-2, MF-3, and MF-6, each of those
//! gatherings standing symmetric in its own form.
//! Pin 6, the standings union relation, computed
//! against the standings reading's own output, fails
//! under MF-1, MF-2, MF-3, MF-5, and MF-6, and passes
//! under MF-4. Pin 7, the rosters agreement, fails
//! under MF-1 alone, the winter pair of the first and
//! third atlases standing in one rosters section and
//! not adjacent in the chain, and it passes under
//! MF-2 and MF-3, the winter pair holding equal
//! rosters, which the roster gathering keeps together
//! entire and which contain each other both ways,
//! under MF-6, the pair sharing three entries, and
//! under MF-4 and MF-5, neither edit parting a pair
//! the rosters reading joins. Pin 8, the closing
//! agreement, fails under all six edits, every edit
//! moving a figure or a line count on at least one of
//! its chains, MF-1 reading four fellows lines on the
//! winter chain against the closing's six, MF-6
//! reading four on the radio chain. Pin 9, the
//! matched agreement, computed against the matched
//! reading's own output, fails under MF-2, MF-3, and
//! MF-6, MF-6 on the radio chain's first step, its
//! atlases sharing exactly one entry where the matched
//! section names it, and it passes under MF-1, every
//! step pair standing together under the step
//! gathering and every step of its chains holding a
//! shared entry, under MF-4, and under MF-5, the self
//! lines naming no step pair. Pin 10, the lonely
//! atlas, fails under all six edits: under MF-1 the
//! third atlas names its neighbour and the second
//! names the third; under MF-2 and MF-3 the first and
//! second atlases part, their rosters differing and
//! neither containing the other; under MF-4 the
//! closing reading fellows 1; under MF-5 every section
//! gaining its own line; and under MF-6 the first and
//! second part, sharing exactly one entry. Pin 11,
//! the distant sharing pin, fails under MF-1, its
//! standing pair standing not adjacent and its lonely
//! atlas naming both its neighbours there; under
//! MF-4 and MF-5; and under MF-6, its pair sharing
//! exactly one entry. It passes under MF-2 and MF-3
//! by the fixture's own coincidence, stated here at
//! design time: the only standing together in the
//! fixture is a pair holding one roster entire, so
//! the roster gathering and the containment gathering
//! each reproduce the reading there whole, and the
//! pin's discrimination stands at MF-1 and MF-6, the
//! two edits its shape was built to part. Pins 12 and
//! 13 state the refusal battery, and their split is
//! stated in the form the Round 16 deviation D2
//! settled: the behaviour the pins pin stands whole
//! on the tree before this slice, in the one
//! presence pass the reading consumes unchanged, the
//! fewer than two atlases refusal and the surrogate
//! read before the line break read, and inside a
//! file that imports render_fellows the two pins
//! share the file's compile absence on that tree,
//! exactly as the pins of every reading file of the
//! family do. Under MF-1 to MF-6 the two pins pass,
//! the edits standing in the gathering and the closing
//! and none in the shared pass.

mod common;

use common::*;
use myriad::atlas::{
    load_lineage_text, render_fellows, render_matched, render_presence, render_rosters,
    render_standing, render_standings, Atlas, AtlasEntry, Lineage,
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

/// The figure after a head at the end of a line, read
/// back out of the rendered text.
fn figure_last(line: &str, head: &str) -> usize {
    line.split(head)
        .nth(1)
        .unwrap_or_else(|| panic!("the figure head {head} stands in the line {line}"))
        .parse()
        .expect("a count")
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

/// The sections of a rosters reading, the closing set
/// aside, each section its heading entry lines and the
/// numbers of its atlas lines in order. A section
/// opens at an entry line standing first or following
/// an atlas line, the heading lines of one roster
/// standing in a row ahead of the atlas lines holding
/// it.
fn rosters_sections(lines: &[String]) -> Vec<(Vec<String>, Vec<usize>)> {
    let mut out: Vec<(Vec<String>, Vec<usize>)> = Vec::new();
    for line in &lines[..lines.len() - 1] {
        if line.starts_with("entry ") {
            let opens = match out.last() {
                None => true,
                Some((_, atlases)) => !atlases.is_empty(),
            };
            if opens {
                out.push((Vec::new(), Vec::new()));
            }
            out.last_mut()
                .expect("a heading line stands inside a section")
                .0
                .push(line.clone());
        } else if line.starts_with("atlas ") {
            let number: usize = line
                .strip_prefix("atlas ")
                .expect("an atlas line")
                .parse()
                .expect("an atlas number");
            out.last_mut()
                .expect("an atlas line stands inside a section")
                .1
                .push(number);
        }
    }
    out
}

/// The sections of a standings reading, the closing
/// set aside, each section the label of its standing
/// line and the numbers of its atlas lines in order.
fn standings_sections(lines: &[String]) -> Vec<(String, Vec<usize>)> {
    let mut out: Vec<(String, Vec<usize>)> = Vec::new();
    for line in &lines[..lines.len() - 1] {
        if line.starts_with("entry ") {
            out.push((label_of(line), Vec::new()));
        } else if line.starts_with("atlas ") {
            let number: usize = line
                .strip_prefix("atlas ")
                .expect("an atlas line")
                .parse()
                .expect("an atlas number");
            out.last_mut()
                .expect("an atlas line stands inside a section")
                .1
                .push(number);
        }
    }
    out
}

/// The counts of a matched reading, the closing set
/// aside, one count for each step section, the entry
/// lines the section names.
fn matched_step_counts(lines: &[String]) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();
    for line in &lines[..lines.len() - 1] {
        if line.starts_with("step ") {
            out.push(0);
        } else if line.starts_with("entry ") {
            *out.last_mut()
                .expect("an entry line stands inside a step section") += 1;
        }
    }
    out
}

/// The number an atlas line names.
fn atlas_number(line: &str) -> Option<usize> {
    line.strip_prefix("atlas ")?.parse().ok()
}

/// The sections of a fellows reading, the closing set
/// aside, one section for each atlas in chain order,
/// each holding the numbers of its fellows lines in
/// order. A fellows line and the next section's
/// heading can carry the same text, so the sections
/// are read by trying each run of fellows lines the
/// heading order admits, the reading's own shape
/// standing as the check: the headings stand at
/// atlas 1 to the chain's count in order, and a
/// section's fellows stand in rising order, none of
/// them the section's own atlas.
fn fellows_sections(lines: &[String], atlases: usize) -> Vec<Vec<usize>> {
    fn parse(
        lines: &[String],
        pos: usize,
        heading: usize,
        atlases: usize,
    ) -> Option<Vec<Vec<usize>>> {
        if heading > atlases {
            return if pos == lines.len() {
                Some(Vec::new())
            } else {
                None
            };
        }
        if pos >= lines.len() || lines[pos] != format!("atlas {heading}") {
            return None;
        }
        let mut run: Vec<usize> = Vec::new();
        let mut end = pos + 1;
        while end < lines.len() {
            let Some(number) = atlas_number(&lines[end]) else {
                break;
            };
            if number == heading {
                break;
            }
            if let Some(&last) = run.last() {
                if number <= last {
                    break;
                }
            }
            run.push(number);
            end += 1;
        }
        for take in (0..=run.len()).rev() {
            if let Some(mut rest) = parse(lines, pos + 1 + take, heading + 1, atlases) {
                let mut out = vec![run[..take].to_vec()];
                out.append(&mut rest);
                return Some(out);
            }
        }
        None
    }
    parse(lines, 0, 1, atlases).expect("the fellows sections stand in chain order")
}

#[test]
fn the_winter_fellows_stand_as_counted() {
    let lines =
        render_fellows(&load("winter-spring-summer-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "atlas 2",
            "atlas 3",
            "atlas 2",
            "atlas 1",
            "atlas 3",
            "atlas 3",
            "atlas 1",
            "atlas 2",
            "fellows counts, fellows 6, atlases 3, entries 4",
        ])
    );
}

#[test]
fn the_radio_fellows_stand_as_counted() {
    let lines = render_fellows(&load("radio-investigation-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "atlas 2",
            "atlas 3",
            "atlas 2",
            "atlas 1",
            "atlas 3",
            "atlas 3",
            "atlas 1",
            "atlas 2",
            "fellows counts, fellows 6, atlases 3, entries 4",
        ])
    );
}

#[test]
fn the_constructed_fellows_stand_as_counted() {
    let lines =
        render_fellows(&load("constructed-departures-lineage.json")).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "atlas 2",
            "atlas 3",
            "atlas 2",
            "atlas 1",
            "atlas 3",
            "atlas 3",
            "atlas 1",
            "atlas 2",
            "fellows counts, fellows 6, atlases 3, entries 4",
        ])
    );
}

#[test]
fn an_atlas_stands_among_the_fellows_of_another_exactly_when_their_presence_sections_name_a_common_entry(
) {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_fellows(&chain).expect("the fellows read");
        let found = fellows_sections(&lines[..lines.len() - 1], chain.atlases.len());
        let presence = render_presence(&chain).expect("the presence reads");
        let by_atlas = presence_sections(&presence);
        let labels: Vec<Vec<String>> = by_atlas
            .iter()
            .map(|section| section.iter().map(|line| label_of(line)).collect())
            .collect();
        for first in 1..=chain.atlases.len() {
            assert!(
                !found[first - 1].contains(&first),
                "an atlas never stands among its own fellows over {name}"
            );
            for second in 1..=chain.atlases.len() {
                if first == second {
                    continue;
                }
                let shared = labels[first - 1]
                    .iter()
                    .any(|label| labels[second - 1].contains(label));
                assert_eq!(
                    found[first - 1].contains(&second),
                    shared,
                    "atlases {first} and {second} stand together exactly on a common entry over {name}"
                );
            }
        }
    }
}

#[test]
fn the_fellows_lines_stand_with_their_mirrors_and_the_closing_counts_each_pair_twice() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_fellows(&chain).expect("the fellows read");
        let found = fellows_sections(&lines[..lines.len() - 1], chain.atlases.len());
        let mut pairs = 0usize;
        for first in 1..=chain.atlases.len() {
            for second in (first + 1)..=chain.atlases.len() {
                assert_eq!(
                    found[first - 1].contains(&second),
                    found[second - 1].contains(&first),
                    "atlases {first} and {second} stand together from both ends or neither over {name}"
                );
                if found[first - 1].contains(&second) {
                    pairs += 1;
                }
            }
        }
        let printed: usize = found.iter().map(|section| section.len()).sum();
        let closing = lines.last().expect("a closing line");
        assert_eq!(
            figure(closing, "fellows counts, fellows ", ","),
            2 * pairs,
            "the fellows figure is twice the unordered pairs over {name}"
        );
        assert_eq!(
            figure(closing, "fellows counts, fellows ", ","),
            printed,
            "the fellows figure is the count of the fellows lines over {name}"
        );
    }
}

#[test]
fn the_fellows_of_an_atlas_are_the_atlases_its_entries_stand_in_with_itself_removed() {
    for name in CHAINS {
        let chain = load(name);
        let lines = render_fellows(&chain).expect("the fellows read");
        let found = fellows_sections(&lines[..lines.len() - 1], chain.atlases.len());
        let presence = render_presence(&chain).expect("the presence reads");
        let by_atlas = presence_sections(&presence);
        let standings = render_standings(&chain).expect("the standings read");
        let sections = standings_sections(&standings);
        for (index, section) in by_atlas.iter().enumerate() {
            let number = index + 1;
            let mut named: Vec<usize> = Vec::new();
            for line in section {
                let label = label_of(line);
                let (_, atlases) = sections
                    .iter()
                    .find(|(held, _)| *held == label)
                    .expect("a standings section stands for the entry");
                for atlas_no in atlases {
                    if *atlas_no != number && !named.contains(atlas_no) {
                        named.push(*atlas_no);
                    }
                }
            }
            named.sort_unstable();
            assert_eq!(
                found[index], named,
                "the fellows of atlas {number} are the atlases its entries stand in, itself removed, over {name}"
            );
        }
    }
}

#[test]
fn two_atlases_holding_one_roster_stand_with_each_other() {
    let expected_pairs = [2usize, 0, 0];
    for (chain_index, name) in CHAINS.iter().enumerate() {
        let chain = load(name);
        let lines = render_fellows(&chain).expect("the fellows read");
        let found = fellows_sections(&lines[..lines.len() - 1], chain.atlases.len());
        let rosters = render_rosters(&chain).expect("the rosters read");
        let sections = rosters_sections(&rosters);
        let mut pairs = 0usize;
        for (heading, atlases) in &sections {
            if heading.is_empty() {
                continue;
            }
            for &first in atlases {
                for &second in atlases {
                    if first == second {
                        continue;
                    }
                    pairs += 1;
                    assert!(
                        found[first - 1].contains(&second),
                        "atlases {first} and {second} hold one roster and stand with each other over {name}"
                    );
                }
            }
        }
        assert_eq!(
            pairs, expected_pairs[chain_index],
            "the ordered pairs standing in one roster over {name}"
        );
    }
}

#[test]
fn the_closing_agrees_with_the_sections_and_the_standing() {
    let expected_fellows = [6usize, 6, 6];
    for (chain_index, name) in CHAINS.iter().enumerate() {
        let chain = load(name);
        let lines = render_fellows(&chain).expect("the fellows read");
        let found = fellows_sections(&lines[..lines.len() - 1], chain.atlases.len());
        let closing = lines.last().expect("a closing line");
        let printed: usize = found.iter().map(|section| section.len()).sum();
        assert_eq!(
            figure(closing, "fellows counts, fellows ", ","),
            printed,
            "the fellows figure is the count of the fellows lines over {name}"
        );
        assert_eq!(
            figure(closing, "fellows counts, fellows ", ","),
            expected_fellows[chain_index],
            "the fellows figure stands as counted over {name}"
        );
        assert_eq!(
            figure(closing, "atlases ", ","),
            chain.atlases.len(),
            "the atlases figure over {name}"
        );
        let standing = render_standing(&chain).expect("the standing reads");
        let standing_closing = standing.last().expect("a closing line");
        assert_eq!(
            figure_last(closing, "entries "),
            figure(standing_closing, "entries ", ","),
            "the entries figure is the standing reading's over {name}"
        );
        let lonely = found.iter().filter(|section| section.is_empty()).count();
        assert_eq!(
            lonely, 0,
            "the sections holding no fellows line stand at none over {name}"
        );
    }
}

#[test]
fn the_two_atlases_of_a_step_stand_with_each_other_exactly_when_the_matched_section_names_an_entry()
{
    for name in CHAINS {
        let chain = load(name);
        let lines = render_fellows(&chain).expect("the fellows read");
        let found = fellows_sections(&lines[..lines.len() - 1], chain.atlases.len());
        let matched = render_matched(&chain).expect("the matched reads");
        let counts = matched_step_counts(&matched);
        assert_eq!(counts.len(), chain.atlases.len() - 1);
        for step in 1..chain.atlases.len() {
            let named = counts[step - 1] > 0;
            assert_eq!(
                found[step - 1].contains(&(step + 1)),
                named,
                "the earlier atlas of step {step} stands with the later exactly on a matched entry over {name}"
            );
            assert_eq!(
                found[step].contains(&step),
                named,
                "the later atlas of step {step} stands with the earlier exactly on a matched entry over {name}"
            );
        }
    }
}

#[test]
fn the_lonely_atlas_holds_a_section_of_its_heading_alone() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", &["alpha", "beta"]),
            atlas("Second", &["beta", "gamma"]),
            atlas("Third", &["delta"]),
        ],
    };
    let lines = render_fellows(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "atlas 2",
            "atlas 2",
            "atlas 1",
            "atlas 3",
            "fellows counts, fellows 2, atlases 3, entries 4",
        ])
    );
}

#[test]
fn the_first_and_third_atlases_stand_together_across_the_second() {
    let chain = Lineage {
        atlases: vec![
            atlas("First", &["solo"]),
            atlas("Second", &["other"]),
            atlas("Third", &["solo"]),
        ],
    };
    let lines = render_fellows(&chain).expect("the chain reads");
    assert_eq!(
        lines,
        strings(&[
            "atlas 1",
            "atlas 3",
            "atlas 2",
            "atlas 3",
            "atlas 1",
            "fellows counts, fellows 2, atlases 3, entries 2",
        ])
    );
}

#[test]
fn a_lineage_holding_fewer_than_two_atlases_is_refused() {
    let chain = Lineage {
        atlases: vec![atlas("Only", &["kept"])],
    };
    assert_eq!(
        render_fellows(&chain).expect_err("the short chain is refused"),
        "the lineage holds fewer than two atlases"
    );
}

#[test]
fn a_label_holding_an_unpaired_surrogate_is_refused_first() {
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
        render_fellows(&chain).expect_err("the doubly broken label names the surrogate first"),
        "an entry label holds an unpaired surrogate"
    );
}
