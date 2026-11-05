//! Pins for the argparse width rule the port mirrors,
//! PORT-8. These pins stand in the port corpus only. They
//! hold the help and error texts at the three terminal
//! widths the contract names, 60, 80, and 120 columns,
//! against the texts the Python surface prints at those
//! widths, captured from the Python console entry before
//! the port was written.
//!
//! The width reaches the formatter the Python way. The
//! COLUMNS value is read as a Python integer, the terminal
//! answers when the value does not stand, and 80 is the
//! floor, with two columns given back to the margin.

mod common;

use myriad::cli::{dispatch_with_columns, parse_python_int, Outcome};
use myriad::pytext::PText;

use common::pt;

fn run(argv: &[&str], columns: usize) -> Outcome {
    let args: Vec<PText> = argv.iter().map(|arg| pt(arg)).collect();
    dispatch_with_columns(&args, Some(columns), &|_| None)
}

const WIDTHS: [usize; 3] = [60, 80, 120];

fn top_help(columns: usize) -> Vec<String> {
    let usage: &[&str] = if columns == 60 {
        &[
            "usage: myriad [-h]",
            "              {map,check,claims,census,atlas,index,changelog,lineage}",
            "              ...",
        ]
    } else {
        &["usage: myriad [-h] {map,check,claims,census,atlas,index,changelog,lineage} ..."]
    };
    let mut lines: Vec<String> = usage.iter().map(|line| line.to_string()).collect();
    lines.push(String::new());
    lines.push("Fork one hard question into branches and map them.".to_string());
    lines.push(String::new());
    lines.push("positional arguments:".to_string());
    lines.push("  {map,check,claims,census,atlas,index,changelog,lineage}".to_string());
    lines.push("    map                 print the map of a tree".to_string());
    lines.push("    check               print the branch counts of a tree".to_string());
    lines.push("    claims              print the claims listing of a tree".to_string());
    if columns == 60 {
        lines.push("    census              print the territory census of a".to_string());
        lines.push("                        tree".to_string());
        lines.push("    atlas               print the rendering of an atlas of".to_string());
        lines.push("                        trees".to_string());
        lines.push("    index               print the index of an atlas of".to_string());
        lines.push("                        trees".to_string());
        lines.push("    changelog           print the changelog of two atlases".to_string());
        lines.push("    lineage             print the lineage of a chain of".to_string());
        lines.push("                        atlases".to_string());
    } else {
        lines.push("    census              print the territory census of a tree".to_string());
        lines.push("    atlas               print the rendering of an atlas of trees".to_string());
        lines.push("    index               print the index of an atlas of trees".to_string());
        lines.push("    changelog           print the changelog of two atlases".to_string());
        lines.push("    lineage             print the lineage of a chain of atlases".to_string());
    }
    lines.push(String::new());
    lines.push("options:".to_string());
    lines.push("  -h, --help            show this help message and exit".to_string());
    lines
}

fn top_usage_lines(columns: usize) -> Vec<String> {
    if columns == 60 {
        common::strings(&[
            "usage: myriad [-h]",
            "              {map,check,claims,census,atlas,index,changelog,lineage}",
            "              ...",
        ])
    } else {
        common::strings(&[
            "usage: myriad [-h] {map,check,claims,census,atlas,index,changelog,lineage} ...",
        ])
    }
}

#[test]
fn the_top_help_stands_at_sixty_eighty_and_one_twenty() {
    for columns in WIDTHS {
        let outcome = run(&["--help"], columns);
        assert_eq!(outcome.code, 0, "columns {columns}");
        assert_eq!(outcome.stdout, top_help(columns), "columns {columns}");
        assert!(outcome.stderr.is_empty(), "columns {columns}");
    }
}

#[test]
fn the_subcommand_help_stands_at_sixty_eighty_and_one_twenty() {
    for columns in WIDTHS {
        for command in ["map", "atlas"] {
            let outcome = run(&[command, "--help"], columns);
            assert_eq!(outcome.code, 0, "columns {columns}");
            assert_eq!(
                outcome.stdout,
                common::strings(&[
                    &format!("usage: myriad {command} [-h] file"),
                    "",
                    "positional arguments:",
                    "  file",
                    "",
                    "options:",
                    "  -h, --help  show this help message and exit",
                ]),
                "columns {columns}"
            );
            assert!(outcome.stderr.is_empty(), "columns {columns}");
        }
    }
}

#[test]
fn the_missing_command_error_stands_at_sixty_eighty_and_one_twenty() {
    for columns in WIDTHS {
        let outcome = run(&[], columns);
        assert_eq!(outcome.code, 2, "columns {columns}");
        assert!(outcome.stdout.is_empty(), "columns {columns}");
        let mut expected = top_usage_lines(columns);
        expected.push("myriad: error: the following arguments are required: command".to_string());
        assert_eq!(outcome.stderr, expected, "columns {columns}");
    }
}

#[test]
fn the_invalid_choice_error_stands_at_sixty_eighty_and_one_twenty() {
    for columns in WIDTHS {
        let outcome = run(&["bogus"], columns);
        assert_eq!(outcome.code, 2, "columns {columns}");
        let mut expected = top_usage_lines(columns);
        expected.push(
            "myriad: error: argument command: invalid choice: 'bogus' \
             (choose from 'map', 'check', 'claims', 'census', 'atlas', 'index', 'changelog', 'lineage')"
                .to_string(),
        );
        assert_eq!(outcome.stderr, expected, "columns {columns}");
    }
}

#[test]
fn the_missing_file_error_stands_at_sixty_eighty_and_one_twenty() {
    for columns in WIDTHS {
        let outcome = run(&["map"], columns);
        assert_eq!(outcome.code, 2, "columns {columns}");
        assert_eq!(
            outcome.stderr,
            common::strings(&[
                "usage: myriad map [-h] file",
                "myriad map: error: the following arguments are required: file",
            ]),
            "columns {columns}"
        );
    }
}

// The PORT-14 pins. The int reading strips the int set
// exactly, so a COLUMNS value padded with a separator
// control the string whitespace set holds fails the
// reading, and the neighbours of the four controls read
// as the Python reading reads them.

/// A COLUMNS body padded on both sides with one code point.
fn padded(unit: u32) -> PText {
    let mut units = vec![unit];
    units.extend("60".chars().map(|ch| ch as u32));
    units.push(unit);
    PText(units)
}

#[test]
fn the_separator_controls_fail_the_int_reading() {
    for unit in [0x1Cu32, 0x1D, 0x1E, 0x1F] {
        assert_eq!(parse_python_int(&padded(unit)), None, "unit {unit:#x}");
        let mut trailing = "60".chars().map(|ch| ch as u32).collect::<Vec<_>>();
        trailing.push(unit);
        assert_eq!(
            parse_python_int(&PText(trailing)),
            None,
            "trailing unit {unit:#x}"
        );
    }
}

#[test]
fn the_int_set_padding_reads_the_number() {
    for unit in [0x09u32, 0x0A, 0x0D, 0x20, 0x85, 0xA0, 0x3000] {
        assert_eq!(parse_python_int(&padded(unit)), Some(60), "unit {unit:#x}");
    }
}

#[test]
fn the_neighbours_of_the_separator_controls_fail_the_int_reading() {
    for unit in [0x1Bu32, 0x200B] {
        assert_eq!(parse_python_int(&padded(unit)), None, "unit {unit:#x}");
    }
}
