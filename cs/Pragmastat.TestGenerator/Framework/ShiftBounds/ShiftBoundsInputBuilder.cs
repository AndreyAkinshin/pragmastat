using JetBrains.Annotations;
using Pragmastat.Distributions;
using Pragmastat.Distributions.Randomization;

namespace Pragmastat.TestGenerator.Framework.ShiftBounds;

[PublicAPI]
public class ShiftBoundsInputBuilder : ReferenceTestCaseInputBuilder<ShiftBoundsInput>
{
  private const int DefaultCount = 3;

  private int seed;

  private ShiftBoundsInputBuilder AddRandomSamples(
    string name,
    AbstractRandomGenerator xGenerator,
    AbstractRandomGenerator yGenerator,
    int[] xSizes,
    int[] ySizes,
    double misrate,
    int count)
  {
    foreach (int n in xSizes)
    {
      foreach (int m in ySizes)
      {
        for (int iteration = 1; iteration <= count; iteration++)
        {
          string suffix = count == 1 ? "" : $"_{iteration}";
          var sampleX = new Sample(xGenerator.Next(n));
          var sampleY = new Sample(yGenerator.Next(m));
          Add($"{name}-{n}-{m}{suffix}", sampleX, sampleY, misrate);
        }
      }
    }

    return this;
  }

  private ShiftBoundsInputBuilder AddDistributionSamples(
    string distributionName,
    IContinuousDistribution xDistribution,
    IContinuousDistribution yDistribution,
    int[] xSizes,
    int[] ySizes,
    double misrate,
    int count)
  {
    AddRandomSamples(distributionName, xDistribution.Random(seed++), yDistribution.Random(seed++), xSizes, ySizes,
      misrate, count);
    return this;
  }

  public ShiftBoundsInputBuilder Add(string name, Sample sampleX, Sample sampleY, double misrate)
  {
    Add(name, new ShiftBoundsInput(sampleX, sampleY, misrate));
    return this;
  }

  public ShiftBoundsInputBuilder AddNatural(int[] xSizes, int[] ySizes, double misrate)
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

  public ShiftBoundsInputBuilder AddAdditive(int[] xSizes, int[] ySizes, double misrate, int count = DefaultCount)
  {
    return AddDistributionSamples("additive", new AdditiveDistribution(10, 1), new AdditiveDistribution(10, 1), xSizes,
      ySizes,
      misrate, count);
  }

  public ShiftBoundsInputBuilder AddUniform(int[] xSizes, int[] ySizes, double misrate, int count = DefaultCount)
  {
    return AddDistributionSamples("uniform", UniformDistribution.Standard, UniformDistribution.Standard, xSizes, ySizes,
      misrate, count);
  }

  public ShiftBoundsInputBuilder AddUnsorted(string name, Sample sampleX, Sample sampleY, double misrate)
  {
    Add($"unsorted-{name}", sampleX, sampleY, misrate);
    return this;
  }

  public ShiftBoundsInputBuilder AddUnsortedVariants(string baseName, int n, int m, double misrate)
  {
    // Create sorted natural sequences
    double[] xSorted = Enumerable.Range(1, n).Select(x => x * 1.0).ToArray();
    double[] ySorted = Enumerable.Range(1, m).Select(x => x * 1.0).ToArray();

    // Create unsorted (reversed) sequences
    double[] xUnsorted = Enumerable.Range(1, n).Select(x => (double)(n - x + 1)).ToArray();
    double[] yUnsorted = Enumerable.Range(1, m).Select(x => (double)(m - x + 1)).ToArray();

    // X unsorted, Y sorted
    Add($"unsorted-x-{baseName}-{n}-{m}", new Sample(xUnsorted), new Sample(ySorted), misrate);

    // X sorted, Y unsorted
    Add($"unsorted-y-{baseName}-{n}-{m}", new Sample(xSorted), new Sample(yUnsorted), misrate);

    // Both unsorted
    Add($"unsorted-both-{baseName}-{n}-{m}", new Sample(xUnsorted), new Sample(yUnsorted), misrate);

    return this;
  }
}

