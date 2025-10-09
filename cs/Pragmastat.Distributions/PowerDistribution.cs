using Pragmastat.Distributions.Randomization;
using Pragmastat.Internal;

namespace Pragmastat.Distributions;

public class PowerDistribution : IContinuousDistribution
{
    public static readonly PowerDistribution Standard = new(1, 1);
    
    public double Xm { get; }
    public double Alpha { get; }

    public PowerDistribution(double xm, double alpha)
    {
        Assertion.Positive(nameof(xm), xm);
        Assertion.Positive(nameof(alpha), alpha);

        Xm = xm;
        Alpha = alpha;
    }

    public double Pdf(double x)
    {
        if (x <= Xm)
            return 0;
        return Alpha * Pow(Xm, Alpha) / Pow(x, Alpha + 1);
    }

    public double Cdf(double x)
    {
        if (x <= Xm)
            return 0;
        return 1 - Pow(Xm / x, Alpha);
    }

    public double Quantile(Probability p) => Xm * Pow(1 - p, -1 / Alpha);

    public AbstractRandomGenerator Random(Random? random = null) =>
        new DistributionRandomGenerator(this, random);

    public double? AsymptoticSpread => null;

    public override string ToString() => $"Pareto({Xm.ToStringInvariant()},{Alpha.ToStringInvariant()})";
}