using Pragmastat.Distributions.Randomization;

namespace Pragmastat.Distributions.Helpers;

public static class DistributionExtensions
{
    public static RandomGenerator Random(this IContinuousDistribution distribution, int seed)
        => distribution.Random(new Random(seed));
}