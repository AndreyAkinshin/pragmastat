using Pragmastat.Metrology;

namespace Pragmastat.Estimators;

public interface ITwoSampleEstimator
{
  Measurement Estimate(Sample x, Sample y);
}
