using Pragmastat.Metrology;

namespace Pragmastat.Tests.Estimators;

/// <summary>
/// No public estimator may return a negative zero. A sample holding both <c>+0.0</c> and
/// <c>-0.0</c> otherwise lets the sorting algorithm, not the data, pick the sign of the answer,
/// and the language ports sort differently: that is a divergence the exact conformance class
/// forbids. Every assertion here compares IEEE 754 payloads, because <c>-0.0 == 0.0</c> is true
/// and an equality test would pass on exactly the values under test.
/// </summary>
public class NegativeZeroTests
{
  /// <summary>Samples where a zero is the answer and both signs of it are reachable.</summary>
  private static readonly double[][] ZeroCenterSamples =
  [
    [0.0, -0.0, 0.0, -0.0, 1.0],
    [-0.0, -0.0],
    [-0.0, 0.0],
    [0.0, -0.0],
    [-0.0, -0.0, -0.0],
    [-1.0, 1.0],
    [-2.0, -0.0, 2.0]
  ];

  private static string Describe(double[] x) => "[" + string.Join(", ", x.Select(v => v.ToString("R"))) + "]";

  [Fact]
  public void Center_ZeroValuedSamples_ReturnPositiveZero()
  {
    foreach (var x in ZeroCenterSamples)
    {
      BitwiseAssert.Equal(0.0, Toolkit.Center(x), $"Center({Describe(x)})");
      BitwiseAssert.Equal(0.0, Toolkit.Center(new Sample(x)).NominalValue, $"Center(Sample({Describe(x)}))");
    }
  }

  [Fact]
  public void CenterBounds_ZeroValuedSample_ReturnsPositiveZeroEndpoints()
  {
    double[] x = [0.0, -0.0, 0.0, -0.0, 1.0, -1.0];

    var raw = Toolkit.CenterBounds(x, 0.3);
    BitwiseAssert.Equal(0.0, raw.Lower, $"CenterBounds({Describe(x)}, 0.3).Lower");
    BitwiseAssert.Equal(0.0, raw.Upper, $"CenterBounds({Describe(x)}, 0.3).Upper");

    var typed = Toolkit.CenterBounds(new Sample(x), 0.3);
    BitwiseAssert.Equal(0.0, typed.Lower, $"CenterBounds(Sample({Describe(x)}), 0.3).Lower");
    BitwiseAssert.Equal(0.0, typed.Upper, $"CenterBounds(Sample({Describe(x)}), 0.3).Upper");

    // A pairwise average of -0.0 and +0.0 lands on the lower endpoint here, which is how this
    // port produced a negative zero before the estimators started normalizing their output.
    double[] asymmetric = [-0.0, 1.0, 0.0, 0.0, -2.0, 1.0];
    var lower = Toolkit.CenterBounds(asymmetric, 0.5);
    BitwiseAssert.Equal(0.0, lower.Lower, $"CenterBounds({Describe(asymmetric)}, 0.5).Lower");
  }

  [Fact]
  public void Shift_ZeroValuedSamples_ReturnPositiveZero()
  {
    // Every pairwise difference is -0.0 here, so the selected order statistic is one too.
    BitwiseAssert.Equal(0.0, Toolkit.Shift([-0.0, -0.0], [0.0, 0.0]), "Shift([-0, -0], [0, 0])");
    // A -0.0 sits among the +0.0 differences; which one the search lands on is the port's sort.
    BitwiseAssert.Equal(0.0, Toolkit.Shift([-1.0, -0.0, 1.0], [-1.0, 0.0, 1.0]),
      "Shift([-1, -0, 1], [-1, 0, 1])");
    BitwiseAssert.Equal(0.0, Toolkit.Shift(new Sample(-0.0, -0.0), new Sample(0.0, 0.0)).NominalValue,
      "Shift(Sample([-0, -0]), Sample([0, 0]))");
  }

  [Fact]
  public void ShiftBounds_ZeroValuedSamples_ReturnPositiveZeroEndpoints()
  {
    // n = m = 1 collapses both endpoints onto the single difference -0.0 - 0.0.
    var single = Toolkit.ShiftBounds([-0.0], [0.0], 1.0);
    BitwiseAssert.Equal(0.0, single.Lower, "ShiftBounds([-0], [0], 1.0).Lower");
    BitwiseAssert.Equal(0.0, single.Upper, "ShiftBounds([-0], [0], 1.0).Upper");

    double[] x = [-1.0, -0.0, 1.0];
    double[] y = [-1.0, 0.0, 1.0];
    var wide = Toolkit.ShiftBounds(x, y, 0.9);
    BitwiseAssert.Equal(0.0, wide.Lower, "ShiftBounds([-1, -0, 1], [-1, 0, 1], 0.9).Lower");
    BitwiseAssert.Equal(0.0, wide.Upper, "ShiftBounds([-1, -0, 1], [-1, 0, 1], 0.9).Upper");
  }

  [Fact]
  public void Disparity_ZeroValuedSamples_ReturnPositiveZero()
  {
    double[] x = [-1.0, -0.0, 1.0];
    double[] y = [-1.0, 0.0, 1.0];
    BitwiseAssert.Equal(0.0, Toolkit.Disparity(x, y), "Disparity([-1, -0, 1], [-1, 0, 1])");
    BitwiseAssert.Equal(0.0, Toolkit.Disparity(new Sample(x), new Sample(y)).NominalValue,
      "Disparity(Sample([-1, -0, 1]), Sample([-1, 0, 1]))");
  }

  [Fact]
  public void Bounds_Constructor_NormalizesBothEndpoints()
  {
    // The single choke point: every bounds estimator returns through this constructor.
    var bounds = new Bounds(-0.0, -0.0, MeasurementUnit.Number);
    BitwiseAssert.Equal(0.0, bounds.Lower, "new Bounds(-0, -0).Lower");
    BitwiseAssert.Equal(0.0, bounds.Upper, "new Bounds(-0, -0).Upper");
  }

  [Fact]
  public void NegativeZeroInput_IsAcceptedAndPreserved()
  {
    // Only outputs are normalized. A sample keeps the values it was given.
    var sample = new Sample(-0.0, 1.0);
    BitwiseAssert.Equal(-0.0, sample.Values[0], "Sample([-0, 1]).Values[0]");
    BitwiseAssert.Equal(-0.0, sample.SortedValues[0], "Sample([-0, 1]).SortedValues[0]");
  }
}
