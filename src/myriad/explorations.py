"""Explorations, their loader, and their map for myriad.

Every function in this module is a pure function of its inputs, and
the loader never raises on malformed content. The obligations this
module carries are MY-1 to MY-3 in CONTRACT.md, in the full form
section.
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


def _check_branch_text(entry, index, name, defects, required):
    """Validate one text field of a branch and return its value.

    Returns None when the field is absent and not required, and also
    returns None when the field drew a defect, so the caller can tell
    a clean value from a defective one by the defect count.
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
    branch_id = _check_branch_text(entry, index, "id", defects, True)
    parent = None
    parent_seen = False
    if "parent" not in entry:
        defects.append(f"branch {index} is missing field parent")
    elif entry["parent"] is None:
        parent_seen = True
    elif isinstance(entry["parent"], str):
        parent = _check_branch_text(entry, index, "parent", defects, True)
        parent_seen = parent is not None
    else:
        defects.append(f"branch {index} field parent has the wrong type")
    prompt = _check_branch_text(entry, index, "prompt", defects, True)
    state = None
    if "state" not in entry:
        defects.append(f"branch {index} is missing field state")
    elif not isinstance(entry["state"], str):
        defects.append(f"branch {index} field state has the wrong type")
    elif entry["state"] not in STATES:
        defects.append(f"branch {index} field state is not a known state")
    else:
        state = entry["state"]
    finding = _check_branch_text(entry, index, "finding", defects, False)
    reason = _check_branch_text(entry, index, "reason", defects, False)
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


def count_by_state(exploration):
    """Return the total and the count in each state, from the tree."""
    counts = {"total": 0, "open": 0, "taken": 0, "abandoned": 0}
    for branch in exploration["branches"]:
        counts["total"] += 1
        counts[branch["state"]] += 1
    return counts


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
        if branch["state"] == "taken":
            lines.append(f"{indent}  finding, {branch['finding']}")
        elif branch["state"] == "abandoned":
            lines.append(f"{indent}  reason, {branch['reason']}")
        for child in reversed(children.get(branch["id"], [])):
            stack.append((child, depth + 1))
    counts = count_by_state(exploration)
    lines.append(
        f"branches {counts['total']}, open {counts['open']}, "
        f"taken {counts['taken']}, abandoned {counts['abandoned']}"
    )
    return lines
