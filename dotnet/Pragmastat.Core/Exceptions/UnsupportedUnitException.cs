using Pragmastat.Core.Metrology;

namespace Pragmastat.Core.Exceptions;

public class UnsupportedUnitException(MeasurementUnit unit) : Exception(GetMessage(unit))
{
    private static string GetMessage(MeasurementUnit unit) => $"Unsupported unit: {unit.FullName}";
}