using Pragmastat.Metrology;

namespace Pragmastat.Tests.Metrology;

/// <summary>
/// The constructor refuses malformed unit definitions. A non-positive BaseUnits poisons every
/// conversion through the unit and makes Finer prefer it over any well-formed unit; an empty
/// Family makes two unrelated units compatible, because IsCompatible compares families.
/// An empty Abbreviation stays legal: the three dimensionless units have none.
/// </summary>
public class MeasurementUnitConstructionTests
{
  [Fact]
  public void EmptyId_IsRefused()
  {
    var ex = Assert.Throws<ArgumentException>(() => new MeasurementUnit("", "Time", "ms", "Millisecond", 1_000_000));

    Assert.Equal("id", ex.ParamName);
    Assert.Contains("id cannot be empty", ex.Message);
  }

  [Fact]
  public void EmptyFamily_IsRefused()
  {
    var ex = Assert.Throws<ArgumentException>(() => new MeasurementUnit("ms", "", "ms", "Millisecond", 1_000_000));

    Assert.Equal("family", ex.ParamName);
    Assert.Contains("family cannot be empty", ex.Message);
  }

  [Fact]
  public void ZeroBaseUnits_IsRefused()
  {
    var ex = Assert.Throws<ArgumentOutOfRangeException>(() => new MeasurementUnit("ms", "Time", "ms", "Millisecond", 0));

    Assert.Equal("baseUnits", ex.ParamName);
    Assert.Contains("baseUnits must be positive", ex.Message);
  }

  [Fact]
  public void NegativeBaseUnits_IsRefused()
  {
    var ex = Assert.Throws<ArgumentOutOfRangeException>(() => new MeasurementUnit("ms", "Time", "ms", "Millisecond", -1));

    Assert.Equal("baseUnits", ex.ParamName);
    Assert.Contains("baseUnits must be positive", ex.Message);
  }

  [Fact]
  public void WellFormedUnit_IsAccepted()
  {
    var ms = new MeasurementUnit("ms", "Time", "ms", "Millisecond", 1_000_000);

    Assert.Equal("ms", ms.Id);
    Assert.Equal("Time", ms.Family);
    Assert.Equal("ms", ms.Abbreviation);
    Assert.Equal("Millisecond", ms.FullName);
    Assert.Equal(1_000_000, ms.BaseUnits);
  }

  [Fact]
  public void DimensionlessUnits_KeepTheirEmptyAbbreviation()
  {
    foreach (var unit in new[] { MeasurementUnit.Number, MeasurementUnit.Ratio, MeasurementUnit.Disparity })
    {
      Assert.Equal("", unit.Abbreviation);
      Assert.Equal(1, unit.BaseUnits);
    }

    Assert.Equal("number", MeasurementUnit.Number.Id);
    Assert.Equal("ratio", MeasurementUnit.Ratio.Id);
    Assert.Equal("disparity", MeasurementUnit.Disparity.Id);
  }
}
