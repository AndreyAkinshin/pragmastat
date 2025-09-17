using Pragmastat.Algorithms;
using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Internal;
using Pragmastat.Core.Metrology;

namespace Pragmastat.Estimators;

public class CenterEstimator : IOneSampleEstimator
{
    public static readonly CenterEstimator Instance = new();

    public Measurement Estimate(Sample x) => FastCenterAlgorithm.Estimate(x.SortedValues, isSorted: true).WithUnitOf(x);
}