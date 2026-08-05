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

// NewNumberMeasurement creates a new Measurement with the given value and NumberUnit.
func NewNumberMeasurement(value float64) Measurement {
	return Measurement{Value: value, Unit: NumberUnit}
}

// formatValue states a double the way the other six ports state it. Both carriers render through
// here so neither can drift from the other.
//
// The verb is 'f' rather than 'g' because 'g' switches to an exponent from 1e+06 upward, printing
// 1e+06 and 1e+14 where Rust, Kotlin, TypeScript, Python and R print 1000000 and 100000000000000.
// Go was the one port that did that. Against Rust's Display over 535 values, covering random bit
// patterns and every power of ten from -30 to 30, 'f' agrees byte for byte and 'g' does not.
func formatValue(v float64) string {
	return strconv.FormatFloat(v, 'f', -1, 64)
}

func (m Measurement) String() string {
	s := formatValue(m.Value)
	if m.Unit != nil && len(m.Unit.Abbreviation()) > 0 {
		return fmt.Sprintf("%s %s", s, m.Unit.Abbreviation())
	}
	return s
}
