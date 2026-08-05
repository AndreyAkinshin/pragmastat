package pragmastat

import (
	"math"
	"testing"
)

// The arguments outside the reduction's domain, which the shared log-function fixture cannot
// carry: JSON has no way to write a negative infinity or a NaN, so the generated suite holds only
// the positive finite range and these are covered here or nowhere.
//
// These are the cases IEEE 754 leaves a platform no freedom about, so agreeing on them is not a
// choice this toolkit makes; it is what makes the replacement a logarithm at all.
func TestLogFunctionOutsideTheReductionDomain(t *testing.T) {
	if !sameFloatBits(logFunction(0.0), math.Inf(-1)) {
		t.Errorf("logFunction(+0) = %v, want -Inf", logFunction(0.0))
	}
	if !sameFloatBits(logFunction(math.Copysign(0, -1)), math.Inf(-1)) {
		t.Errorf("logFunction(-0) = %v, want -Inf", logFunction(math.Copysign(0, -1)))
	}
	if !math.IsNaN(logFunction(-1)) {
		t.Errorf("logFunction(-1) = %v, want NaN", logFunction(-1))
	}
	if !math.IsNaN(logFunction(math.NaN())) {
		t.Errorf("logFunction(NaN) = %v, want NaN", logFunction(math.NaN()))
	}
	if !math.IsInf(logFunction(math.Inf(1)), 1) {
		t.Errorf("logFunction(+Inf) = %v, want +Inf", logFunction(math.Inf(1)))
	}
	// Exactly +0, not -0: ratio exponentiates this result, and the two zeros are distinct doubles
	// that the payload comparison in every other suite would tell apart.
	if !sameFloatBits(logFunction(1), 0.0) {
		t.Errorf("logFunction(1) = %v, want +0", logFunction(1))
	}
}
