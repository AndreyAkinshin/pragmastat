using Pragmastat.Core;
using Pragmastat.Core.Metrology;
using Pragmastat.Estimators;

namespace Pragmastat;

public static class SampleExtensions
{
    public static Measurement Center(this Sample x) => CenterEstimator.Instance.Estimate(x);
    public static Measurement Spread(this Sample x) => SpreadEstimator.Instance.Estimate(x);
    public static Measurement Volatility(this Sample x) => VolatilityEstimator.Instance.Estimate(x);
    public static Measurement Precision(this Sample x) => PrecisionEstimator.Instance.Estimate(x);

    public static Measurement MedShift(this Sample x, Sample y) => MedShiftEstimator.Instance.Estimate(x, y);
    public static Measurement MedRatio(this Sample x, Sample y) => MedRatioEstimator.Instance.Estimate(x, y);
    public static Measurement MedSpread(this Sample x, Sample y) => MedSpreadEstimator.Instance.Estimate(x, y);
    public static Measurement MedDisparity(this Sample x, Sample y) => MedDisparityEstimator.Instance.Estimate(x, y);
}