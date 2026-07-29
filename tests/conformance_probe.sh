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

// Each of these returns the neighbor of the true result, modeling another implementation
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

# Every library call the specification leaves unfixed must be in the substitution list above, or
# the estimator that reaches it is measured as not moving and certified `exact` on that basis.
# The list is four names; this asserts there is no fifth. math.Sqrt is exempt because IEEE 754
# requires it correctly rounded, and the arithmetic helpers below are exact by construction:
# Abs and Copysign move bits, Floor/Ceil/Trunc/Round land on integers, Inf/NaN/IsNaN/IsInf and
# the Float64bits pair are not computations, Ldexp/Frexp scale by a power of two, and
# Nextafter returns a neighbor rather than computing one.
exact_by_construction='Sqrt|Sqrt2|Abs|Copysign|Signbit|Floor|Ceil|Trunc|Round|Mod|Inf|NaN|IsNaN|IsInf|Float64bits|Float64frombits|Float32bits|Float32frombits|Ldexp|Frexp|Max|Min|MaxInt[0-9]*|MinInt[0-9]*|MaxFloat[0-9]*|MaxUint[0-9]*|SmallestNonzeroFloat[0-9]*|Pi|Ln2|E|Nextafter'
# Only CALLS matter, so the pattern requires the opening parenthesis: the four substituted names
# still appear in prose, in the comments that explain why they are substituted. The file defining
# the wrappers is skipped for the same reason.
stray="$(grep -rhoE 'math[.][A-Z][A-Za-z0-9]*[(]' "$work" --include='*.go' \
  --exclude='zz_perturb.go' \
  | sed 's/($//;s/(//' \
  | grep -vE "math[.]($exact_by_construction)$" | sort -u || true)"
if [ -n "$stray" ]; then
  echo "ERROR: the perturbation covers four library functions and these are not among them:" >&2
  echo "$stray" >&2
  echo "An estimator reaching one of these is certified exact without that call being perturbed." >&2
  echo "Add it to the substitution list above, or to exact_by_construction with a reason." >&2
  exit 1
fi

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
