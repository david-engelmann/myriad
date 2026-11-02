"""Pins for the map rendering and the counts in myriad.explorations.

The rendering is pinned line for line on a tree whose list order and
tree order disagree, so the walk order the contract states is the
only order that passes.
"""

import json

from myriad.explorations import (
    count_by_state,
    load_exploration_text,
    render_map,
)


def tree():
    return {
        "question": "How does the town keep its ferry?",
        "branches": [
            {"id": "child", "parent": "vessel", "prompt": "What fuel",
             "state": "open"},
            {"id": "vessel", "parent": None, "prompt": "What vessel",
             "state": "taken", "finding": "The spare boat serves"},
            {"id": "crew", "parent": None, "prompt": "Who crews it",
             "state": "open"},
            {"id": "old", "parent": "vessel", "prompt": "What sank",
             "state": "abandoned", "reason": "Settled by the survey"},
            {"id": "grand", "parent": "child", "prompt": "What dosing",
             "state": "open"},
        ],
    }


def test_the_map_renders_line_for_line():
    assert render_map(tree()) == [
        "question, How does the town keep its ferry?",
        "vessel (taken) What vessel",
        "  finding, The spare boat serves",
        "  child (open) What fuel",
        "    grand (open) What dosing",
        "  old (abandoned) What sank",
        "    reason, Settled by the survey",
        "crew (open) Who crews it",
        "branches 5, open 3, taken 1, abandoned 1",
    ]


def test_roots_stand_in_list_order_and_children_with_them():
    lines = render_map(tree())
    assert lines.index("vessel (taken) What vessel") < lines.index(
        "crew (open) Who crews it"
    )
    assert lines.index("  child (open) What fuel") < lines.index(
        "  old (abandoned) What sank"
    )


def test_the_same_exploration_renders_byte_identical_lines():
    first = render_map(tree())
    second = render_map(tree())
    assert first == second
    assert "\n".join(first).encode() == "\n".join(second).encode()


def test_counts_are_counted_from_the_tree():
    assert count_by_state(tree()) == {
        "total": 5,
        "open": 3,
        "taken": 1,
        "abandoned": 1,
    }


def test_the_summary_line_reads_the_counts():
    exploration = tree()
    counts = count_by_state(exploration)
    summary = render_map(exploration)[-1]
    assert summary == (
        f"branches {counts['total']}, open {counts['open']}, "
        f"taken {counts['taken']}, abandoned {counts['abandoned']}"
    )


def test_an_empty_tree_renders_the_question_and_zero_counts():
    exploration = {"question": "q", "branches": []}
    assert render_map(exploration) == [
        "question, q",
        "branches 0, open 0, taken 0, abandoned 0",
    ]


def test_a_chain_of_five_thousand_renders_in_full():
    branches = [
        {"id": "b0", "parent": None, "prompt": "p0", "state": "open"}
    ]
    for i in range(1, 5000):
        branches.append(
            {"id": f"b{i}", "parent": f"b{i - 1}",
             "prompt": f"p{i}", "state": "open"}
        )
    exploration, defects = load_exploration_text(
        json.dumps({"question": "q", "branches": branches})
    )
    assert defects == []
    lines = render_map(exploration)
    assert len(lines) == 5002
    assert lines[0] == "question, q"
    assert lines[1] == "b0 (open) p0"
    assert lines[-2] == "  " * 4999 + "b4999 (open) p4999"
    assert lines[-1] == "branches 5000, open 5000, taken 0, abandoned 0"


def test_open_branches_carry_no_extra_line():
    exploration = {
        "question": "q",
        "branches": [
            {"id": "a", "parent": None, "prompt": "p", "state": "open"},
        ],
    }
    assert render_map(exploration) == [
        "question, q",
        "a (open) p",
        "branches 1, open 1, taken 0, abandoned 0",
    ]
