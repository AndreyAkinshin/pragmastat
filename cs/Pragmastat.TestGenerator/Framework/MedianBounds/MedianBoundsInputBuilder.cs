using JetBrains.Annotations;
using Pragmastat.Distributions;

namespace Pragmastat.TestGenerator.Framework.MedianBounds;

[PublicAPI]
public class MedianBoundsInputBuilder : ReferenceTestCaseInputBuilder<MedianBoundsInput>
{
  private const int DefaultCount = 3;
  private int seed;

  public MedianBoundsInputBuilder Add(string name, Sample sample, double misrate)
  {
    Add(name, new MedianBoundsInput(sample, misrate));
    return this;
  }

  public MedianBoundsInputBuilder AddNatural(int[] sizes, double misrate)
  {
    foreach (int n in sizes)
    {
      double[] values = Enumerable.Range(1, n).Select(x => x * 1.0).ToArray();
      Add($"natural-{n}", new Sample(values), misrate);
    }

    return this;
  }

  public MedianBoundsInputBuilder AddAdditive(int[] sizes, double misrate, int count = DefaultCount)
  {
    return AddDistributionSamples("additive", new Additive(10, 1), sizes, misrate, count);
  }

  public MedianBoundsInputBuilder AddUniform(int[] sizes, double misrate, int count = DefaultCount)
  {
    return AddDistributionSamples("uniform", Uniform.Standard, sizes, misrate, count);
  }

  public MedianBoundsInputBuilder AddUnsorted(string name, Sample sample, double misrate)
  {
    Add($"unsorted-{name}", sample, misrate);
    return this;
  }

  private MedianBoundsInputBuilder AddDistributionSamples(
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
