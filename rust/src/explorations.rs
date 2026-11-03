//! The exploration loader and its readings.
//!
//! An exploration is one hard question forked into branches.
//! The loader reads an exploration whole or not at all, and
//! the readings here print the indented tree map, the branch
//! counts, the claims listing, and the territory census. The
//! loader and the readings mirror the Python implementation
//! branch for branch, defect phrase for defect phrase.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::json::{parse_json, JsonValue};
use crate::pytext::PText;

/// The three branch states, in the order the contract names
/// them.
pub const STATES: [&str; 3] = ["open", "taken", "abandoned"];

/// A branch state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// The branch still wants exploring.
    Open,
    /// The branch was explored and produced a finding.
    Taken,
    /// The branch was set aside with a reason.
    Abandoned,
}

impl State {
    /// The state name as it stands in the file and the lines.
    pub fn name(&self) -> &'static str {
        match self {
            State::Open => STATES[0],
            State::Taken => STATES[1],
            State::Abandoned => STATES[2],
        }
    }

    fn from_text(text: &PText) -> Option<State> {
        if text.eq_str(STATES[0]) {
            Some(State::Open)
        } else if text.eq_str(STATES[1]) {
            Some(State::Taken)
        } else if text.eq_str(STATES[2]) {
            Some(State::Abandoned)
        } else {
            None
        }
    }
}

/// One loaded branch of an exploration.
#[derive(Clone, Debug, PartialEq)]
pub struct Branch {
    /// The branch identifier, unique across the exploration.
    pub id: PText,
    /// The parent branch identifier, or nothing at a root.
    pub parent: Option<PText>,
    /// What exploring this branch would ask.
    pub prompt: PText,
    /// Where the branch stands.
    pub state: State,
    /// The territory label the branch claims, when it does.
    pub claim: Option<PText>,
    /// What the branch found, standing only on a taken branch.
    pub finding: Option<PText>,
    /// Why the branch was set aside, standing only on an
    /// abandoned branch.
    pub reason: Option<PText>,
}

/// A loaded exploration.
#[derive(Clone, Debug, PartialEq)]
pub struct Exploration {
    /// The question the exploration forks.
    pub question: PText,
    /// The branches in the order the file lists them.
    pub branches: Vec<Branch>,
}

/// The branch counts of an exploration or a union of branches.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Counts {
    /// Every branch.
    pub total: usize,
    /// The open branches.
    pub open: usize,
    /// The taken branches.
    pub taken: usize,
    /// The abandoned branches.
    pub abandoned: usize,
}

#[derive(Clone, Debug)]
struct BranchRecord {
    clean: Option<Branch>,
    id: Option<PText>,
    parent: Option<PText>,
    parent_seen: bool,
    prompt: Option<PText>,
    state: Option<State>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Rooted,
    Cycle,
}

fn check_branch_text(
    entry: &JsonValue,
    index: usize,
    name: &str,
    defects: &mut Vec<String>,
    required: bool,
    rendered: bool,
) -> Option<PText> {
    let value = match entry.get(name) {
        None => {
            if required {
                defects.push(format!("branch {index} is missing field {name}"));
            }
            return None;
        }
        Some(value) => value,
    };
    let text = match value {
        JsonValue::Str(text) => text,
        _ => {
            defects.push(format!("branch {index} field {name} has the wrong type"));
            return None;
        }
    };
    if text.is_empty() {
        defects.push(format!("branch {index} field {name} is empty"));
        return None;
    }
    if text.holds_unpaired_surrogate() {
        defects.push(format!(
            "branch {index} field {name} holds an unpaired surrogate"
        ));
        return None;
    }
    if rendered && text.holds_line_break() {
        defects.push(format!(
            "branch {index} field {name} holds a newline or a carriage return"
        ));
        return None;
    }
    Some(text.clone())
}

fn load_branch(entry: &JsonValue, index: usize, defects: &mut Vec<String>) -> BranchRecord {
    if !entry.is_object() {
        defects.push(format!("branch {index} is not an object"));
        return BranchRecord {
            clean: None,
            id: None,
            parent: None,
            parent_seen: false,
            prompt: None,
            state: None,
        };
    }
    let before = defects.len();
    let id = check_branch_text(entry, index, "id", defects, true, true);
    let mut parent: Option<PText> = None;
    let mut parent_seen = false;
    match entry.get("parent") {
        None => {
            defects.push(format!("branch {index} is missing field parent"));
        }
        Some(JsonValue::Null) => {
            parent_seen = true;
        }
        Some(JsonValue::Str(_)) => {
            parent = check_branch_text(entry, index, "parent", defects, true, true);
            parent_seen = parent.is_some();
        }
        Some(_) => {
            defects.push(format!("branch {index} field parent has the wrong type"));
        }
    }
    let prompt = check_branch_text(entry, index, "prompt", defects, true, true);
    let mut state: Option<State> = None;
    match entry.get("state") {
        None => {
            defects.push(format!("branch {index} is missing field state"));
        }
        Some(JsonValue::Str(text)) => match State::from_text(text) {
            Some(parsed) => state = Some(parsed),
            None => {
                defects.push(format!("branch {index} field state is not a known state"));
            }
        },
        Some(_) => {
            defects.push(format!("branch {index} field state has the wrong type"));
        }
    }
    let claim = check_branch_text(entry, index, "claim", defects, false, true);
    let finding = check_branch_text(entry, index, "finding", defects, false, true);
    let reason = check_branch_text(entry, index, "reason", defects, false, true);
    if state == Some(State::Taken) {
        if entry.get("finding").is_none() {
            defects.push(format!("branch {index} is missing field finding"));
        }
        if entry.get("reason").is_some() {
            defects.push(format!(
                "branch {index} field reason is not allowed in state taken"
            ));
        }
    }
    if state == Some(State::Abandoned) {
        if entry.get("reason").is_none() {
            defects.push(format!("branch {index} is missing field reason"));
        }
        if entry.get("finding").is_some() {
            defects.push(format!(
                "branch {index} field finding is not allowed in state abandoned"
            ));
        }
    }
    if state == Some(State::Open) {
        if entry.get("finding").is_some() {
            defects.push(format!(
                "branch {index} field finding is not allowed in state open"
            ));
        }
        if entry.get("reason").is_some() {
            defects.push(format!(
                "branch {index} field reason is not allowed in state open"
            ));
        }
    }
    let clean = if defects.len() == before {
        Some(Branch {
            id: id.clone().expect("a clean branch holds an id"),
            parent: parent.clone(),
            prompt: prompt.clone().expect("a clean branch holds a prompt"),
            state: state.expect("a clean branch holds a state"),
            claim: if entry.get("claim").is_some() {
                claim.clone()
            } else {
                None
            },
            finding: if state == Some(State::Taken) {
                finding.clone()
            } else {
                None
            },
            reason: if state == Some(State::Abandoned) {
                reason.clone()
            } else {
                None
            },
        })
    } else {
        None
    };
    BranchRecord {
        clean,
        id,
        parent,
        parent_seen,
        prompt,
        state,
    }
}

fn push_cycle_defect(record: &BranchRecord, defects: &mut Vec<String>) {
    let id = record.id.as_ref().expect("a cycle holder holds an id");
    defects.push(format!("branch {id} has a cycle in its parent chain"));
}

fn structural_defects(records: &[BranchRecord]) -> Vec<String> {
    let mut defects: Vec<String> = Vec::new();
    let mut seen_ids: HashSet<PText> = HashSet::new();
    for record in records {
        if let Some(id) = &record.id {
            if seen_ids.contains(id) {
                defects.push(format!("duplicate branch id {id}"));
            } else {
                seen_ids.insert(id.clone());
            }
        }
    }
    // The first branch carrying an identifier defines its
    // parent for the cycle reading, and the order of first
    // occurrences fixes the reading order.
    let mut parent_order: Vec<PText> = Vec::new();
    let mut parent_by_id: HashMap<PText, Option<PText>> = HashMap::new();
    for record in records {
        if let Some(id) = &record.id {
            if !parent_by_id.contains_key(id) {
                parent_by_id.insert(id.clone(), record.parent.clone());
                parent_order.push(id.clone());
            }
        }
    }
    for record in records {
        if record.id.is_none() || !record.parent_seen {
            continue;
        }
        if let Some(parent) = &record.parent {
            if !seen_ids.contains(parent) {
                let id = record.id.as_ref().expect("checked above");
                defects.push(format!("branch {id} parent {parent} does not exist"));
            }
        }
    }
    // Resolve every chain once, carrying the outcome along it.
    let mut outcome_by_id: HashMap<PText, Outcome> = HashMap::new();
    for start in &parent_order {
        if outcome_by_id.contains_key(start) {
            continue;
        }
        let mut path: Vec<PText> = Vec::new();
        let mut on_path: HashSet<PText> = HashSet::new();
        let mut current: Option<PText> = Some(start.clone());
        let outcome = loop {
            match &current {
                None => break Outcome::Rooted,
                Some(node) => {
                    if !parent_by_id.contains_key(node) {
                        break Outcome::Rooted;
                    }
                    if let Some(known) = outcome_by_id.get(node) {
                        break *known;
                    }
                    if on_path.contains(node) {
                        break Outcome::Cycle;
                    }
                    path.push(node.clone());
                    on_path.insert(node.clone());
                    current = parent_by_id
                        .get(node)
                        .expect("a chain node stands in the parent map")
                        .clone();
                }
            }
        };
        for node in path {
            outcome_by_id.insert(node, outcome);
        }
    }
    // Time a depth first walk over the rooted forest, so a
    // repeated identifier can be tested against the subtree of
    // its first occurrence.
    let mut children_by_id: HashMap<PText, Vec<PText>> = HashMap::new();
    let mut roots: Vec<PText> = Vec::new();
    for id in &parent_order {
        if outcome_by_id.get(id) != Some(&Outcome::Rooted) {
            continue;
        }
        match parent_by_id
            .get(id)
            .expect("a forest node stands in the parent map")
        {
            Some(parent)
                if parent_by_id.contains_key(parent)
                    && outcome_by_id.get(parent) == Some(&Outcome::Rooted) =>
            {
                children_by_id
                    .entry(parent.clone())
                    .or_default()
                    .push(id.clone());
            }
            _ => roots.push(id.clone()),
        }
    }
    let mut entry_at: HashMap<PText, u64> = HashMap::new();
    let mut exit_at: HashMap<PText, u64> = HashMap::new();
    let mut tick: u64 = 0;
    for root in &roots {
        let mut stack: Vec<(PText, bool)> = vec![(root.clone(), false)];
        while let Some((node, leaving)) = stack.pop() {
            tick += 1;
            if leaving {
                exit_at.insert(node, tick);
                continue;
            }
            entry_at.insert(node.clone(), tick);
            stack.push((node.clone(), true));
            if let Some(children) = children_by_id.get(&node) {
                for child in children {
                    stack.push((child.clone(), false));
                }
            }
        }
    }
    let mut seen_for_cycle: HashSet<PText> = HashSet::new();
    for record in records {
        let id = match &record.id {
            None => continue,
            Some(id) => id.clone(),
        };
        let first_occurrence = !seen_for_cycle.contains(&id);
        seen_for_cycle.insert(id);
        let parent = match &record.parent {
            None => continue,
            Some(parent) => parent,
        };
        if outcome_by_id.get(parent) == Some(&Outcome::Cycle) {
            push_cycle_defect(record, &mut defects);
        } else if !first_occurrence {
            let inside = match (
                entry_at.get(record.id.as_ref().expect("checked above")),
                entry_at.get(parent),
                exit_at.get(record.id.as_ref().expect("checked above")),
                exit_at.get(parent),
            ) {
                (Some(entry), Some(parent_entry), Some(exit), Some(parent_exit)) => {
                    entry <= parent_entry && exit >= parent_exit
                }
                _ => false,
            };
            if inside {
                push_cycle_defect(record, &mut defects);
            }
        }
    }
    let mut first_open_by_prompt: HashMap<PText, PText> = HashMap::new();
    for record in records {
        if record.state != Some(State::Open) {
            continue;
        }
        if let (Some(id), Some(prompt)) = (&record.id, &record.prompt) {
            if let Some(earlier) = first_open_by_prompt.get(prompt) {
                defects.push(format!(
                    "branch {id} repeats the prompt of open branch {earlier}"
                ));
            } else {
                first_open_by_prompt.insert(prompt.clone(), id.clone());
            }
        }
    }
    defects
}

/// Load an exploration from parsed JSON data.
///
/// The Python loaders share this path between the text and
/// file entry points, and the atlas loader shares it for the
/// exploration embedded in an entry, so the port shares it in
/// the same way.
pub fn load_exploration_data(data: &JsonValue) -> (Option<Exploration>, Vec<String>) {
    if !data.is_object() {
        return (None, vec!["input is not a JSON object".to_string()]);
    }
    let mut defects: Vec<String> = Vec::new();
    let question = match data.get("question") {
        None => {
            defects.push("missing field question".to_string());
            None
        }
        Some(JsonValue::Str(text)) => {
            if text.is_empty() {
                defects.push("field question is empty".to_string());
                None
            } else if text.holds_unpaired_surrogate() {
                defects.push("field question holds an unpaired surrogate".to_string());
                None
            } else if text.holds_line_break() {
                defects.push("field question holds a newline or a carriage return".to_string());
                None
            } else {
                Some(text.clone())
            }
        }
        Some(_) => {
            defects.push("field question has the wrong type".to_string());
            None
        }
    };
    let mut records: Vec<BranchRecord> = Vec::new();
    match data.get("branches") {
        None => {
            defects.push("missing field branches".to_string());
        }
        Some(JsonValue::Arr(items)) => {
            for (index, entry) in items.iter().enumerate() {
                records.push(load_branch(entry, index, &mut defects));
            }
            defects.extend(structural_defects(&records));
        }
        Some(_) => {
            defects.push("field branches has the wrong type".to_string());
        }
    }
    if !defects.is_empty() {
        return (None, defects);
    }
    let branches: Vec<Branch> = records
        .into_iter()
        .map(|record| record.clean.expect("a sound record holds a branch"))
        .collect();
    (
        Some(Exploration {
            question: question.expect("a sound reading holds a question"),
            branches,
        }),
        Vec::new(),
    )
}

/// Read and load one exploration from its JSON text.
pub fn load_exploration_text(text: &str) -> (Option<Exploration>, Vec<String>) {
    match parse_json(text) {
        None => (None, vec!["input is not valid JSON".to_string()]),
        Some(data) => load_exploration_data(&data),
    }
}

/// Read and load one exploration from the file at a path.
pub fn load_exploration_file(path: &Path) -> (Option<Exploration>, Vec<String>) {
    let text = match std::fs::read(path) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(text) => text,
            Err(_) => return (None, vec!["file cannot be read".to_string()]),
        },
        Err(_) => return (None, vec!["file cannot be read".to_string()]),
    };
    load_exploration_text(&text)
}

/// Count branches by state over any slice of branches.
///
/// The counting runs through the one counting path the
/// crate holds, over references, so an owned slice and a
/// borrowed selection count the same way.
pub fn count_branches(branches: &[Branch]) -> Counts {
    let refs: Vec<&Branch> = branches.iter().collect();
    count_refs(&refs)
}

fn count_refs(branches: &[&Branch]) -> Counts {
    let mut counts = Counts {
        total: branches.len(),
        ..Counts::default()
    };
    for branch in branches {
        match branch.state {
            State::Open => counts.open += 1,
            State::Taken => counts.taken += 1,
            State::Abandoned => counts.abandoned += 1,
        }
    }
    counts
}

/// Count an exploration's branches by state.
pub fn count_by_state(exploration: &Exploration) -> Counts {
    count_branches(&exploration.branches)
}

/// The counts wording every reading shares.
pub fn format_counts(counts: &Counts) -> String {
    format!(
        "branches {}, open {}, taken {}, abandoned {}",
        counts.total, counts.open, counts.taken, counts.abandoned
    )
}

/// The refusal a reading returns when a value it prints
/// holds an unpaired surrogate, naming the field as
/// PORT-11 states the refusal.
pub(crate) fn refuse_surrogate(text: &PText, refusal: &str) -> Result<(), String> {
    if text.holds_unpaired_surrogate() {
        Err(refusal.to_string())
    } else {
        Ok(())
    }
}

/// Render the indented tree map of an exploration.
///
/// The finding and reason lines key on the branch state
/// alone, and a hand-built value that breaks the loader
/// preconditions draws the refusal PORT-11 states.
pub fn render_map(exploration: &Exploration) -> Result<Vec<String>, String> {
    refuse_surrogate(
        &exploration.question,
        "the question holds an unpaired surrogate",
    )?;
    let mut lines = vec![format!("question, {}", exploration.question)];
    let mut children: HashMap<PText, Vec<&Branch>> = HashMap::new();
    let mut roots: Vec<&Branch> = Vec::new();
    for branch in &exploration.branches {
        match &branch.parent {
            None => roots.push(branch),
            Some(parent) => children.entry(parent.clone()).or_default().push(branch),
        }
    }
    let mut stack: Vec<(&Branch, usize)> = roots.iter().rev().map(|branch| (*branch, 0)).collect();
    while let Some((branch, depth)) = stack.pop() {
        refuse_surrogate(&branch.id, "a branch id holds an unpaired surrogate")?;
        refuse_surrogate(
            &branch.prompt,
            "a branch prompt holds an unpaired surrogate",
        )?;
        if let Some(claim) = &branch.claim {
            refuse_surrogate(claim, "a claim holds an unpaired surrogate")?;
        }
        match branch.state {
            State::Taken => {
                let finding = branch
                    .finding
                    .as_ref()
                    .ok_or_else(|| "a taken branch holds no finding".to_string())?;
                refuse_surrogate(finding, "a finding holds an unpaired surrogate")?;
            }
            State::Abandoned => {
                let reason = branch
                    .reason
                    .as_ref()
                    .ok_or_else(|| "an abandoned branch holds no reason".to_string())?;
                refuse_surrogate(reason, "a reason holds an unpaired surrogate")?;
            }
            State::Open => {}
        }
        let indent = "  ".repeat(depth);
        lines.push(format!(
            "{indent}{} ({}) {}",
            branch.id,
            branch.state.name(),
            branch.prompt
        ));
        if let Some(claim) = &branch.claim {
            lines.push(format!("{indent}  claimed by {claim}"));
        }
        if branch.state == State::Taken {
            let finding = branch
                .finding
                .as_ref()
                .expect("the guard holds a taken finding");
            lines.push(format!("{indent}  finding, {finding}"));
        }
        if branch.state == State::Abandoned {
            let reason = branch
                .reason
                .as_ref()
                .expect("the guard holds an abandoned reason");
            lines.push(format!("{indent}  reason, {reason}"));
        }
        if let Some(kids) = children.get(&branch.id) {
            for child in kids.iter().rev() {
                stack.push((child, depth + 1));
            }
        }
    }
    lines.push(format_counts(&count_by_state(exploration)));
    Ok(lines)
}

fn group_refs<'a>(branches: &[&'a Branch]) -> (Vec<(PText, Vec<&'a Branch>)>, Vec<&'a Branch>) {
    let mut claimed: Vec<(PText, Vec<&Branch>)> = Vec::new();
    let mut positions: HashMap<PText, usize> = HashMap::new();
    let mut unclaimed: Vec<&Branch> = Vec::new();
    for branch in branches {
        match &branch.claim {
            Some(label) => match positions.get(label) {
                Some(&position) => claimed[position].1.push(branch),
                None => {
                    positions.insert(label.clone(), claimed.len());
                    claimed.push((label.clone(), vec![branch]));
                }
            },
            None => unclaimed.push(branch),
        }
    }
    (claimed, unclaimed)
}

/// Render the claims listing of an exploration.
pub fn render_claims(exploration: &Exploration) -> Result<Vec<String>, String> {
    refuse_surrogate(
        &exploration.question,
        "the question holds an unpaired surrogate",
    )?;
    let all: Vec<&Branch> = exploration.branches.iter().collect();
    let (mut claimed, unclaimed) = group_refs(&all);
    claimed.sort_by(|a, b| a.0.cmp(&b.0));
    let mut lines = vec![format!("question, {}", exploration.question)];
    for (label, group) in &claimed {
        refuse_surrogate(label, "a claim holds an unpaired surrogate")?;
        lines.push(format!(
            "claim {label}, {}",
            format_counts(&count_refs(group))
        ));
    }
    if !unclaimed.is_empty() {
        lines.push(format!(
            "unclaimed, {}",
            format_counts(&count_refs(&unclaimed))
        ));
    }
    Ok(lines)
}

/// Render the territory census of an exploration.
///
/// The closed sections read the finding or the reason the
/// branch state names, and a hand-built value that breaks
/// the loader preconditions draws the refusal PORT-11
/// states instead of failing at the process level.
pub fn render_census(exploration: &Exploration) -> Result<Vec<String>, String> {
    refuse_surrogate(
        &exploration.question,
        "the question holds an unpaired surrogate",
    )?;
    let mut lines = vec![format!("question, {}", exploration.question)];
    census_state_section(&mut lines, &exploration.branches, State::Taken)?;
    census_state_section(&mut lines, &exploration.branches, State::Abandoned)?;
    let open: Vec<&Branch> = exploration
        .branches
        .iter()
        .filter(|branch| branch.state == State::Open)
        .collect();
    if !open.is_empty() {
        lines.push("open".to_string());
        for branch in &open {
            refuse_surrogate(&branch.id, "a branch id holds an unpaired surrogate")?;
            refuse_surrogate(
                &branch.prompt,
                "a branch prompt holds an unpaired surrogate",
            )?;
            lines.push(format!("  {}, prompt, {}", branch.id, branch.prompt));
        }
    }
    lines.push(format_counts(&count_by_state(exploration)));
    Ok(lines)
}

fn census_state_section(
    lines: &mut Vec<String>,
    branches: &[Branch],
    state: State,
) -> Result<(), String> {
    let selected: Vec<&Branch> = branches
        .iter()
        .filter(|branch| branch.state == state)
        .collect();
    if selected.is_empty() {
        return Ok(());
    }
    lines.push(state.name().to_string());
    let (mut claimed, unclaimed) = group_refs(&selected);
    claimed.sort_by(|a, b| a.0.cmp(&b.0));
    for (label, group) in &claimed {
        refuse_surrogate(label, "a claim holds an unpaired surrogate")?;
        lines.push(format!("  claim {label}"));
        for branch in group {
            refuse_surrogate(&branch.id, "a branch id holds an unpaired surrogate")?;
            lines.push(format!(
                "    {}, {}, {}",
                branch.id,
                closed_field_name(state),
                closed_value(branch, state)?
            ));
        }
    }
    if !unclaimed.is_empty() {
        lines.push("  unclaimed".to_string());
        for branch in &unclaimed {
            refuse_surrogate(&branch.id, "a branch id holds an unpaired surrogate")?;
            lines.push(format!(
                "    {}, {}, {}",
                branch.id,
                closed_field_name(state),
                closed_value(branch, state)?
            ));
        }
    }
    Ok(())
}

fn closed_field_name(state: State) -> &'static str {
    match state {
        State::Taken => "finding",
        State::Abandoned => "reason",
        State::Open => unreachable!("the open section prints prompts"),
    }
}

fn closed_value(branch: &Branch, state: State) -> Result<&PText, String> {
    match state {
        State::Taken => {
            let value = branch
                .finding
                .as_ref()
                .ok_or_else(|| "a taken branch holds no finding".to_string())?;
            refuse_surrogate(value, "a finding holds an unpaired surrogate")?;
            Ok(value)
        }
        State::Abandoned => {
            let value = branch
                .reason
                .as_ref()
                .ok_or_else(|| "an abandoned branch holds no reason".to_string())?;
            refuse_surrogate(value, "a reason holds an unpaired surrogate")?;
            Ok(value)
        }
        State::Open => unreachable!("the open section prints prompts"),
    }
}
