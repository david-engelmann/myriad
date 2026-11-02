"""Pins for the exploration loader in myriad.explorations.

Every defect phrase in MY-1 full form is pinned here against the
exact text the loader returns, and the structural checks of the tree
are pinned in both directions, firing on the broken tree and staying
silent on the sound one.
"""

import json

from myriad.explorations import load_exploration_file, load_exploration_text


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


def test_a_clean_exploration_loads_and_keeps_only_the_named_fields():
    data = exploration(
        branch(extra="ignored"),
        branch(id="b", parent="a", state="taken", finding="found it"),
        branch(id="c", parent="a", state="abandoned", reason="it died"),
        extra_top="ignored",
    )
    found, defects = load_exploration_text(json.dumps(data))
    assert defects == []
    assert found == {
        "question": "q",
        "branches": [
            {"id": "a", "parent": None, "prompt": "p", "state": "open"},
            {"id": "b", "parent": "a", "prompt": "p", "state": "taken",
             "finding": "found it"},
            {"id": "c", "parent": "a", "prompt": "p", "state": "abandoned",
             "reason": "it died"},
        ],
    }


def test_an_empty_branches_list_loads_clean():
    found, defects = load_exploration_text(
        json.dumps({"question": "q", "branches": []})
    )
    assert defects == []
    assert found == {"question": "q", "branches": []}


def test_text_that_is_not_json_draws_the_not_valid_json_defect():
    found, defects = load_exploration_text("{not json")
    assert found is None
    assert defects == ["input is not valid JSON"]


def test_input_nested_too_deeply_draws_the_not_valid_json_defect():
    found, defects = load_exploration_text("[" * 10000 + "]" * 10000)
    assert found is None
    assert defects == ["input is not valid JSON"]


def test_json_that_is_not_an_object_draws_its_defect():
    for text in ("[1, 2]", '"a string"', "42", "null", "true"):
        found, defects = load_exploration_text(text)
        assert found is None
        assert defects == ["input is not a JSON object"], text


def test_the_loader_never_raises_on_garbage():
    junk = [
        "",
        " ",
        "{}",
        '{"question": 1}',
        '{"question": "q", "branches": [null, true, [], {}, "x"]}',
        '{"question": "q", "branches": [{"id": [], "parent": {},'
        ' "prompt": null, "state": 9, "finding": [], "reason": {}}]}',
    ]
    for text in junk:
        found, defects = load_exploration_text(text)
        assert found is None, text
        assert defects != [], text


def test_a_file_that_cannot_be_read_draws_the_single_defect(tmp_path):
    found, defects = load_exploration_file(tmp_path / "missing.json")
    assert found is None
    assert defects == ["file cannot be read"]
    found, defects = load_exploration_file(tmp_path)
    assert found is None
    assert defects == ["file cannot be read"]


def test_question_defects_are_named():
    assert defects_of({"branches": []}) == ["missing field question"]
    assert defects_of(exploration(question=7)) == [
        "field question has the wrong type"
    ]
    assert defects_of(exploration(question="")) == [
        "field question is empty"
    ]


def test_a_question_holding_an_unpaired_surrogate_is_named():
    text = '{"question": "\\ud800x", "branches": []}'
    found, defects = load_exploration_text(text)
    assert found is None
    assert defects == ["field question holds an unpaired surrogate"]


def test_branches_defects_are_named():
    assert defects_of({"question": "q"}) == ["missing field branches"]
    assert defects_of(exploration(branches={"a": 1})) == [
        "field branches has the wrong type"
    ]
    assert defects_of(exploration(branches="abc")) == [
        "field branches has the wrong type"
    ]


def test_a_branch_that_is_not_an_object_is_named():
    assert defects_of(exploration(7)) == ["branch 0 is not an object"]
    assert defects_of(exploration(branch(), "x")) == [
        "branch 1 is not an object"
    ]


def test_id_defects_are_named():
    no_id = branch()
    del no_id["id"]
    assert defects_of(exploration(no_id)) == ["branch 0 is missing field id"]
    assert defects_of(exploration(branch(id=7))) == [
        "branch 0 field id has the wrong type"
    ]
    assert defects_of(exploration(branch(id=""))) == [
        "branch 0 field id is empty"
    ]


def test_parent_defects_are_named():
    no_parent = branch()
    del no_parent["parent"]
    assert defects_of(exploration(no_parent)) == [
        "branch 0 is missing field parent"
    ]
    assert defects_of(exploration(branch(parent=7))) == [
        "branch 0 field parent has the wrong type"
    ]
    assert defects_of(exploration(branch(parent=""))) == [
        "branch 0 field parent is empty"
    ]


def test_prompt_defects_are_named():
    no_prompt = branch()
    del no_prompt["prompt"]
    assert defects_of(exploration(no_prompt)) == [
        "branch 0 is missing field prompt"
    ]
    assert defects_of(exploration(branch(prompt=None))) == [
        "branch 0 field prompt has the wrong type"
    ]
    assert defects_of(exploration(branch(prompt=""))) == [
        "branch 0 field prompt is empty"
    ]


def test_a_prompt_holding_an_unpaired_surrogate_is_named():
    text = json.dumps(exploration(branch(prompt="\ud800x")))
    found, defects = load_exploration_text(text)
    assert found is None
    assert defects == ["branch 0 field prompt holds an unpaired surrogate"]


def test_state_defects_are_named():
    no_state = branch()
    del no_state["state"]
    assert defects_of(exploration(no_state)) == [
        "branch 0 is missing field state"
    ]
    assert defects_of(exploration(branch(state=1))) == [
        "branch 0 field state has the wrong type"
    ]
    assert defects_of(exploration(branch(state="gone"))) == [
        "branch 0 field state is not a known state"
    ]


def test_a_taken_branch_without_a_finding_is_named():
    assert defects_of(exploration(branch(state="taken"))) == [
        "branch 0 is missing field finding"
    ]


def test_an_abandoned_branch_without_a_reason_is_named():
    assert defects_of(exploration(branch(state="abandoned"))) == [
        "branch 0 is missing field reason"
    ]


def test_finding_and_reason_values_are_checked():
    assert defects_of(exploration(branch(state="taken", finding=3))) == [
        "branch 0 field finding has the wrong type"
    ]
    assert defects_of(exploration(branch(state="taken", finding=""))) == [
        "branch 0 field finding is empty"
    ]
    assert defects_of(exploration(branch(state="abandoned", reason=[]))) == [
        "branch 0 field reason has the wrong type"
    ]
    assert defects_of(exploration(branch(state="abandoned", reason=""))) == [
        "branch 0 field reason is empty"
    ]


def test_a_finding_or_reason_in_a_forbidding_state_is_named():
    assert defects_of(exploration(branch(finding="f"))) == [
        "branch 0 field finding is not allowed in state open"
    ]
    assert defects_of(exploration(branch(reason="r"))) == [
        "branch 0 field reason is not allowed in state open"
    ]
    assert defects_of(
        exploration(branch(state="taken", finding="f", reason="r"))
    ) == ["branch 0 field reason is not allowed in state taken"]
    assert defects_of(
        exploration(branch(state="abandoned", reason="r", finding="f"))
    ) == ["branch 0 field finding is not allowed in state abandoned"]


def test_one_input_draws_several_defects_in_one_pass():
    data = exploration(
        branch(id=""),
        branch(id="b", prompt="", state="gone"),
        question="",
    )
    assert defects_of(data) == [
        "field question is empty",
        "branch 0 field id is empty",
        "branch 1 field prompt is empty",
        "branch 1 field state is not a known state",
    ]


def test_a_duplicate_id_is_named():
    data = exploration(branch(), branch(prompt="other"))
    assert defects_of(data) == ["duplicate branch id a"]


def test_a_parent_that_does_not_exist_is_named():
    data = exploration(branch(parent="ghost"))
    assert defects_of(data) == ["branch a parent ghost does not exist"]


def test_a_parent_with_a_defective_field_still_exists():
    data = exploration(
        branch(prompt=""),
        branch(id="b", parent="a", prompt="other"),
    )
    assert defects_of(data) == ["branch 0 field prompt is empty"]


def test_a_self_parent_is_a_cycle():
    data = exploration(branch(parent="a"))
    assert defects_of(data) == ["branch a has a cycle in its parent chain"]


def test_a_two_branch_cycle_names_both_branches():
    data = exploration(
        branch(parent="b"),
        branch(id="b", parent="a", prompt="other"),
    )
    assert defects_of(data) == [
        "branch a has a cycle in its parent chain",
        "branch b has a cycle in its parent chain",
    ]


def test_a_chain_leading_into_a_cycle_is_named():
    data = exploration(
        branch(id="a", parent="b", prompt="p1"),
        branch(id="b", parent="c", prompt="p2"),
        branch(id="c", parent="b", prompt="p3"),
        branch(id="d", parent="a", prompt="p4"),
    )
    assert defects_of(data) == [
        "branch a has a cycle in its parent chain",
        "branch b has a cycle in its parent chain",
        "branch c has a cycle in its parent chain",
        "branch d has a cycle in its parent chain",
    ]


def test_a_deep_sound_chain_draws_no_cycle_defect():
    data = exploration(
        branch(id="a"),
        branch(id="b", parent="a", prompt="p2"),
        branch(id="c", parent="b", prompt="p3"),
        branch(id="d", parent="c", prompt="p4"),
    )
    found, defects = load_exploration_text(json.dumps(data))
    assert defects == []
    assert found is not None


def test_a_cycle_of_five_thousand_branches_names_every_branch():
    branches = [
        {"id": f"b{i}", "parent": f"b{(i + 1) % 5000}",
         "prompt": f"p{i}", "state": "open"}
        for i in range(5000)
    ]
    found, defects = load_exploration_text(
        json.dumps({"question": "q", "branches": branches})
    )
    assert found is None
    assert len(defects) == 5000
    assert defects[0] == "branch b0 has a cycle in its parent chain"
    assert defects[-1] == "branch b4999 has a cycle in its parent chain"


def test_a_duplicate_prompt_among_open_branches_is_named():
    data = exploration(
        branch(prompt="same"),
        branch(id="b", prompt="same"),
    )
    assert defects_of(data) == [
        "branch b repeats the prompt of open branch a"
    ]


def test_a_duplicate_prompt_stays_silent_across_states():
    cases = [
        exploration(
            branch(prompt="same"),
            branch(id="b", prompt="same", state="abandoned", reason="r"),
        ),
        exploration(
            branch(prompt="same"),
            branch(id="b", prompt="same", state="taken", finding="f"),
        ),
        exploration(
            branch(prompt="same", state="taken", finding="f"),
            branch(id="b", prompt="same"),
        ),
        exploration(
            branch(prompt="same", state="taken", finding="f"),
            branch(id="b", prompt="same", state="taken", finding="g"),
        ),
        exploration(
            branch(prompt="same", state="abandoned", reason="r"),
            branch(id="b", prompt="same", state="abandoned", reason="s"),
        ),
    ]
    for data in cases:
        found, defects = load_exploration_text(json.dumps(data))
        assert defects == [], data
        assert found is not None
