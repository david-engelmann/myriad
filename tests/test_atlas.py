"""Pins for the atlas loader and the atlas rendering, MY-7 and MY-8.

Every defect wording is pinned exactly, including the prefix an
entry's defects stand under, because a reader finds the malformed
entry by the words the loader prints. The rendering pins hold the
atlas to the map and the claims listing of its own entries, byte
for byte, and to the one counting path, because an atlas that
rendered or counted on its own would disagree with the tool it
stands beside.
"""

import copy
import json
import re

from conftest import FORBIDDEN_WORDS
from myriad.atlas import load_atlas_file, load_atlas_text, render_atlas
from myriad.explorations import (
    count_by_state,
    render_claims,
    render_map,
)
from myriad.fork import fork_skeleton


def exploration(**overrides):
    base = {
        "question": "q",
        "branches": [
            {"id": "a", "parent": None, "prompt": "p", "state": "open"},
        ],
    }
    base.update(overrides)
    return base


def atlas_text(**overrides):
    base = {
        "title": "t",
        "entries": [{"label": "one", "exploration": exploration()}],
    }
    base.update(overrides)
    return json.dumps(base)


def sound_atlas():
    found, defects = load_atlas_text(atlas_text())
    assert defects == []
    return found


def defects_of(data):
    found, defects = load_atlas_text(json.dumps(data))
    assert found is None
    return defects


def test_a_clean_atlas_loads_with_only_the_named_fields():
    text = json.dumps({
        "title": "t",
        "extra": "ignored",
        "entries": [{
            "label": "one",
            "extra": "ignored",
            "exploration": {
                "question": "q",
                "extra": "ignored",
                "branches": [{
                    "id": "a", "parent": None, "prompt": "p",
                    "state": "open", "extra": "ignored",
                }],
            },
        }],
    })
    found, defects = load_atlas_text(text)
    assert defects == []
    assert found == {
        "title": "t",
        "entries": [{
            "label": "one",
            "exploration": {
                "question": "q",
                "branches": [{
                    "id": "a", "parent": None, "prompt": "p",
                    "state": "open",
                }],
            },
        }],
    }


def test_an_unreadable_file_draws_the_cannot_be_read_defect(tmp_path):
    found, defects = load_atlas_file(tmp_path / "missing.json")
    assert found is None
    assert defects == ["file cannot be read"]


def test_text_that_is_not_json_draws_the_not_valid_json_defect():
    found, defects = load_atlas_text("this is not json")
    assert found is None
    assert defects == ["input is not valid JSON"]


def test_input_nested_too_deeply_draws_the_not_valid_json_defect():
    found, defects = load_atlas_text("[" * 10000 + "]" * 10000)
    assert found is None
    assert defects == ["input is not valid JSON"]


def test_json_that_is_not_an_object_draws_the_not_an_object_defect():
    found, defects = load_atlas_text("[1, 2]")
    assert found is None
    assert defects == ["input is not a JSON object"]


def test_a_missing_title_is_named():
    assert defects_of({"entries": []}) == ["missing field title"]


def test_a_title_of_the_wrong_type_is_named():
    assert defects_of({"title": 4, "entries": []}) == [
        "field title has the wrong type"
    ]


def test_an_empty_title_is_named():
    assert defects_of({"title": "", "entries": []}) == [
        "field title is empty"
    ]


def test_a_title_holding_an_unpaired_surrogate_is_named():
    text = '{"title": "\\ud800x", "entries": []}'
    found, defects = load_atlas_text(text)
    assert found is None
    assert defects == ["field title holds an unpaired surrogate"]


def test_a_title_holding_a_line_break_is_named():
    assert defects_of({"title": "two\nlines", "entries": []}) == [
        "field title holds a newline or a carriage return"
    ]
    assert defects_of({"title": "two\rlines", "entries": []}) == [
        "field title holds a newline or a carriage return"
    ]


def test_a_missing_entries_field_is_named():
    assert defects_of({"title": "t"}) == ["missing field entries"]


def test_an_entries_field_of_the_wrong_type_is_named():
    assert defects_of({"title": "t", "entries": "x"}) == [
        "field entries has the wrong type"
    ]


def test_an_empty_entries_list_loads_clean():
    found, defects = load_atlas_text(atlas_text(entries=[]))
    assert defects == []
    assert found == {"title": "t", "entries": []}


def test_an_entry_that_is_not_an_object_is_named_by_index():
    data = {"title": "t", "entries": ["nope"]}
    assert defects_of(data) == ["entry 0 is not an object"]


def test_a_missing_label_is_named_by_index():
    data = {"title": "t", "entries": [{"exploration": exploration()}]}
    assert defects_of(data) == ["entry 0 is missing field label"]


def test_a_label_of_the_wrong_type_is_named_by_index():
    data = {"title": "t", "entries": [
        {"label": 4, "exploration": exploration()},
    ]}
    assert defects_of(data) == ["entry 0 field label has the wrong type"]


def test_an_empty_label_is_named_by_index():
    data = {"title": "t", "entries": [
        {"label": "", "exploration": exploration()},
    ]}
    assert defects_of(data) == ["entry 0 field label is empty"]


def test_a_label_holding_an_unpaired_surrogate_is_named_by_index():
    data = {"title": "t", "entries": [
        {"label": "\ud800x", "exploration": exploration()},
    ]}
    assert defects_of(data) == [
        "entry 0 field label holds an unpaired surrogate"
    ]


def test_a_label_holding_a_line_break_is_named_by_index_only():
    label = "two\nlines"
    data = {"title": "t", "entries": [
        {"label": label, "exploration": exploration()},
    ]}
    defects = defects_of(data)
    assert defects == [
        "entry 0 field label holds a newline or a carriage return"
    ]
    assert all(label not in defect for defect in defects)
    assert all("\n" not in defect for defect in defects)


def test_a_missing_exploration_is_named_under_the_clean_label():
    data = {"title": "t", "entries": [{"label": "ferry"}]}
    assert defects_of(data) == ["entry ferry is missing field exploration"]


def test_a_missing_exploration_is_named_by_index_under_a_bad_label():
    data = {"title": "t", "entries": [{"label": ""}]}
    assert defects_of(data) == [
        "entry 0 field label is empty",
        "entry 0 is missing field exploration",
    ]


def test_an_exploration_that_is_not_an_object_is_named():
    data = {"title": "t", "entries": [
        {"label": "ferry", "exploration": "nope"},
    ]}
    assert defects_of(data) == [
        "entry ferry field exploration is not a JSON object"
    ]


def test_entry_defects_carry_the_label_and_the_my_1_wording():
    data = {"title": "t", "entries": [
        {"label": "ferry", "exploration": {"branches": []}},
        {"label": "dock", "exploration": exploration(branches=[
            {"id": "a", "parent": None, "prompt": "", "state": "open"},
            {"id": "a", "parent": "z", "prompt": "p", "state": "open"},
        ])},
    ]}
    assert defects_of(data) == [
        "entry ferry, missing field question",
        "entry dock, branch 0 field prompt is empty",
        "entry dock, duplicate branch id a",
        "entry dock, branch a parent z does not exist",
    ]


def test_entry_defects_fall_back_to_the_index_under_a_bad_label():
    data = {"title": "t", "entries": [
        {"label": "bad\nlabel",
         "exploration": exploration(question="")},
    ]}
    assert defects_of(data) == [
        "entry 0 field label holds a newline or a carriage return",
        "entry 0, field question is empty",
    ]


def test_a_repeated_label_names_the_label_and_both_positions():
    entry = {"label": "ferry", "exploration": exploration()}
    data = {"title": "t", "entries": [
        entry,
        {"label": "dock", "exploration": exploration()},
        dict(entry),
    ]}
    assert defects_of(data) == [
        "duplicate entry label ferry, entries 0 and 2"
    ]


def test_a_label_carried_three_times_draws_two_defects():
    entry = {"label": "ferry", "exploration": exploration()}
    data = {"title": "t", "entries": [entry, dict(entry), dict(entry)]}
    assert defects_of(data) == [
        "duplicate entry label ferry, entries 0 and 1",
        "duplicate entry label ferry, entries 0 and 2",
    ]


def test_the_repeated_label_check_runs_over_a_malformed_entry():
    data = {"title": "t", "entries": [
        {"label": "ferry", "exploration": exploration()},
        {"label": "ferry", "exploration": exploration(question="")},
    ]}
    assert defects_of(data) == [
        "entry ferry, field question is empty",
        "duplicate entry label ferry, entries 0 and 1",
    ]


def test_labels_that_drew_a_defect_draw_no_duplicate_defect():
    entry = {"label": "bad\nlabel", "exploration": exploration()}
    data = {"title": "t", "entries": [entry, dict(entry)]}
    assert defects_of(data) == [
        "entry 0 field label holds a newline or a carriage return",
        "entry 1 field label holds a newline or a carriage return",
    ]


def test_one_bad_entry_fails_the_whole_atlas_and_naming_continues():
    data = {"title": "t", "entries": [
        {"label": "sound", "exploration": exploration()},
        {"label": "broken", "exploration": exploration(question="")},
        {"label": "alsbroken", "exploration": exploration(branches="x")},
    ]}
    found, defects = load_atlas_text(json.dumps(data))
    assert found is None
    assert defects == [
        "entry broken, field question is empty",
        "entry alsbroken, field branches has the wrong type",
    ]


def test_defects_stand_title_first_entries_next_duplicates_last():
    data = {"title": "", "entries": [
        {"label": "ferry", "exploration": exploration(question="")},
        {"label": "ferry", "exploration": exploration()},
    ]}
    assert defects_of(data) == [
        "field title is empty",
        "entry ferry, field question is empty",
        "duplicate entry label ferry, entries 0 and 1",
    ]


def test_the_loader_never_raises_on_hostile_input():
    hostile = [
        "", "null", "4", '"text"', "[]", "{}",
        json.dumps({"title": None, "entries": None}),
        json.dumps({"title": "t", "entries": [None, 4, [], {}]}),
        json.dumps({"title": "t", "entries": [
            {"label": [], "exploration": []},
            {"label": "x", "exploration": {"question": [], "branches": {}}},
        ]}),
        '{"title": "t", "entries": [{"label": "x", "exploration":'
        ' {"question": "q", "branches": [{"id": 1}]}}]}',
    ]
    for text in hostile:
        found, defects = load_atlas_text(text)
        assert found is None
        assert isinstance(defects, list) and defects


def two_entry_atlas():
    first = {
        "question": "q one",
        "branches": [
            {"id": "a", "parent": None, "prompt": "p", "state": "taken",
             "finding": "f", "claim": "crew"},
            {"id": "b", "parent": "a", "prompt": "p2", "state": "open"},
            {"id": "c", "parent": None, "prompt": "p3",
             "state": "abandoned", "reason": "r"},
        ],
    }
    second = {
        "question": "q two",
        "branches": [
            {"id": "d", "parent": None, "prompt": "p4", "state": "open",
             "claim": "crew"},
            {"id": "e", "parent": "d", "prompt": "p5", "state": "taken",
             "finding": "f2", "claim": "crew"},
        ],
    }
    text = json.dumps({"title": "the title", "entries": [
        {"label": "first", "exploration": first},
        {"label": "second", "exploration": second},
    ]})
    found, defects = load_atlas_text(text)
    assert defects == []
    return found


def sections_of(lines):
    sections = {}
    current = None
    for line in lines[1:-1]:
        if line.startswith("entry "):
            current = line[len("entry "):]
            sections[current] = []
        elif current is not None:
            sections[current].append(line)
    return sections


def test_the_rendering_shape_line_for_line():
    assert render_atlas(two_entry_atlas()) == [
        "atlas, the title",
        "entry first",
        "question, q one",
        "a (taken) p",
        "  claimed by crew",
        "  finding, f",
        "  b (open) p2",
        "c (abandoned) p3",
        "  reason, r",
        "branches 3, open 1, taken 1, abandoned 1",
        "claim crew, branches 1, open 0, taken 1, abandoned 0",
        "unclaimed, branches 2, open 1, taken 0, abandoned 1",
        "entry second",
        "question, q two",
        "d (open) p4",
        "  claimed by crew",
        "  e (taken) p5",
        "    claimed by crew",
        "    finding, f2",
        "branches 2, open 1, taken 1, abandoned 0",
        "claim crew, branches 2, open 1, taken 1, abandoned 0",
        "atlas counts, entries 2, branches 5, open 2, taken 2, abandoned 1",
    ]


def test_each_section_is_the_map_and_the_listing_of_its_exploration():
    atlas = two_entry_atlas()
    sections = sections_of(render_atlas(atlas))
    for entry in atlas["entries"]:
        section = sections[entry["label"]]
        map_lines = render_map(entry["exploration"])
        claim_lines = render_claims(entry["exploration"])[1:]
        assert section == map_lines + claim_lines


def test_the_closing_counts_equal_the_sum_of_the_section_counts():
    atlas = two_entry_atlas()
    closing = render_atlas(atlas)[-1]
    total = {"total": 0, "open": 0, "taken": 0, "abandoned": 0}
    for entry in atlas["entries"]:
        counts = count_by_state(entry["exploration"])
        for key in total:
            total[key] += counts[key]
    union = [b for e in atlas["entries"] for b in e["exploration"]["branches"]]
    by_path = count_by_state({"question": "u", "branches": union})
    assert by_path == total
    assert closing == (
        f"atlas counts, entries 2, branches {total['total']}, "
        f"open {total['open']}, taken {total['taken']}, "
        f"abandoned {total['abandoned']}"
    )


def test_a_title_carrying_the_counts_text_cannot_mint_the_closing_line():
    first = exploration(branches=[
        {"id": "a", "parent": None, "prompt": "p", "state": "open"},
        {"id": "b", "parent": None, "prompt": "p2", "state": "taken",
         "finding": "f"},
        {"id": "c", "parent": None, "prompt": "p3",
         "state": "abandoned", "reason": "r"},
    ])
    second = exploration(branches=[
        {"id": "d", "parent": None, "prompt": "p4", "state": "open"},
        {"id": "e", "parent": None, "prompt": "p5", "state": "open"},
    ])
    data = {
        "title": "entries 2, branches 5, open 3, taken 1, abandoned 1",
        "entries": [
            {"label": "first", "exploration": first},
            {"label": "second", "exploration": second},
        ],
    }
    found, defects = load_atlas_text(json.dumps(data))
    assert defects == []
    lines = render_atlas(found)
    assert lines[0] == (
        "atlas, entries 2, branches 5, open 3, taken 1, abandoned 1"
    )
    assert lines[-1] == (
        "atlas counts, entries 2, branches 5, open 3, taken 1, "
        "abandoned 1"
    )
    assert lines[0] != lines[-1]


def test_sections_stand_in_atlas_order_regardless_of_size():
    def entry(label, prompts):
        return {"label": label,
                "exploration": fork_skeleton(f"q {label}", prompts)}

    big = entry("big", [f"prompt {i}" for i in range(30)])
    small = entry("small", ["only prompt"])
    mid = entry("mid", [f"prompt {i}" for i in range(10)])
    text = json.dumps({"title": "t", "entries": [big, small, mid]})
    found, defects = load_atlas_text(text)
    assert defects == []
    lines = render_atlas(found)
    order = [line for line in lines if line.startswith("entry ")]
    assert order == ["entry big", "entry small", "entry mid"]


def test_the_unclaimed_line_stands_only_where_an_unclaimed_branch_does():
    atlas = two_entry_atlas()
    sections = sections_of(render_atlas(atlas))
    assert any(
        line.startswith("unclaimed") for line in sections["first"]
    )
    assert not any(
        line.startswith("unclaimed") for line in sections["second"]
    )
    assert any(
        line.startswith("claim ") for line in sections["second"]
    )


def test_an_empty_atlas_renders_its_title_and_its_counts():
    found, defects = load_atlas_text(atlas_text(entries=[]))
    assert defects == []
    assert render_atlas(found) == [
        "atlas, t",
        "atlas counts, entries 0, branches 0, open 0, taken 0, abandoned 0",
    ]


def test_an_entry_with_no_branches_renders_its_question_and_counts():
    data = {"title": "t", "entries": [
        {"label": "bare", "exploration": {"question": "q", "branches": []}},
    ]}
    found, defects = load_atlas_text(json.dumps(data))
    assert defects == []
    assert render_atlas(found) == [
        "atlas, t",
        "entry bare",
        "question, q",
        "branches 0, open 0, taken 0, abandoned 0",
        "atlas counts, entries 1, branches 0, open 0, taken 0, abandoned 0",
    ]


def test_the_rendering_carries_no_ranking_or_verdict_word():
    lines = render_atlas(two_entry_atlas())
    text = "\n".join(lines).lower()
    for word in FORBIDDEN_WORDS:
        assert not re.search(rf"\b{word}\b", text), word


def test_the_rendering_is_pure_and_repeatable():
    atlas = two_entry_atlas()
    before = copy.deepcopy(atlas)
    first = render_atlas(atlas)
    assert render_atlas(atlas) == first
    assert atlas == before


def test_an_atlas_summing_past_a_thousand_branches_renders_whole():
    entries = []
    for number in range(3):
        skeleton = fork_skeleton(
            f"question {number}",
            [f"prompt {number} {i}" for i in range(400)],
        )
        entries.append({"label": f"part {number}", "exploration": skeleton})
    found, defects = load_atlas_text(
        json.dumps({"title": "wide", "entries": entries})
    )
    assert defects == []
    lines = render_atlas(found)
    assert lines[-1] == (
        "atlas counts, entries 3, branches 1200, open 1200, "
        "taken 0, abandoned 0"
    )
    assert lines.count("atlas counts, entries 3, branches 1200, "
                        "open 1200, taken 0, abandoned 0") == 1
    assert sum(1 for line in lines if line.startswith("entry ")) == 3


def test_an_atlas_holding_a_deep_exploration_renders_whole():
    depth = 5000
    branches = [
        {
            "id": f"b{i}",
            "parent": f"b{i - 1}" if i else None,
            "prompt": f"p{i}",
            "state": "open",
        }
        for i in range(depth)
    ]
    data = {"title": "deep", "entries": [
        {"label": "chain",
         "exploration": {"question": "q", "branches": branches}},
    ]}
    found, defects = load_atlas_text(json.dumps(data))
    assert defects == []
    lines = render_atlas(found)
    assert lines[-1] == (
        f"atlas counts, entries 1, branches {depth}, open {depth}, "
        "taken 0, abandoned 0"
    )
    deepest = "  " * (depth - 1) + f"b{depth - 1} (open) p{depth - 1}"
    assert deepest in lines
