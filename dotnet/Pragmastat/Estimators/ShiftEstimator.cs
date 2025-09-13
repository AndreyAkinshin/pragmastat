using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Functions;
using Pragmastat.Core.Internal;
using Pragmastat.Core.Metrology;

namespace Pragmastat.Estimators;

public class ShiftEstimator : ITwoSampleEstimator
{
    public static readonly ShiftEstimator Instance = new();

    public Measurement Estimate(Sample x, Sample y)
    {
        Assertion.MatchedUnit(x, y);
        var pairwise = PairwiseSampleTransformer.Transform(x, y, (xi, yj) => xi - yj);
        return MedianEstimator.Instance.Estimate(pairwise);
    }
}