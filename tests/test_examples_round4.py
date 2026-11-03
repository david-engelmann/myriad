"""Pins for the Round 4 example under examples.

The golden index of the winter atlas has to match the index
reading the code produces today byte for byte, and the entry
lines and closing counts are also pinned against the counts
counted by hand from the example file, so the golden cannot
drift with the code. The winter atlas itself does not move in
this round, and its Round 3 golden still has to match it.
"""

import pathlib
import re

from conftest import FORBIDDEN_WORDS
from myriad.atlas import load_atlas_file, render_atlas, render_index

EXAMPLES = pathlib.Path(__file__).resolve().parent.parent / "examples"


def load_atlas_example():
    atlas, defects = load_atlas_file(EXAMPLES / "winter-atlas.json")
    assert defects == []
    return atlas


def test_the_index_golden_matches_the_reading_byte_for_byte():
    golden = (EXAMPLES / "winter-atlas-index.txt").read_text(
        encoding="utf-8"
    )
    lines = render_index(load_atlas_example())
    assert "\n".join(lines) + "\n" == golden


def test_the_index_golden_lines_counted_by_hand():
    golden = (EXAMPLES / "winter-atlas-index.txt").read_text(
        encoding="utf-8"
    )
    assert golden.splitlines() == [
        "atlas, Winter questions for a small harbour",
        "entry ferry link, question, How does the harbour keep a "
        "ferry sailing through the winter, branches 6, open 3, "
        "taken 2, abandoned 1",
        "entry lifeboat cover, question, Who answers a call for "
        "help on the water in winter, branches 4, open 2, taken 1, "
        "abandoned 1",
        "entry harbour lights, question, Which harbour lights stay "
        "lit through the winter nights, branches 0, open 0, "
        "taken 0, abandoned 0",
        "atlas counts, entries 3, branches 10, open 5, taken 3, "
        "abandoned 2",
    ]


def test_the_index_opens_and_closes_with_the_atlas_golden_lines():
    atlas = load_atlas_example()
    atlas_golden = (EXAMPLES / "winter-atlas.txt").read_text(
        encoding="utf-8"
    ).splitlines()
    index = render_index(atlas)
    assert index[0] == atlas_golden[0]
    assert index[-1] == atlas_golden[-1]


def test_the_empty_exploration_carries_the_zero_counts_in_the_index():
    index = render_index(load_atlas_example())
    lights = [
        line for line in index if line.startswith("entry harbour lights")
    ]
    assert lights == [
        "entry harbour lights, question, Which harbour lights stay "
        "lit through the winter nights, branches 0, open 0, "
        "taken 0, abandoned 0"
    ]


def test_the_winter_atlas_still_matches_its_round_three_golden():
    golden = (EXAMPLES / "winter-atlas.txt").read_text(encoding="utf-8")
    lines = render_atlas(load_atlas_example())
    assert "\n".join(lines) + "\n" == golden


def test_the_index_golden_holds_to_the_vocabulary_bar():
    golden = (EXAMPLES / "winter-atlas-index.txt").read_text(
        encoding="utf-8"
    )
    for word in FORBIDDEN_WORDS:
        assert not re.search(rf"\b{word}\b", golden), word


def test_the_index_reading_holds_to_the_vocabulary_bar():
    joined = "\n".join(render_index(load_atlas_example()))
    for word in FORBIDDEN_WORDS:
        assert not re.search(rf"\b{word}\b", joined), word
