"""Pins for the Round 3 example under examples.

The winter atlas holds three explorations of differing shapes in
one file, a tree carrying claims, a chain carrying none, and a
question with no branches at all, and its golden rendering has to
match the rendering the code produces today byte for byte. The
closing counts are also pinned against the counts counted by
hand from the example file, so the golden cannot drift with the
code.
"""

import pathlib
import re

from conftest import FORBIDDEN_WORDS
from myriad.atlas import load_atlas_file, render_atlas

EXAMPLES = pathlib.Path(__file__).resolve().parent.parent / "examples"


def load_atlas_example():
    atlas, defects = load_atlas_file(EXAMPLES / "winter-atlas.json")
    assert defects == []
    return atlas


def test_the_atlas_example_loads_clean_with_three_entries():
    atlas = load_atlas_example()
    assert atlas["title"] == "Winter questions for a small harbour"
    assert [entry["label"] for entry in atlas["entries"]] == [
        "ferry link", "lifeboat cover", "harbour lights",
    ]


def test_the_atlas_example_entries_differ_in_shape():
    atlas = load_atlas_example()
    ferry, lifeboat, lights = (
        entry["exploration"] for entry in atlas["entries"]
    )
    claimed = [b for b in ferry["branches"] if "claim" in b]
    assert len(ferry["branches"]) == 6
    assert len(claimed) == 3
    assert {b["claim"] for b in claimed} == {"boat crew", "harbour office"}
    assert len(lifeboat["branches"]) == 4
    assert not any("claim" in b for b in lifeboat["branches"])
    by_id = {b["id"]: b for b in lifeboat["branches"]}
    chain = [by_id["call"], by_id["launch"], by_id["sea"], by_id["back"]]
    assert [b["parent"] for b in chain] == [None, "call", "launch", "sea"]
    assert lights["branches"] == []


def test_the_atlas_golden_rendering_is_byte_identical():
    atlas = load_atlas_example()
    golden = (EXAMPLES / "winter-atlas.txt").read_bytes()
    rendered = ("\n".join(render_atlas(atlas)) + "\n").encode()
    assert rendered == golden


def test_the_atlas_golden_closes_with_the_hand_counted_counts():
    golden = (EXAMPLES / "winter-atlas.txt").read_text(encoding="utf-8")
    closing = golden.rstrip("\n").splitlines()[-1]
    assert closing == (
        "atlas counts, entries 3, branches 10, open 5, taken 3, abandoned 2"
    )


def test_the_atlas_golden_carries_no_ranking_or_verdict_word():
    golden = (EXAMPLES / "winter-atlas.txt").read_text(
        encoding="utf-8"
    ).lower()
    for word in FORBIDDEN_WORDS:
        assert not re.search(rf"\b{word}\b", golden), word
