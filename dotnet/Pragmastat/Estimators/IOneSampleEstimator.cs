using Pragmastat.Metrology;

namespace Pragmastat.Estimators;

public interface IOneSampleEstimator
{
    Measurement Estimate(Sample x);
}