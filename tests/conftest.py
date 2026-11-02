import pathlib
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent.parent / "src"))


# The words no rendering in this repo may print, held in one place
# so every vocabulary scan in the tests reads the same list.
FORBIDDEN_WORDS = (
    "winner", "verdict", "vote", "rank", "ranked", "ranking", "best",
    "worst", "largest", "smallest", "most", "least", "beats",
    "versus", "compared", "comparison", "top", "leading",
)
