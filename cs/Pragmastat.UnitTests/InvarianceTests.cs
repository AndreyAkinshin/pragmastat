using System.Diagnostics.CodeAnalysis;
using Pragmastat.Distributions;
using Pragmastat.Distributions.Randomization;
using Pragmastat.Metrology;
using static Pragmastat.Toolkit;

namespace Pragmastat.UnitTests;

[SuppressMessage("ReSharper", "ConvertClosureToMethodGroup")]
public class InvarianceTests
{
  private const int Seed = 1729;

  private readonly int[] sampleSizes = [2, 3, 4, 5, 6, 7, 8, 9, 10];

  private void PerformTest(Func<Sample, Measurement> expression1, Func<Sample, Measurement> expression2)
  {
    var generator = UniformDistribution.Standard.Random(Seed);
    foreach (var n in sampleSizes)
    {
      var x = new Sample(generator.Next(n));
      var result1 = expression1(x);
      var result2 = expression2(x);
      Assert.Equal(result1.Unit.FullName, result2.Unit.FullName);
      Assert.Equal(result1.NominalValue, result2.NominalValue, 9);
    }
  }

  private void PerformTest(
    Func<Sample, Sample, Measurement> expression1,
    Func<Sample, Sample, Measurement> expression2)
  {
    var generator = UniformDistribution.Standard.Random(Seed);
    foreach (var n in sampleSizes)
    {
      var x = new Sample(generator.Next(n));
      var y = new Sample(generator.Next(n));
      var result1 = expression1(x, y);
      var result2 = expression2(x, y);
      Assert.Equal(result1.Unit.FullName, result2.Unit.FullName);
      Assert.Equal(result1.NominalValue, result2.NominalValue, 9);
    }
  }

  // Center

  [Fact]
  public void CenterShift() => PerformTest(x => Center(x + 2), x => Center(x) + 2);

  [Fact]
  public void CenterScale() => PerformTest(x => Center(2 * x), x => 2 * Center(x));

  [Fact]
  public void CenterNegate() => PerformTest(x => Center(-1 * x), x => -1 * Center(x));

  // Spread

  [Fact]
  public void SpreadShift() => PerformTest(x => Spread(x + 2), x => Spread(x));

  [Fact]
  public void SpreadScale() => PerformTest(x => Spread(2 * x), x => 2 * Spread(x));

  [Fact]
  public void SpreadNegate() => PerformTest(x => Spread(-1 * x), x => Spread(x));

  // RelSpread

  [Fact]
  public void RelSpreadScale() => PerformTest(x => RelSpread(2 * x), x => RelSpread(x));

  // Shift

  [Fact]
  public void ShiftShift() => PerformTest((x, y) => Shift(x + 3, y + 2), (x, y) => Shift(x, y) + 1);

  [Fact]
  public void ShiftScale() => PerformTest((x, y) => Shift(2 * x, 2 * y), (x, y) => 2 * Shift(x, y));

  [Fact]
  public void ShiftAntisymmetry() => PerformTest((x, y) => Shift(x, y), (x, y) => -1 * Shift(y, x));

  // Ratio

  [Fact]
  public void RatioScale() => PerformTest((x, y) => Ratio(2 * x, 3 * y), (x, y) => 2.0 / 3 * Ratio(x, y));

  // AvgSpread

  [Fact]
  public void AvgSpreadEqual() => PerformTest(x => AvgSpread(x, x), x => Spread(x));

  [Fact]
  public void AvgSpreadSymmetry() => PerformTest((x, y) => AvgSpread(x, y), (x, y) => AvgSpread(y, x));

  [Fact]
  public void AvgSpreadAverage() => PerformTest(x => AvgSpread(x, 5 * x), x => 3 * Spread(x));

  [Fact]
  public void AvgSpreadScale() => PerformTest((x, y) => AvgSpread(-2 * x, -2 * y), (x, y) => 2 * AvgSpread(x, y));

  // Disparity

  [Fact]
  public void DisparityShift() => PerformTest((x, y) => Disparity(x + 2, y + 2), (x, y) => Disparity(x, y));

  [Fact]
  public void DisparityScale() => PerformTest((x, y) => Disparity(2 * x, 2 * y), (x, y) => Disparity(x, y));

  [Fact]
  public void DisparityScaleNeg() => PerformTest((x, y) => Disparity(-2 * x, -2 * y), (x, y) => -1 * Disparity(x, y));

  [Fact]
  public void DisparityAntisymmetry() => PerformTest((x, y) => Disparity(x, y), (x, y) => -1 * Disparity(y, x));
}
