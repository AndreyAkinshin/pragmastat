using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Functions;
using Pragmastat.Core.Metrology;

namespace Pragmastat.Estimators;

public class SpreadEstimator : IOneSampleEstimator
{
    public static readonly SpreadEstimator Instance = new();

    public Measurement Estimate(Sample x)
    {
        if (x.Size == 1)
            return Measurement.Zero(x.Unit);
        var pairwise = PairwiseSampleTransformer.Transform(x, (xi, xj) => Abs(xi - xj), false);
        return MedianEstimator.Instance.Estimate(pairwise);
    }
}