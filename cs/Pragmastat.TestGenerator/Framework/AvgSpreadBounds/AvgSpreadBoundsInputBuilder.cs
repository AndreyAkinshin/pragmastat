using JetBrains.Annotations;
using Pragmastat.Distributions;

namespace Pragmastat.TestGenerator.Framework.AvgSpreadBounds;

[PublicAPI]
public class AvgSpreadBoundsInputBuilder : ReferenceTestCaseInputBuilder<AvgSpreadBoundsInput>
{
  private const int DefaultCount = 3;
  private const string DefaultSeed = "avg-spread-bounds-tests";
  private int seed;

  public AvgSpreadBoundsInputBuilder Add(string name, Sample sampleX, Sample sampleY, double misrate, string? seed = null)
  {
    Add(name, new AvgSpreadBoundsInput(sampleX, sampleY, misrate, seed ?? DefaultSeed));
    return this;
  }

  public AvgSpreadBoundsInputBuilder AddNatural(int[] xSizes, int[] ySizes, double misrate)
  {
    foreach (int n in xSizes)
    {
      foreach (int m in ySizes)
      {
        double[] xValues = Enumerable.Range(1, n).Select(x => x * 1.0).ToArray();
        double[] yValues = Enumerable.Range(1, m).Select(x => x * 1.0).ToArray();
        Add($"natural-{n}-{m}", new Sample(xValues), new Sample(yValues), misrate);
      }
    }

    return this;
  }

  public AvgSpreadBoundsInputBuilder AddAdditive(int[] xSizes, int[] ySizes, double misrate, int count = DefaultCount)
  {
    return AddDistributionSamples("additive", new Additive(10, 1), new Additive(10, 1), xSizes, ySizes, misrate, count);
  }

  public AvgSpreadBoundsInputBuilder AddUniform(int[] xSizes, int[] ySizes, double misrate, int count = DefaultCount)
  {
    return AddDistributionSamples("uniform", Uniform.Standard, Uniform.Standard, xSizes, ySizes, misrate, count);
  }

  public AvgSpreadBoundsInputBuilder AddUnsorted(string name, Sample sampleX, Sample sampleY, double misrate)
  {
    Add($"unsorted-{name}", sampleX, sampleY, misrate);
    return this;
  }

  private AvgSpreadBoundsInputBuilder AddDistributionSamples(
    string distributionName,
    IContinuousDistribution xDistribution,
    IContinuousDistribution yDistribution,
    int[] xSizes,
    int[] ySizes,
    double misrate,
    int count)
  {
    var xGenerator = xDistribution.Random(seed++);
    var yGenerator = yDistribution.Random(seed++);
    foreach (int n in xSizes)
    {
      foreach (int m in ySizes)
      {
        for (int iteration = 1; iteration <= count; iteration++)
        {
          string suffix = count == 1 ? "" : $"_{iteration}";
          var sampleX = new Sample(xGenerator.Next(n));
          var sampleY = new Sample(yGenerator.Next(m));
          Add($"{distributionName}-{n}-{m}{suffix}", sampleX, sampleY, misrate);
        }
      }
    }

    return this;
  }
}
