using Pragmastat.Core;
using Pragmastat.Core.Internal;
using Pragmastat.Distributions.Randomization;

namespace Pragmastat.Distributions;

public class UniformDistribution : IContinuousDistribution
{
    public static readonly UniformDistribution Standard = new(0, 1);

    /// <summary>
    /// The minimum value of the uniform distribution
    /// </summary>
    public double Min { get; }

    /// <summary>
    /// The maximum value of the uniform distribution
    /// </summary>
    public double Max { get; }

    public UniformDistribution(double min, double max)
    {
        if (min >= max)
            throw new ArgumentOutOfRangeException(nameof(min), min, $"{nameof(min)} should be less than {nameof(max)}");

        Min = min;
        Max = max;
    }

    public double Pdf(double x) => x < Min || x > Max ? 0 : 1 / (Max - Min);

    public double Cdf(double x)
    {
        if (x < Min)
            return 0;
        if (x > Max)
            return 1;
        return (x - Min) / (Max - Min);
    }

    public double Quantile(Probability p) => Min + p * (Max - Min);

    public RandomGenerator Random(Random? random = null) => new DistributionRandomGenerator(this, random);

    public double Mean => (Min + Max) / 2;
    public double Median => (Min + Max) / 2;
    public double Variance => (Max - Min).Sqr() / 12;
    public double Sd => Sqrt(Variance);
    public double Skewness => 0;

    public override string ToString() => $"Uniform({Min.ToStringInvariant()},{Max.ToStringInvariant()})";
}