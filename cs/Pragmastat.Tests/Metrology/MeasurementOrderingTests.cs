using Pragmastat.Metrology;

namespace Pragmastat.Tests.Metrology;

/// <summary>
/// Measurement implements IComparable and IComparable&lt;Measurement&gt;, and publishes the four
/// comparison operators over them. That is a contract the framework itself consumes, so a .NET
/// caller gets Array.Sort, OrderBy and Comparer&lt;Measurement&gt;.Default at no cost to this
/// library, which is why it stays where the arithmetic overloads beside it did not.
///
/// Nothing exercised any of it. These do, including the half a reader guesses wrong: comparison
/// converts through BaseUnits, so 1 s is greater than 2 ms even though 1 is less than 2.
/// </summary>
public class MeasurementOrderingTests
{
  private static readonly MeasurementUnit Second =
    new("second", "Time", "s", "Second", 1_000_000_000);

  private static readonly MeasurementUnit Millisecond =
    new("millisecond", "Time", "ms", "Millisecond", 1_000_000);

  [Fact]
  public void ComparisonConvertsThroughTheUnit()
  {
    var oneSecond = new Measurement(1, Second);
    var twoMilliseconds = new Measurement(2, Millisecond);

    Assert.True(oneSecond > twoMilliseconds);
    Assert.True(twoMilliseconds < oneSecond);
    Assert.Equal(1, oneSecond.CompareTo(twoMilliseconds));
  }

  [Fact]
  public void EqualQuantitiesInDifferentUnitsCompareEqual()
  {
    var oneSecond = new Measurement(1, Second);
    var thousandMilliseconds = new Measurement(1000, Millisecond);

    Assert.Equal(0, oneSecond.CompareTo(thousandMilliseconds));
    Assert.True(oneSecond <= thousandMilliseconds);
    Assert.True(oneSecond >= thousandMilliseconds);
  }

  [Fact]
  public void SortingUsesTheContractRatherThanTheNominalValue()
  {
    Measurement[] measurements =
    [
      new Measurement(5, Second),
      new Measurement(2, Millisecond),
      new Measurement(900, Millisecond)
    ];

    Array.Sort(measurements);

    Assert.Equal(["2 ms", "900 ms", "5 s"], measurements.Select(m => m.ToString()));
  }

  [Fact]
  public void UnitsOfDifferentFamiliesCannotBeOrdered()
  {
    var oneSecond = new Measurement(1, Second);
    var plainNumber = new Measurement(1, MeasurementUnit.Number);

    Assert.Throws<InvalidOperationException>(() => oneSecond.CompareTo(plainNumber));
  }

  [Fact]
  public void NullSortsFirstAndAForeignTypeIsRefused()
  {
    var oneSecond = new Measurement(1, Second);

    Assert.Equal(1, oneSecond.CompareTo(null));
    Assert.Equal(1, oneSecond.CompareTo((object?)null));
    Assert.Throws<ArgumentException>(() => oneSecond.CompareTo("not a measurement"));
  }
}
