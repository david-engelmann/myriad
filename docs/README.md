<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="media/myriad-lockup-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="media/myriad-lockup-light.svg">
    <img alt="myriad" src="media/myriad-lockup-light.svg" width="380">
  </picture>
</p>

# Myriad Documentation

Branching multi-agent investigation logs, `dibs` claim maps, multi-question atlases, and temporal lineages.

<p align="center">
  <img alt="myriad terminal demo" src="media/money-shot.gif" width="840">
</p>

## Documentation Map

| Document | What it covers |
| --- | --- |
| [GUIDE.md](GUIDE.md) | Installation, 60-second walkthrough, categorized command matrix, and multi-agent pipeline recipes |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Four-stage processing pipeline, module map, exact arithmetic guarantees, and input fencing |
| [COMPARISON.md](COMPARISON.md) | Spectacle Line 5-tool boundary matrix and comparison against cloud eval SaaS and scripts |
| [PERFORMANCE.md](PERFORMANCE.md) | Measured release binary size, sub-100ms benchmark latencies, and parser memory bounds |
| [FAQ.md](FAQ.md) | Scope boundaries, exit codes (`0`, `1`, `2`, `120`), determinism, and troubleshooting |
| [../FORMATS.md](../FORMATS.md) | Normative version 1 JSON and text input schemas, field rules, and minimal examples |
| [../schemas/](../schemas/) | Draft 2020-12 JSON Schema definitions (`*.schema.json`) for IDE autocomplete and CI validation |
| [../completions/](../completions/) | Ready-to-source `bash`, `zsh`, `fish`, and `PowerShell` tab-completion scripts |
| [../CONTRACT.md](../CONTRACT.md) | Normative behavioural specification and numbered round obligations |
| [../CHANGELOG.md](../CHANGELOG.md) | Release history and packaging milestones |

## Design Guarantees

- **Pure offline execution.** `myriad` makes zero network calls, spawns no background daemons, and mutates none of the exploration, atlas, or lineage files handed to it.
- **Zero external math dependencies.** Branch trees, `dibs` claims, structural changelogs, and fourteen temporal chain readings compute exact integer counts and deterministic set relations using the Rust standard library alone.
- **Whole-input defect reporting.** Malformed JSON files, dangling parent IDs, cycles, and duplicate open prompts return structured defect lists in one pass before printing any reading.
- **Single-line output fencing.** Checked string fields reject unpaired UTF-16 surrogates and line breaks (`\n` and `\r`) so untrusted strings cannot forge tree or summary lines, though operators displaying untrusted text in a terminal should still sanitize raw escape sequences.
