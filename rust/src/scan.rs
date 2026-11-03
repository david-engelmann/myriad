//! The codepoint scan, carried into the library.
//!
//! The scan walks a tree and counts two dash characters in
//! every file in scope, as the Python scan script has done
//! since the first round. The scope is the suffix list the
//! contract states, widened by PORT-9 to the Rust files the
//! port adds, and the report line and the exit reading match
//! the script word for word.

use std::path::Path;

/// The em dash the scan counts.
pub const EM_DASH: char = '\u{2014}';
/// The en dash the scan counts.
pub const EN_DASH: char = '\u{2013}';

const SUFFIXES: [&str; 8] = ["md", "txt", "py", "json", "toml", "yml", "yaml", "rs"];

const IGNORED_DIRS: [&str; 8] = [
    ".git",
    ".venv",
    "venv",
    "dist",
    "build",
    "target",
    "node_modules",
    "__pycache__",
];

/// The counts a scan of one tree produces.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ScanCounts {
    /// Files read.
    pub files: usize,
    /// Em dashes found.
    pub em: usize,
    /// En dashes found.
    pub en: usize,
}

fn in_scope(path: &Path) -> bool {
    match path.extension() {
        None => true,
        Some(ext) => {
            let ext = ext.to_string_lossy();
            ext.is_empty() || SUFFIXES.contains(&ext.as_ref())
        }
    }
}

fn ignored_dir(name: &str) -> bool {
    IGNORED_DIRS.contains(&name) || name.ends_with(".egg-info")
}

fn scan_file(path: &Path, counts: &mut ScanCounts) {
    let Ok(bytes) = std::fs::read(path) else {
        return;
    };
    counts.files += 1;
    let text = String::from_utf8_lossy(&bytes);
    counts.em += text.matches(EM_DASH).count();
    counts.en += text.matches(EN_DASH).count();
}

fn scan_dir(dir: &Path, counts: &mut ScanCounts) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut paths: Vec<_> = entries.flatten().map(|entry| entry.path()).collect();
    paths.sort();
    for path in paths {
        if path.is_dir() {
            let name = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            if !ignored_dir(&name) {
                scan_dir(&path, counts);
            }
        } else if in_scope(&path) {
            scan_file(&path, counts);
        }
    }
}

/// Scan one tree and return its counts.
pub fn scan_tree(root: &Path) -> ScanCounts {
    let mut counts = ScanCounts::default();
    scan_dir(root, &mut counts);
    counts
}

/// The line the scan prints for one pair of counts.
pub fn report_line(counts: &ScanCounts) -> String {
    format!("codepoint scan U+2014={} U+2013={}", counts.em, counts.en)
}

/// Whether the counts fail the scan.
pub fn scan_failed(counts: &ScanCounts) -> bool {
    counts.em > 0 || counts.en > 0
}
