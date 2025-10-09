using Pragmastat.Estimators;
using Pragmastat.Metrology;

namespace Pragmastat;

public static class SampleExtensions
{
  public static Measurement Center(this Sample x) => CenterEstimator.Instance.Estimate(x);
  public static Measurement Spread(this Sample x) => SpreadEstimator.Instance.Estimate(x);
  public static Measurement RelSpread(this Sample x) => RelSpreadEstimator.Instance.Estimate(x);

  public static Measurement Shift(this Sample x, Sample y) => Toolkit.Shift(x, y);
  public static Measurement Ratio(this Sample x, Sample y) => Toolkit.Ratio(x, y);
  public static Measurement AvgSpread(this Sample x, Sample y) => Toolkit.AvgSpread(x, y);
  public static Measurement Disparity(this Sample x, Sample y) => Toolkit.Disparity(x, y);
}
