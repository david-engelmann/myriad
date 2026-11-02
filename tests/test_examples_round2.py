"""Pins for the Round 2 example under examples.

The claimed winter ferry exploration is the Round 2 demo. It is
the Round 1 exploration with claims grown on some branches, held
in a file of its own so the Round 1 example and its golden map
stand untouched, and its golden map, claims listing, and census
have to match the renderings the code produces today byte for
byte.
"""

import json
import pathlib

from myriad.explorations import (
    load_exploration_file,
    render_census,
    render_claims,
    render_map,
)

EXAMPLES = pathlib.Path(__file__).resolve().parent.parent / "examples"


def load_claims_example():
    exploration, defects = load_exploration_file(
        EXAMPLES / "winter-ferry-claims.json"
    )
    assert defects == []
    return exploration


def rendered_bytes(lines):
    return ("\n".join(lines) + "\n").encode()


def test_the_claims_example_loads_clean_with_ten_claimed_branches():
    exploration = load_claims_example()
    claimed = [b for b in exploration["branches"] if "claim" in b]
    assert len(exploration["branches"]) == 26
    assert len(claimed) == 10
    assert {b["claim"] for b in claimed} == {
        "boat crew", "harbour office", "tide watch",
    }


def test_the_claims_example_is_the_round_1_exploration_plus_claims():
    round_1 = json.loads(
        (EXAMPLES / "winter-ferry.json").read_text(encoding="utf-8")
    )
    grown = json.loads(
        (EXAMPLES / "winter-ferry-claims.json").read_text(encoding="utf-8")
    )
    stripped = {
        "question": grown["question"],
        "branches": [
            {key: value for key, value in branch.items() if key != "claim"}
            for branch in grown["branches"]
        ],
    }
    assert stripped == round_1


def test_the_claims_golden_map_is_byte_identical():
    exploration = load_claims_example()
    golden = (EXAMPLES / "winter-ferry-claims-map.txt").read_bytes()
    assert rendered_bytes(render_map(exploration)) == golden


def test_the_claims_golden_listing_is_byte_identical():
    exploration = load_claims_example()
    golden = (EXAMPLES / "winter-ferry-claims-listing.txt").read_bytes()
    assert rendered_bytes(render_claims(exploration)) == golden


def test_the_claims_golden_census_is_byte_identical():
    exploration = load_claims_example()
    golden = (EXAMPLES / "winter-ferry-claims-census.txt").read_bytes()
    assert rendered_bytes(render_census(exploration)) == golden


def test_the_round_1_example_lists_as_wholly_unclaimed():
    exploration, defects = load_exploration_file(
        EXAMPLES / "winter-ferry.json"
    )
    assert defects == []
    assert render_claims(exploration) == [
        "question, How does a small harbour town keep its ferry link "
        "alive through the winter",
        "unclaimed, branches 26, open 16, taken 6, abandoned 4",
    ]
