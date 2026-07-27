#!/usr/bin/env bash
# Runs the Go conformance probe twice, once against the package and once against a copy
# whose library calls are perturbed by one unit in the last place, and asks
# tests/conformance_compare.py whether every estimator held the class it declares.
#
# Go stands in for all seven implementations. The question the probe answers is whether an
# estimator depends on a library function the specification does not fix, and that is a
# property of the algorithm rather than of the language.
#
# The perturbation is applied to a copy, never to the package: an estimator should not carry
# an injection seam for the benefit of its own test.
set -euo pipefail

repo="$(cd "$(dirname "$0")/.." && pwd)"
work="$(mktemp -d)"
cache="$(mktemp -d)"
trap 'rm -rf "$work" "$cache"' EXIT
export GOCACHE="$cache"

cp "$repo"/go/*.go "$repo"/go/go.mod "$work/"
PRAGMASTAT_PROBE_OUT="$work/base.txt" bash -c 'cd "$1" && go test -tags conformance -run TestConformanceProbe -count=1 . >/dev/null' _ "$work"

cat > "$work/zz_perturb.go" <<'PERTURB'
package pragmastat

import "math"

// Each of these returns the neighbour of the true result, modelling another implementation
// of the same function that rounds the other way in the last bit. sqrt is deliberately
// absent: IEEE 754 requires it correctly rounded, so every conforming implementation
// already returns the same bits and perturbing it would overstate the exposure.
func perturbUp(v float64) float64 {
	if math.IsNaN(v) || math.IsInf(v, 0) {
		return v
	}
	return math.Nextafter(v, math.Inf(1))
}

func perturbedLog(x float64) float64    { return perturbUp(math.Log(x)) }
func perturbedExp(x float64) float64    { return perturbUp(math.Exp(x)) }
func perturbedPow(x, y float64) float64 { return perturbUp(math.Pow(x, y)) }
func perturbedCos(x float64) float64    { return perturbUp(math.Cos(x)) }
PERTURB

for f in "$work"/*.go; do
  case "$f" in *zz_perturb.go) continue ;; esac
  sed -i -e 's/math[.]Log(/perturbedLog(/g' \
         -e 's/math[.]Exp(/perturbedExp(/g' \
         -e 's/math[.]Pow(/perturbedPow(/g' \
         -e 's/math[.]Cos(/perturbedCos(/g' "$f"
done

# A file that lost its last math reference no longer needs the import.
for _ in 1 2 3 4 5 6; do
  out="$( cd "$work" && go build -tags conformance ./... 2>&1 || true )"
  [ -z "$out" ] && break
  echo "$out" | grep -oE '[a-z_0-9]+[.]go' | sort -u | while read -r f; do
    [ -f "$work/$f" ] && sed -i -e '/^[[:space:]]*"math"$/d' -e '/^import "math"$/d' "$work/$f"
  done
done

PRAGMASTAT_PROBE_OUT="$work/perturbed.txt" bash -c 'cd "$1" && go test -tags conformance -run TestConformanceProbe -count=1 . >/dev/null' _ "$work"

python3 "$repo/tests/conformance_compare.py" "$work/base.txt" "$work/perturbed.txt" "$repo/tests/manifest.json"
