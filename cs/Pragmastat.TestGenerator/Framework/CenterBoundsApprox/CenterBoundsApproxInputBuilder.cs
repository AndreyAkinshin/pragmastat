using JetBrains.Annotations;
using Pragmastat.Distributions;

namespace Pragmastat.TestGenerator.Framework.CenterBoundsApprox;

[PublicAPI]
public class CenterBoundsApproxInputBuilder : ReferenceTestCaseInputBuilder<CenterBoundsApproxInput>
{
  private const int DefaultCount = 3;
  private int seed;

  public CenterBoundsApproxInputBuilder Add(string name, Sample sample, double misrate, string? bootstrapSeed = null)
  {
    Add(name, new CenterBoundsApproxInput(sample, misrate, bootstrapSeed));
    return this;
  }

  public CenterBoundsApproxInputBuilder AddSeeded(string name, Sample sample, double misrate, string bootstrapSeed)
  {
    Add(name, new CenterBoundsApproxInput(sample, misrate, bootstrapSeed));
    return this;
  }

  public CenterBoundsApproxInputBuilder AddNaturalSeeded(int[] sizes, double misrate, string baseSeed)
  {
    foreach (int n in sizes)
    {
      double[] values = Enumerable.Range(1, n).Select(x => x * 1.0).ToArray();
      Add($"natural-{n}", new Sample(values), misrate, $"{baseSeed}-{n}");
    }

    return this;
  }

  public CenterBoundsApproxInputBuilder AddAdditiveSeeded(int[] sizes, double misrate, string baseSeed, int count = DefaultCount)
  {
    return AddDistributionSamplesSeeded("additive", new Additive(10, 1), sizes, misrate, baseSeed, count);
  }

  public CenterBoundsApproxInputBuilder AddUniformSeeded(int[] sizes, double misrate, string baseSeed, int count = DefaultCount)
  {
    return AddDistributionSamplesSeeded("uniform", Uniform.Standard, sizes, misrate, baseSeed, count);
  }

  private CenterBoundsApproxInputBuilder AddDistributionSamplesSeeded(
    string distributionName,
    IContinuousDistribution distribution,
    int[] sizes,
    double misrate,
    string baseSeed,
    int count)
  {
    var generator = distribution.Random(seed++);
    foreach (int n in sizes)
    {
      for (int iteration = 1; iteration <= count; iteration++)
      {
        string suffix = count == 1 ? "" : $"_{iteration}";
        var sample = new Sample(generator.Next(n));
        string bootstrapSeed = $"{baseSeed}-{n}{suffix}";
        Add($"{distributionName}-{n}{suffix}", sample, misrate, bootstrapSeed);
      }
    }

    return this;
  }
}
