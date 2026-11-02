"""Pins for the synthetic example under examples.

The winter ferry exploration is the slice demo. It has to load
clean, carry tens of branches across all three states, and hold its
stored map byte identical to the map the code renders today.
"""

import pathlib

from myriad.explorations import (
    count_by_state,
    load_exploration_file,
    render_map,
)

EXAMPLES = pathlib.Path(__file__).resolve().parent.parent / "examples"


def load_example():
    exploration, defects = load_exploration_file(
        EXAMPLES / "winter-ferry.json"
    )
    assert defects == []
    return exploration


def test_the_example_exploration_loads_clean():
    exploration = load_example()
    assert exploration["question"] == (
        "How does a small harbour town keep its ferry link "
        "alive through the winter"
    )


def test_the_example_carries_tens_of_branches_across_all_states():
    counts = count_by_state(load_example())
    assert counts == {"total": 26, "open": 16, "taken": 6, "abandoned": 4}
    assert 20 <= counts["total"] <= 99


def test_the_example_map_renders_the_whole_tree():
    exploration = load_example()
    lines = render_map(exploration)
    branch_lines = [
        line for line in lines
        if any(f"{branch['id']} (" in line for branch in exploration["branches"])
    ]
    assert len(branch_lines) == 26
    assert lines[0].startswith("question, ")
    assert lines[-1] == "branches 26, open 16, taken 6, abandoned 4"


def test_the_golden_map_is_byte_identical():
    exploration = load_example()
    rendered = ("\n".join(render_map(exploration)) + "\n").encode()
    golden = (EXAMPLES / "winter-ferry-map.txt").read_bytes()
    assert rendered == golden
