using Pragmastat.Estimators;
using Pragmastat.Functions;
using Pragmastat.Internal;
using Pragmastat.Metrology;

namespace Pragmastat.Extended.Estimators;

public class StandardDeviationEstimator(StandardDeviationEstimator.StandardDeviationFlavor flavor) : IOneSampleEstimator
{
    public enum StandardDeviationFlavor
    {
        Uncorrected,
        Corrected,
        Unbiased
    }

    public static readonly StandardDeviationEstimator Uncorrected = new(StandardDeviationFlavor.Uncorrected);
    public static readonly StandardDeviationEstimator Corrected = new(StandardDeviationFlavor.Corrected);
    public static readonly StandardDeviationEstimator Unbiased = new(StandardDeviationFlavor.Unbiased);

    public Measurement Estimate(Sample x) => flavor switch
    {
        StandardDeviationFlavor.Uncorrected => EstimateUncorrected(x).WithUnitOf(x),
        StandardDeviationFlavor.Corrected => EstimateCorrected(x).WithUnitOf(x),
        StandardDeviationFlavor.Unbiased => EstimateUnbiased(x).WithUnitOf(x),
        _ => throw new ArgumentOutOfRangeException(nameof(flavor), flavor, null)
    };

    private static double EstimateUncorrected(Sample sample)
    {
        int n = sample.Size;
        var values = sample.Values;
        double mean = values.Average();
        double variance = values.Sum(d => Pow(d - mean, 2)) / n;
        return variance.Sqrt();
    }

    private static double EstimateCorrected(Sample sample)
    {
        int n = sample.Size;
        var values = sample.Values;
        double mean = values.Average();
        double variance = n == 1 ? 0 : values.Sum(d => Pow(d - mean, 2)) / (n - 1);
        return variance.Sqrt();
    }

    private static double EstimateUnbiased(Sample sample)
    {
        int n = sample.Size;
        double c4 = Sqrt(2.0 / (n - 1)) * GammaFunction.Value(n / 2.0) / GammaFunction.Value((n - 1) / 2.0);
        return c4 * EstimateCorrected(sample);
    }
}