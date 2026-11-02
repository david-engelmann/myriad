"""Setup pins. Each test can fail for a real reason against a repo that
lost a setup file or let its status drift from its contract."""
import pathlib, re
ROOT = pathlib.Path(__file__).resolve().parent.parent

def test_agents_carries_the_canonical_marker():
    text = (ROOT / "AGENTS.md").read_text(encoding="utf-8")
    assert "<!-- repo annex below this line -->" in text
    assert "39 repo storefront perfection program" in text

def test_readme_states_the_closed_round_count():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    assert re.search(r"Closed perfection rounds, 2 of 100", text)

def test_contract_carries_the_setup_obligations():
    text = (ROOT / "CONTRACT.md").read_text(encoding="utf-8")
    for oid in ("SETUP-1", "SETUP-2", "SETUP-3", "SETUP-4", "MY-1"):
        assert oid in text, oid

def test_readme_reports_the_contract_status():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    expected = (
        "- Contract, SETUP-1 to SETUP-4 and MY-1 to MY-4 met. "
        "MY-1 to MY-4 stand in full form in CONTRACT.md with the Round 1 "
        "repair amendments, and Round 1 closed on the merged slice 2 after "
        "three reviews, a repair pass, and an approving re-review."
    )
    assert expected in text.splitlines()

def test_readme_reports_the_round_two_contract_status():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    expected = (
        "- Contract, MY-5 and MY-6 met. "
        "They stand in full form in CONTRACT.md with the Round 2 "
        "repair amendments, and Round 2 closed on the merged slice 3 after "
        "three reviews, two repair passes, and two re-reviews, the second "
        "approving with no findings."
    )
    assert expected in text.splitlines()

def test_readme_reports_the_round_three_contract_status():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    expected = (
        "- Contract, MY-7 and MY-8 now stand in full form in "
        "CONTRACT.md under the Round 3 obligations, and their "
        "standing is not claimed here, as Round 3 has not closed."
    )
    assert expected in text.splitlines()

def test_readme_reports_the_slice_four_standing():
    text = (ROOT / "README.md").read_text(encoding="utf-8")
    expected = (
        "- Slice 4 is built and in review. It carries the atlas "
        "loader that loads a file of labelled explorations only "
        "whole and names the entry standing behind every defect, "
        "and the atlas rendering that walks the entries in file "
        "order with each section holding the map and the claim "
        "lines of its exploration and the closing counts read over "
        "the union of every entry, with a synthetic example atlas "
        "of three explorations of differing shapes and its golden "
        "rendering under examples."
    )
    assert expected in text.splitlines()
    assert text.count("built and in review") == 1


def test_ci_runs_the_same_gate_as_the_local_gate():
    text = (ROOT / ".github" / "workflows" / "ci.yml").read_text(encoding="utf-8")
    live = "\n".join(
        line.split("#", 1)[0]
        for line in text.splitlines()
        if not line.lstrip().startswith("#")
    )
    assert "python3 scripts/codepoint_scan.py" in live
    assert "python3 -m pytest tests -q" in live
