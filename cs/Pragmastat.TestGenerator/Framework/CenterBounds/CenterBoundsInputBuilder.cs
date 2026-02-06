using JetBrains.Annotations;
using Pragmastat.Distributions;

namespace Pragmastat.TestGenerator.Framework.CenterBounds;

[PublicAPI]
public class CenterBoundsInputBuilder : ReferenceTestCaseInputBuilder<CenterBoundsInput>
{
  private const int DefaultCount = 3;
  private int seed;

  public CenterBoundsInputBuilder Add(string name, Sample sample, double misrate)
  {
    Add(name, new CenterBoundsInput(sample, misrate));
    return this;
  }

  public CenterBoundsInputBuilder AddNatural(int[] sizes, double misrate)
  {
    foreach (int n in sizes)
    {
      double[] values = Enumerable.Range(1, n).Select(x => x * 1.0).ToArray();
      Add($"natural-{n}", new Sample(values), misrate);
    }

    return this;
  }

  public CenterBoundsInputBuilder AddSymmetric(int[] sizes, double misrate)
  {
    foreach (int n in sizes)
    {
      double[] values = new double[n];
      for (int i = 0; i < n; i++)
        values[i] = i - (n - 1) / 2.0;
      Add($"symmetric-{n}", new Sample(values), misrate);
    }

    return this;
  }

  public CenterBoundsInputBuilder AddAdditive(int[] sizes, double misrate, int count = DefaultCount)
  {
    return AddDistributionSamples("additive", new Additive(10, 1), sizes, misrate, count);
  }

  public CenterBoundsInputBuilder AddUniform(int[] sizes, double misrate, int count = DefaultCount)
  {
    return AddDistributionSamples("uniform", Uniform.Standard, sizes, misrate, count);
  }

  public CenterBoundsInputBuilder AddUnsorted(string name, Sample sample, double misrate)
  {
    Add($"unsorted-{name}", sample, misrate);
    return this;
  }

  private CenterBoundsInputBuilder AddDistributionSamples(
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
