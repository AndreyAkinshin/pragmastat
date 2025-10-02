using Pragmastat.Estimators;
using Pragmastat.Metrology;

namespace Pragmastat;

public static class Toolkit
{
    public static Measurement Center(Sample x) => CenterEstimator.Instance.Estimate(x);
    public static Measurement Spread(Sample x) => SpreadEstimator.Instance.Estimate(x);
    public static Measurement RelSpread(Sample x) => RelSpreadEstimator.Instance.Estimate(x);

    public static Measurement Shift(Sample x, Sample y) => ShiftEstimator.Instance.Estimate(x, y);
    public static Measurement Ratio(Sample x, Sample y) => RatioEstimator.Instance.Estimate(x, y);
    public static Measurement AvgSpread(Sample x, Sample y) => AvgSpreadEstimator.Instance.Estimate(x, y);
    public static Measurement Disparity(Sample x, Sample y) => DisparityEstimator.Instance.Estimate(x, y);
}