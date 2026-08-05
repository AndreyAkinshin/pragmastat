using Pragmastat.Exceptions;
using Pragmastat.Metrology;

namespace Pragmastat.Tests.Metrology;

/// <summary>
/// Measurement publishes arithmetic operators, and the two that take a Measurement on both sides
/// carry the metrology: they refuse incompatible families and convert to the finer unit before
/// adding. Nothing exercised any of it, which made the whole set read as unused surface, and it
/// was deleted on that reading.
///
/// It could be deleted without breaking a single call site, because Measurement converts to double
/// implicitly, so every expression kept compiling and started answering in plain numbers: 1 s + 2
/// ms returned 3 instead of 1002 ms, and 1 s + 2 (dimensionless) returned 3 instead of throwing.
/// A deletion that turns a right answer into a quiet wrong one is exactly what a test has to make
/// impossible, so these assert the results rather than the presence of the operators.
/// </summary>
public class MeasurementArithmeticTests
{
  private static readonly MeasurementUnit Second =
    new("second", "Time", "s", "Second", 1_000_000_000);

  private static readonly MeasurementUnit Millisecond =
    new("millisecond", "Time", "ms", "Millisecond", 1_000_000);

  [Fact]
  public void AddingTwoMeasurementsConvertsToTheFinerUnit()
  {
    var sum = new Measurement(1, Second) + new Measurement(2, Millisecond);

    Assert.Equal("1002 ms", sum.ToString());
    Assert.Equal(Millisecond, sum.Unit);
  }

  [Fact]
  public void SubtractingTwoMeasurementsConvertsToTheFinerUnit()
  {
    var difference = new Measurement(1, Second) - new Measurement(2, Millisecond);

    Assert.Equal("998 ms", difference.ToString());
    Assert.Equal(Millisecond, difference.Unit);
  }

  [Fact]
  public void MeasurementsOfDifferentFamiliesCannotBeAdded()
  {
    var oneSecond = new Measurement(1, Second);
    var plainNumber = new Measurement(2, MeasurementUnit.Number);

    Assert.Throws<UnitMismatchException>(() => oneSecond + plainNumber);
    Assert.Throws<UnitMismatchException>(() => oneSecond - plainNumber);
  }

  /// <summary>
  /// The conversion to double is explicit, and the reason is overload resolution rather than
  /// taste: while it was implicit, WriteLine(m) bound WriteLine(double) in preference to
  /// WriteLine(object), so a 300 ns measurement printed 300 where the other six ports print
  /// 300 ns. Both calls compiled and nothing warned. This asserts the printed bytes, so making
  /// the conversion implicit again fails here rather than passing quietly.
  /// </summary>
  [Fact]
  public void PrintingAMeasurementReachesItsStringForm()
  {
    var writer = new StringWriter();
    writer.Write(new Measurement(300, new MeasurementUnit("nanosecond", "Time", "ns", "Nanosecond", 1)));

    Assert.Equal("300 ns", writer.ToString());
  }

  /// <summary>
  /// A scalar carries no unit of its own, so it acts on the value and leaves the unit in place.
  /// Without these overloads the same expressions answer in double and the unit is gone.
  /// </summary>
  [Fact]
  public void ScalarArithmeticKeepsTheUnit()
  {
    var fourSeconds = new Measurement(4, Second);

    Assert.Equal("6 s", (fourSeconds + 2).ToString());
    Assert.Equal("6 s", (2 + fourSeconds).ToString());
    Assert.Equal("2 s", (fourSeconds - 2.0).ToString());
    Assert.Equal("-2 s", (2.0 - fourSeconds).ToString());
    Assert.Equal("8 s", (fourSeconds * 2).ToString());
    Assert.Equal("8 s", (2.0 * fourSeconds).ToString());
    Assert.Equal("2 s", (fourSeconds / 2).ToString());
    Assert.Equal("2 s", (fourSeconds / 2.0).ToString());
  }
}
