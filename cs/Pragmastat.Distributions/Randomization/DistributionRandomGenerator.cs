namespace Pragmastat.Distributions.Randomization;

public class DistributionRandomGenerator(IContinuousDistribution distribution, Random? random = null)
    : AbstractRandomGenerator(random)
{
    public override double Next() => distribution.Quantile(Random.NextDouble());
}