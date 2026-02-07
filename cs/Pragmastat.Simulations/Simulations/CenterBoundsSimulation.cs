using JetBrains.Annotations;
using Pragmastat.Distributions;
using Pragmastat.Functions;
using Pragmastat.Randomization;

namespace Pragmastat.Simulations.Simulations;

[UsedImplicitly]
public class CenterBoundsSimulation : CoverageBoundsSimulationBase
{
  public const string Name = "center-bounds";

  private static readonly Dictionary<string, double> TrueCenters =
    new(StringComparer.OrdinalIgnoreCase)
    {
      ["Additive"] = 0.0,
      ["Uniform"] = 0.5,
    };

  protected override string GetResultFileName() => Name;

  protected override bool IsValidCombination(string distribution, int sampleSize, double misrate)
  {
    if (!TrueCenters.ContainsKey(distribution))
      return false;

    double minMisrate = MinAchievableMisrate.OneSample(sampleSize);
    return misrate >= minMisrate;
  }

  protected override SimulationRow SimulateRow(Input input, Action<double> progressCallback)
  {
    (var distribution, int sampleCount, int sampleSize, double misrate, string baseSeed) = input;
    var rng = new Rng($"{baseSeed}-{distribution.Name}-{sampleSize}");

    double trueValue = TrueCenters[distribution.Name];

    int coverage = 0;
    for (int i = 0; i < sampleCount; i++)
    {
      var values = new double[sampleSize];
      for (int j = 0; j < sampleSize; j++)
        values[j] = distribution.Value.Quantile(rng.Uniform());
      var sample = new Sample(values);
      var bounds = Toolkit.CenterBounds(sample, new Probability(misrate));

      if (bounds.Lower <= trueValue && trueValue <= bounds.Upper)
        coverage++;

      if (i % 1000 == 0)
        progressCallback((i + 1.0) / sampleCount);
    }

    double observedMisrate = 1.0 - (double)coverage / sampleCount;
    return new SimulationRow(distribution.Name, sampleSize, misrate, observedMisrate);
  }
}
