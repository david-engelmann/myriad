//! Pins for the binary surface of the port repair, driven
//! through the built binary the way the console entry
//! drives the Python surface.
//!
//! PORT-12, the file argument is read as pathlib reads it,
//! so the tails pathlib drops open the file on every
//! command. PORT-13, the closed stream bands stand as
//! measured, so a pipe whose reader is already gone reads
//! the exit status the writer model states at each size.
//! PORT-14, a COLUMNS value padded with a separator
//! control falls back the way the Python reading falls
//! back.

mod common;

use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};

use common::{branch_json, exploration_json, jarr, jobj, jstr, temp_dir};

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_myriad")
}

/// A pipe write end whose reader is already gone, the
/// pre-closed surface the pipe pins drive. The pipe is
/// created with pipe2 under O_CLOEXEC, so the reader end
/// is close on exec from the moment it exists and no
/// child a sibling pin spawns can inherit it. Under a
/// plain pipe the window between the pipe call and the
/// close of the reader left that reader inheritable, and
/// a sibling child holding it open kept the pipe alive,
/// so the writer under test could finish its writes and
/// leave by its own status instead of failing on them.
#[cfg(unix)]
fn closed_pipe() -> Stdio {
    use std::os::unix::io::FromRawFd;
    let mut fds = [0 as libc::c_int; 2];
    unsafe {
        assert_eq!(libc::pipe2(fds.as_mut_ptr(), libc::O_CLOEXEC), 0);
        libc::close(fds[0]);
        Stdio::from(std::fs::File::from_raw_fd(fds[1]))
    }
}

fn exit_code(args: &[&str], dir: &Path, stdout: Stdio, stderr: Stdio) -> i32 {
    let mut child = Command::new(binary())
        .args(args)
        .current_dir(dir)
        .stdout(stdout)
        .stderr(stderr)
        .spawn()
        .expect("the binary starts");
    child
        .wait()
        .expect("the binary exits")
        .code()
        .expect("a status")
}

fn run_captured(args: &[&str], dir: &Path) -> (i32, String, String) {
    let mut child = Command::new(binary())
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the binary starts");
    let mut out = String::new();
    let mut err = String::new();
    child
        .stdout
        .take()
        .expect("a stdout pipe")
        .read_to_string(&mut out)
        .expect("the stdout text");
    child
        .stderr
        .take()
        .expect("a stderr pipe")
        .read_to_string(&mut err)
        .expect("the stderr text");
    let code = child
        .wait()
        .expect("the binary exits")
        .code()
        .expect("a status");
    (code, out, err)
}

/// One open branch per line, each line 38 bytes, the shape
/// the pipe probes measured.
fn branches(count: usize) -> Vec<String> {
    (0..count)
        .map(|i| {
            branch_json(
                &format!("b{i:05}"),
                None,
                &format!("unique prompt {i:05} pad"),
                "open",
            )
        })
        .collect()
}

fn write_exploration(dir: &Path, name: &str, question: &str, branch_jsons: &[String]) {
    std::fs::write(dir.join(name), exploration_json(question, branch_jsons))
        .expect("the exploration file");
}

fn write_text(dir: &Path, name: &str, contents: &str) {
    std::fs::write(dir.join(name), contents).expect("the file");
}

#[test]
fn the_path_tails_open_the_file_on_every_command() {
    let dir = temp_dir("tails");
    write_exploration(&dir, "tree.json", "q", &branches(2));
    let atlas_text = jobj(&[
        ("title", jstr("t")),
        (
            "entries",
            jarr(&[jobj(&[
                ("label", jstr("only")),
                ("exploration", exploration_json("q", &branches(1))),
            ])]),
        ),
    ]);
    std::fs::write(dir.join("atlas.json"), atlas_text).expect("the atlas file");
    let tree_name = "tree.json";
    for tail in ["/", "//", "/.", "/./", "//."] {
        let arg = format!("{tree_name}{tail}");
        let (code, out, _) = run_captured(&["map", &arg], &dir);
        assert_eq!(code, 0, "tail {tail}");
        assert!(out.starts_with("question, q\n"), "tail {tail}");
    }
    for (command, name) in [
        ("check", tree_name),
        ("claims", tree_name),
        ("census", tree_name),
        ("atlas", "atlas.json"),
        ("index", "atlas.json"),
    ] {
        let arg = format!("{name}/.");
        let (code, _, _) = run_captured(&[command, &arg], &dir);
        assert_eq!(code, 0, "command {command}");
    }
}

#[test]
fn a_tail_on_a_directory_and_on_a_missing_file_still_refuses() {
    let dir = temp_dir("tails-refused");
    std::fs::create_dir(dir.join("sub")).expect("a directory");
    for arg in ["sub/", "sub/.", "missing.json/", "missing.json/."] {
        let (code, _, _) = run_captured(&["map", arg], &dir);
        assert_eq!(code, 1, "arg {arg}");
    }
}

#[test]
fn a_small_output_on_a_closed_stdout_exits_120() {
    let dir = temp_dir("pipes");
    write_exploration(&dir, "tree.json", "q", &branches(1));
    let code = exit_code(&["map", "tree.json"], &dir, closed_pipe(), Stdio::null());
    assert_eq!(code, 120);
}

#[test]
fn a_small_defect_listing_on_a_closed_stdout_exits_120() {
    let dir = temp_dir("pipes");
    write_text(&dir, "bad.json", "not json");
    let code = exit_code(&["map", "bad.json"], &dir, closed_pipe(), Stdio::null());
    assert_eq!(code, 120);
}

#[test]
fn a_middle_band_output_on_a_closed_stdout_keeps_its_own_status() {
    let dir = temp_dir("pipes");
    write_exploration(&dir, "tree.json", "q", &branches(150));
    let code = exit_code(&["map", "tree.json"], &dir, closed_pipe(), Stdio::null());
    assert_eq!(code, 0);
}

#[test]
fn a_middle_band_defect_listing_on_a_closed_stdout_keeps_its_own_status() {
    let dir = temp_dir("pipes");
    let bad: Vec<String> = (0..200).map(|_| "1".to_string()).collect();
    write_text(&dir, "bad.json", &exploration_json("q", &bad));
    let code = exit_code(&["map", "bad.json"], &dir, closed_pipe(), Stdio::null());
    assert_eq!(code, 1);
}

#[test]
fn a_long_output_on_a_closed_stdout_exits_1() {
    let dir = temp_dir("pipes");
    write_exploration(&dir, "tree.json", "q", &branches(400));
    let code = exit_code(&["census", "tree.json"], &dir, closed_pipe(), Stdio::null());
    assert_eq!(code, 1);
}

#[test]
fn a_giant_line_after_a_small_prefix_on_a_closed_stdout_exits_120() {
    let dir = temp_dir("pipes");
    let giant = "x".repeat(12000);
    write_exploration(
        &dir,
        "tree.json",
        "q",
        &[branch_json("b", None, &giant, "open")],
    );
    let code = exit_code(&["map", "tree.json"], &dir, closed_pipe(), Stdio::null());
    assert_eq!(code, 120);
}

#[test]
fn a_giant_first_line_on_a_closed_stdout_exits_1() {
    let dir = temp_dir("pipes");
    let giant = "x".repeat(9000);
    write_exploration(&dir, "tree.json", &giant, &branches(1));
    let code = exit_code(&["map", "tree.json"], &dir, closed_pipe(), Stdio::null());
    assert_eq!(code, 1);
}

#[test]
fn a_giant_line_after_a_middle_prefix_on_a_closed_stdout_exits_1() {
    let dir = temp_dir("pipes");
    let mut held = branches(131);
    held.push(branch_json("big", None, &"x".repeat(9000), "open"));
    write_exploration(&dir, "tree.json", "q", &held);
    let code = exit_code(&["map", "tree.json"], &dir, closed_pipe(), Stdio::null());
    assert_eq!(code, 1);
}

#[test]
fn a_usage_error_on_a_closed_stderr_exits_120() {
    let dir = temp_dir("pipes");
    let code = exit_code(&[], &dir, Stdio::null(), closed_pipe());
    assert_eq!(code, 120);
}

#[test]
fn a_giant_usage_error_on_a_closed_stderr_exits_120() {
    let dir = temp_dir("pipes");
    let token = "x".repeat(20000);
    let code = exit_code(&[&token], &dir, Stdio::null(), closed_pipe());
    assert_eq!(code, 120);
}

#[test]
fn a_usage_error_on_a_live_stderr_exits_2() {
    let dir = temp_dir("pipes");
    let (code, _, err) = run_captured(&[], &dir);
    assert_eq!(code, 2);
    assert!(err.contains("usage: myriad"));
}

#[test]
fn a_sound_command_on_a_closed_stderr_exits_0() {
    let dir = temp_dir("pipes");
    write_exploration(&dir, "tree.json", "q", &branches(1));
    let code = exit_code(&["map", "tree.json"], &dir, Stdio::piped(), closed_pipe());
    assert_eq!(code, 0);
}

#[test]
fn a_columns_value_padded_with_a_separator_control_falls_back() {
    let dir = temp_dir("columns");
    let run = |columns: Option<String>| {
        let mut command = Command::new(binary());
        command
            .args(["--help"])
            .current_dir(&dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .env_remove("COLUMNS");
        if let Some(value) = columns {
            command.env("COLUMNS", value);
        }
        let mut child = command.spawn().expect("the binary starts");
        let mut out = String::new();
        child
            .stdout
            .take()
            .expect("a stdout pipe")
            .read_to_string(&mut out)
            .expect("the stdout text");
        child.wait().expect("the binary exits");
        out
    };
    let fallback = run(None);
    let padded = run(Some("\u{1c}60".to_string()));
    assert_eq!(padded, fallback);
    let stripped = run(Some("\t60".to_string()));
    assert!(stripped.contains("usage: myriad [-h]\n"));
    assert_ne!(stripped, fallback);
}
