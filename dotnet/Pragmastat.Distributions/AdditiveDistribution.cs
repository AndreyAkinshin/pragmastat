using JetBrains.Annotations;
using Pragmastat.Core;
using Pragmastat.Core.Functions;
using Pragmastat.Core.Internal;
using Pragmastat.Distributions.Randomization;

namespace Pragmastat.Distributions;

/// <summary>
/// Additive distribution ('Normal' or 'Gaussian' in traditional statistics)
/// </summary>
public class AdditiveDistribution : IContinuousDistribution
{
    public static readonly AdditiveDistribution Standard = new();

    [PublicAPI] public double Mean { get; }
    [PublicAPI] public double StdDev { get; }

    public AdditiveDistribution(double mean = 0, double stdDev = 1)
    {
        Assertion.Positive(nameof(stdDev), stdDev);

        Mean = mean;
        StdDev = stdDev;
    }

    public double Pdf(double x) => Exp(-((x - Mean) / StdDev).Sqr() / 2) / (StdDev * Sqrt(2 * PI));

    public double Cdf(double x) => AcmAlgorithm209.Gauss((x - Mean) / StdDev);

    public double Quantile(Probability p) => p.Value switch
    {
        0 => double.NegativeInfinity,
        1 => double.PositiveInfinity,
        _ => Mean + StdDev * Constants.Sqrt2 * ErrorFunction.InverseValue(2 * p - 1)
    };

    private class RandomGenerator(Random? random, AdditiveDistribution distribution) : AbstractRandomGenerator(random)
    {
        public override double Next() =>
            BoxMullerTransform.Apply(distribution.Mean, distribution.StdDev, () => Random.NextDouble());
    }

    public AbstractRandomGenerator Random(Random? random = null) => new RandomGenerator(random, this);

    public double? AsymptoticSpread => 0.9538725524 * StdDev;

    public override string ToString() => $"Additive({Mean.ToStringInvariant()},{StdDev.ToStringInvariant()})";
}