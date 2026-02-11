using JetBrains.Annotations;
using Pragmastat.Distributions;

namespace Pragmastat.TestGenerator.Framework.SpreadBounds;

[PublicAPI]
public class SpreadBoundsInputBuilder : ReferenceTestCaseInputBuilder<SpreadBoundsInput>
{
  private const int DefaultCount = 3;
  private const string DefaultSeed = "spread-bounds-tests";
  private int seed;

  public SpreadBoundsInputBuilder Add(string name, Sample sample, double misrate, string? seed = null)
  {
    Add(name, new SpreadBoundsInput(sample, misrate, seed ?? DefaultSeed));
    return this;
  }

  public SpreadBoundsInputBuilder AddNatural(int[] sizes, double misrate)
  {
    foreach (int n in sizes)
    {
      double[] values = Enumerable.Range(1, n).Select(x => x * 1.0).ToArray();
      Add($"natural-{n}", new Sample(values), misrate);
    }

    return this;
  }

  public SpreadBoundsInputBuilder AddAdditive(int[] sizes, double misrate, int count = DefaultCount)
  {
    return AddDistributionSamples("additive", new Additive(10, 1), sizes, misrate, count);
  }

  public SpreadBoundsInputBuilder AddUniform(int[] sizes, double misrate, int count = DefaultCount)
  {
    return AddDistributionSamples("uniform", Uniform.Standard, sizes, misrate, count);
  }

  public SpreadBoundsInputBuilder AddUnsorted(string name, Sample sample, double misrate)
  {
    Add($"unsorted-{name}", sample, misrate);
    return this;
  }

  private SpreadBoundsInputBuilder AddDistributionSamples(
    string distributionName,
    IContinuousDistribution distribution,
    int[] sizes,
    double misrate,
    int count)
  {
    var generator = distribution.Random(seed++);
    foreach (int n in sizes)
    {
      for (int iteration = 1; iteration <= count; iteration++)
      {
        string suffix = count == 1 ? "" : $"_{iteration}";
        var sample = new Sample(generator.Next(n));
        Add($"{distributionName}-{n}{suffix}", sample, misrate);
      }
    }

    return this;
  }
}
