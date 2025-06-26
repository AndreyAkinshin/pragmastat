using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Internal;
using Pragmastat.Core.Metrology;

namespace Pragmastat.Extended.Estimators;

public class MeanEstimator : IOneSampleEstimator
{
    public static readonly MeanEstimator Instance = new();
    public Measurement Estimate(Sample x) => x.Values.Average().WithUnitOf(x);
}