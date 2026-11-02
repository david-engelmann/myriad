"""Pins for the territory census in MY-6 full form.

The census is pinned line for line on a tree whose branches list
interleaves the states and whose claim labels appear out of
alphabetical order, so the section order, the grouping order, and
the unclaimed heading the contract states are the only readings
that pass.
"""

import json

from myriad.explorations import (
    load_exploration_text,
    render_census,
    render_map,
)


def loads_clean(data):
    found, defects = load_exploration_text(json.dumps(data))
    assert defects == []
    return found


def census_tree():
    return {
        "question": "q",
        "branches": [
            {"id": "o1", "parent": None, "prompt": "first open",
             "state": "open"},
            {"id": "t1", "parent": None, "prompt": "p", "state": "taken",
             "finding": "found one", "claim": "tide watch"},
            {"id": "t2", "parent": None, "prompt": "p", "state": "taken",
             "finding": "found two", "claim": "boat crew"},
            {"id": "t3", "parent": None, "prompt": "p", "state": "taken",
             "finding": "found three", "claim": "tide watch"},
            {"id": "t4", "parent": None, "prompt": "p", "state": "taken",
             "finding": "found four"},
            {"id": "a1", "parent": None, "prompt": "p",
             "state": "abandoned", "reason": "died one",
             "claim": "boat crew"},
            {"id": "a2", "parent": None, "prompt": "p",
             "state": "abandoned", "reason": "died two"},
            {"id": "o2", "parent": None, "prompt": "second open",
             "state": "open", "claim": "tide watch"},
        ],
    }


def test_the_census_renders_line_for_line():
    found = loads_clean(census_tree())
    assert render_census(found) == [
        "question, q",
        "taken",
        "  claim boat crew",
        "    t2, finding, found two",
        "  claim tide watch",
        "    t1, finding, found one",
        "    t3, finding, found three",
        "  unclaimed",
        "    t4, finding, found four",
        "abandoned",
        "  claim boat crew",
        "    a1, reason, died one",
        "  unclaimed",
        "    a2, reason, died two",
        "open",
        "  o1, prompt, first open",
        "  o2, prompt, second open",
        "branches 8, open 2, taken 4, abandoned 2",
    ]


def test_the_open_section_ignores_claims_and_stands_last():
    found = loads_clean(census_tree())
    lines = render_census(found)
    assert lines.index("open") > lines.index("abandoned")
    open_lines = lines[lines.index("open") + 1:-1]
    assert open_lines == [
        "  o1, prompt, first open",
        "  o2, prompt, second open",
    ]


def test_a_section_stands_only_when_its_state_is_held():
    found = loads_clean({
        "question": "q",
        "branches": [
            {"id": "a", "parent": None, "prompt": "p", "state": "taken",
             "finding": "found it"},
        ],
    })
    assert render_census(found) == [
        "question, q",
        "taken",
        "  unclaimed",
        "    a, finding, found it",
        "branches 1, open 0, taken 1, abandoned 0",
    ]


def test_the_census_of_an_empty_exploration_has_no_section():
    found = loads_clean({"question": "q", "branches": []})
    assert render_census(found) == [
        "question, q",
        "branches 0, open 0, taken 0, abandoned 0",
    ]


def test_the_census_counts_line_matches_the_map_counts_line():
    found = loads_clean(census_tree())
    assert render_census(found)[-1] == render_map(found)[-1]
    assert render_census(found)[-1] == (
        "branches 8, open 2, taken 4, abandoned 2"
    )


def test_the_census_carries_a_finding_in_full():
    long_finding = (
        "The dosing already on the shelf covers both boats, "
        "the spare included, through the coldest month on record"
    )
    found = loads_clean({
        "question": "q",
        "branches": [
            {"id": "a", "parent": None, "prompt": "p", "state": "taken",
             "finding": long_finding, "claim": "tide watch"},
        ],
    })
    assert render_census(found) == [
        "question, q",
        "taken",
        "  claim tide watch",
        f"    a, finding, {long_finding}",
        "branches 1, open 0, taken 1, abandoned 0",
    ]
