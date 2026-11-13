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
//! stands absent from, and the arrivals of the chain, the
//! entries that first stand at each atlas, and the
//! departures of the chain, the entries that stand in no
//! later atlas, read by last appearance, and the matched
//! of the chain, the entries that stand on both sides of
//! each step. The loader
//! and the readings mirror the Python implementation entry for
//! entry, defect phrase for defect phrase.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::explorations::{
    count_branches, count_by_state, format_counts, load_exploration_data,
    refuse_exploration_fields, refuse_field, refuse_line_break, refuse_surrogate, Branch,
    Exploration, State,
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

/// The one pass over an atlas the atlas rendering, the
/// index reading, and the changelog reading read before
/// they render, under FIX-MY-14.
///
/// The pass reads the title first, then the entries in
/// the order they stand in the atlas, and inside an
/// entry the label and then the entry exploration in
/// the order the exploration pass reads it, each field
/// for an unpaired surrogate first and then for a line
/// break. The changelog reading reads the pass over the
/// earlier atlas and then over the later atlas, and the
/// lineage reading inherits the refusal through the
/// changelogs it composes and holds no pass of its own.
fn refuse_atlas_fields(atlas: &Atlas) -> Result<(), String> {
    refuse_field(
        &atlas.title,
        "the atlas title holds an unpaired surrogate",
        "the atlas title holds a newline or a carriage return",
    )?;
    for entry in &atlas.entries {
        refuse_field(
            &entry.label,
            "an entry label holds an unpaired surrogate",
            "an entry label holds a newline or a carriage return",
        )?;
        refuse_exploration_fields(&entry.exploration)?;
    }
    Ok(())
}

/// Render the atlas, each entry under its label.
///
/// A section opens with the entry line, then carries the
/// map of the entry exploration exactly as the map renders
/// it, then the claim lines of the entry exploration exactly
/// as the claims listing states them after its question
/// line. A hand-built value that breaks the loader
/// preconditions draws the refusal PORT-11 states. The
/// one pass of FIX-MY-14 stands before the rendering,
/// so a field holding a line break draws its refusal
/// there, in the field order the pass reads.
pub fn render_atlas(atlas: &Atlas) -> Result<Vec<String>, String> {
    refuse_atlas_fields(atlas)?;
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
/// exploration. Under FIX-MY-20 each branch id is read
/// once at its first standing in each exploration, the
/// later map taking the first standing and each walk
/// skipping a standing of an id already read.
fn changelog_branches(
    earlier: &Exploration,
    later: &Exploration,
    lines: &mut Vec<String>,
) -> Result<usize, String> {
    let mut later_by_id: HashMap<&PText, &Branch> = HashMap::new();
    for branch in &later.branches {
        later_by_id.entry(&branch.id).or_insert(branch);
    }
    let earlier_ids: HashSet<&PText> = earlier.branches.iter().map(|branch| &branch.id).collect();
    let mut changed = 0usize;
    let mut seen: HashSet<&PText> = HashSet::new();
    for branch in &earlier.branches {
        if !seen.insert(&branch.id) {
            continue;
        }
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
    let mut seen_later: HashSet<&PText> = HashSet::new();
    for branch in &later.branches {
        if !seen_later.insert(&branch.id) {
            continue;
        }
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
///
/// A label held more than once in one atlas is read
/// once, at its first standing in that atlas, under
/// FIX-MY-19, its section pairing its first standing
/// in the earlier atlas with its first standing in the
/// later, so the counts the reading closes with count
/// each label once, as the lineage reading's sums and
/// the matched reading's relations read them.
fn changelog_reading(earlier: &Atlas, later: &Atlas) -> Result<(Vec<String>, StepCounts), String> {
    refuse_atlas_fields(earlier)?;
    refuse_atlas_fields(later)?;
    refuse_surrogate(
        &earlier.title,
        "the atlas title holds an unpaired surrogate",
    )?;
    refuse_surrogate(&later.title, "the atlas title holds an unpaired surrogate")?;
    let mut lines = vec![
        format!("earlier atlas, {}", earlier.title),
        format!("later atlas, {}", later.title),
    ];
    let mut later_by_label: HashMap<&PText, &AtlasEntry> = HashMap::new();
    for entry in &later.entries {
        later_by_label.entry(&entry.label).or_insert(entry);
    }
    let earlier_labels: HashSet<&PText> =
        earlier.entries.iter().map(|entry| &entry.label).collect();
    let mut arrived = 0usize;
    let mut departed = 0usize;
    let mut matched = 0usize;
    let mut changed_branches = 0usize;
    let mut seen: HashSet<&PText> = HashSet::new();
    for entry in &earlier.entries {
        if !seen.insert(&entry.label) {
            continue;
        }
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
    let mut seen: HashSet<&PText> = HashSet::new();
    for entry in &later.entries {
        if !seen.insert(&entry.label) {
            continue;
        }
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
/// MY-11 states. The one pass of FIX-MY-14 stands before
/// the reading, over the earlier atlas and then over the
/// later, so a field holding a line break draws its
/// refusal there, in the field order the pass reads.
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
/// the atlas the step leads to, a label held more
/// than once in that atlas read once, at its first
/// standing in it, under FIX-MY-17, closing with the
/// returns counts line. A hand-built lineage holding fewer than
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
        let mut seen: HashSet<&PText> = HashSet::new();
        for entry in &lineage.atlases[target].entries {
            if !seen.insert(&entry.label) {
                continue;
            }
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

/// The standing figures of one label read from the
/// presence of a chain, the first atlas it stands in,
/// the last atlas it stands in, and the count of
/// atlases it stands in, each atlas counting from one.
fn standing_figures(presence: &[HashSet<&PText>], label: &PText) -> (usize, usize, usize) {
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
    (first, last, stood)
}

/// The line the standing reading states for one label,
/// in the form MY-18 states.
///
/// The standing reading states the line over the labels
/// of the chain in first standing order, and the
/// arrivals reading states the same line inside the
/// section of the atlas the label arrives at, so the
/// form stands in one home.
fn standing_line(presence: &[HashSet<&PText>], label: &PText, total: usize) -> String {
    let (first, last, stood) = standing_figures(presence, label);
    format!(
        "entry {}, first stood at atlas {}, last stood at atlas {}, \
         stands in {} atlases, {}",
        label,
        first,
        last,
        stood,
        if last == total {
            "stands at the close"
        } else {
            "does not stand at the close"
        }
    )
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
        let (_, last, stood) = standing_figures(&presence, label);
        if stood == total {
            at_every += 1;
        }
        if last == total {
            at_close += 1;
        }
        lines.push(standing_line(&presence, label, total));
    }
    lines.push(format!(
        "standing counts, atlases {total}, entries {}, \
         standing at every atlas {at_every}, standing at the close {at_close}",
        order.len()
    ));
    Ok(lines)
}

/// Render the arrivals of a chain of atlases, the
/// entries that first stand at each atlas of the chain.
///
/// Presence is read once over the whole chain in the
/// one pass of FIX-MY-13, and one section stands for
/// each atlas of the chain in chain order, opening with
/// the number of the atlas and holding one line for
/// each entry that arrives at that atlas, in the order
/// the entries stand in that atlas, the line the
/// standing reading states for that entry. A section
/// in which no entry arrives stands as its opening line
/// alone, and the reading closes with the arrivals
/// counts line, the arrivals at each atlas and the
/// count of entries and of atlases in the chain. A
/// hand-built lineage holding fewer than two atlases
/// draws the refusal MY-13 states, and a label holding
/// an unpaired surrogate draws the refusal PORT-11
/// names for that field, and a label holding a line
/// break draws the refusal FIX-MY-13 names for that
/// field, the labels read in chain order, atlas order
/// then entry order.
pub fn render_arrivals(lineage: &Lineage) -> Result<Vec<String>, String> {
    let presence = presence_of(lineage)?;
    let total = lineage.atlases.len();
    let mut lines: Vec<String> = Vec::new();
    let mut counts: Vec<usize> = Vec::new();
    let mut arrived: HashSet<&PText> = HashSet::new();
    for (index, atlas) in lineage.atlases.iter().enumerate() {
        lines.push(format!("atlas {}", index + 1));
        let mut count = 0usize;
        for entry in &atlas.entries {
            if arrived.insert(&entry.label) {
                lines.push(standing_line(&presence, &entry.label, total));
                count += 1;
            }
        }
        counts.push(count);
    }
    let entries: usize = counts.iter().sum();
    let mut closing = "arrivals counts".to_string();
    for (index, count) in counts.iter().enumerate() {
        closing.push_str(&format!(", atlas {} arrivals {}", index + 1, count));
    }
    closing.push_str(&format!(", entries {entries}, atlases {total}"));
    lines.push(closing);
    Ok(lines)
}

/// Render the departures of a chain of atlases, the
/// entries that stand in no later atlas, read by last
/// appearance.
///
/// Presence is read once over the whole chain in the
/// one pass of FIX-MY-13, and one section stands for
/// each atlas of the chain in chain order, opening with
/// the number of the atlas and holding one line for
/// each entry departing after that atlas, in the order
/// the entries stand in that atlas, a label held more
/// than once in that atlas read once, at its first
/// standing in it, under FIX-MY-17, the line the
/// standing reading states for that entry. The last
/// atlas's section always stands in its empty form,
/// and the reading closes with the departures counts
/// line, the departures after each atlas, the count of
/// entries departed over the chain, the count of
/// entries, and the count of atlases in the chain. A
/// hand-built lineage holding fewer than two atlases
/// draws the refusal MY-13 states, and a label holding
/// an unpaired surrogate draws the refusal PORT-11
/// names for that field, and a label holding a line
/// break draws the refusal FIX-MY-13 names for that
/// field, the labels read in chain order, atlas order
/// then entry order.
pub fn render_departures(lineage: &Lineage) -> Result<Vec<String>, String> {
    let presence = presence_of(lineage)?;
    let total = lineage.atlases.len();
    let mut lines: Vec<String> = Vec::new();
    let mut counts: Vec<usize> = Vec::new();
    for (index, atlas) in lineage.atlases.iter().enumerate() {
        lines.push(format!("atlas {}", index + 1));
        let mut count = 0usize;
        if index + 1 < total {
            let mut seen: HashSet<&PText> = HashSet::new();
            for entry in &atlas.entries {
                if !seen.insert(&entry.label) {
                    continue;
                }
                let stands_later = presence[index + 1..]
                    .iter()
                    .any(|labels| labels.contains(&entry.label));
                if !stands_later {
                    lines.push(standing_line(&presence, &entry.label, total));
                    count += 1;
                }
            }
        }
        counts.push(count);
    }
    let departed: usize = counts.iter().sum();
    let entries = first_standing_order(lineage).len();
    let mut closing = "departures counts".to_string();
    for (index, count) in counts.iter().enumerate() {
        closing.push_str(&format!(", atlas {} departures {}", index + 1, count));
    }
    closing.push_str(&format!(
        ", entries departed {departed}, entries {entries}, atlases {total}"
    ));
    lines.push(closing);
    Ok(lines)
}

/// Render the matched of a chain of atlases, the entries
/// standing on both sides of each step of the chain.
///
/// Presence is read once over the whole chain, each label
/// against the atlases it stands in, in the one pass
/// MY-18 states, and one section stands for each step of
/// the chain in chain order, opening with the line step
/// and the number of the step. Inside a section, one line
/// stands for each entry matched at that step, standing in
/// the atlas the step leads from and in the atlas it leads
/// to, in the order the entries stand in the atlas the
/// step leads from, and the line of a matched entry is the
/// standing line of that entry, through the one standing
/// line helper. Section membership is read from the pass's
/// per-atlas label sets and each atlas is walked with a
/// seen set beside it, so a label held more than once in
/// one atlas is read once, at its first standing in that
/// atlas, the posture FIX-MY-17 settled for the departures
/// and returns readings, held here from the reading's first
/// standing. A section in which no entry is matched stands
/// as its opening line alone. The reading closes with the
/// matched counts line, its head standing on that line
/// alone, so no entry label can render a line equal to it.
/// A hand-built lineage holding fewer than two atlases
/// draws the refusal MY-13 states, and a label holding an
/// unpaired surrogate draws the refusal PORT-11 names for
/// that field, and a label holding a line break draws the
/// refusal FIX-MY-13 names for that field, each label read
/// for an unpaired surrogate first and then for a line
/// break, as the presence pass reads them.
pub fn render_matched(lineage: &Lineage) -> Result<Vec<String>, String> {
    let presence = presence_of(lineage)?;
    let total = lineage.atlases.len();
    let mut lines: Vec<String> = Vec::new();
    let mut counts: Vec<usize> = Vec::new();
    let mut matched_any: HashSet<&PText> = HashSet::new();
    for step in 0..total - 1 {
        lines.push(format!("step {}", step + 1));
        let mut count = 0usize;
        let mut seen: HashSet<&PText> = HashSet::new();
        for entry in &lineage.atlases[step].entries {
            if !seen.insert(&entry.label) {
                continue;
            }
            if presence[step + 1].contains(&entry.label) {
                lines.push(standing_line(&presence, &entry.label, total));
                matched_any.insert(&entry.label);
                count += 1;
            }
        }
        counts.push(count);
    }
    let entries = first_standing_order(lineage).len();
    let mut closing = String::from("matched counts");
    for (index, count) in counts.iter().enumerate() {
        closing.push_str(&format!(", step {} matched {}", index + 1, count));
    }
    closing.push_str(&format!(
        ", entries matched {}, entries {}, steps {}",
        matched_any.len(),
        entries,
        total - 1
    ));
    lines.push(closing);
    Ok(lines)
}

/// The presence reading: one section per atlas of the chain,
/// each holding the standing lines of the entries standing in
/// that atlas, plus a closing line counting the standings per
/// atlas, the distinct labels, and the atlases. An empty atlas
/// leaves its section as its opening line alone. The reading
/// reads the presence pass once and states every line from it;
/// a label held twice in one atlas stands once in its section,
/// the posture FIX-MY-17 settled. The refusals are the presence
/// pass's own, in chain order, each label read for an unpaired
/// surrogate first and then for a line break.
pub fn render_presence(lineage: &Lineage) -> Result<Vec<String>, String> {
    let presence = presence_of(lineage)?;
    let total = lineage.atlases.len();
    let entries = first_standing_order(lineage).len();
    let mut lines: Vec<String> = Vec::new();
    for (index, atlas) in lineage.atlases.iter().enumerate() {
        lines.push(format!("atlas {}", index + 1));
        let mut seen: HashSet<&PText> = HashSet::new();
        for entry in &atlas.entries {
            if !seen.insert(&entry.label) {
                continue;
            }
            lines.push(standing_line(&presence, &entry.label, total));
        }
    }
    let mut closing = String::from("presence counts");
    for (index, set) in presence.iter().enumerate() {
        closing.push_str(&format!(", atlas {} standing {}", index + 1, set.len()));
    }
    closing.push_str(&format!(", entries {}, atlases {}", entries, total));
    lines.push(closing);
    Ok(lines)
}

/// Render the standings of a chain of atlases, the
/// atlases each entry stands in, stated whole under
/// its own standing line.
///
/// Presence is read once over the whole chain, each label
/// against the atlases it stands in, in the one pass
/// MY-18 states, and the pass is inverted in one walk
/// over the atlases of the chain before anything is
/// stated, holding for each label the atlases it stands
/// in, in chain order. One section stands for each label
/// in the order the labels first stand across the chain,
/// opening with the standing line of that entry formed
/// from the inversion itself, and holding one line
/// for each atlas the label stands in, in chain order,
/// in the form atlas N, one line per standing, a label
/// held more than once in one atlas named once there,
/// by the presence pass's own posture. The reading
/// closes with the standings counts line. A hand-built
/// lineage holding fewer than two atlases draws the
/// refusal MY-13 states, and a label holding an
/// unpaired surrogate draws the refusal PORT-11 names
/// for that field, and a label holding a line break
/// draws the refusal FIX-MY-13 names for that field,
/// the labels read in chain order, atlas order then
/// entry order.
pub fn render_standings(lineage: &Lineage) -> Result<Vec<String>, String> {
    let presence = presence_of(lineage)?;
    let order = first_standing_order(lineage);
    let total = lineage.atlases.len();
    let mut positions: HashMap<&PText, usize> = HashMap::new();
    for (index, label) in order.iter().enumerate() {
        positions.insert(*label, index);
    }
    let mut inverted: Vec<Vec<usize>> = vec![Vec::new(); order.len()];
    for (index, labels) in presence.iter().enumerate() {
        for label in labels {
            inverted[positions[label]].push(index + 1);
        }
    }
    let mut lines: Vec<String> = Vec::new();
    let mut standings = 0usize;
    for (label, atlases) in order.iter().zip(inverted.iter()) {
        lines.push(format!(
            "entry {}, first stood at atlas {}, last stood at atlas {}, \
             stands in {} atlases, {}",
            label,
            atlases[0],
            atlases[atlases.len() - 1],
            atlases.len(),
            if atlases[atlases.len() - 1] == total {
                "stands at the close"
            } else {
                "does not stand at the close"
            }
        ));
        for atlas_no in atlases {
            lines.push(format!("atlas {atlas_no}"));
            standings += 1;
        }
    }
    lines.push(format!(
        "standings counts, standings {standings}, entries {}, atlases {total}",
        order.len()
    ));
    Ok(lines)
}

/// Render the company of a chain of atlases, the
/// entries each entry of the chain stands with.
///
/// Presence is read once over the whole chain, each label
/// against the atlases it stands in, and the pass is
/// inverted in one walk over the atlases of the chain
/// before anything is stated, holding for each label
/// the atlases it stands in, in chain order, and the
/// company of each label is gathered from the one pass
/// in one walk over the atlases of the chain, each atlas
/// standing its entries with each other as the pass is
/// consumed. One section stands for each label of the
/// chain in the order the labels first stand across
/// the chain, each section opening with the standing
/// line of that entry formed from the inversion itself,
/// and holding one line for each entry that stands
/// with it, the standing line of that entry formed
/// from the inversion itself, in the order of first
/// standing, two entries standing with each other when
/// one atlas of the chain holds them both, an entry
/// never standing in its own company, and an entry
/// sharing no atlas with another entry holding its
/// heading alone. The reading closes with the company
/// counts line. A hand-built lineage holding fewer
/// than two atlases draws the refusal MY-13 states, and
/// a label holding an unpaired surrogate draws the
/// refusal PORT-11 names for that field, and a label
/// holding a line break draws the refusal FIX-MY-13
/// names for that field, the labels read in chain
/// order, atlas order then entry order.
pub fn render_company(lineage: &Lineage) -> Result<Vec<String>, String> {
    let presence = presence_of(lineage)?;
    let order = first_standing_order(lineage);
    let total = lineage.atlases.len();
    let mut positions: HashMap<&PText, usize> = HashMap::new();
    for (index, label) in order.iter().enumerate() {
        positions.insert(*label, index);
    }
    let mut inverted: Vec<Vec<usize>> = vec![Vec::new(); order.len()];
    for (index, labels) in presence.iter().enumerate() {
        for label in labels {
            inverted[positions[label]].push(index + 1);
        }
    }
    let mut gatherings: Vec<Vec<usize>> = vec![Vec::new(); order.len()];
    for atlas in &lineage.atlases {
        let mut held: Vec<usize> = Vec::new();
        for entry in &atlas.entries {
            let slot = positions[&entry.label];
            if !held.contains(&slot) {
                held.push(slot);
            }
        }
        for &slot in &held {
            for &other in &held {
                if other != slot && !gatherings[slot].contains(&other) {
                    gatherings[slot].push(other);
                }
            }
        }
    }
    for gathering in &mut gatherings {
        gathering.sort_unstable();
    }
    let line_of = |label: &PText, atlases: &[usize]| -> String {
        format!(
            "entry {}, first stood at atlas {}, last stood at atlas {}, \
             stands in {} atlases, {}",
            label,
            atlases[0],
            atlases[atlases.len() - 1],
            atlases.len(),
            if atlases[atlases.len() - 1] == total {
                "stands at the close"
            } else {
                "does not stand at the close"
            }
        )
    };
    let mut lines: Vec<String> = Vec::new();
    let mut company = 0usize;
    for (index, label) in order.iter().enumerate() {
        lines.push(line_of(label, &inverted[index]));
        for &slot in &gatherings[index] {
            lines.push(line_of(order[slot], &inverted[slot]));
            company += 1;
        }
    }
    lines.push(format!(
        "company counts, company {company}, entries {}, atlases {total}",
        order.len()
    ));
    Ok(lines)
}

pub fn render_sets(lineage: &Lineage) -> Result<Vec<String>, String> {
    let presence = presence_of(lineage)?;
    let order = first_standing_order(lineage);
    let total = lineage.atlases.len();
    let mut gatherings: Vec<(Vec<usize>, Vec<&PText>)> = Vec::new();
    for label in &order {
        let mut set: Vec<usize> = Vec::new();
        for (index, labels) in presence.iter().enumerate() {
            if labels.contains(label) {
                set.push(index + 1);
            }
        }
        match gatherings.iter_mut().find(|(held, _)| *held == set) {
            Some((_, labels)) => labels.push(label),
            None => gatherings.push((set, vec![label])),
        }
    }
    let mut lines: Vec<String> = Vec::new();
    let mut entries = 0usize;
    for (set, labels) in &gatherings {
        for atlas_no in set {
            lines.push(format!("atlas {atlas_no}"));
        }
        for label in labels {
            lines.push(standing_line(&presence, label, total));
            entries += 1;
        }
    }
    lines.push(format!(
        "sets counts, sets {}, entries {entries}, atlases {total}",
        gatherings.len()
    ));
    Ok(lines)
}

/// Render the rosters of a chain of atlases, the atlases
/// of the chain gathered by the set of entries each
/// holds.
///
/// Presence is read once over the whole chain, each label
/// against the atlases it stands in, and the atlases are
/// gathered by their rosters in one walk over the chain,
/// each atlas appended to its roster's gathering as the
/// pass is consumed, and one section stands for each
/// distinct roster in the order of the earliest atlas
/// holding it, each section opening with the standing
/// line of each entry of its roster through the one
/// standing line helper, in the order of first standing,
/// and holding one line for each atlas holding the
/// roster in chain order. The reading closes with the
/// rosters counts line. A hand-built lineage holding
/// fewer than two atlases draws the refusal MY-13
/// states, and a label holding an unpaired surrogate
/// draws the refusal PORT-11 names for that field, and
/// a label holding a line break draws the refusal
/// FIX-MY-13 names for that field, the labels read in
/// chain order, atlas order then entry order.
pub fn render_rosters(lineage: &Lineage) -> Result<Vec<String>, String> {
    let presence = presence_of(lineage)?;
    let order = first_standing_order(lineage);
    let total = lineage.atlases.len();
    let mut gatherings: Vec<(Vec<&PText>, Vec<usize>)> = Vec::new();
    for (index, labels) in presence.iter().enumerate() {
        let mut roster: Vec<&PText> = Vec::new();
        for label in &order {
            if labels.contains(label) {
                roster.push(label);
            }
        }
        match gatherings.iter_mut().find(|(held, _)| *held == roster) {
            Some((_, atlases)) => atlases.push(index + 1),
            None => gatherings.push((roster, vec![index + 1])),
        }
    }
    let mut lines: Vec<String> = Vec::new();
    for (roster, atlases) in &gatherings {
        for label in roster {
            lines.push(standing_line(&presence, label, total));
        }
        for atlas_no in atlases {
            lines.push(format!("atlas {atlas_no}"));
        }
    }
    lines.push(format!(
        "rosters counts, rosters {}, atlases {total}, entries {}",
        gatherings.len(),
        order.len()
    ));
    Ok(lines)
}

/// Render the fellows of a chain of atlases, the atlases
/// standing with each atlas of the chain.
///
/// Presence is read once over the whole chain, each label
/// against the atlases it stands in, and the atlases
/// standing with each atlas are gathered from the one
/// pass in one walk over the pass, each entry standing
/// its atlases with each other as the pass is consumed,
/// and one section stands for each atlas of the chain
/// in chain order, each section opening with the bare
/// atlas line of its own atlas and holding one bare
/// atlas line for each atlas standing with it in chain
/// order, an atlas never standing among its own fellows
/// and an atlas sharing no entry with any other atlas
/// holding a section of its heading alone. The reading
/// closes with the fellows counts line. A hand-built
/// lineage holding fewer than two atlases draws the
/// refusal MY-13 states, and a label holding an
/// unpaired surrogate draws the refusal PORT-11 names
/// for that field, and a label holding a line break
/// draws the refusal FIX-MY-13 names for that field,
/// the labels read in chain order, atlas order then
/// entry order.
pub fn render_fellows(lineage: &Lineage) -> Result<Vec<String>, String> {
    let presence = presence_of(lineage)?;
    let order = first_standing_order(lineage);
    let total = lineage.atlases.len();
    let mut gatherings: Vec<Vec<usize>> = vec![Vec::new(); total];
    for label in &order {
        let mut holders: Vec<usize> = Vec::new();
        for (index, labels) in presence.iter().enumerate() {
            if labels.contains(label) {
                holders.push(index);
            }
        }
        for &holder in &holders {
            for &other in &holders {
                if other != holder && !gatherings[holder].contains(&other) {
                    gatherings[holder].push(other);
                }
            }
        }
    }
    let mut lines: Vec<String> = Vec::new();
    let mut fellows = 0usize;
    for (index, gathering) in gatherings.iter().enumerate() {
        lines.push(format!("atlas {}", index + 1));
        for other in 0..total {
            if gathering.contains(&other) {
                lines.push(format!("atlas {}", other + 1));
                fellows += 1;
            }
        }
    }
    lines.push(format!(
        "fellows counts, fellows {fellows}, atlases {total}, entries {}",
        order.len()
    ));
    Ok(lines)
}

/// Render the runs of a chain of atlases, the runs of
/// atlases each entry stands in.
///
/// Presence is read once over the whole chain, each label
/// against the atlases it stands in, and one line stands
/// for each standing run of the chain in the order the
/// labels first stand across the chain, the runs of one
/// label in chain order. A standing run is a run of one
/// or more atlases in a row in which the label stands,
/// where the atlas immediately before the run, when the
/// chain holds one, is one the label does not stand in,
/// and a run that reaches the last atlas of the chain
/// closes with the words to the close. The reading
/// closes with the runs counts line. A hand-built
/// lineage holding fewer than two atlases draws the
/// refusal MY-13 states, and a label holding an unpaired
/// surrogate draws the refusal PORT-11 names for that
/// field, and a label holding a line break draws the
/// refusal FIX-MY-13 names for that field, the labels
/// read in chain order, atlas order then entry order.
pub fn render_runs(lineage: &Lineage) -> Result<Vec<String>, String> {
    let presence = presence_of(lineage)?;
    let order = first_standing_order(lineage);
    let total = lineage.atlases.len();
    let mut lines: Vec<String> = Vec::new();
    let mut runs = 0usize;
    let mut standing_at_close = 0usize;
    for label in &order {
        let mut index = 0usize;
        let mut label_closes = false;
        while index < total {
            if !presence[index].contains(label) {
                index += 1;
                continue;
            }
            let start = index;
            while index < total && presence[index].contains(label) {
                index += 1;
            }
            runs += 1;
            if index == total {
                label_closes = true;
                lines.push(format!(
                    "entry {}, standing from atlas {}, for {} atlases, to the close",
                    label,
                    start + 1,
                    index - start
                ));
            } else {
                lines.push(format!(
                    "entry {}, standing from atlas {}, for {} atlases",
                    label,
                    start + 1,
                    index - start
                ));
            }
        }
        if label_closes {
            standing_at_close += 1;
        }
    }
    lines.push(format!(
        "runs counts, runs {runs}, entries with runs {}, standing at the close {standing_at_close}",
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
/// draws the refusal PORT-11 states. The one pass of
/// FIX-MY-14 stands before the reading, so a field
/// holding a line break draws its refusal there, in the
/// field order the pass reads.
pub fn render_index(atlas: &Atlas) -> Result<Vec<String>, String> {
    refuse_atlas_fields(atlas)?;
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
