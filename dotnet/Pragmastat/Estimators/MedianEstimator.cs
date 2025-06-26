using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Internal;
using Pragmastat.Core.Metrology;

namespace Pragmastat.Estimators;

public class MedianEstimator : IOneSampleEstimator
{
    public static readonly MedianEstimator Instance = new();

    public Measurement Estimate(Sample sample)
    {
        var values = sample.SortedValues;
        int n = values.Count;
        return (n % 2 == 0 ? (values[n / 2 - 1] + values[n / 2]) / 2 : values[n / 2]).WithUnitOf(sample);
    }
}