package pragmastat

import "fmt"

// MeasurementUnit represents a unit of measurement with identity, family, and conversion support.
type MeasurementUnit struct {
	ID           string
	Family       string
	Abbreviation string
	FullName     string
	BaseUnits    int64
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
	NumberUnit    = &MeasurementUnit{ID: "number", Family: "Number", Abbreviation: "", FullName: "Number", BaseUnits: 1}
	RatioUnit     = &MeasurementUnit{ID: "ratio", Family: "Ratio", Abbreviation: "", FullName: "Ratio", BaseUnits: 1}
	DisparityUnit = &MeasurementUnit{ID: "disparity", Family: "Disparity", Abbreviation: "", FullName: "Disparity", BaseUnits: 1}
)
