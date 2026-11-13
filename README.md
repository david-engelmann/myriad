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

Build from source with a current stable Rust toolchain. The crate is edition 2021, and the gate tests on Rust 1.91.1, the toolchain the workflow pins.

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

One exploration file, whole:

```json
{
  "question": "Which week does the harbour ice survey run",
  "branches": [
    {
      "id": "root-q",
      "parent": null,
      "prompt": "Which week does the harbour ice survey run",
      "state": "open"
    },
    {
      "id": "ice-depth",
      "parent": "root-q",
      "prompt": "What does the ice gauge read at the pier",
      "state": "taken",
      "finding": "The ice gauge reads forty centimetres at the pier"
    },
    {
      "id": "ferry-house",
      "parent": "root-q",
      "prompt": "Can the survey crew stay at the old ferry house",
      "state": "abandoned",
      "reason": "The ferry house roof failed its survey in October"
    }
  ]
}
```

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
### arrivals

The arrivals of a chain, one section for each atlas holding the entries that first stand at that atlas, in the line the standing reading states for each entry.

```sh
./target/release/myriad arrivals examples/winter-spring-summer-lineage.json
```

```text
atlas 1
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close
atlas 2
entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close
atlas 3
arrivals counts, atlas 1 arrivals 3, atlas 2 arrivals 1, atlas 3 arrivals 0, entries 4, atlases 3
```
### departures

The departures of a chain, one section for each atlas holding the entries that stand in no later atlas after it, in the line the standing reading states for each entry.

```sh
./target/release/myriad departures examples/winter-spring-summer-lineage.json
```

```text
atlas 1
atlas 2
entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close
atlas 3
departures counts, atlas 1 departures 0, atlas 2 departures 1, atlas 3 departures 0, entries departed 1, entries 4, atlases 3
```
### matched

The matched of a chain, one section for each step of the chain holding the entries standing on both sides of that step, in the line the standing reading states for each entry.

```sh
./target/release/myriad matched examples/winter-spring-summer-lineage.json
```

```text
step 1
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
step 2
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
matched counts, step 1 matched 2, step 2 matched 2, entries matched 2, entries 4, steps 2
```

### runs

The runs of a chain, one line for each standing run of the chain, a run of atlases in a row an entry stands in, standing from the atlas the run opens at, for the count of atlases it runs, and to the close when the run reaches the last atlas.

```sh
./target/release/myriad runs examples/winter-spring-summer-lineage.json
```

```text
entry ferry link, standing from atlas 1, for 3 atlases, to the close
entry lifeboat cover, standing from atlas 1, for 3 atlases, to the close
entry harbour lights, standing from atlas 1, for 1 atlases
entry harbour lights, standing from atlas 3, for 1 atlases, to the close
entry ice report, standing from atlas 2, for 1 atlases
runs counts, runs 5, entries with runs 4, standing at the close 3
```

### presence

The presence of a chain, one section for each atlas of the chain holding the standing line of each entry standing in that atlas, in the line the standing reading states for each entry.

```sh
./target/release/myriad presence examples/winter-spring-summer-lineage.json
```

```text
atlas 1
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close
atlas 2
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close
atlas 3
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close
presence counts, atlas 1 standing 3, atlas 2 standing 3, atlas 3 standing 3, entries 4, atlases 3
```

### standings

The standings of a chain, one section for each entry of the chain under the line of its standing, naming each atlas of the chain the entry stands in, in chain order.

```sh
./target/release/myriad standings examples/winter-spring-summer-lineage.json
```

```text
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
atlas 1
atlas 2
atlas 3
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
atlas 1
atlas 2
atlas 3
entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close
atlas 1
atlas 3
entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close
atlas 2
standings counts, standings 9, entries 4, atlases 3
```

### company

The company of a chain, one section for each entry of the chain under the line of its standing, holding the standing line of each entry the entry stands with, in the order of first standing.

```sh
./target/release/myriad company examples/winter-spring-summer-lineage.json
```

```text
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close
entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close
entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close
entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
company counts, company 10, entries 4, atlases 3
```

### sets

The sets of a chain, one section for each distinct set of atlases the chain holds under the atlas lines of the set, naming the standing line of each entry holding exactly that set, in the order of first standing.

```sh
./target/release/myriad sets examples/winter-spring-summer-lineage.json
```

```text
atlas 1
atlas 2
atlas 3
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
atlas 1
atlas 3
entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close
atlas 2
entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close
sets counts, sets 3, entries 4, atlases 3
```

### rosters

The rosters of a chain, one section for each distinct roster the chain holds under the standing lines of its entries, naming the atlas line of each atlas holding exactly that roster, in the order of first standing.

```sh
./target/release/myriad rosters examples/winter-spring-summer-lineage.json
```

```text
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry harbour lights, first stood at atlas 1, last stood at atlas 3, stands in 2 atlases, stands at the close
atlas 1
atlas 3
entry ferry link, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry lifeboat cover, first stood at atlas 1, last stood at atlas 3, stands in 3 atlases, stands at the close
entry ice report, first stood at atlas 2, last stood at atlas 2, stands in 1 atlases, does not stand at the close
atlas 2
rosters counts, rosters 2, atlases 3, entries 4
```
### fellows

The fellows of a chain, one section for each atlas of the chain opening with its bare atlas line, holding one bare atlas line for each atlas standing with it, in chain order.

```sh
./target/release/myriad fellows examples/winter-spring-summer-lineage.json
```

```text
atlas 1
atlas 2
atlas 3
atlas 2
atlas 1
atlas 3
atlas 3
atlas 1
atlas 2
fellows counts, fellows 6, atlases 3, entries 4
```

## Status

Seeded in October 2026. Setup has landed and Rounds 1 to 22 have closed, with all slices merged on main. Part of the Spectacle line in the 39 repo storefront perfection programme.

## Programme status

- Closed perfection rounds, 22 of 100. A round counts only when a full cycle of work, audit, review, red team, and approval has closed. Seeding and setup are not rounds.
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
- Contract, MY-25 and MY-26 met. They stand in full form in CONTRACT.md, and Round 12 closed on the merged slice 13 after three reviews, with the red team's duplicated label finding repaired under FIX-MY-17.
- Contract, MY-27 and MY-28 met. They stand in full form in CONTRACT.md, and Round 13 closed on the merged slice 14 after three reviews, with the red team's duplicated label counting finding repaired under FIX-MY-19.
- Contract, MY-29 and MY-30 met. They stand in full form in CONTRACT.md, and Round 14 closed on the merged slice 15 after three reviews, with the branch level counting of the changelog repaired under FIX-MY-20.
- Slice 15 has landed. It carries the runs reading of MY-30 that names each standing run of a chain, one line for each run of atlases in a row an entry stands in, with its goldens under examples, the changelog branch counting repair of FIX-MY-20 that reads each branch id once at its first standing inside a matched entry, the documentation repairs of the matched section of the Usage and the enumeration of the doc head, and the generated fixture family extended with the runs golden and its relations at the stated shapes.
- Contract, MY-31 and MY-32 met. They stand in full form in CONTRACT.md, and Round 15 closed on the merged slice 16 after three reviews, with the third relation restated under the owner's ruling and the restated headers' split paragraphs repaired.
- Slice 16 has landed. It carries the presence reading of MY-32 that names the entries standing in each atlas of a chain in the lines of their standing, one section for each atlas, with its goldens under examples, the documentation repairs of the runs section of the Usage and the enumeration of the doc head carried to nine chain readings, and the generated fixture family extended with the presence golden and its relations at the stated shapes.
- Contract, MY-33 and MY-34 met. They stand in full form in CONTRACT.md, and Round 16 closed on the merged slice 17 after three reviews, with the three recorded deviations judged faithful reconciliations and the version 2 forms amendment deferred under its named trigger.
- Slice 17 has landed. It carries the standings reading of MY-34 that names the atlases each entry of a chain stands in, in one section for each entry under the line of its standing, with its goldens under examples, the documentation repairs of the presence section of the Usage and the enumeration of the doc head carried to ten chain readings, and the generated fixture family extended with the standings golden and its relations at the stated shapes.
- Contract, MY-35 and MY-36 met. They stand in full form in CONTRACT.md, and Round 17 closed on the merged slice 18 after three reviews, with the two recorded deviations judged faithful, the standards Minor repaired in the slice, and the forced clippy gate repair landed in the slice.
- Slice 18 has landed. It carries the company reading of MY-36 that names the entries each entry of a chain stands with, in one section for each entry under the line of its standing, with its goldens under examples, the documentation repairs of the standings section of the Usage and the enumeration of the doc head carried to eleven chain readings, and the generated fixture family extended with the company golden and its relations at the stated shapes.
- Contract, MY-37 and MY-38 met. They stand in full form in CONTRACT.md, and Round 18 closed on the merged slice 19 after three reviews, with the spec review clean and no repair required, and the red team's two named evidence voids carried to the Round 19 design warm-up.
- Slice 19 has landed. It carries the sets reading of MY-38 that gathers the entries of a chain by the set of atlases each stands in, in one section for each distinct set under the atlas lines of its set and the standing lines of the entries holding it, with its goldens under examples, the documentation repairs of the company section of the Usage and the enumeration of the doc head carried to twelve chain readings, and the generated fixture family extended with the sets golden and its relations at the stated shapes.
- Contract, MY-39 and MY-40 met. They stand in full form in CONTRACT.md, and Round 19 closed on the merged slice 20 after three reviews, with the red team clean, the figure repair and the header restatement repair landed in the slice, and the record corrections of the round carried in its closure record.
- Slice 20 has landed. It carries the rosters reading of MY-40 that gathers the atlases of a chain by the set of entries each holds, in one section for each distinct roster under the standing lines of its entries and the atlas lines of the atlases holding it, with its goldens under examples, the documentation repairs of the sets section of the Usage and the enumeration of the doc head carried to thirteen chain readings, and the generated fixture family extended with the rosters golden and its relations at the stated shapes.
- Contract, MY-41 and MY-42 met. They stand in full form in CONTRACT.md, and Round 20 closed on the merged slice 21 after three reviews, with the red team approving on recomputation of every fellows golden, FIX-MY-22 amending the reconstruction direction and the sharing clause of MY-42, and the empty atlas pinned beside it.
- Slice 21 has landed. It carries the fellows reading of MY-42 that names for each atlas of a chain the atlases standing with it, in one section for each atlas under its bare atlas line and the atlas lines of the atlases standing with it, with its goldens under examples, the documentation repairs of the rosters section of the Usage and the enumeration of the doc head carried to fourteen chain readings, and the generated fixture family extended with the fellows golden and its relations at the stated shapes.
- Contract, MY-43 and MY-44 met. They stand in full form in CONTRACT.md with FIX-MY-23, and Round 21 closed on the merged slice 22 after three reviews, with the fold confirmed behaviour-neutral on the red team's executions, the heading relation pin's MI-3 clause restated in the slice, and the third relation of MY-42 restated without its colon under FIX-MY-23.
- Slice 22 has landed. It carries the standings inversion fold of MY-43 that reads the standings of a chain in one walk over the presence pass with its heading figures taken from the inversion itself, the large shape pins and the execution record of MY-44 with the fellows header restatement the executions forced, the colon repair of FIX-MY-23 that restates the third relation of MY-42 without its colon, and the documentation repair of the fellows section of the Usage, and no new command and no new reading stands in the slice.
- Contract, MY-45 and MY-46 met. They stand in full form in CONTRACT.md, and Round 22 closed on the merged slice 23 after three reviews, with the company gathering fold of MY-45 confirmed behaviour-neutral and the large shape pins and the execution record of MY-46 standing beside it, the spec review rejecting on the one Major of the deleted render_sets doc head and the standards review and the red team approving with that same Major, the doc head restored verbatim in the slice and the spec axis closed on its delta review, and the gathering membership held in a set in the slice on CodeRabbit's Minor.
- Slice 23 has landed. It carries the company gathering fold of MY-45 that reads the company of a chain from the one presence pass gathered in one walk with its lines formed from the inversion of the pass, the large shape pins and the execution record of MY-46 that sets the MI-1 to MI-5 executions of the standings fold and the base tree record of Round 21 beside the company fold's measured record, and no new command and no new reading and no repair amendment stands in the slice.
- Slice 14 has landed. It carries the matched reading of MY-28 that names the entries standing on both sides of each step of a chain in the lines of their standing, with its goldens under examples, the posture paragraph that records the judgement of the two observations of the Round 12 red team with its two posture pins, the documentation repairs of the departures section of the Usage and the enumeration of the doc head, and the generated fixture family extended with the matched golden and the two measured bands at the stated shapes.
- Slice 13 has landed. It carries the formats amendment of FIX-MY-15 that names the three formats at version 1 in one consolidated section of the contract, the value semantics settlement of FIX-MY-16 that settles the cloning, the equality, and the formatting of a value at the depth the parser admits, the departures reading of MY-26 that names the entries that stand in no later atlas after the atlas they were last seen in, with its goldens under examples, the documentation repairs of the arrivals section, the worked example, the toolchain floor, the annex, and the settled wording, and the generated fixture family, one chain of eight loggings at the stated shapes with the goldens of its five chain readings under examples.
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
