//! Myriad reads logs of branching investigations of a
//! question and prints readings over them.
//!
//! The library carries the exploration loader and its
//! readings (the indented tree map, the branch counts, the
//! claims listing, and the territory census), the atlas
//! loader and its readings (the atlas rendering, the
//! atlas index, and the atlas changelog), the lineage
//! loader and its readings over a chain of atlases, the
//! lineage reading and the returns reading and the
//! standing reading and the absence reading and the
//! arrivals reading and the departures reading and the
//! matched reading and the runs reading and the
//! presence reading and the standings reading and
//! the company reading and the sets reading and the
//! rosters reading and the fellows reading, the
//! fork helper that builds an exploration skeleton,
//! the JSON reading the
//! loaders rest on, mirroring the Python json
//! semantics, and the codepoint scan. The
//! binary is a thin shell over the command line module.

pub mod atlas;
pub mod cli;
pub mod explorations;
pub mod fork;
pub mod json;
mod pyprint_table;
pub mod pytext;
pub mod scan;
