#!/usr/bin/env python3
"""Fail when a British spelling appears in prose the project writes in American English.

Every language port, the manual, and the test scaffolding are written in one variety, and a sweep
of main for the fifteen British forms listed below returns nothing at all. The consistency is not
decorative: a reader grepping for "normalize" should find every place that normalizes, and a
contributor copying a nearby comment should not have to guess which variety that file uses.
Model-written prose drifts toward British forms without anyone choosing them, which is exactly how
the fifteen this check first caught got in.

The file list comes from `git ls-files`, not from walking the tree. Walking picked up build
artifacts: r:check and r:test copy the whole tests/ directory into r/pragmastat/tests/tests and
leave an R CMD check tree in r/pragmastat.Rcheck, so the second `mise run ci` on any machine
failed on a copy of THIS file, whose table of forbidden forms is itself a list of British
spellings. Tracked files are the set the rule is about, and asking git for them skips every
generated tree for free.
"""

import re
import subprocess
import sys
from pathlib import Path

# left: the British form as a regex; right: what to write instead. Matched without a word boundary
# on the left, because the ones that got through a boundary-anchored sweep were the compounds:
# "denormalise", "subnormalised", "mislabelled".
#
# "analyse" carries a right boundary and its inflections are listed separately. Dropping the
# boundary flags "analyses", which is the American plural of "analysis", and the message then tells
# the author to write "analyzes", a different word.
FORMS = [
    (r"neighbour", "neighbor"),
    (r"centre", "center"),
    (r"normalis", "normaliz"),
    (r"behaviour", "behavior"),
    (r"analyse\b", "analyze"),
    (r"analysing", "analyzing"),
    (r"analysed", "analyzed"),
    (r"initialis", "initializ"),
    (r"modelling", "modeling"),
    (r"labelled", "labeled"),
    (r"cancelled", "canceled"),
    (r"practise", "practice"),
    (r"licence", "license"),
    (r"defence", "defense"),
    (r"judgement", "judgment"),
]

# Directories whose prose the rule covers. web/src carries the site's own copy; README.md and the
# per-port AGENTS.md files are prose too, and were outside the sweep until this list included them.
SEARCHED = ("manual", "tests", "tools/src", "go", "rs", "py", "ts", "kt", "cs", "r", "web/src")
SEARCHED_FILES = ("README.md", "AGENTS.md")

# Fixture payloads are data, not prose, so they are exempt by PATH rather than by suffix.
# tests/manifest.json is neither: it holds the conformance paragraphs the manual quotes, and a
# suffix rule aimed at fixtures was hiding 13 KB of English.
FIXTURE_DIRS = ("cs/tests/",)
SKIPPED_SUFFIXES = {".pdf", ".png", ".svg", ".ico", ".lock", ".woff", ".woff2", ".ttf"}
# references.yaml quotes published titles; this file has to spell the forms it forbids.
SKIPPED_FILES = {"manual/references.yaml", "tests/check_spelling.py"}

# Tracked files that are not text. Named rather than detected, because "cannot decode it" is also
# what a text file with a broken encoding looks like, and that one has to be a finding.
BINARY_FILES = {
    "cs/pragmastatKey.snk",
    "kt/gradle/wrapper/gradle-wrapper.jar",
}

# How many files the sweep reads after the skips above. Pinned because the check reports success by
# finding nothing, and reading nothing looks exactly the same: an empty `git ls-files`, a bad root
# argument, or a widened skip rule would all have printed "spelling is consistent" and exited 0.
EXPECTED_FILES = 868


def is_fixture(rel: str) -> bool:
    if rel.startswith(FIXTURE_DIRS):
        return True
    # tests/<suite>/*.json are generated fixtures; tests/*.json (the manifest) is prose.
    return rel.startswith("tests/") and rel.endswith(".json") and rel.count("/") > 1


def tracked_files(root: Path) -> list[str]:
    result = subprocess.run(
        ["git", "-C", str(root), "ls-files", "--", *SEARCHED, *SEARCHED_FILES],
        capture_output=True,
        text=True,
        check=True,
    )
    return [line for line in result.stdout.split("\n") if line]


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    pattern = re.compile("|".join(f"(?P<g{i}>{b})" for i, (b, _) in enumerate(FORMS)), re.IGNORECASE)

    tracked = tracked_files(root)
    if not tracked:
        print(f"git ls-files listed nothing under {root}; the sweep read no files", file=sys.stderr)
        return 1

    findings = []
    unreadable = []
    read = 0
    for rel in tracked:
        if rel in SKIPPED_FILES or is_fixture(rel) or Path(rel).suffix in SKIPPED_SUFFIXES:
            continue
        try:
            text = (root / rel).read_text()
        except (UnicodeDecodeError, OSError) as error:
            if rel not in BINARY_FILES:
                unreadable.append((rel, error))
            continue
        read += 1
        for number, line in enumerate(text.split("\n"), start=1):
            match = pattern.search(line)
            if match:
                index = int(match.lastgroup[1:])
                findings.append((rel, number, match.group(0), FORMS[index][1]))

    if findings:
        print("British spellings found; this project writes American English:", file=sys.stderr)
        for rel, number, found, want in findings:
            print(f"  {rel}:{number}: {found!r} should be {want!r}", file=sys.stderr)
        return 1

    if unreadable:
        print("tracked files the sweep could not read:", file=sys.stderr)
        for rel, error in unreadable:
            print(f"  {rel}: {error}", file=sys.stderr)
        print("add a genuinely binary file to BINARY_FILES; fix the encoding otherwise", file=sys.stderr)
        return 1

    if read != EXPECTED_FILES:
        print(f"the sweep read {read} files, expected {EXPECTED_FILES}", file=sys.stderr)
        print("re-measure and update EXPECTED_FILES if the tree legitimately changed", file=sys.stderr)
        return 1

    print(f"spelling is consistent across {read} files")
    return 0


if __name__ == "__main__":
    sys.exit(main())
