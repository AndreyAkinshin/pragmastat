using Pragmastat.Core;

namespace Pragmastat.Distributions.Helpers;

public class SampleBuilder(Random random)
{
    private readonly List<double> values = [];

    public SampleBuilder(int seed) : this(new Random(seed))
    {
    }

    public SampleBuilder AddRandom(IContinuousDistribution distribution, int count)
    {
        values.AddRange(distribution.Random(random).Next(count));
        return this;
    }

    public Sample Build() => new(values);
}