using JetBrains.Annotations;
using Pragmastat.Distributions;
using Pragmastat.Distributions.Randomization;

namespace Pragmastat.ReferenceTests.ReferenceTesting.OneSample;

[PublicAPI]
public class OneSampleInputBuilder : ReferenceTestCaseInputBuilder<OneSampleInput>
{
  private const int DefaultCount = 3;
  private const bool DefaultWeighted = false;

  private int seed;

  private OneSampleInputBuilder AddRandomSamples(
    string name,
    AbstractRandomGenerator valueGenerator,
    int[] sizes,
    int count,
    bool weighted = true)
  {
    foreach (int n in sizes)
    {
      for (int iteration = 1; iteration <= count; iteration++)
      {
        string suffix = count == 1 ? "" : $"_{iteration}";
        var sample = new Sample(valueGenerator.Next(n));
        Add($"{name}-{n}{suffix}", sample);

        if (weighted)
        {
          var weightGenerator = UniformDistribution.Standard.Random(seed++);
          double[] values = valueGenerator.Next(n);
          double[] weights = weightGenerator.Next(n);
          var weightedSample = new Sample(values, weights);
          Add($"{name}-weighted-{n}{suffix}", weightedSample);
        }
      }
    }

    return this;
  }

  private OneSampleInputBuilder AddDistributionSamples(
    string distributionName,
    IContinuousDistribution distribution,
    int[] sizes,
    int count,
    bool weighted = true)
  {
    AddRandomSamples(distributionName, distribution.Random(seed++), sizes, count, weighted);
    return this;
  }

  public OneSampleInputBuilder Add(string name, Sample sample)
  {
    Add(name, new OneSampleInput(sample));
    return this;
  }

  public OneSampleInputBuilder AddNatural(int[] sizes)
  {
    foreach (int n in sizes)
    {
      double[] values = Enumerable.Range(1, n).Select(x => x * 1.0).ToArray();
      Add($"natural-{n}", new Sample(values));
    }

    return this;
  }

  public OneSampleInputBuilder AddZero(int[] sizes)
  {
    foreach (int n in sizes)
    {
      double[] values = Enumerable.Repeat(0.0, n).ToArray();
      var sample = new Sample(values);
      Add($"zeros-{n}", sample);
    }

    return this;
  }

  public OneSampleInputBuilder AddNormal(int[] sizes, int count = DefaultCount, bool weighted = DefaultWeighted)
  {
    return AddDistributionSamples("normal", new AdditiveDistribution(10), sizes, count, weighted);
  }

  public OneSampleInputBuilder AddUniform(int[] sizes, int count = DefaultCount, bool weighted = DefaultWeighted)
  {
    return AddDistributionSamples("uniform", UniformDistribution.Standard, sizes, count, weighted);
  }
}
