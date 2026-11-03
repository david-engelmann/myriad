//! The myriad binary, a thin shell over the library
//! command line.
//!
//! The shell reads the files the command names, writes the
//! outcome lines through the stream model PORT-13 states,
//! and exits with the status the model reads.

use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;

use myriad::cli::{decode_os_arg, dispatch, encode_os_bytes, Outcome};
use myriad::pytext::PText;

/// The size the writer model hands its accumulation down
/// at, and half of it the size the end of stream failure
/// stands at, as PORT-13 states both figures.
const HAND_DOWN_AT: usize = 8192;
const STANDS_AT: usize = 4096;

/// Drop the trailing slash and trailing slash dot tail
/// pathlib drops when it parses a path, as PORT-12 states,
/// so the file argument names the file itself.
fn drop_path_tail(bytes: &[u8]) -> Vec<u8> {
    let mut end = bytes.len();
    loop {
        if end > 1 && bytes[end - 1] == b'/' {
            end -= 1;
        } else if end > 2 && bytes[end - 2] == b'/' && bytes[end - 1] == b'.' {
            end -= 2;
        } else {
            break;
        }
    }
    bytes[..end].to_vec()
}

/// Write the standard output lines through the writer
/// model PORT-13 states, returning the exit status.
///
/// The lines accumulate, and the accumulation is handed
/// down in one write when it reaches the hand down size.
/// A handing down that fails while the command runs exits
/// with status 1, and what remains at the end is written
/// last, where a failure stands for status 120 at the
/// stands size or fewer bytes and is absorbed above it,
/// so the command keeps its own status. A single line of
/// the hand down size or more first hands down what stands
/// before it, and a failure there reads by the size of the
/// part before it, status 120 at the stands size or fewer
/// and status 1 above it, and a failure on the line itself
/// exits with status 1.
fn write_stdout(lines: &[String], own: i32) -> i32 {
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let mut pending: Vec<u8> = Vec::new();
    for line in lines {
        let mut chunk = line.as_bytes().to_vec();
        chunk.push(b'\n');
        if chunk.len() >= HAND_DOWN_AT {
            if !pending.is_empty() {
                if out.write_all(&pending).is_err() {
                    return if pending.len() <= STANDS_AT { 120 } else { 1 };
                }
                pending.clear();
            }
            if out.write_all(&chunk).is_err() {
                return 1;
            }
            continue;
        }
        pending.extend_from_slice(&chunk);
        if pending.len() >= HAND_DOWN_AT {
            if out.write_all(&pending).is_err() {
                return 1;
            }
            pending.clear();
        }
    }
    if !pending.is_empty() && out.write_all(&pending).is_err() {
        return if pending.len() <= STANDS_AT { 120 } else { own };
    }
    let _ = out.flush();
    own
}

/// Write the error lines, reporting whether any write
/// failed, since a failed write of the error stream exits
/// with status 120, as PORT-13 states.
fn write_stderr(lines: &[String]) -> bool {
    let stderr = std::io::stderr();
    let mut err = stderr.lock();
    let mut failed = false;
    for line in lines {
        if writeln!(err, "{line}").is_err() {
            failed = true;
            break;
        }
    }
    if err.flush().is_err() {
        failed = true;
    }
    failed
}

fn main() -> ExitCode {
    let argv: Vec<PText> = std::env::args_os()
        .skip(1)
        .map(|arg| decode_os_arg(&arg))
        .collect();
    let read_file = |path: &PText| -> Option<String> {
        let bytes = drop_path_tail(&encode_os_bytes(path));
        #[cfg(unix)]
        let path_buf = {
            use std::os::unix::ffi::OsStringExt;
            PathBuf::from(std::ffi::OsString::from_vec(bytes))
        };
        #[cfg(not(unix))]
        let path_buf = PathBuf::from(String::from_utf8_lossy(&bytes).into_owned());
        std::fs::read(path_buf)
            .ok()
            .and_then(|data| String::from_utf8(data).ok())
    };
    let outcome: Outcome = dispatch(&argv, &read_file);
    let mut code = write_stdout(&outcome.stdout, outcome.code);
    if write_stderr(&outcome.stderr) {
        code = 120;
    }
    ExitCode::from(code as u8)
}
