# myriad

One hard question, a thousand subagents down every plausible branch, and an explorable map of everything they found.

## What this is

A single agent answers a hard question by walking one path and hoping it was the right one. Myriad refuses to hope. It forks the question into a thousand subagents, sends them down every plausible branch of the answer, and renders the whole storm as a map you can walk. Paths taken light up. Paths abandoned stay visible with the reasons they died. The answer you get is not a paragraph. It is a territory.

## Status

Seeded in October 2026. Setup has landed and Round 1 has closed, with slice 2 merged on main. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 1 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
- Contract, SETUP-1 to SETUP-4 and MY-1 to MY-4 met. MY-1 to MY-4 stand in full form in CONTRACT.md with the Round 1 repair amendments, and Round 1 closed on the merged slice 2 after three reviews, a repair pass, and an approving re-review.
- Contract, MY-5 and MY-6 now stand in full form in CONTRACT.md under the Round 2 obligations, and their standing is not claimed here, as Round 2 has not closed.
- Slice 3 is built and in review. It carries the dibs claim on a branch and the claims listing that groups the branches under the fork that claimed them, and the territory census that reads the map back by state, with a second synthetic example exploration carrying claims on ten of its 26 branches and its golden map, listing, and census under examples.
- Slice 2 has landed. It carries the exploration loader that names every malformed field and the structural defects of the tree, the explorable map with its counts read from the tree, the fork helper that builds an open skeleton from a list of prompts, and the map and check commands, with one synthetic example exploration of 26 branches and its golden map under examples. The first slice forks one question into tens of branches, and no branch declares a winner, because a map is not a vote.
- Local gate, python3 scripts/codepoint_scan.py and python3 -m pytest tests -q. CI runs the same commands.

## Licence

MIT. See LICENSE.
