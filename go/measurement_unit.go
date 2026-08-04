package pragmastat

import "fmt"

// MeasurementUnit represents a unit of measurement with identity, family, and conversion support.
// Build one with NewMeasurementUnit, which rejects malformed definitions.
type MeasurementUnit struct {
	ID           string
	Family       string
	Abbreviation string
	FullName     string
	BaseUnits    int64
}

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
		ID:           id,
		Family:       family,
		Abbreviation: abbreviation,
		FullName:     fullName,
		BaseUnits:    baseUnits,
	}
}

// IsCompatible returns true if both units belong to the same family.
func (u *MeasurementUnit) IsCompatible(other *MeasurementUnit) bool {
	return u.Family == other.Family
}

// Finer returns the unit with smaller BaseUnits (higher precision).
func Finer(a, b *MeasurementUnit) *MeasurementUnit {
	if a.BaseUnits <= b.BaseUnits {
		return a
	}
	return b
}

// ConversionFactor returns the multiplier to convert from one unit to another.
func ConversionFactor(from, to *MeasurementUnit) float64 {
	return float64(from.BaseUnits) / float64(to.BaseUnits)
}

func (u *MeasurementUnit) String() string {
	return u.Abbreviation
}

// UnitMismatchError is returned when incompatible units are used together.
type UnitMismatchError struct {
	Unit1 *MeasurementUnit
	Unit2 *MeasurementUnit
}

func (e *UnitMismatchError) Error() string {
	return fmt.Sprintf("can't convert %s to %s", e.Unit1.FullName, e.Unit2.FullName)
}

// Standard units
var (
	NumberUnit    = NewMeasurementUnit("number", "Number", "", "Number", 1)
	RatioUnit     = NewMeasurementUnit("ratio", "Ratio", "", "Ratio", 1)
	DisparityUnit = NewMeasurementUnit("disparity", "Disparity", "", "Disparity", 1)
)
