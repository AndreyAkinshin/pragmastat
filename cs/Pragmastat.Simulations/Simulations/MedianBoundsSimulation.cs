using JetBrains.Annotations;
using Pragmastat.Distributions;
using Pragmastat.Functions;

namespace Pragmastat.Simulations.Simulations;

[UsedImplicitly]
public class MedianBoundsSimulation : CoverageBoundsSimulationBase
{
  public const string Name = "median-bounds";

  protected override string GetResultFileName() => Name;

  protected override bool IsValidCombination(string distribution, int sampleSize, double misrate)
  {
    double minMisrate = MinAchievableMisrate.OneSample(sampleSize);
    return misrate >= minMisrate;
  }

  protected override SimulationRow SimulateRow(Input input, Action<double> progressCallback)
  {
    (var distribution, int sampleCount, int sampleSize, double misrate, int baseSeed) = input;
    var random = distribution.Value.Random(baseSeed + sampleSize);

    double trueValue = distribution.Value.Quantile(0.5);

    int coverage = 0;
    for (int i = 0; i < sampleCount; i++)
    {
      var sample = random.NextSample(sampleSize);
      var bounds = Toolkit.MedianBounds(sample, new Probability(misrate));

      if (bounds.Lower <= trueValue && trueValue <= bounds.Upper)
        coverage++;

      if (i % 1000 == 0)
        progressCallback((i + 1.0) / sampleCount);
    }

    double observedMisrate = 1.0 - (double)coverage / sampleCount;
    return new SimulationRow(distribution.Name, sampleSize, misrate, observedMisrate);
  }
}
