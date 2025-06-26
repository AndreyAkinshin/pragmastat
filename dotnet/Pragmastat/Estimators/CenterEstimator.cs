using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Functions;
using Pragmastat.Core.Metrology;

namespace Pragmastat.Estimators;

public class CenterEstimator : IOneSampleEstimator
{
    public static readonly CenterEstimator Instance = new();

    public Measurement Estimate(Sample x)
    {
        var pairwise = PairwiseSampleTransformer.Transform(x, (xi, xj) => (xi + xj) / 2, true);
        return MedianEstimator.Instance.Estimate(pairwise);
    }
}