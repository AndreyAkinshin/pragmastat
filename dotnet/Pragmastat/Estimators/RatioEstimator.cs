using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Functions;
using Pragmastat.Core.Internal;
using Pragmastat.Core.Metrology;
using Pragmastat.Core.Metrology.Units;

namespace Pragmastat.Estimators;

public class RatioEstimator : ITwoSampleEstimator
{
    public static readonly RatioEstimator Instance = new();

    public Measurement Estimate(Sample x, Sample y)
    {
        Assertion.MatchedUnit(x, y);
        Assertion.Positive("y", y.Values);
        var pairwise = PairwiseSampleTransformer.Transform(x, y, (xi, yj) => xi / yj);
        return MedianEstimator.Instance.Estimate(pairwise).NominalValue.WithUnit(RatioUnit.Instance);
    }
}