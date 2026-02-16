using JetBrains.Annotations;
using Pragmastat.Distributions;

namespace Pragmastat.TestGenerator.Framework.DisparityBounds;

[PublicAPI]
public class DisparityBoundsInputBuilder : ReferenceTestCaseInputBuilder<DisparityBoundsInput>
{
  private const int DefaultCount = 3;
  private const string DefaultSeed = "disparity-bounds-tests";
  private int seed;

  public DisparityBoundsInputBuilder Add(
    string name,
    Sample sampleX,
    Sample sampleY,
    double misrate,
    string? seed = null)
  {
    Add(name, new DisparityBoundsInput(sampleX, sampleY, misrate, seed ?? DefaultSeed));
    return this;
  }

  public DisparityBoundsInputBuilder AddNatural(int[] xSizes, int[] ySizes, double misrate)
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

  public DisparityBoundsInputBuilder AddAdditive(int[] xSizes, int[] ySizes, double misrate, int count = DefaultCount)
  {
    return AddDistributionSamples("additive", new Additive(10, 1), new Additive(10, 1), xSizes, ySizes, misrate, count);
  }

  public DisparityBoundsInputBuilder AddUniform(int[] xSizes, int[] ySizes, double misrate, int count = DefaultCount)
  {
    return AddDistributionSamples("uniform", Uniform.Standard, Uniform.Standard, xSizes, ySizes, misrate, count);
  }

  public DisparityBoundsInputBuilder AddUnsorted(string name, Sample sampleX, Sample sampleY, double misrate)
  {
    Add($"unsorted-{name}", sampleX, sampleY, misrate);
    return this;
  }

  private DisparityBoundsInputBuilder AddDistributionSamples(
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
