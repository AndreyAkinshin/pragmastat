package pragmastat

import (
	"fmt"
	"strconv"
)

// Measurement represents a value with a unit.
type Measurement struct {
	Value float64
	Unit  *MeasurementUnit
}

// NewMeasurement creates a new Measurement with the given value and unit.
func NewMeasurement(value float64, unit *MeasurementUnit) Measurement {
	if unit == nil {
		unit = NumberUnit
	}
	return Measurement{Value: value, Unit: unit}
}

func (m Measurement) String() string {
	s := strconv.FormatFloat(m.Value, 'G', -1, 64)
	if m.Unit != nil && len(m.Unit.Abbreviation) > 0 {
		return fmt.Sprintf("%s %s", s, m.Unit.Abbreviation)
	}
	return s
}
