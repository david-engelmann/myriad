"""Explorations, their loader, and their map for myriad.

Every function in this module is a pure function of its inputs, and
the loader never raises on malformed content. The obligations this
module carries are MY-1 to MY-3 in CONTRACT.md, in the full form
section, and MY-5 and MY-6 in the Round 2 section.
"""

import json
import pathlib

STATES = ("open", "taken", "abandoned")


def _holds_unpaired_surrogate(text):
    """Return whether a string fails UTF-8 encoding.

    A string holding an unpaired surrogate cannot be encoded, and an
    exploration carrying one cannot be printed, so the loader names
    it as a defect instead of letting it reach the printer.
    """
    try:
        text.encode("utf-8")
    except UnicodeEncodeError:
        return True
    return False


def _holds_line_break(text):
    """Return whether a string holds a newline or a carriage return.

    Every rendering prints one field to a line, so a text value
    holding a line break forges the structure of the printed
    output, and the loader names it as a defect under FIX-MY-6
    instead of letting it reach the printer.
    """
    return "\n" in text or "\r" in text


def _check_branch_text(entry, index, name, defects, required,
                       rendered=False):
    """Validate one text field of a branch and return its value.

    Returns None when the field is absent and not required, and also
    returns None when the field drew a defect, so the caller can tell
    a clean value from a defective one by the defect count. A field
    whose value reaches a rendering is also checked for a line
    break, under FIX-MY-6.
    """
    if name not in entry:
        if required:
            defects.append(f"branch {index} is missing field {name}")
        return None
    value = entry[name]
    if not isinstance(value, str):
        defects.append(f"branch {index} field {name} has the wrong type")
        return None
    if value == "":
        defects.append(f"branch {index} field {name} is empty")
        return None
    if _holds_unpaired_surrogate(value):
        defects.append(
            f"branch {index} field {name} holds an unpaired surrogate"
        )
        return None
    if rendered and _holds_line_break(value):
        defects.append(
            f"branch {index} field {name} holds a newline "
            "or a carriage return"
        )
        return None
    return value


def _load_branch(entry, index, defects):
    """Validate one branch entry and return what later checks need.

    The returned record carries the clean branch dict, or None when
    the branch drew any defect, and the values the structural checks
    follow, each of them None when the field itself was defective.
    """
    before = len(defects)
    if not isinstance(entry, dict):
        defects.append(f"branch {index} is not an object")
        return {"clean": None, "id": None, "parent": None,
                "parent_seen": False, "prompt": None, "state": None}
    branch_id = _check_branch_text(entry, index, "id", defects, True,
                                   rendered=True)
    parent = None
    parent_seen = False
    if "parent" not in entry:
        defects.append(f"branch {index} is missing field parent")
    elif entry["parent"] is None:
        parent_seen = True
    elif isinstance(entry["parent"], str):
        parent = _check_branch_text(entry, index, "parent", defects, True,
                                    rendered=True)
        parent_seen = parent is not None
    else:
        defects.append(f"branch {index} field parent has the wrong type")
    prompt = _check_branch_text(entry, index, "prompt", defects, True,
                                rendered=True)
    state = None
    if "state" not in entry:
        defects.append(f"branch {index} is missing field state")
    elif not isinstance(entry["state"], str):
        defects.append(f"branch {index} field state has the wrong type")
    elif entry["state"] not in STATES:
        defects.append(f"branch {index} field state is not a known state")
    else:
        state = entry["state"]
    claim = _check_branch_text(entry, index, "claim", defects, False,
                               rendered=True)
    finding = _check_branch_text(entry, index, "finding", defects, False,
                                 rendered=True)
    reason = _check_branch_text(entry, index, "reason", defects, False,
                                rendered=True)
    if state == "taken":
        if "finding" not in entry:
            defects.append(f"branch {index} is missing field finding")
        if "reason" in entry:
            defects.append(
                f"branch {index} field reason is not allowed in state taken"
            )
    elif state == "abandoned":
        if "reason" not in entry:
            defects.append(f"branch {index} is missing field reason")
        if "finding" in entry:
            defects.append(
                f"branch {index} field finding is not allowed "
                "in state abandoned"
            )
    elif state == "open":
        if "finding" in entry:
            defects.append(
                f"branch {index} field finding is not allowed in state open"
            )
        if "reason" in entry:
            defects.append(
                f"branch {index} field reason is not allowed in state open"
            )
    clean = None
    if len(defects) == before:
        clean = {
            "id": branch_id,
            "parent": parent,
            "prompt": prompt,
            "state": state,
        }
        if "claim" in entry:
            clean["claim"] = claim
        if state == "taken":
            clean["finding"] = finding
        elif state == "abandoned":
            clean["reason"] = reason
    return {"clean": clean, "id": branch_id,
            "parent": parent,
            "parent_seen": parent_seen,
            "prompt": prompt, "state": state}


def _structural_defects(records):
    """Return the structural defects of the tree, in branch order.

    The checks run over the values the field pass found clean. A
    parent is satisfied when any branch carries the id, and a cycle
    is a parent chain that loops without reaching a root. The
    parent function is resolved once for the whole exploration,
    each id learning whether its chain reaches a root or loops,
    and the rooted chains are timed once so a later branch with a
    repeated id can tell whether its chain passes through its own
    id. The pass stays linear in the size of the tree however deep
    it grows.
    """
    defects = []
    seen_ids = set()
    for record in records:
        branch_id = record["id"]
        if branch_id is None:
            continue
        if branch_id in seen_ids:
            defects.append(f"duplicate branch id {branch_id}")
        else:
            seen_ids.add(branch_id)
    parent_by_id = {}
    for record in records:
        branch_id = record["id"]
        if branch_id is None or branch_id in parent_by_id:
            continue
        parent_by_id[branch_id] = record["parent"]
    for record in records:
        branch_id = record["id"]
        if branch_id is None or not record["parent_seen"]:
            continue
        parent = record["parent"]
        if parent is not None and parent not in seen_ids:
            defects.append(
                f"branch {branch_id} parent {parent} does not exist"
            )
    outcome_by_id = {}
    for start_id in parent_by_id:
        if start_id in outcome_by_id:
            continue
        path = []
        on_path = set()
        current = start_id
        while True:
            if current is None or current not in parent_by_id:
                result = "rooted"
                break
            if current in outcome_by_id:
                result = outcome_by_id[current]
                break
            if current in on_path:
                result = "cycle"
                break
            path.append(current)
            on_path.add(current)
            current = parent_by_id[current]
        for node in path:
            outcome_by_id[node] = result
    children_by_id = {}
    forest_roots = []
    for node_id, parent in parent_by_id.items():
        if outcome_by_id[node_id] != "rooted":
            continue
        if parent in parent_by_id and outcome_by_id[parent] == "rooted":
            children_by_id.setdefault(parent, []).append(node_id)
        else:
            forest_roots.append(node_id)
    entry_at = {}
    exit_at = {}
    tick = 0
    for root in forest_roots:
        stack = [(root, False)]
        while stack:
            node, leaving = stack.pop()
            tick += 1
            if leaving:
                exit_at[node] = tick
                continue
            entry_at[node] = tick
            stack.append((node, True))
            for child in children_by_id.get(node, []):
                stack.append((child, False))
    seen_for_cycle = set()
    for record in records:
        branch_id = record["id"]
        if branch_id is None:
            continue
        first_occurrence = branch_id not in seen_for_cycle
        seen_for_cycle.add(branch_id)
        parent = record["parent"]
        if parent is None:
            continue
        if outcome_by_id.get(parent, "rooted") == "cycle":
            defects.append(
                f"branch {branch_id} has a cycle in its parent chain"
            )
        elif (
            not first_occurrence
            and branch_id in entry_at
            and parent in entry_at
            and entry_at[branch_id] <= entry_at[parent]
            and exit_at[branch_id] >= exit_at[parent]
        ):
            defects.append(
                f"branch {branch_id} has a cycle in its parent chain"
            )
    open_prompts = {}
    for record in records:
        if record["state"] != "open":
            continue
        branch_id = record["id"]
        prompt = record["prompt"]
        if branch_id is None or prompt is None:
            continue
        if prompt in open_prompts:
            defects.append(
                f"branch {branch_id} repeats the prompt of "
                f"open branch {open_prompts[prompt]}"
            )
        else:
            open_prompts[prompt] = branch_id
    return defects


def load_exploration_text(text):
    """Parse exploration text into an exploration, naming every defect.

    Returns a pair of the exploration and the defect list. A
    malformed input returns no exploration, and this function never
    raises on bad content.
    """
    try:
        data = json.loads(text)
    except (ValueError, RecursionError):
        return None, ["input is not valid JSON"]
    return _load_exploration_data(data)


def _load_exploration_data(data):
    """Validate parsed JSON data as an exploration.

    This is the validation the text and file loaders run once the
    input is parsed, and the atlas loader runs it over the
    exploration each entry embeds, so an embedded exploration is
    held to exactly the rules an exploration on its own is held
    to, under MY-7.
    """
    if not isinstance(data, dict):
        return None, ["input is not a JSON object"]
    defects = []
    question = None
    if "question" not in data:
        defects.append("missing field question")
    elif not isinstance(data["question"], str):
        defects.append("field question has the wrong type")
    elif data["question"] == "":
        defects.append("field question is empty")
    elif _holds_unpaired_surrogate(data["question"]):
        defects.append("field question holds an unpaired surrogate")
    elif _holds_line_break(data["question"]):
        defects.append(
            "field question holds a newline or a carriage return"
        )
    else:
        question = data["question"]
    records = []
    if "branches" not in data:
        defects.append("missing field branches")
    elif not isinstance(data["branches"], list):
        defects.append("field branches has the wrong type")
    else:
        for index, entry in enumerate(data["branches"]):
            records.append(_load_branch(entry, index, defects))
        defects.extend(_structural_defects(records))
    if defects:
        return None, defects
    exploration = {
        "question": question,
        "branches": [record["clean"] for record in records],
    }
    return exploration, []


def load_exploration_file(path):
    """Read an exploration from disk, with the same contract as text."""
    try:
        text = pathlib.Path(path).read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return None, ["file cannot be read"]
    return load_exploration_text(text)


def _count_branches(branches):
    """Return the total and the count in each state, for a branch list."""
    counts = {"total": 0, "open": 0, "taken": 0, "abandoned": 0}
    for branch in branches:
        counts["total"] += 1
        counts[branch["state"]] += 1
    return counts


def count_by_state(exploration):
    """Return the total and the count in each state, from the tree."""
    return _count_branches(exploration["branches"])


def _format_counts(counts):
    """Return the counts line for a counts mapping, in its one form.

    The map, the census, and the claims listing all close or count
    with this line, so the form stands here and nowhere else.
    """
    return (
        f"branches {counts['total']}, open {counts['open']}, "
        f"taken {counts['taken']}, abandoned {counts['abandoned']}"
    )


def render_map(exploration):
    """Return the map lines for one exploration.

    The walk is depth first and iterative, carrying its depth on an
    explicit stack, so a loaded exploration renders at any depth.
    Roots stand in the order they appear in the branches list, and
    the children of each branch stand in the order they appear in
    the branches list. The report is a pure function of the
    exploration.
    """
    lines = [f"question, {exploration['question']}"]
    children = {}
    roots = []
    for branch in exploration["branches"]:
        if branch["parent"] is None:
            roots.append(branch)
        else:
            children.setdefault(branch["parent"], []).append(branch)

    stack = [(root, 0) for root in reversed(roots)]
    while stack:
        branch, depth = stack.pop()
        indent = "  " * depth
        lines.append(
            f"{indent}{branch['id']} ({branch['state']}) {branch['prompt']}"
        )
        if "claim" in branch:
            lines.append(f"{indent}  claimed by {branch['claim']}")
        if branch["state"] == "taken":
            lines.append(f"{indent}  finding, {branch['finding']}")
        elif branch["state"] == "abandoned":
            lines.append(f"{indent}  reason, {branch['reason']}")
        for child in reversed(children.get(branch["id"], [])):
            stack.append((child, depth + 1))
    lines.append(_format_counts(count_by_state(exploration)))
    return lines


def _group_by_claim(branches):
    """Group branches by claim label, in branches order within a group.

    Returns a pair of a mapping from claim label to the branches
    carrying it, each list in the order the branches appear in the
    input, and the list of branches carrying no claim.
    """
    claimed = {}
    unclaimed = []
    for branch in branches:
        if "claim" in branch:
            claimed.setdefault(branch["claim"], []).append(branch)
        else:
            unclaimed.append(branch)
    return claimed, unclaimed


def render_claims(exploration):
    """Return the claims listing lines for one exploration.

    One line stands for each claim label, with the labels in
    alphabetical order of the label text, and a last line counts the
    branches carrying no claim when any do. The listing is a pure
    function of the exploration.
    """
    lines = [f"question, {exploration['question']}"]
    claimed, unclaimed = _group_by_claim(exploration["branches"])
    for label in sorted(claimed):
        counts = _format_counts(_count_branches(claimed[label]))
        lines.append(f"claim {label}, {counts}")
    if unclaimed:
        counts = _format_counts(_count_branches(unclaimed))
        lines.append(f"unclaimed, {counts}")
    return lines


def _census_state_section(lines, branches, state, field):
    """Append one grouped section of the census to the line list.

    The branches in the given state stand grouped under their claim
    labels in alphabetical order, with the branches carrying no
    claim in a last group. Each branch renders the text of the named
    field, the finding of a taken branch or the reason of an
    abandoned one, in full.
    """
    selected = [branch for branch in branches if branch["state"] == state]
    if not selected:
        return
    lines.append(state)
    claimed, unclaimed = _group_by_claim(selected)
    for label in sorted(claimed):
        lines.append(f"  claim {label}")
        for branch in claimed[label]:
            lines.append(f"    {branch['id']}, {field}, {branch[field]}")
    if unclaimed:
        lines.append("  unclaimed")
        for branch in unclaimed:
            lines.append(f"    {branch['id']}, {field}, {branch[field]}")


def render_census(exploration):
    """Return the territory census lines for one exploration.

    The census reads the map back by state. Taken branches stand
    with their findings and abandoned branches with their reasons,
    each grouped under the claim label the branch carries, and the
    open branches stand last with their prompts. The closing counts
    line reads its numbers from the same count the map reads.
    """
    branches = exploration["branches"]
    lines = [f"question, {exploration['question']}"]
    _census_state_section(lines, branches, "taken", "finding")
    _census_state_section(lines, branches, "abandoned", "reason")
    open_branches = [
        branch for branch in branches if branch["state"] == "open"
    ]
    if open_branches:
        lines.append("open")
        for branch in open_branches:
            lines.append(f"  {branch['id']}, prompt, {branch['prompt']}")
    lines.append(_format_counts(count_by_state(exploration)))
    return lines
