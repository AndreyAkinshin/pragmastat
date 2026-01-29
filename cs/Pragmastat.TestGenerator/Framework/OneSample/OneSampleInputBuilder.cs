using JetBrains.Annotations;
using Pragmastat.Distributions;

namespace Pragmastat.TestGenerator.Framework.OneSample;

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
          var weightGenerator = Uniform.Standard.Random(seed++);
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
    return AddDistributionSamples("normal", new Additive(10, 1), sizes, count, weighted);
  }

  public OneSampleInputBuilder AddAdditive(int[] sizes, int count = DefaultCount, bool weighted = DefaultWeighted)
  {
    return AddDistributionSamples("additive", new Additive(10, 1), sizes, count, weighted);
  }

  public OneSampleInputBuilder AddUniform(int[] sizes, int count = DefaultCount, bool weighted = DefaultWeighted)
  {
    return AddDistributionSamples("uniform", Uniform.Standard, sizes, count, weighted);
  }

  public OneSampleInputBuilder AddUnsortedReverse(int[] sizes)
  {
    foreach (int n in sizes)
    {
      double[] values = Enumerable.Range(1, n).Select(x => (double)(n - x + 1)).ToArray();
      Add($"unsorted-reverse-{n}", new Sample(values));
    }

    return this;
  }

  public OneSampleInputBuilder AddUnsortedShuffle(string name, params double[] values)
  {
    Add($"unsorted-{name}", new Sample(values));
    return this;
  }

  public OneSampleInputBuilder AddUnsortedPattern(string name, Sample sample)
  {
    Add($"unsorted-{name}", sample);
    return this;
  }
}
