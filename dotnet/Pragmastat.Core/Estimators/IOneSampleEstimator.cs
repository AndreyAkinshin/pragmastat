using Pragmastat.Core.Metrology;

namespace Pragmastat.Core.Estimators;

public interface IOneSampleEstimator
{
    Measurement Estimate(Sample x);
}