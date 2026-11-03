//! The atlas loader and its readings.
//!
//! An atlas is a set of named explorations read together. The
//! loader reads an atlas whole or not at all, every defect of
//! every entry blocking the whole reading, and the readings
//! print the atlas rendering and the atlas index. The loader
//! and the readings mirror the Python implementation entry for
//! entry, defect phrase for defect phrase.

use std::path::Path;

use crate::explorations::{
    count_branches, count_by_state, format_counts, load_exploration_data, refuse_surrogate, Branch,
    Exploration,
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
