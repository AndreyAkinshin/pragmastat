using JetBrains.Annotations;
using Pragmastat.Distributions.Randomization;
using Pragmastat.Internal;

namespace Pragmastat.Distributions;

public class UniformDistribution : IContinuousDistribution
{
    public static readonly UniformDistribution Standard = new(0, 1);

    /// <summary>
    /// The minimum value of the uniform distribution
    /// </summary>
    [PublicAPI]
    public double Min { get; }

    /// <summary>
    /// The maximum value of the uniform distribution
    /// </summary>
    [PublicAPI]
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

    public AbstractRandomGenerator Random(Random? random = null) =>
        new DistributionRandomGenerator(this, random);

    public double Width => Max - Min;

    public double? AsymptoticSpread => (1 - 1.0 / Sqrt(2.0)) * Width;

    public override string ToString() => $"Uniform({Min.ToStringInvariant()},{Max.ToStringInvariant()})";
}