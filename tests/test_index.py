"""Pins for the atlas index reading of MY-9.

The index is the atlas as a table of contents. Its first line is
the title line of the atlas rendering and its last line is the
atlas counts line of the atlas rendering, and one line stands for
each entry between them, carrying the label, the question, and
the counts of that entry alone, read from the one counting path.
"""

import re

from conftest import FORBIDDEN_WORDS
from myriad.atlas import render_atlas, render_index
from myriad.explorations import render_census


def branch(id, state, parent=None, prompt=None, **extra):
    made = {
        "id": id,
        "parent": parent,
        "prompt": prompt if prompt is not None else f"prompt of {id}",
        "state": state,
    }
    made.update(extra)
    return made


def exploration(question, branches):
    return {"question": question, "branches": branches}


def entry(label, question, branches):
    return {"label": label, "exploration": exploration(question, branches)}


SOUND = {
    "title": "the title",
    "entries": [
        entry("first", "q one", [
            branch("a", "taken", finding="f", claim="crew"),
            branch("b", "open", parent="a"),
        ]),
        entry("second", "q two", []),
        entry("third", "q three", [
            branch("c", "abandoned", reason="r"),
            branch("d", "abandoned", reason="r2"),
            branch("e", "open"),
            branch("f", "taken", finding="f2"),
            branch("g", "open"),
        ]),
    ],
}


def test_the_index_of_a_sound_atlas_line_for_line():
    assert render_index(SOUND) == [
        "atlas, the title",
        "entry first, question, q one, "
        "branches 2, open 1, taken 1, abandoned 0",
        "entry second, question, q two, "
        "branches 0, open 0, taken 0, abandoned 0",
        "entry third, question, q three, "
        "branches 5, open 2, taken 1, abandoned 2",
        "atlas counts, entries 3, branches 7, open 3, taken 2, "
        "abandoned 2",
    ]


def test_the_index_opens_and_closes_with_the_atlas_rendering_lines():
    rendering = render_atlas(SOUND)
    index = render_index(SOUND)
    assert index[0] == rendering[0]
    assert index[-1] == rendering[-1]


def test_one_entry_line_stands_for_each_entry_in_file_order():
    atlas = {
        "title": "t",
        "entries": [
            entry("wide", "q wide", [
                branch(f"w{i}", "open") for i in range(9)
            ]),
            entry("narrow", "q narrow", [branch("n", "open")]),
            entry("middle", "q middle", [
                branch(f"m{i}", "open") for i in range(4)
            ]),
        ],
    }
    index = render_index(atlas)
    assert len(index) == len(atlas["entries"]) + 2
    labels = [
        line.split(", question,")[0] for line in index[1:-1]
    ]
    assert labels == ["entry wide", "entry narrow", "entry middle"]


def test_each_entry_line_counts_its_own_entry_alone():
    for line, item in zip(render_index(SOUND)[1:-1], SOUND["entries"]):
        census_closing = render_census(item["exploration"])[-1]
        assert line.endswith(census_closing)


def test_the_entry_counts_sum_to_the_closing_counts():
    index = render_index(SOUND)
    totals = [0, 0, 0, 0]
    for line in index[1:-1]:
        counts = re.search(
            r"branches (\d+), open (\d+), taken (\d+), "
            r"abandoned (\d+)$",
            line,
        )
        for place in range(4):
            totals[place] += int(counts.group(place + 1))
    closing = re.search(
        r"branches (\d+), open (\d+), taken (\d+), abandoned (\d+)$",
        index[-1],
    )
    assert [int(closing.group(place + 1)) for place in range(4)] == totals


def test_an_entry_with_no_branches_carries_the_zero_counts():
    atlas = {"title": "t", "entries": [entry("empty", "q", [])]}
    assert render_index(atlas) == [
        "atlas, t",
        "entry empty, question, q, "
        "branches 0, open 0, taken 0, abandoned 0",
        "atlas counts, entries 1, branches 0, open 0, taken 0, "
        "abandoned 0",
    ]


def test_an_atlas_with_no_entries_renders_two_lines():
    atlas = {"title": "t", "entries": []}
    assert render_index(atlas) == [
        "atlas, t",
        "atlas counts, entries 0, branches 0, open 0, taken 0, "
        "abandoned 0",
    ]


def test_the_index_carries_no_branch_claim_or_prompt_text():
    atlas = {
        "title": "t",
        "entries": [
            entry("only", "the question", [
                branch(
                    "a", "taken",
                    prompt="a distinctive prompt",
                    finding="a distinctive finding",
                    claim="a distinctive claim",
                ),
            ]),
        ],
    }
    lines = render_index(atlas)
    assert len(lines) == 3
    joined = "\n".join(lines)
    assert "a distinctive prompt" not in joined
    assert "a distinctive finding" not in joined
    assert "a distinctive claim" not in joined
    assert "claimed by" not in joined


def test_the_index_of_a_wide_atlas_renders_whole():
    atlas = {
        "title": "t",
        "entries": [
            entry(f"e{i}", f"q {i}", [
                branch(f"b{i}-{j}", "open") for j in range(400)
            ])
            for i in range(3)
        ],
    }
    lines = render_index(atlas)
    assert len(lines) == 5
    assert lines[-1] == (
        "atlas counts, entries 3, branches 1200, open 1200, "
        "taken 0, abandoned 0"
    )


def test_the_index_reading_holds_to_the_vocabulary_bar():
    joined = "\n".join(render_index(SOUND))
    for word in FORBIDDEN_WORDS:
        assert not re.search(rf"\b{word}\b", joined), word
