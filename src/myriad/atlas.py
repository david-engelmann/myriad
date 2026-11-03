"""Atlases, their loader, and their rendering for myriad.

An atlas holds many explorations in one file, each entry carrying
a label and the whole exploration it names, and the rendering
walks the entries in atlas order. The loader never raises on
malformed content, each entry is validated by the same validation
the exploration loader runs, and the rendering is built from the
map and the claims listing over the one counting path, so an
atlas can never disagree with the renderings of its own entries.
The obligations this module carries are MY-7 and MY-8 in
CONTRACT.md, in the Round 3 section, and MY-9 in the Round 4
section.
"""

import json
import pathlib

from myriad.explorations import (
    _count_branches,
    _format_counts,
    _holds_line_break,
    _holds_unpaired_surrogate,
    _load_exploration_data,
    render_claims,
    render_map,
)


def _check_label(entry, index, defects):
    """Validate the label of one entry and return its clean value.

    A label defect names the entry by its index and never prints
    the label value, because a label that drew a defect is a value
    no rendering may carry. Returns None when the label is absent
    or drew a defect.
    """
    if "label" not in entry:
        defects.append(f"entry {index} is missing field label")
        return None
    value = entry["label"]
    if not isinstance(value, str):
        defects.append(f"entry {index} field label has the wrong type")
        return None
    if value == "":
        defects.append(f"entry {index} field label is empty")
        return None
    if _holds_unpaired_surrogate(value):
        defects.append(
            f"entry {index} field label holds an unpaired surrogate"
        )
        return None
    if _holds_line_break(value):
        defects.append(
            f"entry {index} field label holds a newline "
            "or a carriage return"
        )
        return None
    return value


def _load_entry(entry, index, defects):
    """Validate one atlas entry and return what later checks need.

    The returned record carries the clean entry dict, or None when
    the entry drew any defect, and the clean label, or None when
    the label itself drew a defect. The defects of the entry name
    it under the prefix of its clean label, or of its index when
    the label drew a defect.
    """
    before = len(defects)
    if not isinstance(entry, dict):
        defects.append(f"entry {index} is not an object")
        return {"clean": None, "label": None}
    label = _check_label(entry, index, defects)
    if label is not None:
        prefix = f"entry {label}"
    else:
        prefix = f"entry {index}"
    exploration = None
    if "exploration" not in entry:
        defects.append(f"{prefix} is missing field exploration")
    elif not isinstance(entry["exploration"], dict):
        defects.append(
            f"{prefix} field exploration is not a JSON object"
        )
    else:
        exploration, inner = _load_exploration_data(entry["exploration"])
        for defect in inner:
            defects.append(f"{prefix}, {defect}")
    clean = None
    if len(defects) == before:
        clean = {"label": label, "exploration": exploration}
    return {"clean": clean, "label": label}


def _repeated_label_defects(records):
    """Return the repeated label defects, in later entry order.

    The check runs over every entry carrying a label that drew no
    defect, whatever else is malformed in the entry. Each later
    entry carrying a label already carried draws one defect naming
    the label and the index of its first entry and of the later
    one.
    """
    defects = []
    first_at = {}
    for index, record in enumerate(records):
        label = record["label"]
        if label is None:
            continue
        if label in first_at:
            defects.append(
                f"duplicate entry label {label}, entries "
                f"{first_at[label]} and {index}"
            )
        else:
            first_at[label] = index
    return defects


def load_atlas_text(text):
    """Parse atlas text into an atlas, naming every defect.

    Returns a pair of the atlas and the defect list. A malformed
    input returns no atlas, and this function never raises on bad
    content. An atlas carrying any defect in any entry returns no
    atlas at all, so the atlas loads only whole.
    """
    try:
        data = json.loads(text)
    except (ValueError, RecursionError):
        return None, ["input is not valid JSON"]
    if not isinstance(data, dict):
        return None, ["input is not a JSON object"]
    defects = []
    title = None
    if "title" not in data:
        defects.append("missing field title")
    elif not isinstance(data["title"], str):
        defects.append("field title has the wrong type")
    elif data["title"] == "":
        defects.append("field title is empty")
    elif _holds_unpaired_surrogate(data["title"]):
        defects.append("field title holds an unpaired surrogate")
    elif _holds_line_break(data["title"]):
        defects.append(
            "field title holds a newline or a carriage return"
        )
    else:
        title = data["title"]
    records = []
    if "entries" not in data:
        defects.append("missing field entries")
    elif not isinstance(data["entries"], list):
        defects.append("field entries has the wrong type")
    else:
        for index, entry in enumerate(data["entries"]):
            records.append(_load_entry(entry, index, defects))
        defects.extend(_repeated_label_defects(records))
    if defects:
        return None, defects
    atlas = {
        "title": title,
        "entries": [record["clean"] for record in records],
    }
    return atlas, []


def load_atlas_file(path):
    """Read an atlas from disk, with the same contract as text."""
    try:
        text = pathlib.Path(path).read_text(encoding="utf-8")
    except (OSError, UnicodeError):
        return None, ["file cannot be read"]
    return load_atlas_text(text)


def render_atlas(atlas):
    """Return the atlas rendering lines for one atlas.

    One section stands for each entry, in the order the entries
    stand in the atlas. A section opens with the entry line, then
    carries the map of the entry exploration exactly as the map
    renders it, then the claim lines of the entry exploration
    exactly as the claims listing states them after its question
    line. The closing counts are read over the union of the
    branches of every entry, by the same counting path the map,
    the claims listing, and the census read, so the closing
    counts always equal the sum of the section counts. The
    rendering is a pure function of the atlas.
    """
    lines = [f"atlas, {atlas['title']}"]
    union = []
    for entry in atlas["entries"]:
        exploration = entry["exploration"]
        lines.append(f"entry {entry['label']}")
        lines.extend(render_map(exploration))
        lines.extend(render_claims(exploration)[1:])
        union.extend(exploration["branches"])
    counts = _format_counts(_count_branches(union))
    lines.append(f"atlas counts, entries {len(atlas['entries'])}, {counts}")
    return lines


def render_index(atlas):
    """Return the index reading lines for one atlas.

    The first line is the title line of the atlas rendering and
    the last line is its atlas counts line, and one line stands
    for each entry between them, carrying the entry label, the
    question of its exploration, and the counts of its
    exploration counted over its branches alone. Every count is
    read from the one counting path the map, the claims listing,
    the census, and the atlas rendering read, so the counts of
    the closing line always equal the sum of the counts of the
    entry lines. The reading is a pure function of the atlas.
    """
    lines = [f"atlas, {atlas['title']}"]
    union = []
    for entry in atlas["entries"]:
        exploration = entry["exploration"]
        counts = _format_counts(_count_branches(exploration["branches"]))
        lines.append(
            f"entry {entry['label']}, question, "
            f"{exploration['question']}, {counts}"
        )
        union.extend(exploration["branches"])
    counts = _format_counts(_count_branches(union))
    lines.append(f"atlas counts, entries {len(atlas['entries'])}, {counts}")
    return lines
