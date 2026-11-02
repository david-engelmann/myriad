"""Pins for the dibs claim in MY-5 full form.

Every defect phrase the claim field can draw is pinned at its exact
text, and the claims listing is pinned line for line on a tree whose
labels appear out of alphabetical order, so the ordering the
contract states is the only order that passes.
"""

import json

from myriad.explorations import (
    load_exploration_text,
    render_claims,
    render_map,
)
from myriad.fork import fork_skeleton


def branch(**overrides):
    base = {"id": "a", "parent": None, "prompt": "p", "state": "open"}
    base.update(overrides)
    return base


def exploration(*branches, **overrides):
    data = {"question": "q", "branches": list(branches)}
    data.update(overrides)
    return data


def defects_of(data):
    exploration_found, defects = load_exploration_text(json.dumps(data))
    assert exploration_found is None
    return defects


def loads_clean(data):
    found, defects = load_exploration_text(json.dumps(data))
    assert defects == []
    return found


def test_a_claimed_branch_loads_and_carries_its_claim():
    found = loads_clean(exploration(
        branch(claim="north fork"),
        branch(id="b", parent="a", state="taken", finding="found it",
               claim="north fork"),
        branch(id="c", parent="a", state="abandoned", reason="it died",
               claim="south fork"),
        branch(id="d", parent="a", prompt="p4"),
    ))
    assert found["branches"][0]["claim"] == "north fork"
    assert found["branches"][1]["claim"] == "north fork"
    assert found["branches"][2]["claim"] == "south fork"
    assert "claim" not in found["branches"][3]


def test_a_claim_of_the_wrong_type_is_named():
    assert defects_of(exploration(branch(claim=7))) == [
        "branch 0 field claim has the wrong type"
    ]
    assert defects_of(exploration(branch(claim=None))) == [
        "branch 0 field claim has the wrong type"
    ]
    assert defects_of(exploration(
        branch(), branch(id="b", parent="a", prompt="p2",
                         claim=["north fork"])
    )) == ["branch 1 field claim has the wrong type"]


def test_an_empty_claim_is_named():
    assert defects_of(exploration(branch(claim=""))) == [
        "branch 0 field claim is empty"
    ]


def test_a_claim_holding_an_unpaired_surrogate_is_named():
    assert defects_of(exploration(branch(claim="bad \ud800 label"))) == [
        "branch 0 field claim holds an unpaired surrogate"
    ]


def test_a_claim_defect_does_not_stop_the_naming():
    assert defects_of(exploration(
        branch(state="open", finding="found it", claim="")
    )) == [
        "branch 0 field claim is empty",
        "branch 0 field finding is not allowed in state open",
    ]


def test_a_claim_is_allowed_in_every_state():
    found = loads_clean(exploration(
        branch(claim="north fork"),
        branch(id="b", parent="a", state="taken", finding="found it",
               claim="north fork"),
        branch(id="c", parent="a", state="abandoned", reason="it died",
               claim="north fork"),
    ))
    assert [b["claim"] for b in found["branches"]] == ["north fork"] * 3


def test_the_loader_output_differs_only_by_the_claim():
    bare = loads_clean(exploration(branch()))
    claimed = loads_clean(exploration(branch(claim="north fork")))
    assert claimed["branches"][0] == {**bare["branches"][0],
                                      "claim": "north fork"}
    assert claimed["question"] == bare["question"]


def test_the_map_names_the_claim_on_a_line_of_its_own():
    exploration_found = loads_clean(exploration(
        branch(state="taken", finding="found it", claim="north fork"),
        branch(id="b", parent="a", prompt="p2"),
    ))
    assert render_map(exploration_found) == [
        "question, q",
        "a (taken) p",
        "  claimed by north fork",
        "  finding, found it",
        "  b (open) p2",
        "branches 2, open 1, taken 1, abandoned 0",
    ]


def test_the_map_of_a_claim_free_exploration_keeps_its_round_1_lines():
    exploration_found = loads_clean(exploration(
        branch(state="taken", finding="found it"),
        branch(id="b", parent="a", prompt="p2"),
        branch(id="c", prompt="p3", state="abandoned", reason="it died"),
    ))
    assert render_map(exploration_found) == [
        "question, q",
        "a (taken) p",
        "  finding, found it",
        "  b (open) p2",
        "c (abandoned) p3",
        "  reason, it died",
        "branches 3, open 1, taken 1, abandoned 1",
    ]


def listing_tree():
    return exploration(
        branch(state="taken", finding="found it", claim="tide watch"),
        branch(id="b", parent="a", prompt="p2", claim="boat crew"),
        branch(id="c", prompt="p3", state="abandoned", reason="it died",
               claim="tide watch"),
        branch(id="d", prompt="p4", state="taken", finding="found more",
               claim="harbour office"),
        branch(id="e", prompt="p5"),
        branch(id="f", prompt="p6", state="taken", finding="found last"),
    )


def test_the_claims_listing_stands_in_alphabetical_label_order():
    found = loads_clean(listing_tree())
    assert render_claims(found) == [
        "question, q",
        "claim boat crew, branches 1, open 1, taken 0, abandoned 0",
        "claim harbour office, branches 1, open 0, taken 1, abandoned 0",
        "claim tide watch, branches 2, open 0, taken 1, abandoned 1",
        "unclaimed, branches 2, open 1, taken 1, abandoned 0",
    ]


def test_the_unclaimed_line_stands_only_when_a_branch_is_unclaimed():
    found = loads_clean(exploration(
        branch(claim="north fork"),
        branch(id="b", parent="a", prompt="p2", claim="north fork"),
    ))
    assert render_claims(found) == [
        "question, q",
        "claim north fork, branches 2, open 2, taken 0, abandoned 0",
    ]


def test_the_listing_of_an_empty_exploration_is_the_question_line():
    found = loads_clean({"question": "q", "branches": []})
    assert render_claims(found) == ["question, q"]


def test_a_fork_skeleton_lists_as_unclaimed_branches():
    skeleton = fork_skeleton("q", ["p1", "p2", "p3"])
    found = loads_clean(skeleton)
    assert all("claim" not in b for b in found["branches"])
    assert render_claims(found) == [
        "question, q",
        "unclaimed, branches 3, open 3, taken 0, abandoned 0",
    ]
