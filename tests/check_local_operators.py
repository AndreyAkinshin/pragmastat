"""Fails when a manual source defines a math operator locally instead of in definitions.typ.

The website converter builds its symbol table from manual/definitions.yaml, so an operator
declared with `#let X = math.op("X")` inside a chapter is invisible to it. Typst honours the
local definition and sets the name upright; the website does not and sets it in italic maths.
Both outputs come from one source and quietly disagree.

That happened with MedianBounds, which rendered upright in the PDF and as eleven italic letters
on the page, beside a CenterBounds that was upright in both. Nothing failed, because the LaTeX
was valid.

There are two routes to that state and the check covers both. One is defining the operator inside
a chapter, where the converter never looks. The other is defining it in definitions.typ and not in
definitions.yaml: the PDF reads the first file and the website reads the second, so an operator
present in one and missing from the other produces the same silent divergence.
"""

import re
import sys
from pathlib import Path

LOCAL_OP = re.compile(r"^\s*#let\s+(\w+)\s*=\s*math\.(op|underline|upright|bold|bb)\b", re.M)
SHARED_OP = re.compile(r"^#let\s+(\w+)\s*=\s*math\.(?:op|underline|upright|bold|bb)\b", re.M)
YAML_ENTRY = re.compile(r"^([A-Za-z]\w*)\s*:", re.M)

# Only one direction is a defect. An operator in definitions.typ with no macro in definitions.yaml
# renders upright in the PDF and italic on the page, which is the divergence this file exists to
# stop. The reverse, a macro with no operator, is a dead entry: Typst fails loudly the moment
# anything uses the name, so nothing can ship in that state.


def main(repo_root):
    root = Path(repo_root)
    shared = root / "manual" / "definitions.typ"
    macros = root / "manual" / "definitions.yaml"
    failures = []

    for source in sorted(root.glob("manual/**/*.typ")):
        if source == shared:
            continue
        for match in LOCAL_OP.finditer(source.read_text()):
            failures.append(f"  {source.relative_to(root)}: #let {match.group(1)} = math.{match.group(2)}(...)")

    # The PDF renders from definitions.typ and the page from definitions.yaml.
    typst_ops = set(SHARED_OP.findall(shared.read_text()))
    yaml_ops = set(YAML_ENTRY.findall(macros.read_text()))
    for name in sorted(typst_ops - yaml_ops):
        failures.append(f"  manual/definitions.yaml: no macro for {name}, which definitions.typ defines")

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
