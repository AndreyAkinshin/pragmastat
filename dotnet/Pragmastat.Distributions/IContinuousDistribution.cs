using Pragmastat.Core;
using Pragmastat.Distributions.Randomization;

namespace Pragmastat.Distributions;

public interface IContinuousDistribution
{
    /// <summary>
    /// Probability density function 
    /// </summary>
    double Pdf(double x);

    /// <summary>
    /// Cumulative distribution function
    /// </summary>
    double Cdf(double x);

    /// <summary>
    /// Quantile function
    /// </summary>
    double Quantile(Probability p);

    RandomGenerator Random(Random? random = null);

    double Mean { get; }
    double Median { get; }
    double Variance { get; }
    double Sd { get; }
}