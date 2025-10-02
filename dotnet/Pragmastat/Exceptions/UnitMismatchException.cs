using Pragmastat.Metrology;

namespace Pragmastat.Exceptions;

public class UnitMismatchException(MeasurementUnit unit1, MeasurementUnit unit2) : Exception(GetMessage(unit1, unit2))
{
    private static string GetMessage(MeasurementUnit unit1, MeasurementUnit unit2) =>
        $"Can't convert {unit1.FullName} to {unit2.FullName}";
}