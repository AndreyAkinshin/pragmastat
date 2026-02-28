namespace Pragmastat.Metrology;

public class RatioUnit() : MeasurementUnit("ratio", "Ratio", "", "Ratio", 1)
{
  public static readonly RatioUnit Instance = new();
}
