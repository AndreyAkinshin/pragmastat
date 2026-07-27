"""Compares two runs of the Go conformance probe and enforces the manifest's classes.

The probe prints one tagged line per estimator evaluation with every float written as an
exact binary64 payload. `mise run tests:check:conformance` runs it twice, once normally and
once against a copy of the package whose calls to log, exp, pow and cos each return the
neighbouring representable value. That single ulp is the smallest difference two conforming
implementations of those functions can legitimately have, so it stands in for the whole
space of libraries the seven ports may be built against.

An estimator whose output never moves under that perturbation cannot diverge for this
reason in any language. That is a proof rather than an observation, and it is what the
`exact` class in tests/manifest.json asserts. This script fails the build when the two
disagree.

sqrt is deliberately not perturbed: IEEE 754 requires it correctly rounded, so every
conforming implementation already returns the same bits, and perturbing it would overstate
the exposure.
"""

import json
import struct
import sys
from collections import Counter
from pathlib import Path

# Probe tag -> manifest suite. misrateFloor has no suite of its own: it is the domain of
# every bounds estimator, so a language that computes it differently accepts a different
# set of inputs, which is at least as bad as computing a different answer.
SUITE_OF = {
    "center": "center",
    "spread": "spread",
    "shift": "shift",
    "ratio": "ratio",
    "disparity": "disparity",
    "avgSpread": "avg-spread",
    "centerBounds": "center-bounds",
    "spreadBounds": "spread-bounds",
    "shiftBounds": "shift-bounds",
    "ratioBounds": "ratio-bounds",
    "disparityBounds": "disparity-bounds",
    "pairwiseMargin": "pairwise-margin",
    "signedRankMargin": "signed-rank-margin",
    "misrateFloor": None,
}

def known_exposure(estimator, key):
    """True for the one soft spot the manual already states, and only there.

    SpreadBounds is labelled exact because it matches bitwise on every fixture, but it is
    not exact by construction: signMargin inverts the binomial distribution function in log
    space, so at the one-sample misrate floor 2^(1-floor(n/2)) an exact tie is settled by
    rounding inside the logarithm.

    The allowance is a predicate rather than a count on purpose. A tolerated number would
    drift with the size of the sweep and would quietly absorb a second, unrelated exposure;
    this accepts movement at exactly the point that is documented and fails anywhere else.
    Deleting this function is the point of the follow-up that takes signMargin out of log
    space; widening it is not.
    """
    if estimator != "spreadBounds":
        return False
    case = key.split("/")[0]
    try:
        size = int(case.split(".")[0].lstrip("n"))
        misrate = payload(case.split(".m", 1)[1])
    except (IndexError, ValueError):
        return False
    return misrate == 2.0 ** (1 - size // 2)


def payload(token):
    """Parses the 'MpE' form Go's strconv emits for 'b', or returns None for an integer."""
    if "p" not in token:
        return None
    mantissa, exponent = token.split("p")
    return float(int(mantissa)) * (2.0 ** int(exponent))


def ulps(a, b):
    ia = struct.unpack("<q", struct.pack("<d", a))[0]
    ib = struct.unpack("<q", struct.pack("<d", b))[0]
    if ia < 0:
        ia = -(ia & 0x7FFFFFFFFFFFFFFF)
    if ib < 0:
        ib = -(ib & 0x7FFFFFFFFFFFFFFF)
    return abs(ia - ib)


def load(path):
    rows = {}
    for line in Path(path).read_text().splitlines():
        if "\t" not in line:
            continue
        fields = line.split("\t")
        # Every probe tag is "<case>/<estimator>". The go test runner's own summary line is
        # tab-separated too, so the shape is what tells them apart.
        if "/" not in fields[0]:
            continue
        rows[fields[0]] = fields[1:]
    return rows


def main(base_path, perturbed_path, manifest_path):
    base = load(base_path)
    perturbed = load(perturbed_path)
    if not base:
        sys.exit("conformance: the baseline probe produced no rows")

    manifest = json.loads(Path(manifest_path).read_text())
    classes = {name: s.get("conformance", "exact") for name, s in manifest["suites"].items()}

    # A key present on only one side means the perturbation changed which inputs are
    # admissible at all, which is a divergence in the domain rather than in the value.
    only_one_side = set(base) ^ set(perturbed)

    total = Counter()
    moved = Counter()
    worst = Counter()
    tolerated = Counter()
    for key in set(base) & set(perturbed):
        estimator = key.split("/")[-1]
        total[estimator] += 1
        if base[key] == perturbed[key]:
            continue
        if known_exposure(estimator, key):
            tolerated[estimator] += 1
            continue
        moved[estimator] += 1
        for a, b in zip(base[key], perturbed[key]):
            if a == b:
                continue
            pa, pb = payload(a), payload(b)
            worst[estimator] = max(worst[estimator], ulps(pa, pb) if pa is not None and pb is not None else -1)
    for key in only_one_side:
        estimator = key.split("/")[-1]
        total[estimator] += 1
        if known_exposure(estimator, key):
            tolerated[estimator] += 1
        else:
            moved[estimator] += 1
            worst[estimator] = max(worst[estimator], -1)

    print(f"{'estimator':<20}{'class':>13}{'probed':>9}{'moved':>8}{'max ulp':>10}{'known':>8}")
    failures = []
    for estimator in sorted(total):
        suite = SUITE_OF.get(estimator)
        cls = "exact" if suite is None else classes.get(suite, "exact")
        w = worst[estimator]
        shown = "domain" if w == -1 else (str(w) if moved[estimator] else "-")
        known = str(tolerated[estimator]) if tolerated[estimator] else "-"
        print(f"{estimator:<20}{cls:>13}{total[estimator]:>9}{moved[estimator]:>8}{shown:>10}{known:>8}")
        if cls == "exact" and moved[estimator]:
            failures.append(
                f"  {estimator}: declared exact in tests/manifest.json, but a one-ulp "
                f"perturbation of the library functions moved it on {moved[estimator]} of "
                f"{total[estimator]} probes"
            )

    if failures:
        sys.stdout.flush()
        print("\nERROR: an estimator does not hold the class it declares:", file=sys.stderr)
        print("\n".join(failures), file=sys.stderr)
        print(
            "\nEither the estimator has acquired a dependency on a library function the\n"
            "specification does not fix, or its class in tests/manifest.json is wrong.\n"
            "Both are worth knowing; neither is fixed by widening the tolerance.",
            file=sys.stderr,
        )
        return 1

    print("\nevery estimator holds the conformance class it declares")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1], sys.argv[2], sys.argv[3]))
