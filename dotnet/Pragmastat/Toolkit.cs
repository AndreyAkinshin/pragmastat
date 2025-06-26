using Pragmastat.Core;
using Pragmastat.Core.Metrology;
using Pragmastat.Estimators;

namespace Pragmastat;

public static class Toolkit
{
    public static Measurement Center(Sample x) => CenterEstimator.Instance.Estimate(x);
    public static Measurement Spread(Sample x) => SpreadEstimator.Instance.Estimate(x);
    public static Measurement Volatility(Sample x) => VolatilityEstimator.Instance.Estimate(x);
    public static Measurement Precision(Sample x) => PrecisionEstimator.Instance.Estimate(x);

    public static Measurement MedShift(Sample x, Sample y) => MedShiftEstimator.Instance.Estimate(x, y);
    public static Measurement MedRatio(Sample x, Sample y) => MedRatioEstimator.Instance.Estimate(x, y);
    public static Measurement MedSpread(Sample x, Sample y) => MedSpreadEstimator.Instance.Estimate(x, y);
    public static Measurement MedDisparity(Sample x, Sample y) => MedDisparityEstimator.Instance.Estimate(x, y);
}