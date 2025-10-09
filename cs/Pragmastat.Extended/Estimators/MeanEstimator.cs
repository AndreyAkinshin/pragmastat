using Pragmastat.Estimators;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Extended.Estimators;

public class MeanEstimator : IOneSampleEstimator
{
    public static readonly MeanEstimator Instance = new();
    public Measurement Estimate(Sample x) => x.Values.Average().WithUnitOf(x);
}