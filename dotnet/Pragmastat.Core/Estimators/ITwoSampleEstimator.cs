using Pragmastat.Core.Metrology;

namespace Pragmastat.Core.Estimators;

public interface ITwoSampleEstimator
{
    Measurement Estimate(Sample x, Sample y);
}