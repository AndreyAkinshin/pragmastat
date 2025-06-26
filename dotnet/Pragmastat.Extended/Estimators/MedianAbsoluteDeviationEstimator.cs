using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Internal;
using Pragmastat.Core.Metrology;
using Pragmastat.Estimators;

namespace Pragmastat.Extended.Estimators;

/// <summary>
/// The median absolute deviation (MAD).
/// MAD = median(abs(x[i] - median(x)))
/// </summary>
public class MedianAbsoluteDeviationEstimator(IOneSampleEstimator medianEstimator) : IOneSampleEstimator
{
    public static readonly MedianAbsoluteDeviationEstimator Instance = new(MedianEstimator.Instance);

    public Measurement Estimate(Sample x)
    {
        Assertion.NotNull(nameof(x), x);
        if (x.Size == 1)
            return Measurement.Zero(x.Unit);

        double median = medianEstimator.Estimate(x);
        double[] deviations = new double[x.Size];
        for (int i = 0; i < x.Size; i++)
            deviations[i] = Abs(x.Values[i] - median);
        return medianEstimator.Estimate(new Sample(deviations));
    }
}