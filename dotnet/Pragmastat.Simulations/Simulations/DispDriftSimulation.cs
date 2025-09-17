using JetBrains.Annotations;
using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Simulations.Misc;

namespace Pragmastat.Simulations.Simulations;

[UsedImplicitly]
public class DispDriftSimulation : DriftSimulationBase
{
    public const string Name = "disp-drift";

    protected override string GetResultFileName() => Name;
    protected override NameRegistry<IOneSampleEstimator> EstimatorRegistry => Registries.DispersionEstimators;

    protected override double Drift(Input input, Sample sampling)
    {
        int n = input.SampleSize;
        return Math.Sqrt(n) * sampling.RelSpread();
    }
}