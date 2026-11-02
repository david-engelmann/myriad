"""Pins for the myriad command line surface.

Each command is pinned on its printed output and its exit status
together, on a sound file and on a malformed one.
"""

import json

from myriad.cli import main

SOUND = {
    "question": "q",
    "branches": [
        {"id": "a", "parent": None, "prompt": "p", "state": "open"},
        {"id": "b", "parent": "a", "prompt": "p2", "state": "taken",
         "finding": "f"},
        {"id": "c", "parent": None, "prompt": "p3", "state": "abandoned",
         "reason": "r"},
    ],
}


def write(tmp_path, data):
    path = tmp_path / "exploration.json"
    path.write_text(json.dumps(data), encoding="utf-8")
    return str(path)


def test_map_prints_the_map_and_exits_zero(tmp_path, capsys):
    status = main(["map", write(tmp_path, SOUND)])
    assert status == 0
    assert capsys.readouterr().out == (
        "question, q\n"
        "a (open) p\n"
        "  b (taken) p2\n"
        "    finding, f\n"
        "c (abandoned) p3\n"
        "  reason, r\n"
        "branches 3, open 1, taken 1, abandoned 1\n"
    )


def test_check_prints_the_counts_and_exits_zero(tmp_path, capsys):
    status = main(["check", write(tmp_path, SOUND)])
    assert status == 0
    assert capsys.readouterr().out == (
        "branches 3\nopen 1\ntaken 1\nabandoned 1\n"
    )


def test_map_on_a_malformed_file_prints_defects_and_exits_one(
    tmp_path, capsys
):
    bad = {"question": "q", "branches": [{"id": "a", "parent": "ghost",
                                          "prompt": "p", "state": "open"}]}
    status = main(["map", write(tmp_path, bad)])
    assert status == 1
    assert capsys.readouterr().out == (
        "invalid exploration\n"
        "defect branch a parent ghost does not exist\n"
    )


def test_check_on_a_malformed_file_prints_defects_and_exits_one(
    tmp_path, capsys
):
    status = main(["check", write(tmp_path, {"branches": []})])
    assert status == 1
    assert capsys.readouterr().out == (
        "invalid exploration\ndefect missing field question\n"
    )


def test_map_prints_a_chain_of_five_thousand_in_full(tmp_path, capsys):
    branches = [
        {"id": "b0", "parent": None, "prompt": "p0", "state": "open"}
    ]
    for i in range(1, 5000):
        branches.append(
            {"id": f"b{i}", "parent": f"b{i - 1}",
             "prompt": f"p{i}", "state": "open"}
        )
    data = {"question": "q", "branches": branches}
    status = main(["map", write(tmp_path, data)])
    assert status == 0
    lines = capsys.readouterr().out.splitlines()
    assert len(lines) == 5002
    assert lines[0] == "question, q"
    assert lines[1] == "b0 (open) p0"
    assert lines[-2] == "  " * 4999 + "b4999 (open) p4999"
    assert lines[-1] == "branches 5000, open 5000, taken 0, abandoned 0"


def test_a_missing_file_exits_one_with_the_read_defect(tmp_path, capsys):
    status = main(["map", str(tmp_path / "missing.json")])
    assert status == 1
    assert capsys.readouterr().out == (
        "invalid exploration\ndefect file cannot be read\n"
    )
