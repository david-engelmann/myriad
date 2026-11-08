//! Tests for FIX-MY-16, the JsonValue value semantics
//! repair.
//!
//! The parser admits nesting to MAX_OPEN_CONTAINERS, and
//! the derived Clone, PartialEq, and Debug recursed far
//! inside that range, the derived clone aborting with a
//! stack overflow at depth 3000 on an 8 MiB test thread in
//! the Round 11 red team's probe. The pins here run on the
//! standard test thread with no raised stack, because a
//! raised stack would settle nothing. Six of the nine
//! pins discriminate against the unrepaired type, the
//! five boundary pins aborting the test binary outright
//! and the alternate form pin failing by assertion. The
//! three remaining pins pass on the unrepaired type by
//! design, the small value Debug pin, the NaN self
//! inequality pin, and the clone pair order pin, because
//! they pin the exact behaviour the repair had to
//! preserve, and they fail the moment a repair alters it.

use myriad::json::{parse_json, JsonValue, Num, MAX_OPEN_CONTAINERS};
use myriad::pytext::PText;

/// The boundary text, an integer nested inside
/// MAX_OPEN_CONTAINERS arrays.
fn boundary_text(innermost: &str) -> String {
    format!(
        "{}{}{}",
        "[".repeat(MAX_OPEN_CONTAINERS),
        innermost,
        "]".repeat(MAX_OPEN_CONTAINERS)
    )
}

#[test]
fn a_value_at_the_boundary_clones_and_drops_whole() {
    let value = parse_json(&boundary_text("1")).expect("the boundary text parses");
    let copied = value.clone();
    drop(copied);
    drop(value);
}

#[test]
fn the_clone_of_a_boundary_value_equals_the_original() {
    let value = parse_json(&boundary_text("1")).expect("the boundary text parses");
    let copied = value.clone();
    assert_eq!(copied, value);
}

#[test]
fn a_boundary_value_with_a_different_innermost_integer_does_not_equal() {
    // An equality that reads shallowly sees two arrays of
    // one array of one array, and calls them equal. Only
    // an equality that settles at the admitted depth reads
    // the integers the two values end in.
    let first = parse_json(&boundary_text("1")).expect("the boundary text parses");
    let second = parse_json(&boundary_text("2")).expect("the boundary text parses");
    assert!(first != second);
}

#[test]
fn a_boundary_value_with_an_object_at_its_deepest_does_not_equal() {
    let first = parse_json(&boundary_text("1")).expect("the boundary text parses");
    let deep_object = format!(
        "{}{}{}",
        "[".repeat(MAX_OPEN_CONTAINERS - 1),
        "{\"k\": 1}",
        "]".repeat(MAX_OPEN_CONTAINERS - 1)
    );
    let second = parse_json(&deep_object).expect("the boundary text parses");
    assert!(first != second);
}

#[test]
fn the_debug_of_a_boundary_value_completes_at_the_computed_length() {
    let value = parse_json(&boundary_text("1")).expect("the boundary text parses");
    let text = format!("{value:?}");
    // The length is computed from the derived form's
    // pieces. The innermost value prints as Num(Int("1")),
    // thirteen characters, and each array around it adds
    // the five characters of its opening and the two of
    // its closing.
    let expected = 13 + (5 + 2) * MAX_OPEN_CONTAINERS;
    assert_eq!(text.len(), expected);
    assert!(text.starts_with("Arr([Arr(["));
    assert!(text.ends_with("])])"));
}

#[test]
fn the_debug_of_small_values_matches_the_derived_output() {
    // The expected strings are the derived output,
    // captured from the unrepaired type before the change.
    let cases: Vec<(JsonValue, &str)> = vec![
        (JsonValue::Null, "Null"),
        (JsonValue::Bool(true), "Bool(true)"),
        (JsonValue::Num(Num::Int("2".to_string())), "Num(Int(\"2\"))"),
        (JsonValue::Num(Num::Float(1.5)), "Num(Float(1.5))"),
        (JsonValue::Num(Num::Float(f64::NAN)), "Num(Float(NaN))"),
        (JsonValue::Str(PText(vec![0xD800])), "Str(PText([55296]))"),
        (JsonValue::Str(PText::from_text("q")), "Str(PText([113]))"),
        (
            JsonValue::Arr(vec![JsonValue::Null, JsonValue::Bool(true)]),
            "Arr([Null, Bool(true)])",
        ),
        (
            JsonValue::Obj(vec![(
                PText::from_text("key"),
                JsonValue::Num(Num::Int("2".to_string())),
            )]),
            "Obj([(PText([107, 101, 121]), Num(Int(\"2\")))])",
        ),
        (JsonValue::Arr(vec![]), "Arr([])"),
        (JsonValue::Obj(vec![]), "Obj([])"),
    ];
    for (value, expected) in cases {
        assert_eq!(format!("{value:?}"), expected);
    }
}

#[test]
fn the_alternate_form_writes_the_plain_form() {
    // The one deliberate deviation FIX-MY-16 names. The
    // derived alternate pretty form is not reproduced,
    // and the manual implementation writes the plain form
    // under both flags.
    let value = JsonValue::Arr(vec![JsonValue::Null]);
    assert_eq!(format!("{value:#?}"), "Arr([Null])");
    assert_eq!(format!("{value:#?}"), format!("{value:?}"));
}

#[test]
fn a_nan_stands_unequal_to_itself() {
    // The derived semantics stand exactly, a float by the
    // equality of its double, so a NaN equals nothing, not
    // even its own clone.
    let value = JsonValue::Num(Num::Float(f64::NAN));
    assert!(value != value.clone());
}

#[test]
fn a_clone_keeps_the_pairs_in_document_order() {
    let text = "{\"b\": [1, {\"a\": true}], \"a\": null}";
    let value = parse_json(text).expect("the text parses");
    let copied = value.clone();
    assert_eq!(copied, value);
    assert_eq!(format!("{copied:?}"), format!("{value:?}"));
}
