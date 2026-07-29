"""Checks that every fixture count stated in the manual matches the directory it describes.

Twenty chapters open with a sentence of the form "The <Estimator> test suite contains N test
cases". Those sentences are prose, and prose does not move when a directory does. All twenty were
correct at 14.0.1 and four of them went stale the moment fixtures were added at the decision
boundaries, in a form that ships: the numbers are compiled into the PDF and rendered onto the
website under a version number.

Nothing in docs:ci compared the two, so the drift was invisible to the pipeline that exists to
catch exactly this. A count that has to track a directory is either generated from it or checked
against it; leaving it to be remembered is how it broke.

The sub-counts inside the parentheses are not checked. They partition the suite by an editorial
notion of case family that the filenames only loosely carry, and a checker that guessed at it
would fail for reasons that are not defects. The total is the claim a reader actually relies on.
"""

import re
import sys
from pathlib import Path

# "The $Foo$ test suite contains 206 test cases" and the one chapter that says
# "37 correctness test cases" instead.
PATTERN = re.compile(r"test suite contains (\d+) (?:correctness )?test cases")


def main(repo_root):
    root = Path(repo_root)
    failures = []
    checked = 0

    for chapter in sorted(root.glob("manual/*/*-tests.typ")):
        suite = chapter.parent.name
        match = PATTERN.search(chapter.read_text())
        if not match:
            # A chapter whose sentence stops matching drops out of this check silently, and the
            # check then reports success over a smaller set than it did the run before. The
            # directory decides: a chapter that describes a shared suite has to state its size.
            if (root / "tests" / suite).is_dir() and any((root / "tests" / suite).glob("*.json")):
                failures.append(
                    f"  {chapter.relative_to(root)}: tests/{suite} holds fixtures, but this chapter"
                    f" states no count in a form this check recognizes"
                )
            continue
        directory = root / "tests" / suite
        if not directory.is_dir():
            failures.append(f"  {chapter.relative_to(root)}: states a count but tests/{suite} does not exist")
            continue
        stated = int(match.group(1))
        actual = len(list(directory.glob("*.json")))
        checked += 1
        if stated != actual:
            failures.append(
                f"  {chapter.relative_to(root)}: says {stated} test cases, tests/{suite} holds {actual}"
            )

    if failures:
        print("ERROR: the manual states fixture counts that no longer match:", file=sys.stderr)
        print("\n".join(failures), file=sys.stderr)
        print(
            "\nThese numbers are compiled into the PDF and the website. Update the chapter, and\n"
            "the breakdown in its parentheses along with it.",
            file=sys.stderr,
        )
        return 1

    print(f"all {checked} stated fixture counts match their directories")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else "."))
