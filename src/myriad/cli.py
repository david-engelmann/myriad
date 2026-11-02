"""The myriad command line surface.

Four commands read explorations from local files. The map command
prints the explorable map of the tree, the check command prints
the branch counts read from the tree, the claims command prints
the claims listing of the tree, and the census command prints the
territory census of the tree. Each command prints the defects of
a malformed exploration and exits with status one. Nothing here
touches the network.
"""

import argparse
import sys

from myriad.explorations import (
    count_by_state,
    load_exploration_file,
    render_census,
    render_claims,
    render_map,
)


def _load_or_print_defects(path):
    exploration, defects = load_exploration_file(path)
    if defects:
        print("invalid exploration")
        for defect in defects:
            print(f"defect {defect}")
        return None
    return exploration


def _command_map(args):
    exploration = _load_or_print_defects(args.file)
    if exploration is None:
        return 1
    for line in render_map(exploration):
        print(line)
    return 0


def _command_check(args):
    exploration = _load_or_print_defects(args.file)
    if exploration is None:
        return 1
    counts = count_by_state(exploration)
    print(f"branches {counts['total']}")
    print(f"open {counts['open']}")
    print(f"taken {counts['taken']}")
    print(f"abandoned {counts['abandoned']}")
    return 0


def _command_claims(args):
    exploration = _load_or_print_defects(args.file)
    if exploration is None:
        return 1
    for line in render_claims(exploration):
        print(line)
    return 0


def _command_census(args):
    exploration = _load_or_print_defects(args.file)
    if exploration is None:
        return 1
    for line in render_census(exploration):
        print(line)
    return 0


def main(argv=None):
    """Run the myriad command line and return the exit status."""
    parser = argparse.ArgumentParser(
        prog="myriad",
        description="Fork one hard question into branches and map them.",
    )
    commands = parser.add_subparsers(dest="command", required=True)
    map_parser = commands.add_parser("map", help="print the map of a tree")
    map_parser.add_argument("file")
    map_parser.set_defaults(handler=_command_map)
    check_parser = commands.add_parser(
        "check", help="print the branch counts of a tree"
    )
    check_parser.add_argument("file")
    check_parser.set_defaults(handler=_command_check)
    claims_parser = commands.add_parser(
        "claims", help="print the claims listing of a tree"
    )
    claims_parser.add_argument("file")
    claims_parser.set_defaults(handler=_command_claims)
    census_parser = commands.add_parser(
        "census", help="print the territory census of a tree"
    )
    census_parser.add_argument("file")
    census_parser.set_defaults(handler=_command_census)
    args = parser.parse_args(argv)
    return args.handler(args)


if __name__ == "__main__":
    sys.exit(main())
