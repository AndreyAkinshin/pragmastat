using Pragmastat.Randomization;

namespace Pragmastat.Tests.Estimators;

public class EstimatorInvarianceTests
{
  private const long Seed = 1729;
  private static readonly int[] SampleSizes = [2, 3, 4, 5, 6, 7, 8, 9, 10];
  private const double Tolerance = 1e-9;

  private static Sample UniformSample(Rng rng, int n)
  {
    var values = new double[n];
    for (int i = 0; i < n; i++)
      values[i] = rng.UniformDouble();
    return new Sample(values);
  }

  // Each expression answers in double. The left-hand side transforms the Sample, whose operators
  // stay; the right-hand side does its arithmetic on the estimate, which reaches double through
  // Measurement's implicit conversion. That is what the six sibling ports write, over estimators
  // that return plain numbers, and it is why removing Measurement's own arithmetic overloads left
  // these three lines untouched.
  private static void PerformTestOne(Func<Sample, double> expr1, Func<Sample, double> expr2)
  {
    var rng = new Rng(Seed);
    foreach (int n in SampleSizes)
    {
      var x = UniformSample(rng, n);
      double result1 = expr1(x);
      double result2 = expr2(x);
      Assert.True(Math.Abs(result1 - result2) < Tolerance,
        $"Failed for n={n}: {result1} != {result2}");
    }
  }

  private static void PerformTestTwo(Func<Sample, Sample, double> expr1, Func<Sample, Sample, double> expr2)
  {
    var rng = new Rng(Seed);
    foreach (int n in SampleSizes)
    {
      var x = UniformSample(rng, n);
      var y = UniformSample(rng, n);
      double result1 = expr1(x, y);
      double result2 = expr2(x, y);
      Assert.True(Math.Abs(result1 - result2) < Tolerance,
        $"Failed for n={n}: {result1} != {result2}");
    }
  }

  // Center tests

  [Fact]
  public void CenterShift()
  {
    PerformTestOne(x => Toolkit.Center(x + 2).Value, x => Toolkit.Center(x).Value + 2);
  }

  [Fact]
  public void CenterScale()
  {
    PerformTestOne(x => Toolkit.Center(x * 2).Value, x => 2 * Toolkit.Center(x).Value);
  }

  [Fact]
  public void CenterNegate()
  {
    PerformTestOne(x => Toolkit.Center(x * -1).Value, x => -1 * Toolkit.Center(x).Value);
  }

  // Spread tests

  [Fact]
  public void SpreadShift()
  {
    PerformTestOne(x => Toolkit.Spread(x + 2).Value, x => Toolkit.Spread(x).Value);
  }

  [Fact]
  public void SpreadScale()
  {
    PerformTestOne(x => Toolkit.Spread(x * 2).Value, x => 2 * Toolkit.Spread(x).Value);
  }

  [Fact]
  public void SpreadNegate()
  {
    PerformTestOne(x => Toolkit.Spread(x * -1).Value, x => Toolkit.Spread(x).Value);
  }

  // Shift tests

  [Fact]
  public void ShiftShift()
  {
    PerformTestTwo((x, y) => Toolkit.Shift(x + 3, y + 2).Value, (x, y) => Toolkit.Shift(x, y).Value + 1);
  }

  [Fact]
  public void ShiftScale()
  {
    PerformTestTwo((x, y) => Toolkit.Shift(x * 2, y * 2).Value, (x, y) => 2 * Toolkit.Shift(x, y).Value);
  }

  [Fact]
  public void ShiftAntisymmetry()
  {
    PerformTestTwo((x, y) => Toolkit.Shift(x, y).Value, (x, y) => -1 * Toolkit.Shift(y, x).Value);
  }

  // Ratio tests

  [Fact]
  public void RatioScale()
  {
    PerformTestTwo((x, y) => Toolkit.Ratio(x * 2, y * 3).Value, (x, y) => (2.0 / 3) * Toolkit.Ratio(x, y).Value);
  }

  // AvgSpread tests

  [Fact]
  public void AvgSpreadEqual()
  {
    PerformTestOne(x => Toolkit.AvgSpread(x, x).Value, x => Toolkit.Spread(x).Value);
  }

  [Fact]
  public void AvgSpreadSymmetry()
  {
    PerformTestTwo((x, y) => Toolkit.AvgSpread(x, y).Value, (x, y) => Toolkit.AvgSpread(y, x).Value);
  }

  [Fact]
  public void AvgSpreadAverage()
  {
    PerformTestOne(x => Toolkit.AvgSpread(x, x * 5).Value, x => 3 * Toolkit.Spread(x).Value);
  }

  [Fact]
  public void AvgSpreadScale()
  {
    PerformTestTwo((x, y) => Toolkit.AvgSpread(x * -2, y * -2).Value, (x, y) => 2 * Toolkit.AvgSpread(x, y).Value);
  }

  // Disparity tests

  [Fact]
  public void DisparityShift()
  {
    PerformTestTwo((x, y) => Toolkit.Disparity(x + 2, y + 2).Value, (x, y) => Toolkit.Disparity(x, y).Value);
  }

  [Fact]
  public void DisparityScale()
  {
    PerformTestTwo((x, y) => Toolkit.Disparity(x * 2, y * 2).Value, (x, y) => Toolkit.Disparity(x, y).Value);
  }

  [Fact]
  public void DisparityScaleNeg()
  {
    PerformTestTwo((x, y) => Toolkit.Disparity(x * -2, y * -2).Value, (x, y) => -1 * Toolkit.Disparity(x, y).Value);
  }

  [Fact]
  public void DisparityAntisymmetry()
  {
    PerformTestTwo((x, y) => Toolkit.Disparity(x, y).Value, (x, y) => -1 * Toolkit.Disparity(y, x).Value);
  }
}
