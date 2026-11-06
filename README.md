# myriad

Myriad is a Rust library and command line tool that reads recorded explorations of a question and atlases of explorations and prints readings over them.

## What this is

Myriad reads recorded JSON files and prints readings over them. It works offline and makes no live calls, and the same file always prints the same lines. The library also carries a fork helper that builds an exploration skeleton from a question and a list of prompts, one root branch per prompt with every branch in state open, so recorded exploration files can be prepared without a model.

An exploration file holds one question and a list of branches that form a tree. Each branch holds an id, a parent that is null for a root branch or the id of another branch, a prompt, and a state of open, taken, or abandoned. A taken branch holds a finding, an abandoned branch holds a reason, and an open branch holds neither. A branch in any state may hold a claim, the label of the fork that claimed the branch. An atlas file holds a title and a list of entries, and each entry holds a label and one exploration embedded whole. A lineage file holds two or more atlases embedded whole in chain order.

Each file loads whole or not at all. A malformed file yields no reading, and its loader names every malformed field and structural defect it finds, among them a duplicate branch id, a parent that names no branch, a cycle in a parent chain, and a duplicate entry label in an atlas.

The myriad command prints the readings. The map, check, claims, and census commands each take one exploration file and print the map of the branches with their counts, the branch counts alone, the claims listing that groups the branches by claim label, and the census that reads the branches back by state with their findings, reasons, and prompts. The atlas and index commands each take one atlas file and print the atlas rendering, one section for each entry, and the index, one line for each entry with the counts of its exploration. The changelog command takes two atlas files, an earlier and a later, and prints the entries and branches that arrived, departed, or changed between them. The lineage and returns commands each take one lineage file and print the changelog of each step of the chain composed in order and the entries that stand again in the chain after an absence. A command over a malformed file prints an invalid line and then one defect line for each defect and exits with status one. A sound file exits with status zero. The examples directory holds synthetic files of each kind beside the golden output of the real commands over them.

## Status

Seeded in October 2026. Setup has landed and Rounds 1 to 7 have closed, with all slices merged on main. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 7 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
- Contract, SETUP-1 to SETUP-4 and MY-1 to MY-4 met. MY-1 to MY-4 stand in full form in CONTRACT.md with the Round 1 repair amendments, and Round 1 closed on the merged slice 2 after three reviews, a repair pass, and an approving re-review.
- Contract, MY-5 and MY-6 met. They stand in full form in CONTRACT.md with the Round 2 repair amendments, and Round 2 closed on the merged slice 3 after three reviews, two repair passes, and two re-reviews, the second approving with no findings.
- Contract, MY-7 and MY-8 met. They stand in full form in CONTRACT.md with the Round 3 repair amendments, and Round 3 closed on the merged slice 4 after three reviews, a repair pass, and an approving re-review.
- Contract, MY-9 and MY-10 met. They stand in full form in CONTRACT.md, and Round 4 closed on the merged slice 5 after three reviews approving with no findings.
- Contract, MY-11 and MY-12 met. They stand in full form in CONTRACT.md, and Round 5 closed on the merged slice 6 after three reviews, a repair pass, and an approving re-review.
- Contract, MY-13 and MY-14 met. They stand in full form in CONTRACT.md, and Round 6 closed on the merged slice 7 after three reviews approving with no findings.
- Contract, MY-15 and MY-16 met. They stand in full form in CONTRACT.md, and Round 7 closed on the merged slice 8 after three reviews approving with no findings.
- Slice 9 is built and in review. It carries the standing reading that reads the presence of every entry of a chain in every atlas of the chain and names the first atlas each entry stands in, the last atlas it stands in, the count of atlases it stands in, and whether it stands at the close, with the golden standing of the winter, spring, and summer chain under examples.
- Slice 8 has landed. It carries the returns reading that joins the presences of the entries of a chain across the whole chain and names each entry that stands again after an absence, with the golden returns of the winter, spring, and summer chain under examples.
- Slice 7 has landed. It carries the atlas lineage that reads a chain of atlases as the changelogs of its steps composed and closes a longer chain with the summed counts of its steps, with the winter atlas, its spring successor, and the summer successor and their golden lineage under examples.
- Slice 6 has landed. It carries the atlas changelog that reads two atlases against each other and names the entries and branches that arrived, departed, or changed between them, with the winter atlas and its stated successor and their golden changelog under examples.
- Slice 5 has landed. It carries the atlas index that reads one atlas as a table of contents, one line for each entry carrying its label, its question, and the counts of its exploration alone, standing between the title line and the atlas counts line of the atlas rendering, with the golden index of the winter atlas under examples.
- The Rust port has landed. It carries the same behavior as the Python package it replaces, checked against the same goldens, and the Python tree stands removed from the repo.
- Slice 4 has landed. It carries the atlas loader that loads a file of labelled explorations only whole and names the entry standing behind every defect, and the atlas rendering that walks the entries in file order with each section holding the map and the claim lines of its exploration and the closing counts read over the union of every entry, with a synthetic example atlas of three explorations of differing shapes and its golden rendering under examples.
- Slice 3 has landed. It carries the dibs claim on a branch and the claims listing that groups the branches under the fork that claimed them, and the territory census that reads the map back by state, with a second synthetic example exploration carrying claims on ten of its 26 branches and its golden map, listing, and census under examples.
- Slice 2 has landed. It carries the exploration loader that names every malformed field and the structural defects of the tree, the explorable map with its counts read from the tree, the fork helper that builds an open skeleton from a list of prompts, and the map and check commands, with one synthetic example exploration of 26 branches and its golden map under examples. The first slice forks one question into tens of branches, and no branch declares a winner, because a map is not a vote.
- Local gate, cargo test, cargo clippy --all-targets -- -D warnings, and cargo fmt --check, and the codepoint scan, cargo run --bin codepoint_scan. CI runs the same commands.

## Licence

MIT. See LICENSE.
