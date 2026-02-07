using JetBrains.Annotations;
using Pragmastat.Algorithms;
using Pragmastat.Distributions;
using Pragmastat.Functions;
using Pragmastat.Randomization;

namespace Pragmastat.Simulations.Simulations;

[UsedImplicitly]
public class RatioBoundsSimulation : CoverageBoundsSimulationBase
{
  public const string Name = "ratio-bounds";

  private static readonly HashSet<string> PositiveDistributions =
    new(["Multiplic", "Exp", "Uniform"], StringComparer.OrdinalIgnoreCase);

  protected override string GetResultFileName() => Name;

  protected override bool IsValidCombination(string distribution, int sampleSize, double misrate)
  {
    return PositiveDistributions.Contains(distribution);
  }

  protected override SimulationRow SimulateRow(Input input, Action<double> progressCallback)
  {
    (var distribution, int sampleCount, int sampleSize, double misrate, string baseSeed) = input;
    var rng = new Rng($"{baseSeed}-{distribution.Name}-{sampleSize}");

    const double trueValue = 1.0;

    int n = sampleSize, m = sampleSize;
    long total = (long)n * m;

    int margin = PairwiseMargin.Instance.Calc(n, m, misrate);
    long halfMargin = Math.Min(margin / 2L, (total - 1) / 2);
    long kLeft = halfMargin;
    long kRight = total - 1 - halfMargin;
    double denominator = total - 1 > 0 ? total - 1 : 1;
    double[] p = [kLeft / denominator, kRight / denominator];

    int coverage = 0;
    for (int i = 0; i < sampleCount; i++)
    {
      var xValues = new double[sampleSize];
      for (int j = 0; j < sampleSize; j++)
        xValues[j] = distribution.Value.Quantile(rng.Uniform());
      var x = new Sample(xValues);
      var yValues = new double[sampleSize];
      for (int j = 0; j < sampleSize; j++)
        yValues[j] = distribution.Value.Quantile(rng.Uniform());
      var y = new Sample(yValues);

      double lower, upper;
      if (total == 1)
      {
        double value = Math.Exp(Math.Log(x.Values[0]) - Math.Log(y.Values[0]));
        lower = upper = value;
      }
      else
      {
        var logX = x.Log();
        var logY = y.Log();
        double[] bounds = FastShift.Estimate(logX.SortedValues, logY.SortedValues, p, assumeSorted: true);
        lower = Math.Exp(Math.Min(bounds[0], bounds[1]));
        upper = Math.Exp(Math.Max(bounds[0], bounds[1]));
      }

      if (lower <= trueValue && trueValue <= upper)
        coverage++;

      if (i % 1000 == 0)
        progressCallback((i + 1.0) / sampleCount);
    }

    double observedMisrate = 1.0 - (double)coverage / sampleCount;
    return new SimulationRow(distribution.Name, sampleSize, misrate, observedMisrate);
  }
}
