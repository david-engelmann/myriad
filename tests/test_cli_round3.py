"""Pins for the Round 3 command of the myriad command line surface.

The atlas command is pinned on its printed output and its exit
status together, on a sound file and on a malformed one, exactly
as the earlier commands were pinned.
"""

import json

from myriad.cli import main

SOUND = {
    "title": "the title",
    "entries": [
        {"label": "first", "exploration": {
            "question": "q one",
            "branches": [
                {"id": "a", "parent": None, "prompt": "p",
                 "state": "taken", "finding": "f", "claim": "crew"},
                {"id": "b", "parent": "a", "prompt": "p2",
                 "state": "open"},
            ],
        }},
        {"label": "second", "exploration": {
            "question": "q two",
            "branches": [],
        }},
    ],
}


def write(tmp_path, data):
    path = tmp_path / "atlas.json"
    path.write_text(json.dumps(data), encoding="utf-8")
    return str(path)


def test_atlas_prints_the_rendering_and_exits_zero(tmp_path, capsys):
    status = main(["atlas", write(tmp_path, SOUND)])
    assert status == 0
    assert capsys.readouterr().out == (
        "atlas, the title\n"
        "entry first\n"
        "question, q one\n"
        "a (taken) p\n"
        "  claimed by crew\n"
        "  finding, f\n"
        "  b (open) p2\n"
        "branches 2, open 1, taken 1, abandoned 0\n"
        "claim crew, branches 1, open 0, taken 1, abandoned 0\n"
        "unclaimed, branches 1, open 1, taken 0, abandoned 0\n"
        "entry second\n"
        "question, q two\n"
        "branches 0, open 0, taken 0, abandoned 0\n"
        "atlas counts, entries 2, branches 2, open 1, taken 1, abandoned 0\n"
    )


def test_atlas_on_a_malformed_file_prints_defects_and_exits_one(
    tmp_path, capsys
):
    broken = {
        "title": "t",
        "entries": [
            {"label": "ferry", "exploration": {"branches": []}},
            {"label": "ferry",
             "exploration": {"question": "q", "branches": []}},
        ],
    }
    status = main(["atlas", write(tmp_path, broken)])
    assert status == 1
    assert capsys.readouterr().out == (
        "invalid atlas\n"
        "defect entry ferry, missing field question\n"
        "defect duplicate entry label ferry, entries 0 and 1\n"
    )


def test_atlas_on_a_missing_file_prints_the_read_defect(tmp_path, capsys):
    status = main(["atlas", str(tmp_path / "missing.json")])
    assert status == 1
    assert capsys.readouterr().out == (
        "invalid atlas\n"
        "defect file cannot be read\n"
    )
