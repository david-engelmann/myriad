//! Pins for the codepoint scan scope stated in SETUP-3 as
//! amended by FIX-MY-1, translated from tests/test_scan.py.
//!
//! The Python pins run the scan as a subprocess against a
//! planted tree, because the rule is about which files on
//! disk the scan opens. The Rust pins plant the same trees
//! and read them through the scan library the crate carries,
//! which is the same scope read from the same side.
//!
//! The last pin is the PORT-9 addition. A dash standing in a
//! Rust file fails the scan, because the scan covers the
//! .rs suffix from the port on.

mod common;

use std::fs;
use std::path::PathBuf;

use myriad::scan::{report_line, scan_failed, scan_tree};

use common::temp_dir;

const EM_DASH: &str = "\u{2014}";
const EN_DASH: &str = "\u{2013}";

fn plant(tag: &str, name: &str, contents: &str) -> PathBuf {
    let root = temp_dir(tag);
    let path = root.join(name);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("a planted directory");
    }
    fs::write(&path, contents).expect("a planted file");
    root
}

#[test]
fn a_clean_planted_tree_passes() {
    let root = plant("scan-clean", "notes.md", "plain text\n");
    let counts = scan_tree(&root);
    assert!(!scan_failed(&counts));
    assert_eq!(report_line(&counts), "codepoint scan U+2014=0 U+2013=0");
}

#[test]
fn a_dash_in_an_extensionless_file_fails_the_scan() {
    let root = plant(
        "scan-license",
        "LICENSE",
        &format!("licence text {EM_DASH} more text\n"),
    );
    let counts = scan_tree(&root);
    assert!(scan_failed(&counts));
    assert!(report_line(&counts).contains("U+2014=1"));
}

#[test]
fn a_dash_in_a_python_file_fails_the_scan() {
    let root = plant(
        "scan-python",
        "module.py",
        &format!("# a comment {EN_DASH} here\n"),
    );
    let counts = scan_tree(&root);
    assert!(scan_failed(&counts));
    assert!(report_line(&counts).contains("U+2013=1"));
}

#[test]
fn a_dash_in_a_json_file_fails_the_scan() {
    let root = plant(
        "scan-json",
        "data.json",
        &format!("{{\"note\": \"before {EM_DASH} after\"}}\n"),
    );
    let counts = scan_tree(&root);
    assert!(scan_failed(&counts));
    assert!(report_line(&counts).contains("U+2014=1"));
}

#[test]
fn a_dash_in_a_rust_file_fails_the_scan() {
    let root = plant(
        "scan-rust",
        "module.rs",
        &format!("// a comment {EM_DASH} here\n"),
    );
    let counts = scan_tree(&root);
    assert!(scan_failed(&counts));
    assert!(report_line(&counts).contains("U+2014=1"));
}

#[test]
fn dashes_inside_an_ignored_directory_pass_the_scan() {
    let root = plant(
        "scan-ignored",
        ".venv/planted.md",
        &format!("{EM_DASH} and {EN_DASH}\n"),
    );
    let counts = scan_tree(&root);
    assert!(!scan_failed(&counts));
    assert_eq!(report_line(&counts), "codepoint scan U+2014=0 U+2013=0");
}
