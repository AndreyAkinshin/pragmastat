namespace Pragmastat.Estimators;

public interface ITwoSampleBoundsEstimator
{
  Bounds Estimate(Sample x, Sample y, Probability misrate);
}
