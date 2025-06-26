using JetBrains.Annotations;
using Pragmastat.Core;
using Pragmastat.Distributions;
using Pragmastat.Distributions.Helpers;
using Pragmastat.Distributions.Randomization;

namespace Pragmastat.ReferenceTests.ReferenceTesting.TwoSample;

[PublicAPI]
public class TwoSampleInputBuilder : ReferenceTestCaseInputBuilder<TwoSampleInput>
{
    private const int DefaultCount = 3;

    private int seed;

    private TwoSampleInputBuilder AddRandomSamples(
        string name,
        RandomGenerator xGenerator,
        RandomGenerator yGenerator,
        int[] xSizes,
        int[] ySizes,
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
                    Add($"{name}-{n}-{m}{suffix}", sampleX, sampleY);
                }
            }
        }

        return this;
    }

    private TwoSampleInputBuilder AddDistributionSamples(
        string distributionName,
        IContinuousDistribution xDistribution,
        IContinuousDistribution yDistribution,
        int[] xSizes,
        int[] ySizes,
        int count)
    {
        AddRandomSamples(distributionName, xDistribution.Random(seed++), yDistribution.Random(seed++), xSizes, ySizes, count);
        return this;
    }

    public TwoSampleInputBuilder Add(string name, Sample sampleX, Sample sampleY)
    {
        Add(name, new TwoSampleInput(sampleX, sampleY));
        return this;
    }

    public TwoSampleInputBuilder AddNatural(int[] xSizes, int[] ySizes)
    {
        foreach (int n in xSizes)
        {
            foreach (int m in ySizes)
            {
                double[] xValues = Enumerable.Range(1, n).Select(x => x * 1.0).ToArray();
                double[] yValues = Enumerable.Range(1, m).Select(x => x * 1.0).ToArray();
                Add($"natural-{n}-{m}", new Sample(xValues), new Sample(yValues));
            }
        }

        return this;
    }

    public TwoSampleInputBuilder AddZero(int[] xSizes, int[] ySizes)
    {
        foreach (int n in xSizes)
        {
            foreach (int m in ySizes)
            {
                double[] xValues = Enumerable.Repeat(0.0, n).ToArray();
                double[] yValues = Enumerable.Repeat(0.0, m).ToArray();
                var sampleX = new Sample(xValues);
                var sampleY = new Sample(yValues);
                Add($"zeros-{n}-{m}", sampleX, sampleY);
            }
        }

        return this;
    }

    public TwoSampleInputBuilder AddNormal(int[] xSizes, int[] ySizes, int count = DefaultCount)
    {
        return AddDistributionSamples("normal", new NormalDistribution(10), new NormalDistribution(10), xSizes, ySizes, count);
    }

    public TwoSampleInputBuilder AddUniform(int[] xSizes, int[] ySizes, int count = DefaultCount)
    {
        return AddDistributionSamples("uniform", UniformDistribution.Standard, UniformDistribution.Standard, xSizes, ySizes, count);
    }
}