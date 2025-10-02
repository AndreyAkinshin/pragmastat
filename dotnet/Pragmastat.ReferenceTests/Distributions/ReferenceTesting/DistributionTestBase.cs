using Pragmastat.Distributions;
using Pragmastat.Internal;
using Pragmastat.ReferenceTests.ReferenceTesting;

namespace Pragmastat.ReferenceTests.Distributions.ReferenceTesting;

public abstract class DistributionTestBase<TDistribution> : ReferenceTestBase<DistributionInput, DistributionOutput>
    where TDistribution : IContinuousDistribution
{
    protected override ReferenceTestController<DistributionInput, DistributionOutput> CreateController() =>
        new DistributionController(GetSuiteName(),
            parameters => CtorArgumentSerializer.Deserialize<TDistribution>(parameters));

    protected override ReferenceTestCaseInputBuilder<DistributionInput> GetInputBuilder() =>
        new DistributionInputBuilder();
}