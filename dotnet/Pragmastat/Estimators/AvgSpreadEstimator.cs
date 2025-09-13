using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Internal;
using Pragmastat.Core.Metrology;

namespace Pragmastat.Estimators;

public class AvgSpreadEstimator(IOneSampleEstimator spread) : ITwoSampleEstimator
{
    public static readonly AvgSpreadEstimator Instance = new(SpreadEstimator.Instance);

    public Measurement Estimate(Sample x, Sample y)
    {
        Assertion.MatchedUnit(x, y);
        return (x.Size * spread.Estimate(x) + y.Size * spread.Estimate(y)) / (x.Size + y.Size);
    }
}