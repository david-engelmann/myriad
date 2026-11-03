//! The codepoint scan binary, the gate face of the scan
//! the library carries.
//!
//! The binary prints the line the Python scan script
//! printed and exits with the status the script took, so
//! the gate command survives the script leaving the repo.
//! It scans the tree at the path it is given, and the
//! current directory when it is given none, and the gate
//! runs it from the root of the repo.

use std::path::PathBuf;
use std::process::ExitCode;

use myriad::scan::{report_line, scan_failed, scan_tree};

fn main() -> ExitCode {
    let root = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let counts = scan_tree(&root);
    println!("{}", report_line(&counts));
    if scan_failed(&counts) {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
