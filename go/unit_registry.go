package pragmastat

import "fmt"

// UnitRegistry stores measurement units and enables lookup by ID.
type UnitRegistry struct {
	byID map[string]*MeasurementUnit
}

// NewUnitRegistry creates an empty registry.
func NewUnitRegistry() *UnitRegistry {
	return &UnitRegistry{
		byID: make(map[string]*MeasurementUnit),
	}
}

// Register adds a unit to the registry.
func (r *UnitRegistry) Register(unit *MeasurementUnit) error {
	if _, exists := r.byID[unit.ID]; exists {
		return fmt.Errorf("unit with id '%s' is already registered", unit.ID)
	}
	r.byID[unit.ID] = unit
	return nil
}

// Resolve looks up a unit by ID.
func (r *UnitRegistry) Resolve(id string) (*MeasurementUnit, error) {
	if unit, ok := r.byID[id]; ok {
		return unit, nil
	}
	return nil, fmt.Errorf("unknown unit id: '%s'", id)
}

// StandardRegistry returns a registry pre-populated with Number, Ratio, and Disparity units.
func StandardRegistry() *UnitRegistry {
	r := NewUnitRegistry()
	_ = r.Register(NumberUnit)
	_ = r.Register(RatioUnit)
	_ = r.Register(DisparityUnit)
	return r
}
