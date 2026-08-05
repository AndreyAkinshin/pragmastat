#!/usr/bin/env python3
"""Assert the coefficients compiled into the seven ports are the ones the fit scripts produce.

The manual says the coefficients are "reproducible rather than transcribed", and points at
tests/oracles/fit_exp.py and tests/oracles/fit_additive_cumulative.py as their source. Nothing checked
that. Both scripts printed their output and exited, no task ran them, and the constants live as
literals in seven files: an edit to a script, or to one port's literal, would have gone unnoticed
until somebody re-derived the fit by hand.

Every coefficient is compared as a binary64 payload, not as text, because the ports spell the same
double differently: Go writes 1.6086622436215554e-10, C# writes 1.6086622436215554E-10, and R
writes 1.6086622436215554e-10 with a different number of trailing digits in places.

The reference oracle's own self-check runs here too, for the same reason: it was given a failing
exit status and still nothing invoked it.
"""

import re
import subprocess
import sys
from pathlib import Path

# One port per language. The parser finds the Horner chain by its shape rather than by line
# numbers: an initializing assignment of the accumulator to a literal, then one step per
# coefficient, each multiplying the accumulator by the reduced argument and adding the next one.
SOURCES = {
    "exp": [
        ("go/exp_function.go", "q", "r"),
        ("py/pragmastat/exp_function.py", "q", "r"),
        ("rs/pragmastat/src/exp_function.rs", "q", "r"),
        ("ts/src/expFunction.ts", "q", "r"),
        ("kt/src/main/kotlin/dev/pragmastat/ExpFunction.kt", "q", "r"),
        ("cs/Pragmastat/Functions/ExpFunction.cs", "q", "r"),
        ("r/pragmastat/R/exp_function.R", "q", "r"),
    ],
    "additive": [
        ("go/additive_cumulative.go", "p", "u"),
        ("py/pragmastat/additive_cumulative.py", "p", "u"),
        ("rs/pragmastat/src/additive_cumulative.rs", "p", "u"),
        ("ts/src/additiveCumulative.ts", "p", "u"),
        ("kt/src/main/kotlin/dev/pragmastat/AdditiveCumulative.kt", "p", "u"),
        ("cs/Pragmastat/Functions/AdditiveCumulative.cs", "p", "u"),
        ("r/pragmastat/R/additive_cumulative.R", "p", "u"),
    ],
}

# The logarithm stores its polynomial as an array and walks it in a loop, so there is no Horner
# chain in the source to find. The guard covered only the two functions that happened to be
# written the other way, which is how a coefficient in a new function ends up checked by nothing.
ARRAYS = {
    "log": [
        ("go/log_function.go", "logPolyP"),
        ("py/pragmastat/log_function.py", "POLY_P"),
        ("rs/pragmastat/src/log_function.rs", "POLY_P"),
        ("ts/src/logFunction.ts", "POLY_P"),
        ("kt/src/main/kotlin/dev/pragmastat/LogFunction.kt", "LOG_POLY_P"),
        ("cs/Pragmastat/Functions/LogFunction.cs", "PolyP"),
        ("r/pragmastat/R/log_function.R", "LOG_POLY_P"),
    ],
}

# An exponent is optional: fit_exp.py prints every coefficient in scientific notation and
# fit_log.py prints shortest round-trip, where the leading terms have none. Requiring one
# silently matched nothing in the second case.
FLOAT = re.compile(r"[-+]?\d+\.\d+(?:[eE][-+]?\d+)?")


def run_fit(root: Path, script: str) -> list[list[float]]:
    """Run a fit script and return one list of coefficients per polynomial it prints."""
    python = root / "py" / ".venv" / "bin" / "python"
    interpreter = str(python) if python.exists() else sys.executable
    result = subprocess.run(
        [interpreter, str(root / "tests" / "oracles" / script)],
        capture_output=True,
        text=True,
        cwd=root,
    )
    if result.returncode != 0:
        print(f"ERROR: {script} exited {result.returncode}", file=sys.stderr)
        print(result.stderr, file=sys.stderr)
        raise SystemExit(1)
    chains = []
    for line in result.stdout.split("\n"):
        if line.startswith("POLY_"):
            chains.append([float(v) for v in FLOAT.findall(line)])
    return chains


def horner_coefficients(root: Path, rel: str, accumulator: str, argument: str) -> list[list[float]]:
    """One list of coefficients per Horner chain in a port, each in source order.

    A step is any line that assigns the accumulator from itself times the argument, plus or minus a
    literal: the seven ports spell the assignment differently (`=`, `<-`, `let mut`) and Go wraps
    the product in a float64() conversion, so the pattern anchors on the two variable names and the
    trailing literal rather than on the syntax around them.
    """
    literal = r"([-+]?\d+\.\d+(?:[eE][-+]?\d+)?)"
    step = re.compile(
        rf"^\s*{accumulator}\s*(?:=|<-)\s*.*\b{accumulator}\s*\*\s*(?:float64\()?{argument}"
        rf"[^\n]*?([-+])\s*{literal}\s*;?\s*$",
        re.M,
    )
    start = re.compile(
        rf"^\s*(?:let\s+(?:mut\s+)?|var\s+|val\s+|double\s+|const\s+)?{accumulator}\s*(?::?=|<-)\s*{literal}\s*;?\s*$",
        re.M,
    )

    chains: list[list[float]] = []
    for line in (root / rel).read_text().split("\n"):
        begin = start.match(line)
        if begin:
            chains.append([float(begin.group(1))])
            continue
        move = step.match(line)
        if move and chains:
            sign, value = move.group(1), move.group(2)
            chains[-1].append(float(value) if sign == "+" else -float(value))
    if not chains:
        print(f"ERROR: found no Horner chain for {accumulator} in {rel}", file=sys.stderr)
        raise SystemExit(1)
    return chains


def array_coefficients(root: Path, rel: str, name: str) -> list[float]:
    """The coefficients of an array-valued polynomial, in source order.

    Anchors on the declaration of `name` followed by an opening bracket, then reads every float
    literal until the bracket closes. The seven ports spell the declaration differently
    (`var x = [7]float64{`, `POLY_P = (`, `doubleArrayOf(`, `c(`, `[`), so the pattern keys on the
    name and the delimiters rather than on the syntax between them. The array lengths in Go's
    `[7]float64` and Rust's `[f64; 7]` are integers, which a float literal pattern cannot match.
    """
    text = (root / rel).read_text()
    match = re.search(rf"\b{re.escape(name)}\b\s*(?::[^=\n]*)?(?:=|<-)", text)
    if not match:
        print(f"ERROR: found no declaration of {name} in {rel}", file=sys.stderr)
        raise SystemExit(1)

    depth, coefficients, started = 0, [], False
    for token in re.finditer(r"[\[\](){}]|[-+]?\d+\.\d+(?:[eE][-+]?\d+)?", text[match.end():]):
        piece = token.group(0)
        if piece in "[({":
            depth += 1
            started = True
        elif piece in "])}":
            depth -= 1
            # Only a bracket pair that actually held literals ends the array. Go declares
            # `[7]float64{...}` and Rust `[f64; 7] = [...]`, where the first pair carries the
            # length and closes before the coefficients begin; stopping there read nothing and
            # reported an empty array rather than the wrong one.
            if started and depth <= 0 and coefficients:
                break
        elif started:
            coefficients.append(float(piece))
    if not coefficients:
        print(f"ERROR: {name} in {rel} declares no coefficients", file=sys.stderr)
        raise SystemExit(1)
    return coefficients


def compare_arrays(name: str, expected: list[list[float]], root: Path) -> list[str]:
    """Every port's array against the single polynomial the fit script prints."""
    failures = []
    if len(expected) != 1:
        return [f"  fit for {name} printed {len(expected)} polynomials, expected 1"]
    want = expected[0]
    for rel, symbol in ARRAYS[name]:
        found = array_coefficients(root, rel, symbol)
        if len(found) != len(want):
            failures.append(f"  {rel}: {symbol} has {len(found)} coefficients, the fit produces {len(want)}")
            continue
        for index, (a, b) in enumerate(zip(found, want)):
            if a != b:
                failures.append(f"  {rel}: {symbol}[{index}] is {a!r}, the fit produces {b!r}")
    return failures

def compare(name: str, expected: list[list[float]], root: Path) -> list[str]:
    failures = []
    for rel, accumulator, argument in SOURCES[name]:
        chains = horner_coefficients(root, rel, accumulator, argument)
        if len(chains) != len(expected):
            failures.append(f"  {rel}: {len(chains)} Horner chains, the fit produces {len(expected)}")
            continue
        for chain_index, (found, want) in enumerate(zip(chains, expected)):
            # The ports evaluate each chain from the highest degree down; the scripts print the
            # coefficients lowest first.
            if found and want and found[0] != want[0]:
                found = list(reversed(found))
            if len(found) != len(want):
                failures.append(
                    f"  {rel}: chain {chain_index} has {len(found)} coefficients, the fit produces {len(want)}"
                )
                continue
            for index, (a, b) in enumerate(zip(found, want)):
                if a != b:
                    failures.append(
                        f"  {rel}: chain {chain_index} coefficient {index} is {a!r}, the fit produces {b!r}"
                    )
    return failures


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()

    exp_coefficients = run_fit(root, "fit_exp.py")
    additive_coefficients = run_fit(root, "fit_additive_cumulative.py")
    log_coefficients = run_fit(root, "fit_log.py")

    failures = (
        compare("exp", exp_coefficients, root)
        + compare("additive", additive_coefficients, root)
        + compare_arrays("log", log_coefficients, root)
    )

    python = root / "py" / ".venv" / "bin" / "python"
    interpreter = str(python) if python.exists() else sys.executable
    oracle = subprocess.run(
        [interpreter, str(root / "tests" / "oracles" / "additive_reference.py")],
        capture_output=True,
        text=True,
        cwd=root,
    )
    if oracle.returncode != 0:
        failures.append("  tests/oracles/additive_reference.py: its own self-check failed")
        failures.append("    " + oracle.stdout.strip().replace("\n", "\n    "))

    if failures:
        print("ERROR: the shipped coefficients no longer match the scripts that produce them:", file=sys.stderr)
        print("\n".join(failures), file=sys.stderr)
        return 1

    total = (
        sum(len(c) for c in exp_coefficients)
        + sum(len(c) for c in additive_coefficients)
        + sum(len(c) for c in log_coefficients)
    )
    print(f"{total} coefficients match the fit scripts that produce them, in all seven ports")
    return 0


if __name__ == "__main__":
    sys.exit(main())
