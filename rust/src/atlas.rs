//! The atlas loader and its readings.
//!
//! An atlas is a set of named explorations read together. The
//! loader reads an atlas whole or not at all, every defect of
//! every entry blocking the whole reading, and the readings
//! print the atlas rendering, the atlas index, and the
//! changelog of two atlases. A lineage is a chain of atlases
//! read in order, its loader reads the chain whole or not at
//! all, and its readings are the changelogs of its steps
//! composed and the returns of the chain, the entries that
//! stand again after an absence, and the standing of the
//! chain, where in the chain each entry stands, and the
//! absence of the chain, the runs of atlases each entry
//! stands absent from. The loader
//! and the readings mirror the Python implementation entry for
//! entry, defect phrase for defect phrase.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::explorations::{
    count_branches, count_by_state, format_counts, load_exploration_data, refuse_line_break,
    refuse_surrogate, Branch, Exploration, State,
};
use crate::json::{parse_json, JsonValue};
use crate::pytext::PText;

/// One loaded atlas entry, a label over an exploration.
#[derive(Clone, Debug, PartialEq)]
pub struct AtlasEntry {
    /// The entry label, unique across the atlas.
    pub label: PText,
    /// The exploration the entry names.
    pub exploration: Exploration,
}

/// A loaded atlas.
#[derive(Clone, Debug, PartialEq)]
pub struct Atlas {
    /// The atlas title.
    pub title: PText,
    /// The entries in the order the file lists them.
    pub entries: Vec<AtlasEntry>,
}

/// A loaded lineage, an ordered chain of atlases.
#[derive(Clone, Debug, PartialEq)]
pub struct Lineage {
    /// The atlases of the chain in the order the chain
    /// reads them, the first atlas the earliest.
    pub atlases: Vec<Atlas>,
}

fn check_label(entry: &JsonValue, index: usize, defects: &mut Vec<String>) -> Option<PText> {
    let value = match entry.get("label") {
        None => {
            defects.push(format!("entry {index} is missing field label"));
            return None;
        }
        Some(value) => value,
    };
    let text = match value {
        JsonValue::Str(text) => text,
        _ => {
            defects.push(format!("entry {index} field label has the wrong type"));
            return None;
        }
    };
    if text.is_empty() {
        defects.push(format!("entry {index} field label is empty"));
        return None;
    }
    if text.holds_unpaired_surrogate() {
        defects.push(format!(
            "entry {index} field label holds an unpaired surrogate"
        ));
        return None;
    }
    if text.holds_line_break() {
        defects.push(format!(
            "entry {index} field label holds a newline or a carriage return"
        ));
        return None;
    }
    Some(text.clone())
}

fn load_entry(
    entry: &JsonValue,
    index: usize,
    defects: &mut Vec<String>,
) -> (Option<PText>, Option<Exploration>) {
    if !entry.is_object() {
        defects.push(format!("entry {index} is not an object"));
        return (None, None);
    }
    let label = check_label(entry, index, defects);
    let prefix = match &label {
        Some(label) => format!("entry {label}"),
        None => format!("entry {index}"),
    };
    match entry.get("exploration") {
        None => {
            defects.push(format!("{prefix} is missing field exploration"));
            (label, None)
        }
        Some(value) => {
            if !value.is_object() {
                defects.push(format!("{prefix} field exploration is not a JSON object"));
                return (label, None);
            }
            let (exploration, inner) = load_exploration_data(value);
            for defect in &inner {
                defects.push(format!("{prefix}, {defect}"));
            }
            (label, exploration)
        }
    }
}

fn repeated_label_defects(labels: &[Option<PText>]) -> Vec<String> {
    let mut defects: Vec<String> = Vec::new();
    let mut first_at: std::collections::HashMap<PText, usize> = std::collections::HashMap::new();
    for (index, label) in labels.iter().enumerate() {
        let label = match label {
            None => continue,
            Some(label) => label,
        };
        if let Some(first) = first_at.get(label) {
            defects.push(format!(
                "duplicate entry label {label}, entries {first} and {index}"
            ));
        } else {
            first_at.insert(label.clone(), index);
        }
    }
    defects
}

fn load_atlas_data(data: &JsonValue) -> (Option<Atlas>, Vec<String>) {
    if !data.is_object() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    let mut defects: Vec<String> = Vec::new();
    let title = match data.get("title") {
        None => {
            defects.push("missing field title".to_string());
            None
        }
        Some(JsonValue::Str(text)) => {
            if text.is_empty() {
                defects.push("field title is empty".to_string());
                None
            } else if text.holds_unpaired_surrogate() {
                defects.push("field title holds an unpaired surrogate".to_string());
                None
            } else if text.holds_line_break() {
                defects.push("field title holds a newline or a carriage return".to_string());
                None
            } else {
                Some(text.clone())
            }
        }
        Some(_) => {
            defects.push("field title has the wrong type".to_string());
            None
        }
    };
    let mut entries: Vec<AtlasEntry> = Vec::new();
    let mut labels: Vec<Option<PText>> = Vec::new();
    match data.get("entries") {
        None => {
            defects.push("missing field entries".to_string());
        }
        Some(JsonValue::Arr(items)) => {
            for (index, entry) in items.iter().enumerate() {
                let (label, exploration) = load_entry(entry, index, &mut defects);
                labels.push(label.clone());
                if let (Some(label), Some(exploration)) = (label, exploration) {
                    entries.push(AtlasEntry { label, exploration });
                }
            }
            defects.extend(repeated_label_defects(&labels));
        }
        Some(_) => {
            defects.push("field entries has the wrong type".to_string());
        }
    }
    if !defects.is_empty() {
        return (None, defects);
    }
    (
        Some(Atlas {
            title: title.expect("a sound reading holds a title"),
            entries,
        }),
        Vec::new(),
    )
}

/// Read and load one atlas from its JSON text.
pub fn load_atlas_text(text: &str) -> (Option<Atlas>, Vec<String>) {
    match parse_json(text) {
        None => (None, vec!["input is not valid JSON".to_string()]),
        Some(data) => load_atlas_data(&data),
    }
}

/// Read and load one atlas from the file at a path.
pub fn load_atlas_file(path: &Path) -> (Option<Atlas>, Vec<String>) {
    let text = match std::fs::read(path) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(text) => text,
            Err(_) => return (None, vec!["file cannot be read".to_string()]),
        },
        Err(_) => return (None, vec!["file cannot be read".to_string()]),
    };
    load_atlas_text(&text)
}

fn load_lineage_data(data: &JsonValue) -> (Option<Lineage>, Vec<String>) {
    if !data.is_object() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    let mut defects: Vec<String> = Vec::new();
    let mut atlases: Vec<Atlas> = Vec::new();
    match data.get("atlases") {
        None => {
            defects.push("missing field atlases".to_string());
        }
        Some(JsonValue::Arr(items)) => {
            if items.len() < 2 {
                defects.push("field atlases holds fewer than two atlases".to_string());
            }
            for (index, member) in items.iter().enumerate() {
                let (atlas, member_defects) = load_atlas_data(member);
                for defect in &member_defects {
                    defects.push(format!("atlas {index}, {defect}"));
                }
                if let Some(atlas) = atlas {
                    atlases.push(atlas);
                }
            }
        }
        Some(_) => {
            defects.push("field atlases has the wrong type".to_string());
        }
    }
    if !defects.is_empty() {
        return (None, defects);
    }
    (Some(Lineage { atlases }), Vec::new())
}

/// Read and load one lineage from its JSON text.
pub fn load_lineage_text(text: &str) -> (Option<Lineage>, Vec<String>) {
    match parse_json(text) {
        None => (None, vec!["input is not valid JSON".to_string()]),
        Some(data) => load_lineage_data(&data),
    }
}

/// Read and load one lineage from the file at a path.
pub fn load_lineage_file(path: &Path) -> (Option<Lineage>, Vec<String>) {
    let text = match std::fs::read(path) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(text) => text,
            Err(_) => return (None, vec!["file cannot be read".to_string()]),
        },
        Err(_) => return (None, vec!["file cannot be read".to_string()]),
    };
    load_lineage_text(&text)
}

/// Render the atlas, each entry under its label.
///
/// A section opens with the entry line, then carries the
/// map of the entry exploration exactly as the map renders
/// it, then the claim lines of the entry exploration exactly
/// as the claims listing states them after its question
/// line. A hand-built value that breaks the loader
/// preconditions draws the refusal PORT-11 states.
pub fn render_atlas(atlas: &Atlas) -> Result<Vec<String>, String> {
    refuse_surrogate(&atlas.title, "the atlas title holds an unpaired surrogate")?;
    let mut lines = vec![format!("atlas, {}", atlas.title)];
    let mut union: Vec<Branch> = Vec::new();
    for entry in &atlas.entries {
        refuse_surrogate(&entry.label, "an entry label holds an unpaired surrogate")?;
        lines.push(format!("entry {}", entry.label));
        lines.extend(crate::explorations::render_map(&entry.exploration)?);
        let claims = crate::explorations::render_claims(&entry.exploration)?;
        lines.extend(claims.into_iter().skip(1));
        union.extend(entry.exploration.branches.iter().cloned());
    }
    lines.push(format!(
        "atlas counts, entries {}, {}",
        atlas.entries.len(),
        format_counts(&count_branches(&union))
    ));
    Ok(lines)
}

fn parent_word(parent: &Option<PText>) -> String {
    match parent {
        None => "root".to_string(),
        Some(parent) => parent.to_string(),
    }
}

fn claim_word(claim: &Option<PText>) -> String {
    match claim {
        None => "unclaimed".to_string(),
        Some(claim) => claim.to_string(),
    }
}

/// The lines of one matched branch pair, in the field
/// order MY-11 states, and whether any line stands.
///
/// The pair is read for its id, its parents, its prompts,
/// its claims, and, when its state stands the same in both
/// readings, the finding or the reason that state carries,
/// each pair in earlier then later order, and a field that
/// breaks the loader preconditions draws the refusal
/// MY-11 states in that reading order.
fn changelog_branch(
    earlier: &Branch,
    later: &Branch,
    lines: &mut Vec<String>,
) -> Result<bool, String> {
    refuse_surrogate(&earlier.id, "a branch id holds an unpaired surrogate")?;
    if let Some(parent) = &earlier.parent {
        refuse_surrogate(parent, "a branch parent holds an unpaired surrogate")?;
    }
    if let Some(parent) = &later.parent {
        refuse_surrogate(parent, "a branch parent holds an unpaired surrogate")?;
    }
    refuse_surrogate(
        &earlier.prompt,
        "a branch prompt holds an unpaired surrogate",
    )?;
    refuse_surrogate(&later.prompt, "a branch prompt holds an unpaired surrogate")?;
    if let Some(claim) = &earlier.claim {
        refuse_surrogate(claim, "a claim holds an unpaired surrogate")?;
    }
    if let Some(claim) = &later.claim {
        refuse_surrogate(claim, "a claim holds an unpaired surrogate")?;
    }
    let mut own: Vec<String> = Vec::new();
    if earlier.state != later.state {
        own.push(format!(
            "  branch {}, state changed, {} to {}",
            earlier.id,
            earlier.state.name(),
            later.state.name()
        ));
    }
    if earlier.parent != later.parent {
        own.push(format!(
            "  branch {}, parent changed, {} to {}",
            earlier.id,
            parent_word(&earlier.parent),
            parent_word(&later.parent)
        ));
    }
    if earlier.prompt != later.prompt {
        own.push(format!(
            "  branch {}, prompt changed, {} to {}",
            earlier.id, earlier.prompt, later.prompt
        ));
    }
    if earlier.claim != later.claim {
        own.push(format!(
            "  branch {}, claim changed, {} to {}",
            earlier.id,
            claim_word(&earlier.claim),
            claim_word(&later.claim)
        ));
    }
    if earlier.state == later.state {
        match earlier.state {
            State::Taken => {
                let before = earlier
                    .finding
                    .as_ref()
                    .ok_or_else(|| "a taken branch holds no finding".to_string())?;
                refuse_surrogate(before, "a finding holds an unpaired surrogate")?;
                let after = later
                    .finding
                    .as_ref()
                    .ok_or_else(|| "a taken branch holds no finding".to_string())?;
                refuse_surrogate(after, "a finding holds an unpaired surrogate")?;
                if before != after {
                    own.push(format!(
                        "  branch {}, finding changed, {} to {}",
                        earlier.id, before, after
                    ));
                }
            }
            State::Abandoned => {
                let before = earlier
                    .reason
                    .as_ref()
                    .ok_or_else(|| "an abandoned branch holds no reason".to_string())?;
                refuse_surrogate(before, "a reason holds an unpaired surrogate")?;
                let after = later
                    .reason
                    .as_ref()
                    .ok_or_else(|| "an abandoned branch holds no reason".to_string())?;
                refuse_surrogate(after, "a reason holds an unpaired surrogate")?;
                if before != after {
                    own.push(format!(
                        "  branch {}, reason changed, {} to {}",
                        earlier.id, before, after
                    ));
                }
            }
            State::Open => {}
        }
    }
    let changed = !own.is_empty();
    lines.extend(own);
    Ok(changed)
}

/// The branch lines of one matched entry, and the count
/// of branches they name.
///
/// The lines stand in the order the branches stand in the
/// earlier exploration, the matched and the departed
/// branches among them, and the arrived branches stand
/// after them in the order they stand in the later
/// exploration.
fn changelog_branches(
    earlier: &Exploration,
    later: &Exploration,
    lines: &mut Vec<String>,
) -> Result<usize, String> {
    let later_by_id: HashMap<&PText, &Branch> = later
        .branches
        .iter()
        .map(|branch| (&branch.id, branch))
        .collect();
    let earlier_ids: HashSet<&PText> = earlier.branches.iter().map(|branch| &branch.id).collect();
    let mut changed = 0usize;
    for branch in &earlier.branches {
        match later_by_id.get(&branch.id) {
            None => {
                refuse_surrogate(&branch.id, "a branch id holds an unpaired surrogate")?;
                lines.push(format!("  branch {}, departed", branch.id));
                changed += 1;
            }
            Some(other) => {
                if changelog_branch(branch, other, lines)? {
                    changed += 1;
                }
            }
        }
    }
    for branch in &later.branches {
        if earlier_ids.contains(&branch.id) {
            continue;
        }
        refuse_surrogate(&branch.id, "a branch id holds an unpaired surrogate")?;
        lines.push(format!("  branch {}, arrived", branch.id));
        changed += 1;
    }
    Ok(changed)
}

/// The counts one changelog closes with, read from the
/// matching itself.
struct StepCounts {
    arrived: usize,
    departed: usize,
    matched: usize,
    changed_branches: usize,
}

/// Render the changelog of two atlases and the counts it
/// closes with, the one pass the changelog and the
/// lineage readings both read.
fn changelog_reading(earlier: &Atlas, later: &Atlas) -> Result<(Vec<String>, StepCounts), String> {
    refuse_surrogate(
        &earlier.title,
        "the atlas title holds an unpaired surrogate",
    )?;
    refuse_surrogate(&later.title, "the atlas title holds an unpaired surrogate")?;
    let mut lines = vec![
        format!("earlier atlas, {}", earlier.title),
        format!("later atlas, {}", later.title),
    ];
    let later_by_label: HashMap<&PText, &AtlasEntry> = later
        .entries
        .iter()
        .map(|entry| (&entry.label, entry))
        .collect();
    let earlier_labels: HashSet<&PText> =
        earlier.entries.iter().map(|entry| &entry.label).collect();
    let mut arrived = 0usize;
    let mut departed = 0usize;
    let mut matched = 0usize;
    let mut changed_branches = 0usize;
    for entry in &earlier.entries {
        refuse_surrogate(&entry.label, "an entry label holds an unpaired surrogate")?;
        match later_by_label.get(&entry.label) {
            None => {
                departed += 1;
                lines.push(format!("entry {}, departed", entry.label));
            }
            Some(later_entry) => {
                matched += 1;
                let mut section: Vec<String> = Vec::new();
                refuse_surrogate(
                    &entry.exploration.question,
                    "the question holds an unpaired surrogate",
                )?;
                refuse_surrogate(
                    &later_entry.exploration.question,
                    "the question holds an unpaired surrogate",
                )?;
                if entry.exploration.question != later_entry.exploration.question {
                    section.push(format!(
                        "  earlier question, {}",
                        entry.exploration.question
                    ));
                    section.push(format!(
                        "  later question, {}",
                        later_entry.exploration.question
                    ));
                }
                changed_branches +=
                    changelog_branches(&entry.exploration, &later_entry.exploration, &mut section)?;
                if section.is_empty() {
                    lines.push(format!("entry {}, unchanged", entry.label));
                } else {
                    lines.push(format!("entry {}", entry.label));
                    lines.extend(section);
                }
            }
        }
    }
    for entry in &later.entries {
        if earlier_labels.contains(&entry.label) {
            continue;
        }
        refuse_surrogate(&entry.label, "an entry label holds an unpaired surrogate")?;
        arrived += 1;
        lines.push(format!("entry {}, arrived", entry.label));
    }
    lines.push(format!(
        "changelog counts, entries arrived {arrived}, departed {departed}, \
         matched {matched}, branches changed {changed_branches}"
    ));
    Ok((
        lines,
        StepCounts {
            arrived,
            departed,
            matched,
            changed_branches,
        },
    ))
}

/// Render the changelog of two atlases, the later read
/// against the earlier.
///
/// Entries match by label and branches inside a matched
/// entry match by id. The reading names each entry that
/// arrived or departed and each branch inside a matched
/// entry that arrived, departed, or changed, and closes
/// with the counts of the matching. A hand-built value
/// that breaks the loader preconditions draws the refusal
/// MY-11 states.
pub fn render_changelog(earlier: &Atlas, later: &Atlas) -> Result<Vec<String>, String> {
    let (lines, _) = changelog_reading(earlier, later)?;
    Ok(lines)
}

/// Render the lineage of a chain of atlases, the
/// changelogs of its steps composed.
///
/// One step stands for each pair of atlases the chain
/// holds in a row, and the lines of a step are the lines
/// of the changelog reading of that pair, so a chain of
/// exactly two atlases renders the changelog of the pair
/// byte for byte. When the chain holds more than two
/// atlases the reading closes with the lineage counts
/// line, carrying the count of steps and the sums of the
/// counts the steps close with, read from the step
/// readings themselves. A hand-built lineage holding
/// fewer than two atlases draws the refusal MY-13
/// states, and a hand-built value inside a step draws
/// the refusal of its step, the steps read in chain
/// order.
pub fn render_lineage(lineage: &Lineage) -> Result<Vec<String>, String> {
    if lineage.atlases.len() < 2 {
        return Err("the lineage holds fewer than two atlases".to_string());
    }
    let mut lines: Vec<String> = Vec::new();
    let mut arrived = 0usize;
    let mut departed = 0usize;
    let mut matched = 0usize;
    let mut changed_branches = 0usize;
    for step in lineage.atlases.windows(2) {
        let (step_lines, counts) = changelog_reading(&step[0], &step[1])?;
        lines.extend(step_lines);
        arrived += counts.arrived;
        departed += counts.departed;
        matched += counts.matched;
        changed_branches += counts.changed_branches;
    }
    if lineage.atlases.len() > 2 {
        lines.push(format!(
            "lineage counts, steps {}, entries arrived {arrived}, departed {departed}, \
             matched {matched}, branches changed {changed_branches}",
            lineage.atlases.len() - 1
        ));
    }
    Ok(lines)
}

/// Read the presence of a chain of atlases once, each
/// label against the atlases it stands in.
///
/// A hand-built lineage holding fewer than two atlases
/// draws the refusal MY-13 states, and a label holding
/// an unpaired surrogate draws the refusal PORT-11 names
/// for that field, and a label holding a line break
/// draws the refusal FIX-MY-13 names for that field, the
/// labels read in chain order, atlas order then entry
/// order, and each label read for a surrogate first and
/// then for a line break, as the loader reads them. The
/// returns reading, the standing reading, and the
/// absence reading all read presence through this one
/// pass, so the refusal text, the refusal order, and the
/// per-atlas label sets stand in one home.
fn presence_of(lineage: &Lineage) -> Result<Vec<HashSet<&PText>>, String> {
    if lineage.atlases.len() < 2 {
        return Err("the lineage holds fewer than two atlases".to_string());
    }
    let mut presence: Vec<HashSet<&PText>> = Vec::new();
    for atlas in &lineage.atlases {
        let mut labels: HashSet<&PText> = HashSet::new();
        for entry in &atlas.entries {
            refuse_surrogate(&entry.label, "an entry label holds an unpaired surrogate")?;
            refuse_line_break(
                &entry.label,
                "an entry label holds a newline or a carriage return",
            )?;
            labels.insert(&entry.label);
        }
        presence.push(labels);
    }
    Ok(presence)
}

/// Render the returns of a chain of atlases, the entries
/// that stand again after an absence.
///
/// Presence is read once over the whole chain, each label
/// against the atlases it stands in. An entry returns at a
/// step when it stands in the atlas the step leads to,
/// does not stand in the atlas the step leads from, and
/// stands in at least one atlas earlier than the atlas the
/// step leads from, and one line stands for each return in
/// chain order and then in the order the entries stand in
/// the atlas the step leads to, closing with the returns
/// counts line. A hand-built lineage holding fewer than
/// two atlases draws the refusal MY-13 states, and a label
/// holding an unpaired surrogate draws the refusal
/// PORT-11 names for that field, and a label holding a
/// line break draws the refusal FIX-MY-13 names for that
/// field, the labels read in chain order, atlas order
/// then entry order.
pub fn render_returns(lineage: &Lineage) -> Result<Vec<String>, String> {
    let presence = presence_of(lineage)?;
    let mut lines: Vec<String> = Vec::new();
    let mut returns = 0usize;
    let mut returned: HashSet<&PText> = HashSet::new();
    for target in 1..lineage.atlases.len() {
        for entry in &lineage.atlases[target].entries {
            if presence[target - 1].contains(&entry.label) {
                continue;
            }
            let stood_earlier = presence[..target - 1]
                .iter()
                .any(|labels| labels.contains(&entry.label));
            if !stood_earlier {
                continue;
            }
            let mut absent = 0usize;
            let mut at = target - 1;
            while !presence[at].contains(&entry.label) {
                absent += 1;
                at -= 1;
            }
            lines.push(format!(
                "entry {}, returned at step {}, absent {} atlases",
                entry.label, target, absent
            ));
            returns += 1;
            returned.insert(&entry.label);
        }
    }
    lines.push(format!(
        "returns counts, returns {returns}, entries returned {}",
        returned.len()
    ));
    Ok(lines)
}

/// The labels of a chain in the order they first stand
/// across it, the atlases read in chain order and the
/// entries of each atlas read in the order they stand in
/// it, the first standing of a label fixing its place.
///
/// The standing reading and the absence reading both read
/// their lines in this order, so the order stands in one
/// home.
fn first_standing_order(lineage: &Lineage) -> Vec<&PText> {
    let mut order: Vec<&PText> = Vec::new();
    let mut seen: HashSet<&PText> = HashSet::new();
    for atlas in &lineage.atlases {
        for entry in &atlas.entries {
            if seen.insert(&entry.label) {
                order.push(&entry.label);
            }
        }
    }
    order
}

/// Render the standing of a chain of atlases, where in
/// the chain each entry stands.
///
/// Presence is read once over the whole chain, each label
/// against the atlases it stands in, and one line stands
/// for each label in the order the labels first stand
/// across the chain, naming the first atlas the entry
/// stands in, the last atlas it stands in, the count of
/// atlases it stands in, and whether it stands at the
/// close, closing with the standing counts line. A
/// hand-built lineage holding fewer than two atlases
/// draws the refusal MY-13 states, and a label holding
/// an unpaired surrogate draws the refusal PORT-11 names
/// for that field, and a label holding a line break
/// draws the refusal FIX-MY-13 names for that field, the
/// labels read in chain order, atlas order then entry
/// order.
pub fn render_standing(lineage: &Lineage) -> Result<Vec<String>, String> {
    let presence = presence_of(lineage)?;
    let order = first_standing_order(lineage);
    let total = lineage.atlases.len();
    let mut lines: Vec<String> = Vec::new();
    let mut at_every = 0usize;
    let mut at_close = 0usize;
    for label in &order {
        let mut first = 0usize;
        let mut last = 0usize;
        let mut stood = 0usize;
        for (index, labels) in presence.iter().enumerate() {
            if labels.contains(label) {
                if stood == 0 {
                    first = index + 1;
                }
                last = index + 1;
                stood += 1;
            }
        }
        if stood == total {
            at_every += 1;
        }
        let closes = last == total;
        if closes {
            at_close += 1;
        }
        lines.push(format!(
            "entry {}, first stood at atlas {}, last stood at atlas {}, \
             stands in {} atlases, {}",
            label,
            first,
            last,
            stood,
            if closes {
                "stands at the close"
            } else {
                "does not stand at the close"
            }
        ));
    }
    lines.push(format!(
        "standing counts, atlases {total}, entries {}, \
         standing at every atlas {at_every}, standing at the close {at_close}",
        order.len()
    ));
    Ok(lines)
}

/// Render the absence of a chain of atlases, the runs of
/// atlases each entry stands absent from.
///
/// Presence is read once over the whole chain, each label
/// against the atlases it stands in, and one line stands
/// for each absence run of the chain in the order the
/// labels first stand across the chain, the runs of one
/// label in chain order. An absence run is a run of one
/// or more atlases in a row in which the label does not
/// stand, where the atlas immediately before the run is
/// one the label stands in, and a run that reaches the
/// last atlas of the chain closes with the words to the
/// close. The atlases before the first atlas a label
/// stands in hold no run of that label. The reading
/// closes with the absence counts line. A hand-built
/// lineage holding fewer than two atlases draws the
/// refusal MY-13 states, and a label holding an unpaired
/// surrogate draws the refusal PORT-11 names for that
/// field, and a label holding a line break draws the
/// refusal FIX-MY-13 names for that field, the labels
/// read in chain order, atlas order then entry order.
pub fn render_absence(lineage: &Lineage) -> Result<Vec<String>, String> {
    let presence = presence_of(lineage)?;
    let order = first_standing_order(lineage);
    let total = lineage.atlases.len();
    let mut lines: Vec<String> = Vec::new();
    let mut runs = 0usize;
    let mut with_absences = 0usize;
    let mut absent_at_close = 0usize;
    for label in &order {
        let mut label_runs = 0usize;
        let mut stood_before = false;
        let mut index = 0usize;
        while index < total {
            if presence[index].contains(label) {
                stood_before = true;
                index += 1;
                continue;
            }
            if !stood_before {
                index += 1;
                continue;
            }
            let start = index;
            while index < total && !presence[index].contains(label) {
                index += 1;
            }
            runs += 1;
            label_runs += 1;
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
        if label_runs > 0 {
            with_absences += 1;
        }
    }
    lines.push(format!(
        "absence counts, absences {runs}, entries with absences {with_absences}, \
         absent at the close {absent_at_close}"
    ));
    Ok(lines)
}

/// Render the atlas index, the cheap reading of an atlas.
///
/// A hand-built value that breaks the loader preconditions
/// draws the refusal PORT-11 states.
pub fn render_index(atlas: &Atlas) -> Result<Vec<String>, String> {
    refuse_surrogate(&atlas.title, "the atlas title holds an unpaired surrogate")?;
    let mut lines = vec![format!("atlas, {}", atlas.title)];
    let mut union: Vec<Branch> = Vec::new();
    for entry in &atlas.entries {
        refuse_surrogate(&entry.label, "an entry label holds an unpaired surrogate")?;
        refuse_surrogate(
            &entry.exploration.question,
            "the question holds an unpaired surrogate",
        )?;
        lines.push(format!(
            "entry {}, question, {}, {}",
            entry.label,
            entry.exploration.question,
            format_counts(&count_by_state(&entry.exploration))
        ));
        union.extend(entry.exploration.branches.iter().cloned());
    }
    lines.push(format!(
        "atlas counts, entries {}, {}",
        atlas.entries.len(),
        format_counts(&count_branches(&union))
    ));
    Ok(lines)
}
