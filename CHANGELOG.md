# Changelog

All notable product and packaging changes to `myriad` stand recorded in this file.

## Unreleased

### Added

- Prebuilt release workflow covering Linux, macOS, and Windows archives alongside a `SHA256SUMS` verification installer (`install.sh`) for Linux and macOS.
- Crate packaging and `cargo-binstall` metadata in `Cargo.toml`, excluding the internal `codepoint_scan` gate binary from published crates, plus `deny.toml` license and advisory checks.
- Documentation suite under `docs/` covering the operator guide, pipeline architecture, multi-agent orchestration recipes, and frequently asked questions, paired with dark and light vector diagrams under `docs/media/`.

## 0.1.0

### Added

- Pure Rust loaders and defect validators for version 1 exploration logs, multi question atlases, and chronological atlas lineages.
- Map, check, claims, and census readings over branching investigations with open, taken, and abandoned branch states plus dibs claim tracking.
- Three atlas readings covering full atlas rendering, question index summaries, and two atlas structural changelogs.
- Fourteen temporal chain readings tracking lineage changelogs, returns, standing, absence, arrivals, departures, matched steps, runs, presence, standings, company, sets, rosters, and fellows.
- Cross platform binary surface supporting Linux, macOS, and Windows with terminal width aware help formatting and newline injection fencing.
