using Pragmastat.Distributions;
using Pragmastat.Internal;

namespace Pragmastat.TestGenerator.Framework.Distributions;

/// <summary>
/// Generic distribution controller that automatically handles serialization/deserialization.
/// </summary>
public class DistributionController<TDistribution>(string name, double eps = 1e-9)
  : DistributionController(name, parameters => CtorArgumentSerializer.Deserialize<TDistribution>(parameters), eps)
  where TDistribution : IContinuousDistribution
{
}

