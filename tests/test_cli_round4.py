"""Pins for the Round 4 command of the myriad command line surface.

The index command is pinned on its printed output and its exit
status together, on a sound file and on a malformed one, exactly
as the earlier commands were pinned. A malformed atlas prints the
defects the atlas command prints and no line of the index reading,
because the command loads the atlas whole before it prints.
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

MALFORMED = {
    "title": "the title",
    "entries": [
        {"label": "first", "exploration": {
            "question": "q one",
            "branches": [
                {"id": "a", "parent": None, "prompt": "p",
                 "state": "taken", "finding": "f"},
            ],
        }},
        {"label": "first", "exploration": {
            "question": "q two",
            "branches": [],
        }},
        {"exploration": {"question": "q three", "branches": []}},
    ],
}


def write(tmp_path, data):
    path = tmp_path / "atlas.json"
    path.write_text(json.dumps(data), encoding="utf-8")
    return str(path)


def test_index_prints_the_reading_and_exits_zero(tmp_path, capsys):
    status = main(["index", write(tmp_path, SOUND)])
    assert status == 0
    assert capsys.readouterr().out == (
        "atlas, the title\n"
        "entry first, question, q one, "
        "branches 2, open 1, taken 1, abandoned 0\n"
        "entry second, question, q two, "
        "branches 0, open 0, taken 0, abandoned 0\n"
        "atlas counts, entries 2, branches 2, open 1, taken 1, "
        "abandoned 0\n"
    )


def test_index_on_a_malformed_file_prints_defects_and_exits_one(
    tmp_path, capsys
):
    status = main(["index", write(tmp_path, MALFORMED)])
    out = capsys.readouterr().out
    assert status == 1
    assert out == (
        "invalid atlas\n"
        "defect entry 2 is missing field label\n"
        "defect duplicate entry label first, entries 0 and 1\n"
    )


def test_index_prints_the_same_defects_as_the_atlas_command(
    tmp_path, capsys
):
    path = write(tmp_path, MALFORMED)
    assert main(["atlas", path]) == 1
    atlas_out = capsys.readouterr().out
    assert main(["index", path]) == 1
    index_out = capsys.readouterr().out
    assert index_out == atlas_out


def test_index_prints_no_index_line_for_a_malformed_atlas(
    tmp_path, capsys
):
    status = main(["index", write(tmp_path, MALFORMED)])
    out = capsys.readouterr().out
    assert status == 1
    for line in out.splitlines():
        assert line == "invalid atlas" or line.startswith("defect ")


def test_index_on_a_missing_file_exits_one(tmp_path, capsys):
    status = main(["index", str(tmp_path / "absent.json")])
    assert status == 1
    assert capsys.readouterr().out == (
        "invalid atlas\n"
        "defect file cannot be read\n"
    )
