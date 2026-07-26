package pragmastat

import "math"

// Every product below is wrapped in float64(), and none of those conversions is
// redundant. The Go specification lets an implementation fuse a multiply into a
// following add as a single rounding, and gc does exactly that on arm64,
// ppc64le, s390x, riscv64 and loong64 while never doing it on amd64, where CI
// runs. A fused Horner step lands a fraction of an ULP away from the same step
// in the other six languages, none of which fuse, and gaussCdf feeds threshold
// comparisons that turn a last-bit difference into a different order statistic.
// An explicit conversion is the only way the language offers to pin an
// intermediate rounding, so every product gets one. The two sites that scale by
// a power of two are inert rather than dangerous: scaling by a power of two is
// exact, and the residual the fused form carries is far too small to move the
// scaled result, so fused and unfused agree bit for bit. They are pinned anyway,
// so that no future reader has to re-derive which sites were the safe ones.
//
// Each chain is written as one product per statement rather than as a nested
// expression: a Horner chain in which every level carries its own conversion is
// unreadable.

// gaussCdf computes the standard normal CDF using ACM Algorithm 209.
// Calculates (1/sqrt(2*pi)) * integral from -infinity to x of e^(-u^2/2) du
// Returns P(Z <= x) where Z is a standard normal random variable.
func gaussCdf(x float64) float64 {
	var z float64
	if math.Abs(x) < 1e-9 {
		z = 0.0
	} else {
		y := float64(math.Abs(x) / 2)
		if y >= 3.0 {
			z = 1.0
		} else if y < 1.0 {
			w := y * y
			p := 0.000124818987
			p = float64(p*w) - 0.001075204047
			p = float64(p*w) + 0.005198775019
			p = float64(p*w) - 0.019198292004
			p = float64(p*w) + 0.059054035642
			p = float64(p*w) - 0.151968751364
			p = float64(p*w) + 0.319152932694
			p = float64(p*w) - 0.531923007300
			p = float64(p*w) + 0.797884560593
			z = float64(p*y) * 2.0
		} else {
			y = y - 2.0
			p := -0.000045255659
			p = float64(p*y) + 0.000152529290
			p = float64(p*y) - 0.000019538132
			p = float64(p*y) - 0.000676904986
			p = float64(p*y) + 0.001390604284
			p = float64(p*y) - 0.000794620820
			p = float64(p*y) - 0.002034254874
			p = float64(p*y) + 0.006549791214
			p = float64(p*y) - 0.010557625006
			p = float64(p*y) + 0.011630447319
			p = float64(p*y) - 0.009279453341
			p = float64(p*y) + 0.005353579108
			p = float64(p*y) - 0.002141268741
			p = float64(p*y) + 0.000535310849
			z = float64(p*y) + 0.999936657524
		}
	}

	if x > 0.0 {
		return (z + 1.0) / 2
	}
	return (1.0 - z) / 2
}
