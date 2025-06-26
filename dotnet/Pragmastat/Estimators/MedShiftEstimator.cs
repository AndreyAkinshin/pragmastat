using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Functions;
using Pragmastat.Core.Internal;
using Pragmastat.Core.Metrology;

namespace Pragmastat.Estimators;

public class MedShiftEstimator : ITwoSampleEstimator
{
    public static readonly MedShiftEstimator Instance = new();

    public Measurement Estimate(Sample x, Sample y)
    {
        Assertion.MatchedUnit(x, y);
        var pairwise = PairwiseSampleTransformer.Transform(x, y, (xi, yj) => xi - yj);
        return MedianEstimator.Instance.Estimate(pairwise);
    }
}