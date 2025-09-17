using Pragmastat.Algorithms;
using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Internal;
using Pragmastat.Core.Metrology;

namespace Pragmastat.Estimators;

public class SpreadEstimator : IOneSampleEstimator
{
    public static readonly SpreadEstimator Instance = new();

    public Measurement Estimate(Sample x)
    {
        if (x.Size == 1)
            return Measurement.Zero(x.Unit);
        return FastSpreadAlgorithm.Estimate(x.SortedValues, isSorted: true).WithUnitOf(x);
    }
}