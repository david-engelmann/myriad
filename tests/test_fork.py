"""Pins for the fork helper in myriad.fork.

The skeleton is pinned against the MY-1 loader and the MY-2 map, so
the shape a live forker would emit is held by the same checks every
other exploration passes.
"""

import json

import pytest

from myriad.explorations import (
    count_by_state,
    load_exploration_text,
    render_map,
)
from myriad.fork import fork_skeleton


def test_the_skeleton_holds_one_open_root_per_prompt():
    skeleton = fork_skeleton("q", ["one", "two", "three"])
    assert skeleton == {
        "question": "q",
        "branches": [
            {"id": "b01", "parent": None, "prompt": "one", "state": "open"},
            {"id": "b02", "parent": None, "prompt": "two", "state": "open"},
            {"id": "b03", "parent": None, "prompt": "three", "state": "open"},
        ],
    }


def test_the_skeleton_loads_clean_and_renders():
    skeleton = fork_skeleton("q", ["one", "two"])
    found, defects = load_exploration_text(json.dumps(skeleton))
    assert defects == []
    assert found == skeleton
    assert render_map(found) == [
        "question, q",
        "b01 (open) one",
        "b02 (open) two",
        "branches 2, open 2, taken 0, abandoned 0",
    ]


def test_the_skeleton_forks_tens_of_branches():
    prompts = [f"branch number {n}" for n in range(1, 31)]
    skeleton = fork_skeleton("q", prompts)
    assert count_by_state(skeleton) == {
        "total": 30,
        "open": 30,
        "taken": 0,
        "abandoned": 0,
    }
    assert skeleton["branches"][-1]["id"] == "b30"


def test_an_empty_prompt_list_builds_an_empty_skeleton():
    skeleton = fork_skeleton("q", [])
    found, defects = load_exploration_text(json.dumps(skeleton))
    assert defects == []
    assert found == {"question": "q", "branches": []}


def test_a_question_holding_an_unpaired_surrogate_raises_value_error():
    with pytest.raises(ValueError):
        fork_skeleton("\ud800x", ["one"])


def test_a_prompt_holding_an_unpaired_surrogate_raises_value_error():
    with pytest.raises(ValueError):
        fork_skeleton("q", ["one", "bad \ud800x"])


def test_unsound_inputs_raise_value_error():
    with pytest.raises(ValueError):
        fork_skeleton("", ["one"])
    with pytest.raises(ValueError):
        fork_skeleton(7, ["one"])
    with pytest.raises(ValueError):
        fork_skeleton("q", [""])
    with pytest.raises(ValueError):
        fork_skeleton("q", [None])
    with pytest.raises(ValueError):
        fork_skeleton("q", ["same", "same"])
