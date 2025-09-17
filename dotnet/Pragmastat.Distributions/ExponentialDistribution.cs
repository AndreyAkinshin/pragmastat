using JetBrains.Annotations;
using Pragmastat.Core;
using Pragmastat.Core.Internal;
using Pragmastat.Distributions.Randomization;

namespace Pragmastat.Distributions;

public class ExponentialDistribution : IContinuousDistribution
{
    [PublicAPI] public static readonly ExponentialDistribution Standard = new();

    [PublicAPI] public double Rate { get; }

    public ExponentialDistribution(double rate = 1.0)
    {
        Assertion.Positive(nameof(rate), rate);

        Rate = rate;
    }

    public double Pdf(double x)
    {
        if (x < 0)
            return 0;
        return Rate * Exp(-Rate * x);
    }

    public double Cdf(double x)
    {
        if (x < 0)
            return 0;
        return 1 - Exp(-Rate * x);
    }

    public double Quantile(Probability p) => -Log(1 - p) / Rate;

    public AbstractRandomGenerator Random(Random? random = null) => new DistributionRandomGenerator(this, random);

    public double? AsymptoticSpread => null;

    public override string ToString() => $"Exp({Rate.ToStringInvariant()})";
}