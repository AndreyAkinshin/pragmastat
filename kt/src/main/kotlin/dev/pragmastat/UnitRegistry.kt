package dev.pragmastat

/**
 * A registry for looking up [MeasurementUnit] instances by their [MeasurementUnit.id].
 *
 * Use [standard] to get a pre-populated registry with the built-in Number, Ratio, and Disparity units.
 * Use [register] to add custom units.
 */
class UnitRegistry private constructor(
    private val byId: MutableMap<String, MeasurementUnit>,
) {
    constructor() : this(mutableMapOf())

    /**
     * Register a unit in this registry.
     *
     * @throws IllegalArgumentException if a unit with the same ID is already registered
     */
    fun register(unit: MeasurementUnit) {
        require(!byId.containsKey(unit.id)) {
            "unit with id '${unit.id}' is already registered"
        }
        byId[unit.id] = unit
    }

    /**
     * Look up a unit by its ID.
     *
     * @throws IllegalArgumentException if no unit with the given ID is registered
     */
    fun resolve(id: String): MeasurementUnit {
        return byId[id]
            ?: throw IllegalArgumentException("unknown unit id: '$id'")
    }

    companion object {
        /** Returns a registry pre-populated with Number, Ratio, and Disparity units. */
        fun standard(): UnitRegistry {
            val registry = UnitRegistry()
            registry.register(NumberUnit)
            registry.register(RatioUnit)
            registry.register(DisparityUnit)
            return registry
        }
    }
}
