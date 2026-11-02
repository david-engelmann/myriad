"""Pins for the Round 2 commands of the myriad command line surface.

The claims and census commands are pinned on their printed output
and their exit status together, on a sound file and on a malformed
one, exactly as the map and check commands were pinned in Round 1.
"""

import json

from myriad.cli import main

SOUND = {
    "question": "q",
    "branches": [
        {"id": "a", "parent": None, "prompt": "p", "state": "open",
         "claim": "north fork"},
        {"id": "b", "parent": "a", "prompt": "p2", "state": "taken",
         "finding": "f", "claim": "north fork"},
        {"id": "c", "parent": None, "prompt": "p3", "state": "abandoned",
         "reason": "r"},
    ],
}


def write(tmp_path, data):
    path = tmp_path / "exploration.json"
    path.write_text(json.dumps(data), encoding="utf-8")
    return str(path)


def test_claims_prints_the_listing_and_exits_zero(tmp_path, capsys):
    status = main(["claims", write(tmp_path, SOUND)])
    assert status == 0
    assert capsys.readouterr().out == (
        "question, q\n"
        "claim north fork, branches 2, open 1, taken 1, abandoned 0\n"
        "unclaimed, branches 1, open 0, taken 0, abandoned 1\n"
    )


def test_census_prints_the_census_and_exits_zero(tmp_path, capsys):
    status = main(["census", write(tmp_path, SOUND)])
    assert status == 0
    assert capsys.readouterr().out == (
        "question, q\n"
        "taken\n"
        "  claim north fork\n"
        "    b, finding, f\n"
        "abandoned\n"
        "  unclaimed\n"
        "    c, reason, r\n"
        "open\n"
        "  a, prompt, p\n"
        "branches 3, open 1, taken 1, abandoned 1\n"
    )


def test_claims_on_a_malformed_file_prints_defects_and_exits_one(
    tmp_path, capsys
):
    bad = {"question": "q", "branches": [
        {"id": "a", "parent": None, "prompt": "p", "state": "open",
         "claim": ""},
    ]}
    status = main(["claims", write(tmp_path, bad)])
    assert status == 1
    assert capsys.readouterr().out == (
        "invalid exploration\n"
        "defect branch 0 field claim is empty\n"
    )


def test_census_on_a_malformed_file_prints_defects_and_exits_one(
    tmp_path, capsys
):
    status = main(["census", write(tmp_path, {"branches": []})])
    assert status == 1
    assert capsys.readouterr().out == (
        "invalid exploration\ndefect missing field question\n"
    )


def test_census_on_a_missing_file_exits_one_with_the_read_defect(
    tmp_path, capsys
):
    status = main(["census", str(tmp_path / "missing.json")])
    assert status == 1
    assert capsys.readouterr().out == (
        "invalid exploration\ndefect file cannot be read\n"
    )
