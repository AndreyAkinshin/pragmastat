namespace Pragmastat.Metrology;

public class NumberUnit() : MeasurementUnit("number", "Number", "", "Number", 1)
{
  public static readonly NumberUnit Instance = new();
}
