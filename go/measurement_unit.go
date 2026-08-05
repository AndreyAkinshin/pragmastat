package pragmastat

import "fmt"

// MeasurementUnit represents a unit of measurement with identity, family, and conversion support.
// Build one with NewMeasurementUnit, which rejects malformed definitions.
// The fields are unexported and read through accessors. A unit is not a value a caller holds:
// NumberUnit is what every Sample, every Measurement and every conversion in the process reaches,
// so `NumberUnit.BaseUnits = 5` did not corrupt one object, it rescaled ConversionFactor and Finer
// for everything computed afterwards. Nothing detects that and nothing can, since a unit is never
// revalidated after construction, so the wrong number surfaces somewhere else entirely.
type MeasurementUnit struct {
	id           string
	family       string
	abbreviation string
	fullName     string
	baseUnits    int64
}

// ID returns the unit's unique identifier.
func (u *MeasurementUnit) ID() string { return u.id }

// Family returns the unit family used for compatibility checks.
func (u *MeasurementUnit) Family() string { return u.family }

// Abbreviation returns the short display abbreviation, empty for the dimensionless units.
func (u *MeasurementUnit) Abbreviation() string { return u.abbreviation }

// FullName returns the human-readable full name.
func (u *MeasurementUnit) FullName() string { return u.fullName }

// BaseUnits returns the number of base units, used for conversion.
func (u *MeasurementUnit) BaseUnits() int64 { return u.baseUnits }

// NewMeasurementUnit creates a measurement unit.
// An empty Abbreviation is legal: the three dimensionless units have none.
// Panics if id or family is empty, or if baseUnits is not positive.
func NewMeasurementUnit(id, family, abbreviation, fullName string, baseUnits int64) *MeasurementUnit {
	// A non-positive baseUnits poisons every conversion through this unit, and it makes Finer
	// pick it over any well-formed one. An empty family makes unrelated units compatible.
	if id == "" {
		panic("id cannot be empty")
	}
	if family == "" {
		panic("family cannot be empty")
	}
	if baseUnits <= 0 {
		panic("baseUnits must be positive")
	}
	return &MeasurementUnit{
		id:           id,
		family:       family,
		abbreviation: abbreviation,
		fullName:     fullName,
		baseUnits:    baseUnits,
	}
}

// IsCompatible returns true if both units belong to the same family.
func (u *MeasurementUnit) IsCompatible(other *MeasurementUnit) bool {
	return u.family == other.family
}

// Finer returns the unit with smaller BaseUnits (higher precision).
func Finer(a, b *MeasurementUnit) *MeasurementUnit {
	if a.baseUnits <= b.baseUnits {
		return a
	}
	return b
}

// ConversionFactor returns the multiplier to convert from one unit to another.
func ConversionFactor(from, to *MeasurementUnit) float64 {
	return float64(from.baseUnits) / float64(to.baseUnits)
}

func (u *MeasurementUnit) String() string {
	return u.abbreviation
}

// UnitMismatchError is returned when incompatible units are used together.
type UnitMismatchError struct {
	Unit1 *MeasurementUnit
	Unit2 *MeasurementUnit
}

func (e *UnitMismatchError) Error() string {
	return fmt.Sprintf("can't convert %s to %s", e.Unit1.FullName(), e.Unit2.FullName())
}

// Standard units
var (
	NumberUnit    = NewMeasurementUnit("number", "Number", "", "Number", 1)
	RatioUnit     = NewMeasurementUnit("ratio", "Ratio", "", "Ratio", 1)
	DisparityUnit = NewMeasurementUnit("disparity", "Disparity", "", "Disparity", 1)
)
