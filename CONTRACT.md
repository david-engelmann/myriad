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
  and written for this repo. Nothing real is ever committed. Check, a
  manual sanitization review of every added fixture and example, with
  its result recorded in the gate report.

## First slice obligations, stub form

The full Contract for the first slice is the first build act of this repo. These stubs state the obligations the first slice will be held to, in the form they will be extended.

- MY-1. An exploration holds one question and a tree of branches, each branch holding its parent, its prompt, its state as open, taken, or abandoned, and, when abandoned, the reason it died. Test, the first slice tests.
- MY-2. The map renders the whole tree as text a reader can walk, with taken and abandoned paths both visible and abandoned paths carrying their reasons. The same exploration always renders byte identical output. Test, the first slice tests.
- MY-3. No two open branches in one exploration carry the same prompt, so branches cannot silently duplicate each other. Test, the first slice tests.
- MY-4. The first slice forks one question into tens of branches, not a thousand, and the count of branches is reported from the tree itself. Test, the first slice tests.

## First slice obligations, full form

This section extends the stub section above, which stays standing as the record of the form these obligations started in. The full form below is the text slice 2 is held to. Where the two disagree, the full form governs.

### The exploration shape

An exploration is one JSON object with the fields named here. The field question is a non empty string. The field branches is a list of branch objects, and the list may be empty. Each branch object holds an id field with a non empty string, a parent field holding null for a root or the id string of another branch, a prompt field with a non empty string, and a state field holding one of the three state words open, taken, and abandoned. A branch in state taken holds a finding field with a non empty string and holds no reason field. A branch in state abandoned holds a reason field with a non empty string and holds no finding field. A branch in state open holds neither a finding field nor a reason field. Fields the contract does not name are ignored, and the exploration the loader returns carries only the named fields. Test, tests/test_explorations.py.

### MY-1 full form, the loader names every malformed field and never raises

The loader entry points are load_exploration_text and load_exploration_file in the myriad.explorations module. Each returns a pair of the exploration and the list of defects it found. A clean input returns the exploration and an empty defect list. A malformed input returns no exploration and a defect list naming every malformed field the loader could reach, and neither entry point ever raises on malformed content. A file that cannot be read returns the single defect named file cannot be read. Text that is not JSON at all draws the defect named input is not valid JSON, and input nested too deeply for the JSON parser to read draws the same defect. JSON that parses to something other than an object draws the defect named input is not a JSON object. An absent top level field draws missing field and the field name. A top level field of the wrong kind draws field and the field name and has the wrong type. An empty question draws field question is empty. A branch that is not an object draws branch and its index and is not an object, and branch indexes count from zero. Inside a branch, an absent field draws branch and the index and is missing field and the field name. A field of the wrong kind draws branch and the index and field and the field name and has the wrong type. An empty string field draws branch and the index and field and the field name and is empty. A state string that is none of the three state words draws branch and the index and field state is not a known state. A finding or a reason carried by a branch in a state that forbids it draws branch and the index and field and the field name and is not allowed in state and the state word. A string holding an unpaired surrogate cannot be printed or written as UTF-8 text, and a text field holding one draws the holds an unpaired surrogate defect in the shape of its field. One malformed input can draw several defects in one pass, and the loader keeps naming fields after the first defect it finds. Test, tests/test_explorations.py.

The loader also names the structural defects of the tree. A branch whose id repeats the id of an earlier branch draws duplicate branch id and the id. A branch whose parent names no branch in the exploration draws branch and the branch id and parent and the parent id and does not exist. A branch whose parent chain loops without reaching a root draws branch and the branch id and has a cycle in its parent chain. The duplicate id check runs over every branch carrying a valid id. The parent checks run over every branch carrying a valid id and a valid parent value, and a parent is satisfied when any branch in the exploration carries that id. An exploration carrying any structural defect returns no exploration. Test, tests/test_explorations.py.

### MY-2 full form, the map renders the whole tree and never changes

The render_map function in the myriad.explorations module takes a loaded exploration and returns the map as a list of lines, and the map is a pure function of the exploration. The first line names the question, in the form question, and the question text. The walk that follows is depth first. Roots stand in the order they appear in the branches list, and the children of each branch stand in the order they appear in the branches list. Each branch line carries two spaces of indent per depth level, then the branch id, then the state word in parentheses, then the prompt. A taken branch carries one more line of its own, indented one level deeper than the branch line, in the form finding, and the finding text. An abandoned branch carries one more line of its own, indented one level deeper than the branch line, in the form reason, and the reason text. The last line counts the branches, in the form branches and the total and open and the open count and taken and the taken count and abandoned and the abandoned count. The count_by_state function in the same module returns the total and the three counts, counted by walking the branches of the exploration, and the summary line and the check command both read their numbers from it. The same exploration always renders the same lines, byte identical. Test, tests/test_map.py, and the golden map held in tests/test_examples.py.

### MY-3 full form, no two open branches carry the same prompt

No two branches in state open in one exploration carry the same prompt, and the loader enforces the rule at load time. A later open branch whose prompt repeats the prompt of an earlier open branch draws branch and the branch id and repeats the prompt of open branch and the earlier branch id. A prompt shared between an open branch and a branch in state taken or abandoned draws no defect, and two branches outside state open may share a prompt. Test, tests/test_explorations.py.

### MY-4 full form, recorded forking into tens of branches

Branch generation in this slice is recorded, not live. No live call is made anywhere in the slice, and no branch committed to this repo was produced by a model. The fork_skeleton function in the myriad.fork module takes a question and a list of branch prompts and builds the exploration skeleton a live forker would emit, one root branch per prompt, every branch in state open, with ids in the form b01, b02, and onward in prompt order. The skeleton loads clean under the MY-1 loader whenever its inputs are sound. The helper raises ValueError on an empty question, on a prompt that is not a string, on an empty prompt, and on two prompts that are the same, because no sound skeleton exists for those inputs. The slice forks one question into tens of branches, not a thousand, and the example exploration under examples carries tens of branches across all three states. The dibs claim the annex names stays a later obligation, and no branch in this slice carries one. Test, tests/test_fork.py and tests/test_examples.py.

### The command line surface

The myriad.cli module provides a main function and a console script named myriad with two commands. The map command takes one file holding an exploration and prints the map, or prints its defects and exits with status one when the exploration is malformed. The check command takes one file holding an exploration and prints the branch counts read from the tree, or prints its defects and exits with status one when the exploration is malformed. The counts print as four lines naming the total and then the count in each state. Every printed output is deterministic, so the same inputs always print the same bytes. Test, tests/test_cli.py.

## Line uniformity amendments

This slice carries two amendments that keep this repo in line with the paceghost and corked Round 1 repairs. Each amendment names the obligation it amends, and every earlier text stays standing. Where an amendment and an earlier text disagree, the amendment governs.

### FIX-MY-1, amending SETUP-3

The scan counts U+2014 and U+2013 in every file whose suffix is .md, .txt, .py, .json, .toml, .yml, or .yaml, and in every extensionless file, and it skips the files inside the ignored directories the scan names. A test runs the scan against a planted tree and pins a counted file and a skipped file. Test, tests/test_scan.py.

### FIX-MY-2, amending the SETUP-3 pin in tests/test_setup.py

The test that pins the CI workflow to the local gate reads only the live text of the workflow. A line that is a comment from its first character is dropped, and every other line is cut at its first hash character, so a gate command surviving only inside a tail comment does not satisfy the pin. Test, tests/test_setup.py.

## Round 1 repair amendments

This section records the repairs the Round 1 reviews required. Each amendment names the obligation it amends and the finding it closes, and every earlier text stays standing. Where an amendment and an earlier text disagree, the amendment governs.

### FIX-MY-3, amending MY-2

The map rendering walks the tree without recursion, so an exploration the loader accepts renders at any depth the loader accepts. The map command prints the map of a certified sound file and never dies on one. This amendment closes the red team Major finding that a loader valid chain raised RecursionError in the renderer once its depth passed the interpreter limit. Test, tests/test_map.py and tests/test_cli.py.

### FIX-MY-4, amending MY-1

The loader resolves each parent chain once for the whole exploration, sharing the outcome of every chain it has already walked. Load time grows with the size of the exploration and not with the square of its depth, and a deep cycle is named as fast as a shallow one. Every defect the loader named before this amendment keeps its wording, a chain leading into a cycle still draws the cycle defect, and a parent whose own fields are defective still counts as existing. This amendment closes the red team Minor finding that the parent chain walk was quadratic in tree depth. Test, tests/test_explorations.py.

### FIX-MY-5, amending MY-4

The fork helper refuses a question or a prompt holding an unpaired surrogate with the same ValueError it raises for its other unsound inputs. MY-1 names such a string as malformed, and the loads clean promise of MY-4 holds only when the helper refuses every input no sound skeleton exists for. This amendment closes the red team Minor finding and the spec fidelity condition on the fork boundary. Test, tests/test_fork.py.

## Round 2 obligations, full form

This section states the obligations the third slice is held to, in full form from the start. The stub section and the first slice sections stay standing above. Where this section and an earlier text disagree, this section governs.

### The exploration shape, Round 2 extension

A branch object may hold one more field beyond the fields the first slice shape names. The field claim holds a non empty string, the label of the fork that claimed the branch. The field is optional in every state, and a branch holding no claim field is the same branch the first slice shape describes. Fields the contract does not name are still ignored, and the exploration the loader returns carries the claim only on a branch whose input carried one. Test, tests/test_claims.py.

### MY-5 full form, the dibs claim on a branch

The claim a branch may carry is the dibs claim the repo annex names, and this obligation discharges the later obligation the annex and MY-4 full form recorded. The sentence in MY-4 full form that the dibs claim stays a later obligation speaks for the first slice and stays standing as its record. From this round the claim is part of the branch shape, and a branch in any slice may carry one.

The loader reads the claim field under the same reading it gives every other optional text field of a branch. A claim of the wrong kind draws branch and the index and field claim has the wrong type. An empty claim draws branch and the index and field claim is empty. A claim holding an unpaired surrogate draws branch and the index and field claim holds an unpaired surrogate. A claim that draws a defect makes the exploration malformed, as any field defect does, and the loader keeps naming fields after it. A claim is allowed in all three states and never interacts with the finding and reason rules of the state the branch stands in. Test, tests/test_claims.py.

Two branches may carry the same claim label, and one label may span branches in different states and in different parts of the tree. A claim names which fork owns the exploration work on a branch, and it says nothing about the worth of the branch. No rendering in this repo reads a claim as a judgement on the branch it stands on.

The map renders a claimed branch on its branch line, with the prompt followed by a comma and the words claimed by and the claim label. A branch carrying no claim renders exactly the branch line the first slice rendered, and the map of an exploration carrying no claims anywhere is byte identical to the map the first slice rendered. The counts line of the map is unchanged. Test, tests/test_claims.py and tests/test_map.py.

The render_claims function in the myriad.explorations module takes a loaded exploration and returns the claims listing as a list of lines, and the listing is a pure function of the exploration. The first line names the question, in the form question, and the question text. Then one line stands for each claim label the exploration carries, with the labels in alphabetical order of the label text, in the form claim and the label and branches and the count of branches carrying that label and open and the open count among them and taken and the taken count among them and abandoned and the abandoned count among them. Then, when at least one branch carries no claim, one last line stands in the form unclaimed and branches and the count of branches carrying no claim and open and the open count among them and taken and the taken count among them and abandoned and the abandoned count among them. The listing of an exploration with no branches is the question line alone. Test, tests/test_claims.py.

The fork helper builds skeletons whose branches carry no claim field, and a skeleton loads and lists as an exploration of unclaimed branches. Test, tests/test_claims.py.

### MY-6 full form, the territory census

The render_census function in the myriad.explorations module takes a loaded exploration and returns the territory census as a list of lines, and the census is a pure function of the exploration. The first line names the question, in the form question, and the question text.

The taken section stands next, and it stands only when the exploration holds at least one branch in state taken. The section opens with the line taken. Inside it, the taken branches stand grouped under the claim label each one carries, with the labels in alphabetical order of the label text, and each group opens with a line of two spaces and the word claim and the claim label. Each branch in a group stands on its own line of four spaces, in the form the branch id and finding and the finding text in full, and the branches inside a group stand in the order they appear in the branches list. Taken branches carrying no claim stand in one last group inside the section, and that group opens with a line of two spaces and the word unclaimed.

The abandoned section follows, in the same shape with the word reason in place of the word finding, and it stands only when the exploration holds at least one branch in state abandoned.

The open section stands last, and it stands only when the exploration holds at least one branch in state open. The section opens with the line open, and each open branch stands on its own line of two spaces, in the form the branch id and prompt and the prompt text, in the order the open branches appear in the branches list, with no grouping by claim.

The census closes with the counts line in the same form the map closes with. The census and the map read their counts from the same count_by_state function, so the two renderings of one exploration always agree on the counts they state. An exploration with no branches renders the question line and the counts line and no section. Test, tests/test_census.py.

### The command line surface, Round 2 extension

The myriad.cli module gains two commands beside map and check. The claims command takes one file holding an exploration and prints the claims listing, and the census command takes one file holding an exploration and prints the census. Each prints the defects of a malformed exploration and exits with status one, exactly as the map command does, and a sound file exits with status zero. Test, tests/test_cli_round2.py.

## Round 2 repair amendments

This section records the repairs the Round 2 reviews required. Each amendment names the obligation it amends and the finding it closes, and every earlier text stays standing. Where an amendment and an earlier text disagree, the amendment governs.

### FIX-MY-6, amending MY-1 and MY-5

Every rendering in this repo prints one field to a line, so a text value holding a line break forges the structure of the printed output and can print a line the tool never computed. A question holding a newline or a carriage return draws field question holds a newline or a carriage return. A prompt, a finding, a reason, or a claim holding a newline or a carriage return draws branch and the index and field and the field name and holds a newline or a carriage return. The rule is one rule across the loader, stated once here for every field it covers, and a value that draws the defect makes the exploration malformed, as any field defect does, and the loader keeps naming fields after it. This amendment closes the Round 2 red team finding that a claim or a finding holding a newline printed a forged counts line inside the renderings. Test, tests/test_round2_repairs.py.

### FIX-MY-7, amending MY-5

The map renders a claimed branch with its claim as a bracketed tag standing immediately before the prompt text, in the form an opening square bracket and the claim label and a closing square bracket and the prompt, so the branch line reads the indent and the branch id and the state word in parentheses and the bracketed tag and the prompt. The form MY-5 first stated, the prompt followed by a comma and the words claimed by and the claim label, is retired. Under that form an unclaimed branch whose prompt ended in the words claimed by and a label rendered in the same shape as a claimed branch, and the map alone could not tell the two apart. A branch carrying no claim renders exactly the branch line the first slice rendered, as MY-5 already states. The claims listing and the census keep the forms MY-5 and MY-6 state, because the listing names each label on a line of its own and the census groups branches under a heading line, so in those renderings a claim never stands inside a prompt and is never read as part of one. This amendment closes the Round 2 red team finding on the ambiguous claimed branch line. Test, tests/test_round2_repairs.py and tests/test_claims.py.

The alphabetical order MY-5 and MY-6 state is the order of the label text itself, character by character, so a label opening with a capital letter stands before a label carrying the same letters in lowercase, and the listing and both census sections read that order the same way.

### FIX-MY-8, amending MY-4

The fork helper refuses every input no sound skeleton exists for, and FIX-MY-5 stated that principle for a question or a prompt holding an unpaired surrogate. A question or a prompt holding a newline or a carriage return is an input of the same kind, because FIX-MY-6 names such a value malformed and no skeleton built on it loads clean under the loader. The fork helper refuses a question or a prompt holding a newline or a carriage return with the same ValueError it raises for its other unsound inputs. This amendment closes the Round 2 re-review finding that the helper built skeletons the loader refuses. Test, tests/test_round2_repairs.py.

### FIX-MY-9, amending FIX-MY-6

FIX-MY-6 named the fields its line break rule covers, and the branch id and the branch parent were not in its list. Both fields reach print all the same. The map opens every branch line with the branch id and the census prints branch ids in the same place, and a defect the loader draws names a parent inside its own text when the parent does not exist. The rule of FIX-MY-6 extends to the branch id and the branch parent. An id holding a newline or a carriage return draws branch and the index and field id holds a newline or a carriage return, and a parent holding a newline or a carriage return draws branch and the index and field parent holds a newline or a carriage return, each in the phrase shape FIX-MY-6 states. This amendment closes the Round 2 re-review finding that an id holding a line break forged lines in the printed renderings. Test, tests/test_round2_repairs.py.

### FIX-MY-10, amending FIX-MY-7

The bracketed tag FIX-MY-7 stated stands inside the branch line, and the text the loader admits can itself hold brackets. A prompt opening with a bracketed word renders in the shape of the tag itself, and a claim label holding brackets blurs the edge of the tag, so the map alone can again fail to tell a claimed branch from an unclaimed one. The bracketed tag is retired. The map renders a claimed branch with the branch line in the untagged form MY-2 states, the branch id and the state word in parentheses and the prompt, and the claim stands on one more line of the branch, immediately under the branch line and before any finding or reason line the branch carries, at the indent the finding and reason lines use, in the form claimed by and the claim label, the words MY-5 first used. A branch carrying no claim renders exactly the branch line the first slice rendered and no claim line. With every field the renderings print refusing line breaks under FIX-MY-6 as FIX-MY-9 extends it, no field text can forge the claim line or split it, so the claim line is a structural line of the rendering that only a carried claim produces. The claims listing stays the authoritative count of claims, because it groups branches by the loaded claim field and never by the shape of a line. The claims listing and the census keep the forms MY-5 and MY-6 state. This amendment closes the Round 2 re-review finding on the bracketed tag. Test, tests/test_round2_repairs.py and tests/test_claims.py.
