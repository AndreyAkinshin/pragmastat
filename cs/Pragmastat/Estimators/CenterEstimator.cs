using Pragmastat.Algorithms;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Estimators;

public class CenterEstimator : IOneSampleEstimator
{
  public static readonly CenterEstimator Instance = new();

  public Measurement Estimate(Sample x) => FastCenterAlgorithm.Estimate(x.SortedValues, isSorted: true).WithUnitOf(x);
}
