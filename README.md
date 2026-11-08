# myriad

Myriad is a Rust library and command line tool that reads logs of branching investigations of a question, single investigations, sets of them, and chains of them logged over time, and prints readings over them.

Myriad reads recorded logs and prints readings over them. An exploration is the log of one branching investigation of a question, its branches the forks the investigation took as it ran. An atlas is a set of such logs standing together under one title, and a lineage is the same ground logged again at successive times, one atlas per logging, and the readings print the map of a tree, the changes between two loggings, and the standings, returns, and absences of a chain. It works offline and makes no live calls, and the same file prints the same lines every time.

[![ci](https://github.com/david-engelmann/myriad/actions/workflows/ci.yml/badge.svg)](https://github.com/david-engelmann/myriad/actions/workflows/ci.yml)

```text
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close
entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close
standing counts, atlases 3, entries 4, standing at every atlas 2, standing at the close 3
```

## Install

Build from source with a Rust toolchain.

```sh
git clone https://github.com/david-engelmann/myriad.git
cd myriad
cargo build --release
```

The binary stands at target/release/myriad, and the library builds beside it.

## Quickstart

Run the standing reading over the example chain, the winter atlas, its spring successor, and the summer successor.

```sh
./target/release/myriad standing examples/winter-spring-summer-lineage.json
```

```text
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close
entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close
standing counts, atlases 3, entries 4, standing at every atlas 2, standing at the close 3
```

## Usage

The readings read three kinds of file. An exploration file holds one question and a list of branches that form a tree. Each branch holds an id, a parent that is null for a root branch or the id of another branch, a prompt, and a state of open, taken, or abandoned. A taken branch holds a finding, an abandoned branch holds a reason, and an open branch holds neither. A branch in any state may hold a claim, the label of the fork that claimed the branch. An atlas file holds a title and a list of entries, and each entry holds a label and one exploration embedded whole. A lineage file holds two or more atlases embedded whole in chain order.

Each file loads whole or not at all. A command over a malformed file prints an invalid line and then one defect line for each defect and exits with status one, and a sound file exits with status zero. The loader names every malformed field and structural defect it finds, among them a duplicate branch id, a parent that names no branch, a cycle in a parent chain, and a duplicate entry label in an atlas.

The library also carries a fork helper that builds an exploration skeleton from a question and a list of prompts, one root branch per prompt with every branch in state open, so recorded exploration files can be prepared without a model.

The examples directory holds synthetic files of each kind beside the golden output of the real commands over them, and every output below is captured from the binary over those files. The map, check, claims, and census commands also stand over one exploration file and print the branch counts alone, the claims listing that groups the branches by claim label, and the census that reads the branches back by state.

### map

The map of one exploration, its branches as a tree in file order with the branch counts at the close.

```sh
./target/release/myriad map examples/winter-ferry.json
```

```text
question, How does a small harbour town keep its ferry link alive through the winter
service (open) What service pattern keeps the link alive through the winter
  service-night (open) Should the last sailing move earlier in midwinter
    night-lights (taken) Do the pier lights meet the night sailing rules
      finding, Two pier lights fall short and are already on the works list
  service-weekend (taken) Does a weekend only pattern hold the link
    finding, A weekend only pattern breaks the Thursday hospital run
    weekend-hospital (taken) What covers the Thursday hospital run
      finding, The freight boat already covers Thursdays under its own contract
  service-weather (open) Which weather calls cancel a sailing
    weather-harbour (open) What does the harbour office need to call a cancellation
      harbour-radio (open) Which radio check opens the morning call
vessel (taken) What does the vessel need before the cold sets in
  finding, The spare vessel needs heated fuel lines fitted before November
  vessel-fuel (taken) What fuel treatment stops winter gelling
    finding, The additive dosing already on the shelf covers both boats
    fuel-storage (open) Where does treated fuel sit between deliveries
  vessel-ramp (abandoned) Can the ramp take ice loading at low tide
    reason, The ramp survey is booked for spring and cannot move
crew (open) Who crews the boat when the summer crew leaves
  crew-roster (open) What roster covers sickness in a crew of four
    roster-swap (open) Can skippers swap runs with the freight boat crew
      swap-licences (open) Do the licences allow skippers to swap boats
  crew-housing (open) Where do relief crew sleep on the island
    ferry-house (abandoned) Can the old ferry house take relief crew again
      reason, The ferry house roof failed its survey in October
money (open) Where does the winter money come from
  money-fares (open) Should winter fares differ from summer fares
    fares-passes (abandoned) Would a winter resident pass steady the income
      reason, The fare audit ruled out a resident pass after last winter
  money-grant (taken) Which grant windows open before the season
    finding, The lifeline routes grant opens for bids in September
    grant-match (open) What match funding does the lifeline grant demand
      match-council (open) Will the council underwrite the match share
  money-freight (open) Can freight contracts carry the quiet months
demand (abandoned) Who actually rides the ferry in January
  reason, The harbour survey settled the January counts last year
branches 26, open 16, taken 6, abandoned 4
```
### atlas

The atlas rendering, one section for each entry with the map and the claim lines of its exploration, and the atlas counts at the close.

```sh
./target/release/myriad atlas examples/winter-atlas.json
```

```text
atlas, Winter questions for a small harbour
entry ferry link
question, How does the harbour keep a ferry sailing through the winter
berth (open) Where does the ferry tie up when the harbour ices over
  claimed by boat crew
  crew-rest (open) Where does the crew rest between sailings
fuel (taken) What fuel does the boat burn in the cold months
  claimed by boat crew
  finding, The boat burns winter grade diesel from the harbour tank
  fuel-store (taken) Where is the winter diesel stored ashore
    claimed by harbour office
    finding, Two heated tanks stand behind the harbour office
timetable (open) How often does the boat sail in January
  night (abandoned) Does the boat sail after dark in midwinter
    reason, The harbour bylaw settles the sailing hours already
branches 6, open 3, taken 2, abandoned 1
claim boat crew, branches 2, open 1, taken 1, abandoned 0
claim harbour office, branches 1, open 0, taken 1, abandoned 0
unclaimed, branches 3, open 2, taken 0, abandoned 1
entry lifeboat cover
question, Who answers a call for help on the water in winter
call (open) How does a call for help reach the lifeboat crew
  launch (taken) How does the boat get down the slipway on ice
    finding, The slipway is gritted and the tractor is kept fuelled
    sea (open) What seas can the boat go out in
      back (abandoned) How does the crew get word back to shore
        reason, The radio drill in the spring log already answers this
branches 4, open 2, taken 1, abandoned 1
unclaimed, branches 4, open 2, taken 1, abandoned 1
entry harbour lights
question, Which harbour lights stay lit through the winter nights
branches 0, open 0, taken 0, abandoned 0
atlas counts, entries 3, branches 10, open 5, taken 3, abandoned 2
```
### index

The index of one atlas, one line for each entry with its label, its question, and the counts of its exploration alone.

```sh
./target/release/myriad index examples/winter-atlas.json
```

```text
atlas, Winter questions for a small harbour
entry ferry link, question, How does the harbour keep a ferry sailing through the winter, branches 6, open 3, taken 2, abandoned 1
entry lifeboat cover, question, Who answers a call for help on the water in winter, branches 4, open 2, taken 1, abandoned 1
entry harbour lights, question, Which harbour lights stay lit through the winter nights, branches 0, open 0, taken 0, abandoned 0
atlas counts, entries 3, branches 10, open 5, taken 3, abandoned 2
```
### changelog

The changelog of two atlases, an earlier and a later, naming the entries and branches that arrived, departed, or changed between them.

```sh
./target/release/myriad changelog examples/winter-atlas.json examples/spring-atlas.json
```

```text
earlier atlas, Winter questions for a small harbour
later atlas, Spring questions for a small harbour
entry ferry link
  branch berth, state changed, open to taken
  branch fuel, finding changed, The boat burns winter grade diesel from the harbour tank to The boat burns winter grade diesel drawn from the town depot
  branch timetable, claim changed, unclaimed to harbour office
  branch fuel-store, parent changed, fuel to timetable
  branch night, departed
  branch crew-rest, prompt changed, Where does the crew rest between sailings to Where does the crew rest between sailings in the thaw
  branch ice-watch, arrived
entry lifeboat cover
  earlier question, Who answers a call for help on the water in winter
  later question, Who answers a call for help on the water in the thaw
  branch sea, state changed, open to abandoned
entry harbour lights, departed
entry ice report, arrived
changelog counts, entries arrived 1, departed 1, matched 2, branches changed 8
```
### lineage

The lineage of a chain of atlases, the changelog of each step of the chain composed in order, closing with the summed counts of its steps.

```sh
./target/release/myriad lineage examples/winter-spring-summer-lineage.json
```

```text
earlier atlas, Winter questions for a small harbour
later atlas, Spring questions for a small harbour
entry ferry link
  branch berth, state changed, open to taken
  branch fuel, finding changed, The boat burns winter grade diesel from the harbour tank to The boat burns winter grade diesel drawn from the town depot
  branch timetable, claim changed, unclaimed to harbour office
  branch fuel-store, parent changed, fuel to timetable
  branch night, departed
  branch crew-rest, prompt changed, Where does the crew rest between sailings to Where does the crew rest between sailings in the thaw
  branch ice-watch, arrived
entry lifeboat cover
  earlier question, Who answers a call for help on the water in winter
  later question, Who answers a call for help on the water in the thaw
  branch sea, state changed, open to abandoned
entry harbour lights, departed
entry ice report, arrived
changelog counts, entries arrived 1, departed 1, matched 2, branches changed 8
earlier atlas, Spring questions for a small harbour
later atlas, Summer questions for a small harbour
entry ferry link
  branch timetable, prompt changed, How often does the boat sail in January to How often does the boat sail in July
  branch ice-watch, state changed, open to taken
entry lifeboat cover, unchanged
entry ice report, departed
entry harbour lights, arrived
changelog counts, entries arrived 1, departed 1, matched 2, branches changed 2
lineage counts, steps 2, entries arrived 2, departed 2, matched 4, branches changed 10
```
### returns

The returns of a chain, each entry that stands again after an absence, with the count of atlases it stood absent before it stood again.

```sh
./target/release/myriad returns examples/winter-spring-summer-lineage.json
```

```text
entry harbour lights, returned at step 2, absent 1 atlases
returns counts, returns 1, entries returned 1
```
### standing

The standing of a chain, the first atlas each entry stands in, the last atlas it stands in, the count of atlases it stands in, and whether it stands at the close.

```sh
./target/release/myriad standing examples/winter-spring-summer-lineage.json
```

```text
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close
entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close
standing counts, atlases 3, entries 4, standing at every atlas 2, standing at the close 3
```
### absence

The absence of a chain, each run of atlases an entry stands absent from after it has stood, with the runs that reach the close marked to the close.

```sh
./target/release/myriad absence examples/winter-spring-summer-lineage.json
```

```text
entry harbour lights, absent from atlas 2, for 1 atlases
entry ice report, absent from atlas 3, for 1 atlases, to the close
absence counts, absences 2, entries with absences 2, absent at the close 1
```

## Status

Seeded in October 2026. Setup has landed and Rounds 1 to 11 have closed, with all slices merged on main. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 11 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
- Contract, SETUP-1 to SETUP-4 and MY-1 to MY-4 met. MY-1 to MY-4 stand in full form in CONTRACT.md with the Round 1 repair amendments, and Round 1 closed on the merged slice 2 after three reviews, a repair pass, and an approving re-review.
- Contract, MY-5 and MY-6 met. They stand in full form in CONTRACT.md with the Round 2 repair amendments, and Round 2 closed on the merged slice 3 after three reviews, two repair passes, and two re-reviews, the second approving with no findings.
- Contract, MY-7 and MY-8 met. They stand in full form in CONTRACT.md with the Round 3 repair amendments, and Round 3 closed on the merged slice 4 after three reviews, a repair pass, and an approving re-review.
- Contract, MY-9 and MY-10 met. They stand in full form in CONTRACT.md, and Round 4 closed on the merged slice 5 after three reviews approving with no findings.
- Contract, MY-11 and MY-12 met. They stand in full form in CONTRACT.md, and Round 5 closed on the merged slice 6 after three reviews, a repair pass, and an approving re-review.
- Contract, MY-13 and MY-14 met. They stand in full form in CONTRACT.md, and Round 6 closed on the merged slice 7 after three reviews approving with no findings.
- Contract, MY-15 and MY-16 met. They stand in full form in CONTRACT.md, and Round 7 closed on the merged slice 8 after three reviews approving with no findings.
- Contract, MY-17 and MY-18 met. They stand in full form in CONTRACT.md, and Round 8 closed on the merged slice 9 after three reviews, a repair pass, and an approving re-review.
- Contract, MY-19 and MY-20 met. They stand in full form in CONTRACT.md, and Round 9 closed on the merged slice 10 after three reviews, with the red team's one library surface finding carried to Round 10 as a fencing component.
- Contract, MY-21 and MY-22 met. They stand in full form in CONTRACT.md, and Round 10 closed on the merged slice 11 after three reviews, with the red team's observation on the non chain renderings carried to a later round as a fencing candidate.
- Contract, MY-23 and MY-24 met. They stand in full form in CONTRACT.md, and Round 11 closed on the merged slice 12 after three reviews, with the red team's observation on the JsonValue value semantics carried to a later round as a repair candidate.
- Slice 12 has landed. It carries the rendering fencing closure of FIX-MY-14 that refuses a hand built value whose printed field holds a line break in the map, claims, census, atlas, index, and changelog renderings and in the lineage through them, and the arrivals reading that reads the presence of every entry of a chain in the one presence pass and names the entries arriving at each atlas in the lines of their standing, with the golden arrivals of the winter, spring, and summer chain and of the radio investigation chain under examples.
- Slice 11 has landed. It carries the chain fencing closure of FIX-MY-13 that refuses a hand built lineage whose entry label holds a line break in the returns, standing, and absence readings, the investigation log example of the three loggings of one branching investigation with its golden standing, returns, and absence under examples, and the investigation log framing of the first screen and this status in the scoped terms of MY-22.
- Slice 10 has landed. It carries the absence reading that reads the presence of every entry of a chain in every atlas of the chain and names each run of atlases an entry stands absent from after it has stood, the inside runs and the closing run told apart, with the golden absence of the winter, spring, and summer chain under examples, and it brings the README to the fleet README standard, with an install and a quickstart that run and the standing reading of the example chain standing as the captured output of its first screen.
- Slice 9 has landed. It carries the standing reading that reads the presence of every entry of a chain in every atlas of the chain and names the first atlas each entry stands in, the last atlas it stands in, the count of atlases it stands in, and whether it stands at the close, with the golden standing of the winter, spring, and summer chain under examples.
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
