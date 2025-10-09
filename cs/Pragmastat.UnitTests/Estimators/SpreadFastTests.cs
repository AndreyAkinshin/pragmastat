using System.Diagnostics;
using Pragmastat.Distributions;
using Pragmastat.Distributions.Randomization;
using Pragmastat.Estimators;
using Pragmastat.Functions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.UnitTests.Estimators;

public class SpreadFastTests
{
  [Fact]
  public void SpreadFastTest()
  {
    var random = AdditiveDistribution.Standard.Random(1729);
    for (int n = 1; n <= 100; n++)
    for (int iteration = 0; iteration < n; iteration++)
    {
      var x = random.Next(n).ToSample();
      var actual = SpreadEstimator.Instance.Estimate(x);
      var expected = SpreadSimple(x);
      Assert.Equal(expected.Unit, actual.Unit);
      Assert.Equal(expected.NominalValue, actual.NominalValue, 9);
    }
  }

  [Fact]
  public void SpreadFastTest2()
  {
    var random = AdditiveDistribution.Standard.Random(1729);
    var x = random.Next(100_000).ToSample();
    var stopwatch = Stopwatch.StartNew();
    Trace.WriteLine(SpreadEstimator.Instance.Estimate(x));
    Trace.WriteLine($"Elapsed: {stopwatch.ElapsedMilliseconds}ms");
    Assert.True(stopwatch.Elapsed.TotalSeconds < 5);
  }

  private Measurement SpreadSimple(Sample x)
  {
    if (x.Size == 1)
      return Measurement.Zero(x.Unit);
    var pairwise = PairwiseSampleTransformer.Transform(x, (xi, xj) => Math.Abs(xi - xj), false);
    return MedianEstimator.Instance.Estimate(pairwise);
  }
}
