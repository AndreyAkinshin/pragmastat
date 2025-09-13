using Pragmastat.Core;
using Pragmastat.Core.Metrology;
using Pragmastat.Estimators;

namespace Pragmastat;

public static class SampleExtensions
{
    public static Measurement Center(this Sample x) => CenterEstimator.Instance.Estimate(x);
    public static Measurement Spread(this Sample x) => SpreadEstimator.Instance.Estimate(x);
    public static Measurement RelSpread(this Sample x) => RelSpreadEstimator.Instance.Estimate(x);

    public static Measurement Shift(this Sample x, Sample y) => ShiftEstimator.Instance.Estimate(x, y);
    public static Measurement Ratio(this Sample x, Sample y) => RatioEstimator.Instance.Estimate(x, y);
    public static Measurement AvgSpread(this Sample x, Sample y) => AvgSpreadEstimator.Instance.Estimate(x, y);
    public static Measurement Disparity(this Sample x, Sample y) => DisparityEstimator.Instance.Estimate(x, y);
}