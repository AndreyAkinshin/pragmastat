using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Internal;
using Pragmastat.Core.Metrology;

namespace Pragmastat.Estimators;

public class MedSpreadEstimator(IOneSampleEstimator spread) : ITwoSampleEstimator
{
    public static readonly MedSpreadEstimator Instance = new(SpreadEstimator.Instance);

    public Measurement Estimate(Sample x, Sample y)
    {
        Assertion.MatchedUnit(x, y);
        return (x.Size * spread.Estimate(x) + y.Size * spread.Estimate(y)) / (x.Size + y.Size);
    }
}