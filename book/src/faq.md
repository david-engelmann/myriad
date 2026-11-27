# Myriad FAQ

## Why is the crates.io package named `myriad-atlas` while the binary is `myriad`?

The bare crate name `myriad` was registered years ago on crates.io by an unrelated library. Publishing the package metadata under `myriad-atlas` allows clean `cargo install myriad-atlas` and `cargo binstall myriad-atlas` installation (with internal gate binaries excluded) while keeping the installed executable named `myriad` and the Rust library target named `myriad`.

## How does Myriad differ from Grandjury and Deadheat?

- **`myriad`** maps problems that do not have candidate answers yet, tracking a branching tree of open questions, confirmed findings (`taken`), and falsified dead ends (`abandoned`).
- **`grandjury`** polls a panel of models once discrete candidate answers already exist, tallying votes, abstentions, and agreement.
- **`deadheat`** races two competing implementations against a pass/fail test oracle to pick one winner.

## What is a `claim` ("dibs") on a branch?

Each branch in an exploration may carry an optional `"claim": "<worker-label>"` string (`MY-5`). Claims record which agent or human investigator holds dibs on exploring that branch, summarized by `myriad claims <exploration.json>` and grouped inside `myriad census <exploration.json>`.

## Does Myriad reject cycles, dangling parent IDs, or duplicate open prompts?

Yes. `load_exploration_text` verifies that every non-null `parent` names an existing branch `id` in the same exploration, that parent pointers form a strict cycle-free forest, and that no two `open` branches repeat the exact same `prompt` (`MY-3`). Any violation refuses the load and prints the exact defect phrase.

## What do the exit codes mean?

- **`0`** Valid exploration confirmed (`check`) or reading printed cleanly.
- **`1`** Malformed exploration, atlas, or lineage file refused with structured defect lines on `stdout`, or a closed `stdout` pipe on large output (`>= 8192` bytes under `PORT-13`, so scripts should distinguish input validation failures by inspecting output or running `myriad check` first).
- **`2`** Unknown subcommand, unrecognized flags, or wrong file argument count on `stderr`.
- **`120`** Failed `stderr` stream write, or a closed `stdout` pipe on short output (`<= 4096` bytes under `PORT-13`).
