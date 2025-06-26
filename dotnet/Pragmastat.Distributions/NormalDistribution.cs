using Pragmastat.Core;
using Pragmastat.Core.Functions;
using Pragmastat.Core.Internal;
using Pragmastat.Distributions.Randomization;

namespace Pragmastat.Distributions;

public class NormalDistribution : IContinuousDistribution
{
    public static readonly NormalDistribution Standard = new();

    public double Mean { get; }
    public double Sd { get; }

    public NormalDistribution(double mean = 0, double sd = 1)
    {
        Assertion.Positive(nameof(sd), sd);

        Mean = mean;
        Sd = sd;
    }

    public double Pdf(double x) => Exp(-((x - Mean) / Sd).Sqr() / 2) / (Sd * Sqrt(2 * PI));

    public double Cdf(double x) => AcmAlgorithm209.Gauss((x - Mean) / Sd);

    public double Quantile(Probability p)
    {
        return p.Value switch
        {
            0 => double.NegativeInfinity,
            1 => double.PositiveInfinity,
            _ => Mean + Sd * Constants.Sqrt2 * ErrorFunction.InverseValue(2 * p - 1)
        };
    }

    public class NormalRandomGenerator : RandomGenerator
    {
        private readonly NormalDistribution distribution;

        public NormalRandomGenerator(NormalDistribution distribution)
        {
            this.distribution = distribution;
        }

        public NormalRandomGenerator(int seed, NormalDistribution distribution) : base(seed)
        {
            this.distribution = distribution;
        }

        public NormalRandomGenerator(Random? random, NormalDistribution distribution) : base(random)
        {
            this.distribution = distribution;
        }

        /// <summary>
        /// Generate next random number from the normal distribution
        /// </summary>
        /// <remarks>
        /// The method uses the Box–Muller transform.
        /// See: Box, George EP. "A note on the generation of random normal deviates." Ann. Math. Stat. 29 (1958): 610-611.
        /// </remarks>
        public override double Next() =>
            BoxMullerTransform.Transform(distribution.Mean, distribution.Sd, () => Random.NextDouble());
    }

    public RandomGenerator Random() => new NormalRandomGenerator(this);

    public RandomGenerator Random(int seed) => new NormalRandomGenerator(seed, this);

    public RandomGenerator Random(Random? random) => new NormalRandomGenerator(random, this);

    public double Median => Mean;
    public double Variance => Sd.Sqr();
    public double Skewness => 0;

    public override string ToString() => $"Normal({Mean.ToStringInvariant()},{Sd.ToStringInvariant()}^2)";
}