# myriad Contract

This Contract is the executable spec for myriad. Every obligation is checked by a named test or by a named check. The Contract is reported as met or not met in the README programme status.

## Setup obligations

- SETUP-1. The repo carries AGENTS.md with the canonical template verbatim
  above its marked line and the repo annex below it. Test,
  tests/test_setup.py.
- SETUP-2. The README programme status states the closed round count out
  of 100 and reports this Contract as met or not met. Test,
  tests/test_setup.py.
- SETUP-3. The codepoint scan at scripts/codepoint_scan.py counts U+2014
  and U+2013 in every Markdown and text file in the repo and requires
  zero of each. The scan runs in the local gate and in CI.
- SETUP-4. Every fixture and example committed to this repo is synthetic
  and written for this repo. Nothing real is ever committed.

## First slice obligations, stub form

The full Contract for the first slice is the first build act of this repo. These stubs state the obligations the first slice will be held to, in the form they will be extended.

- MY-1. An exploration holds one question and a tree of branches, each branch holding its parent, its prompt, its state as open, taken, or abandoned, and, when abandoned, the reason it died. Test, the first slice tests.
- MY-2. The map renders the whole tree as text a reader can walk, with taken and abandoned paths both visible and abandoned paths carrying their reasons. The same exploration always renders byte identical output. Test, the first slice tests.
- MY-3. No two open branches in one exploration carry the same prompt, so branches cannot silently duplicate each other. Test, the first slice tests.
- MY-4. The first slice forks one question into tens of branches, not a thousand, and the count of branches is reported from the tree itself. Test, the first slice tests.
