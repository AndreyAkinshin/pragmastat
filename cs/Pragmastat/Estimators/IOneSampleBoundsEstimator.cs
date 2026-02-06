namespace Pragmastat.Estimators;

public interface IOneSampleBoundsEstimator
{
  Bounds Estimate(Sample x, Probability misrate);
}
