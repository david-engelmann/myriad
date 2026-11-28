# Myriad User Guide

Myriad reads structured logs of branching investigations (`explorations`), collections of investigations (`atlases`), and chronological sequences of collections (`lineages`), printing twenty-one deterministic views over open questions, settled findings, ruled-out dead ends, and agent `dibs` claims.

## Installation

### Prebuilt binary (Linux and macOS)

```sh
curl -fsSL https://raw.githubusercontent.com/david-engelmann/myriad/main/install.sh | sh
export PATH="$HOME/.local/bin:$PATH"
```

Pass `--to <DIR>` to install somewhere other than `$HOME/.local/bin`, and ensure the destination directory stands on `PATH`.

### Cargo Binstall

```sh
cargo binstall myriad-atlas
```

### Build from source

Note that `myriad` requires **Rust `1.87` or newer** (CI runs on `1.91.1`).

```sh
git clone https://github.com/david-engelmann/myriad.git
cd myriad
cargo install --path . --bin myriad --locked
```

## 60-Second Walkthrough

From the root of a `myriad` repository checkout (`git clone https://github.com/david-engelmann/myriad.git && cd myriad`), render an exploration tree, check its branch state counts, inspect `dibs` claim totals, summarize findings and dead ends with `census`, and trace how an atlas of investigations evolved across three seasonal checkpoints.

```sh
myriad map examples/winter-ferry.json
myriad check examples/winter-ferry.json
myriad claims examples/winter-ferry-claims.json
myriad census examples/winter-ferry-claims.json
myriad index examples/winter-atlas.json
myriad changelog examples/winter-atlas.json examples/spring-atlas.json
myriad lineage examples/winter-spring-summer-lineage.json
```

## Command Reference Matrix (21 Commands)

All three version 1 JSON schemas (`myriad-exploration`, `myriad-atlas`, and `myriad-lineage`) stand documented with copy-pasteable examples in [../FORMATS.md](../FORMATS.md).

### Single exploration commands (take `<exploration.json>`)

| Command | Purpose |
| --- | --- |
| `map` | Render the full branching tree showing each branch `id`, `state`, `prompt`, `claim`, `finding`, or `reason` |
| `check` | Validate an exploration file and print four count lines (`branches`, `open`, `taken`, `abandoned`) or every defect |
| `claims` | Aggregate branch counts (`branches`, `open`, `taken`, `abandoned`) per `claim` ("dibs") holder plus unclaimed totals |
| `census` | List settled `taken` findings grouped by claim, `abandoned` reasons grouped by claim, `open` prompts, and state totals |

### Multi-question atlas commands

| Command | Arguments | Purpose |
| --- | --- | --- |
| `atlas` | `<atlas.json>` | Render every labelled exploration inside an atlas in entry order |
| `index` | `<atlas.json>` | Print a compact table-of-contents summary of entry labels, questions, and branch state counts |
| `changelog` | `<first-atlas.json> <second-atlas.json>` | Compute structural differences between two atlases (added/removed entries, newly forked branches, state transitions) |

### Temporal lineage commands (all take `<lineage.json>`)

| Command | Question answered across the chronological chain of atlases |
| --- | --- |
| `lineage` | Step-by-step composed changelogs across every consecutive pair of atlases in the chain |
| `standing` | At which first and last atlas steps does each entry stand, and whether it stands at the close |
| `presence` | Which entries stand present inside each numbered atlas of the chain |
| `standings` | Per-entry stretches and standing atlas numbers across the chain |
| `arrivals` | Which entries appear for the very first time at each atlas step |
| `departures` | Which entries make their final appearance at each atlas step and never appear later |
| `returns` | Which entries disappear for one or more steps and then return in a later atlas |
| `absence` | Contiguous runs of consecutive atlases from which a known entry stands absent |
| `runs` | Contiguous runs of consecutive atlases in which each entry stands uninterrupted |
| `matched` | Entries standing on both sides of each consecutive step transition (`atlas k` and `atlas k+1`) |
| `company` | Pairs of entries that co-occur together in at least one atlas, and how many atlases they share |
| `sets` | Equivalence classes of entries that share the exact same set of standing atlases |
| `rosters` | Equivalence classes of atlases that carry the exact same roster of entry labels |
| `fellows` | Pairs of atlases that share at least one common entry label |

## Multi-Agent Pipeline Integration Recipe

In multi-agent discovery workflows (root-cause incident investigation, architectural design spikes, or open-ended literature surveys where candidate answers are not known upfront), use `myriad` exploration JSON files as a shared blackboard.

1. **Seed the root question.** Create an exploration file with `"question"` and distinct initial root `"branches"` (`"parent": null`, `"state": "open"`).
2. **Coordinate parallel workers via `claim`.** Before a subagent investigates an open branch, record `"claim": "<agent-id>"` on that branch so coordinators can audit active workload balance via `myriad claims exploration.json` and `myriad map exploration.json`.
3. **Settle or rule out.** When a worker finishes probing a hypothesis, transition `"state"` to `"taken"` with a concise `"finding"` (or `"abandoned"` with `"reason"` when falsified) and append newly discovered sub-questions as child branches (`"parent": "<branch-id>"`).
4. **Snapshot into lineages.** Archive periodic atlas snapshots (`atlas-t1.json`, `atlas-t2.json`) and bundle them into a `myriad-lineage` JSON file so orchestrators can audit step-by-step progress (`myriad changelog atlas-t1.json atlas-t2.json`, `myriad lineage chain.json`, `myriad arrivals chain.json`, `myriad departures chain.json`).
