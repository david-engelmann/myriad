//! Myriad, a tool for forking one hard question into
//! branches and mapping them.
//!
//! The library carries the exploration loader and its
//! readings (the indented tree map, the branch counts, the
//! claims listing, and the territory census), the atlas
//! loader and its readings (the atlas rendering and the
//! atlas index), the fork helper that builds an exploration
//! skeleton, the JSON reading the loaders rest on, mirroring
//! the Python json semantics, and the codepoint scan. The
//! binary is a thin shell over the command line module.

pub mod atlas;
pub mod cli;
pub mod explorations;
pub mod fork;
pub mod json;
mod pyprint_table;
pub mod pytext;
pub mod scan;
