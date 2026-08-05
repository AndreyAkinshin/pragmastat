package pragmastat

import "testing"

// NewMeasurementUnit refuses malformed unit definitions. A non-positive BaseUnits poisons every
// conversion through the unit and makes Finer prefer it over any well-formed unit; an empty
// Family makes two unrelated units compatible, because IsCompatible compares families.
// An empty Abbreviation stays legal: the three dimensionless units have none.

func expectPanic(t *testing.T, want string, build func()) {
	t.Helper()
	defer func() {
		r := recover()
		if r == nil {
			t.Fatalf("NewMeasurementUnit should panic with %q", want)
		}
		got, ok := r.(string)
		if !ok || got != want {
			t.Fatalf("panic = %v, want %q", r, want)
		}
	}()
	build()
}

func TestNewMeasurementUnitRefusesEmptyID(t *testing.T) {
	expectPanic(t, "id cannot be empty", func() {
		NewMeasurementUnit("", "Time", "ms", "Millisecond", 1000000)
	})
}

func TestNewMeasurementUnitRefusesEmptyFamily(t *testing.T) {
	expectPanic(t, "family cannot be empty", func() {
		NewMeasurementUnit("ms", "", "ms", "Millisecond", 1000000)
	})
}

func TestNewMeasurementUnitRefusesZeroBaseUnits(t *testing.T) {
	expectPanic(t, "baseUnits must be positive", func() {
		NewMeasurementUnit("ms", "Time", "ms", "Millisecond", 0)
	})
}

func TestNewMeasurementUnitRefusesNegativeBaseUnits(t *testing.T) {
	expectPanic(t, "baseUnits must be positive", func() {
		NewMeasurementUnit("ms", "Time", "ms", "Millisecond", -1)
	})
}

func TestNewMeasurementUnitAcceptsWellFormedUnit(t *testing.T) {
	ms := NewMeasurementUnit("ms", "Time", "ms", "Millisecond", 1000000)
	if ms.ID() != "ms" || ms.Family() != "Time" || ms.Abbreviation() != "ms" ||
		ms.FullName() != "Millisecond" || ms.BaseUnits() != 1000000 {
		t.Fatalf("unexpected unit: %+v", *ms)
	}
}

func TestDimensionlessUnitsKeepTheirEmptyAbbreviation(t *testing.T) {
	units := map[string]*MeasurementUnit{
		"number":    NumberUnit,
		"ratio":     RatioUnit,
		"disparity": DisparityUnit,
	}
	for id, unit := range units {
		if unit.ID() != id {
			t.Errorf("ID = %q, want %q", unit.ID(), id)
		}
		if unit.Abbreviation() != "" {
			t.Errorf("%s: Abbreviation = %q, want empty", id, unit.Abbreviation())
		}
		if unit.BaseUnits() != 1 {
			t.Errorf("%s: BaseUnits = %d, want 1", id, unit.BaseUnits())
		}
	}
}

// A unit is not a value a caller holds: NumberUnit is what every Sample, every Measurement and
// every conversion in the process reaches, so a write to one of its fields rescaled everything
// computed afterwards rather than corrupting one object. Measured before the fields were
// unexported: NumberUnit.BaseUnits = 5 moved ConversionFactor(RatioUnit, NumberUnit) from 1 to 0.2,
// and nothing could detect it, since a unit is never revalidated after construction.
//
// The refusal itself is the compiler's, and a test cannot ask for code that does not compile. What
// this pins is the half a later change could quietly undo: the accessors answer, and the three
// shared units still hold the scale the rest of the library assumes.
func TestSharedUnitsKeepTheirScale(t *testing.T) {
	if got := ConversionFactor(RatioUnit, NumberUnit); got != 1 {
		t.Errorf("ConversionFactor(RatioUnit, NumberUnit) = %v, want 1", got)
	}
	if got := Finer(NumberUnit, RatioUnit); got != NumberUnit {
		t.Errorf("Finer(NumberUnit, RatioUnit) = %v, want NumberUnit", got.ID())
	}

	for _, unit := range []*MeasurementUnit{NumberUnit, RatioUnit, DisparityUnit} {
		if unit.BaseUnits() != 1 {
			t.Errorf("%s.BaseUnits() = %d, want 1", unit.ID(), unit.BaseUnits())
		}
		if unit.Abbreviation() != "" {
			t.Errorf("%s.Abbreviation() = %q, want empty", unit.ID(), unit.Abbreviation())
		}
		if unit.Family() == "" || unit.FullName() == "" {
			t.Errorf("%s has an empty family or full name", unit.ID())
		}
	}
}
