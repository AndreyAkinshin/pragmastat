"""Fails when a manual source defines a math operator locally instead of in definitions.typ.

The website converter builds its symbol table from manual/definitions.yaml, so an operator
declared with `#let X = math.op("X")` inside a chapter is invisible to it. Typst honours the
local definition and sets the name upright; the website does not and sets it in italic maths.
Both outputs come from one source and quietly disagree.

That happened with MedianBounds, which rendered upright in the PDF and as eleven italic letters
on the page, beside a CenterBounds that was upright in both. Nothing failed, because the LaTeX
was valid.
"""

import re
import sys
from pathlib import Path

LOCAL_OP = re.compile(r"^\s*#let\s+(\w+)\s*=\s*math\.(op|underline|upright|bold|bb)\b", re.M)


def main(repo_root):
    root = Path(repo_root)
    shared = root / "manual" / "definitions.typ"
    failures = []

    for source in sorted(root.glob("manual/**/*.typ")):
        if source == shared:
            continue
        for match in LOCAL_OP.finditer(source.read_text()):
            failures.append(f"  {source.relative_to(root)}: #let {match.group(1)} = math.{match.group(2)}(...)")

    if failures:
        print("ERROR: math operators are defined outside manual/definitions.typ:", file=sys.stderr)
        print("\n".join(failures), file=sys.stderr)
        print(
            "\nThe website's converter reads its symbol table from manual/definitions.yaml, so a\n"
            "locally defined operator renders upright in the PDF and italic on the page. Move the\n"
            "definition into manual/definitions.typ and add the matching line to definitions.yaml.",
            file=sys.stderr,
        )
        return 1

    print("every math operator is defined in manual/definitions.typ")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else "."))
