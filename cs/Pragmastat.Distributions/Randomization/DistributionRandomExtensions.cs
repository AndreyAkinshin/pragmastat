namespace Pragmastat.Distributions.Randomization;

public static class DistributionRandomExtensions
{
  public static AbstractRandomGenerator Random(this IContinuousDistribution distribution, int seed)
    => distribution.Random(new Random(seed));
}
