<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="media/myriad-lockup-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="media/myriad-lockup-light.svg">
    <img alt="myriad" src="media/myriad-lockup-light.svg" width="380">
  </picture>
</p>

# Myriad Documentation

Branching multi-agent investigation logs, `dibs` claim maps, multi-question atlases, and temporal lineages.

## Documentation Map

| Document | What it covers |
| --- | --- |
| [GUIDE.md](GUIDE.md) | Installation, 60-second walkthrough, categorized command matrix, and multi-agent pipeline recipes |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Four-stage processing pipeline, module map, exact arithmetic guarantees, and input fencing |
| [FAQ.md](FAQ.md) | Scope boundaries against sibling Spectacle tools, exit codes, determinism, and common troubleshooting |
| [../FORMATS.md](../FORMATS.md) | Complete version 1 JSON and text input schemas, field rules, and minimal examples |
| [../CONTRACT.md](../CONTRACT.md) | Normative behavioural specification and numbered round obligations |
| [../CHANGELOG.md](../CHANGELOG.md) | Release history and packaging milestones |

## Design Guarantees

- **Pure offline execution.** `myriad` makes zero network calls, spawns no background daemons, and mutates none of the exploration, atlas, or lineage files handed to it.
- **Zero external math dependencies.** Branch trees, `dibs` claims, structural changelogs, and fourteen temporal chain readings compute exact integer counts and deterministic set relations using the Rust standard library alone.
- **Whole-input defect reporting.** Malformed JSON files, dangling parent IDs, cycles, and duplicate open prompts return structured defect lists in one pass before printing any reading.
- **Single-line output fencing.** Checked string fields reject unpaired UTF-16 surrogates and line breaks (`\n` and `\r`) so untrusted strings cannot forge tree or summary lines, though operators displaying untrusted text in a terminal should still sanitize raw escape sequences.
