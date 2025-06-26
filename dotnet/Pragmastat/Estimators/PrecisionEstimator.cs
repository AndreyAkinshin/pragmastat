using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Metrology;

namespace Pragmastat.Estimators;

public class PrecisionEstimator : IOneSampleEstimator
{
    public static readonly PrecisionEstimator Instance = new();
    public Measurement Estimate(Sample x) => 2 * x.Spread() / Sqrt(x.Size);
}