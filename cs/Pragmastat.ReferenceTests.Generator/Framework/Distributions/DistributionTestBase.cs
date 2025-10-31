using Pragmastat.Distributions;
using Pragmastat.Internal;

namespace Pragmastat.ReferenceTests.Generator.Framework.Distributions;

public abstract class DistributionTestBase<TDistribution> : ReferenceTestBase<DistributionInput, DistributionOutput>
  where TDistribution : IContinuousDistribution
{
  protected override ReferenceTestController<DistributionInput, DistributionOutput> CreateController() =>
    new DistributionController(GetSuiteName(),
      parameters => CtorArgumentSerializer.Deserialize<TDistribution>(parameters));

  protected override ReferenceTestCaseInputBuilder<DistributionInput> GetInputBuilder() =>
    new DistributionInputBuilder();
}
