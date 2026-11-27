# Changelog

All notable product and packaging changes to `myriad` stand recorded in this file.

## Unreleased

### Added

- Prebuilt five-target release workflow (`release.yml`) bundling shell completions and JSON Schemas into Linux, macOS, and Windows archives alongside a `SHA256SUMS` verification installer (`install.sh`).
- Crate packaging and `cargo-binstall` metadata in `Cargo.toml`, excluding internal gate binaries and handbook build sources so published `.crate` archives stay compact, plus `deny.toml` license and advisory checks.
- Draft 2020-12 JSON Schemas under `schemas/` and static tab-completion scripts under `completions/` for `bash`, `zsh`, `fish`, and `PowerShell`.
- Searchable `mdBook` operator handbook under `book/` paired with GitHub Pages CI (`docs.yml`), clean-room installer verification (`clean-install.yml`), and three-OS matrix validation (`cross-platform.yml`).
- Documentation and visual media suite under `docs/` and `docs/media/` covering the operator guide, pipeline architecture, Spectacle Line comparison, measured Apple Silicon release benchmarks, social preview cards, animated CLI showcase, and pinned-compatible storefront README draft.

## 0.1.0

### Added

- Pure Rust loaders and defect validators for version 1 exploration logs, multi question atlases, and chronological atlas lineages.
- Map, check, claims, and census readings over branching investigations with open, taken, and abandoned branch states plus dibs claim tracking.
- Three atlas readings covering full atlas rendering, question index summaries, and two atlas structural changelogs.
- Fourteen temporal chain readings tracking lineage changelogs, returns, standing, absence, arrivals, departures, matched steps, runs, presence, standings, company, sets, rosters, and fellows.
- Cross platform binary surface supporting Linux, macOS, and Windows with terminal width aware help formatting and newline injection fencing.
