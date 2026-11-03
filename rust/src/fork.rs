//! The fork helper.
//!
//! The helper builds the widest part of a search by hand, the
//! skeleton of an exploration holding one open root branch per
//! seed prompt, ready to write as a file and grow. The Python
//! helper raises one error at a time in a fixed check order,
//! and the port keeps that order, so a caller sees the same
//! first refusal the Python caller sees.

use crate::explorations::{Branch, Exploration, State};
use crate::pytext::PText;

/// Build an exploration skeleton from a question and prompts.
///
/// The result is the skeleton, or the text of the one refusal
/// the input draws, in the check order the Python helper
/// states. The Python helper also refuses a question or a
/// prompt that is not a string at all. That case has no shape
/// here, because the parameter types admit only text, so the
/// refusal stands by construction rather than by a check.
pub fn fork_skeleton(question: &PText, prompts: &[PText]) -> Result<Exploration, String> {
    if question.is_empty() {
        return Err("the question must be a non empty string".to_string());
    }
    if question.holds_unpaired_surrogate() {
        return Err("the question holds an unpaired surrogate".to_string());
    }
    if question.holds_line_break() {
        return Err("the question holds a newline or a carriage return".to_string());
    }
    let mut branches: Vec<Branch> = Vec::new();
    let mut seen: Vec<&PText> = Vec::new();
    for (offset, prompt) in prompts.iter().enumerate() {
        if prompt.is_empty() {
            return Err("each prompt must be a non empty string".to_string());
        }
        if prompt.holds_unpaired_surrogate() {
            return Err("each prompt must hold no unpaired surrogate".to_string());
        }
        if prompt.holds_line_break() {
            return Err("each prompt must hold no newline or carriage return".to_string());
        }
        if seen.contains(&prompt) {
            return Err(format!("two prompts are the same, {prompt}"));
        }
        seen.push(prompt);
        branches.push(Branch {
            id: PText::from_text(&format!("b{:02}", offset + 1)),
            parent: None,
            prompt: prompt.clone(),
            state: State::Open,
            claim: None,
            finding: None,
            reason: None,
        });
    }
    Ok(Exploration {
        question: question.clone(),
        branches,
    })
}
