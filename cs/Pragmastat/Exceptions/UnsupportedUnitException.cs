using Pragmastat.Metrology;

namespace Pragmastat.Exceptions;

public class UnsupportedUnitException(MeasurementUnit unit) : Exception(GetMessage(unit))
{
  private static string GetMessage(MeasurementUnit unit) => $"Unsupported unit: {unit.FullName}";
}
