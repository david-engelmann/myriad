"""Pins for the Round 2 repair amendments, in two repair passes.

FIX-MY-6 names a newline or a carriage return in every text field
whose value reaches a rendering, at the exact defect text the
amendment states. FIX-MY-7 first rendered a claimed branch on the
map with a bracketed tag before the prompt, and FIX-MY-10 retired
the tag for a claim line of the branch's own, so the form pins
below state the claim line form.

The second pass pins FIX-MY-8, FIX-MY-9, and FIX-MY-10. FIX-MY-8
refuses a line break in the fork helper, FIX-MY-9 extends the
line break rule to the branch id and the branch parent, and
FIX-MY-10 stands the claim on its own line under the branch line.
"""

import json

import pytest

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


LINE_BREAK = "holds a newline or a carriage return"


def test_a_claim_holding_a_forged_counts_line_is_named():
    forged = "north\nbranches 99, open 99, taken 0, abandoned 0"
    assert defects_of(exploration(branch(claim=forged))) == [
        f"branch 0 field claim {LINE_BREAK}"
    ]


def test_a_claim_holding_a_carriage_return_is_named():
    assert defects_of(exploration(branch(claim="north\rfork"))) == [
        f"branch 0 field claim {LINE_BREAK}"
    ]


def test_a_prompt_holding_a_newline_is_named():
    assert defects_of(exploration(branch(prompt="first\nsecond"))) == [
        f"branch 0 field prompt {LINE_BREAK}"
    ]


def test_a_prompt_holding_a_carriage_return_is_named():
    assert defects_of(exploration(branch(prompt="first\rsecond"))) == [
        f"branch 0 field prompt {LINE_BREAK}"
    ]


def test_a_question_holding_a_newline_is_named():
    assert defects_of(exploration(question="one\nquestion")) == [
        f"field question {LINE_BREAK}"
    ]


def test_a_question_holding_a_carriage_return_is_named():
    assert defects_of(exploration(question="one\rquestion")) == [
        f"field question {LINE_BREAK}"
    ]


def test_a_finding_holding_a_newline_is_named():
    assert defects_of(exploration(
        branch(state="taken", finding="found\nbranches 1, open 0")
    )) == [f"branch 0 field finding {LINE_BREAK}"]


def test_a_reason_holding_a_newline_is_named():
    assert defects_of(exploration(
        branch(state="abandoned", reason="it died\nquietly")
    )) == [f"branch 0 field reason {LINE_BREAK}"]


def test_the_loader_keeps_naming_after_a_line_break_defect():
    assert defects_of(exploration(
        branch(prompt="first\nsecond"),
        branch(id="b", parent="a", prompt="p2", claim="north\nfork"),
    )) == [
        f"branch 0 field prompt {LINE_BREAK}",
        f"branch 1 field claim {LINE_BREAK}",
    ]


def test_fields_without_line_breaks_still_load_clean():
    found = loads_clean(exploration(
        branch(state="taken", finding="found it, all of it",
               claim="north fork"),
        branch(id="b", parent="a", prompt="p2 [draft]"),
    ))
    assert found["branches"][0]["claim"] == "north fork"


def test_a_claimed_branch_renders_its_claim_on_its_own_line():
    found = loads_clean(exploration(
        branch(state="taken", finding="found it", claim="north fork"),
        branch(id="b", parent="a", prompt="p2"),
    ))
    assert render_map(found) == [
        "question, q",
        "a (taken) p",
        "  claimed by north fork",
        "  finding, found it",
        "  b (open) p2",
        "branches 2, open 1, taken 1, abandoned 0",
    ]


def test_the_retired_suffix_text_no_longer_reads_as_a_claim():
    found = loads_clean(exploration(
        branch(prompt="route", claim="route"),
        branch(id="b", prompt="route, claimed by route"),
    ))
    assert render_map(found) == [
        "question, q",
        "a (open) route",
        "  claimed by route",
        "b (open) route, claimed by route",
        "branches 2, open 2, taken 0, abandoned 0",
    ]


def test_the_listing_counts_the_suffix_prompt_as_unclaimed():
    found = loads_clean(exploration(
        branch(prompt="route", claim="route"),
        branch(id="b", prompt="route, claimed by route"),
    ))
    assert render_claims(found) == [
        "question, q",
        "claim route, branches 1, open 1, taken 0, abandoned 0",
        "unclaimed, branches 1, open 1, taken 0, abandoned 0",
    ]


def test_the_fork_helper_refuses_a_question_holding_a_newline():
    with pytest.raises(ValueError):
        fork_skeleton("one\nquestion", ["one"])


def test_the_fork_helper_refuses_a_question_holding_a_carriage_return():
    with pytest.raises(ValueError):
        fork_skeleton("one\rquestion", ["one"])


def test_the_fork_helper_refuses_a_prompt_holding_a_newline():
    with pytest.raises(ValueError):
        fork_skeleton("q", ["one", "two\nlines"])


def test_the_fork_helper_refuses_a_prompt_holding_a_carriage_return():
    with pytest.raises(ValueError):
        fork_skeleton("q", ["one", "two\rlines"])


def test_the_fork_helper_refuses_a_prompt_holding_a_crlf_pair():
    with pytest.raises(ValueError):
        fork_skeleton("q", ["one", "two\r\nlines"])


def test_an_id_holding_a_forged_counts_line_is_named():
    forged = "a\nbranches 99, open 99, taken 0, abandoned 0"
    assert defects_of(exploration(branch(id=forged))) == [
        f"branch 0 field id {LINE_BREAK}"
    ]


def test_a_parent_holding_a_carriage_return_is_named():
    assert defects_of(exploration(branch(parent="root\rforged"))) == [
        f"branch 0 field parent {LINE_BREAK}"
    ]


def test_a_parent_holding_a_newline_draws_no_forged_defect_text():
    forged = "ghost\nbranches 99, open 99, taken 0, abandoned 0"
    assert defects_of(exploration(branch(parent=forged))) == [
        f"branch 0 field parent {LINE_BREAK}"
    ]


def test_the_loader_keeps_naming_after_an_id_line_break_defect():
    assert defects_of(exploration(
        branch(id="a\nb", prompt="first\nsecond"),
    )) == [
        f"branch 0 field id {LINE_BREAK}",
        f"branch 0 field prompt {LINE_BREAK}",
    ]


def test_a_bracketed_prompt_does_not_read_as_a_claim():
    found = loads_clean(exploration(
        branch(prompt="route", claim="north"),
        branch(id="b", prompt="[north] route"),
    ))
    assert render_map(found) == [
        "question, q",
        "a (open) route",
        "  claimed by north",
        "b (open) [north] route",
        "branches 2, open 2, taken 0, abandoned 0",
    ]


def test_a_claim_label_holding_brackets_renders_on_its_claim_line():
    found = loads_clean(exploration(
        branch(prompt="route", claim="a] [b"),
        branch(id="b", parent="a", prompt="p2", state="abandoned",
               reason="it died", claim="[north]"),
    ))
    assert render_map(found) == [
        "question, q",
        "a (open) route",
        "  claimed by a] [b",
        "  b (abandoned) p2",
        "    claimed by [north]",
        "    reason, it died",
        "branches 2, open 1, taken 0, abandoned 1",
    ]
