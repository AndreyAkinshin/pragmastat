using JetBrains.Annotations;
using Pragmastat.Estimators;
using Pragmastat.Simulations.Misc;

namespace Pragmastat.Simulations.Simulations;

[UsedImplicitly]
public class AvgDriftSimulation : DriftSimulationBase
{
    public const string Name = "avg-drift";

    protected override string GetResultFileName() => Name;
    protected override NameRegistry<IOneSampleEstimator> EstimatorRegistry => Registries.AverageEstimators;

    protected override double Drift(Input input, Sample sampling)
    {
        int n = input.SampleSize;
        var distribution = input.Distribution.Value;
        return Math.Sqrt(n) * sampling.Spread() / GetAsymptoticSpread(distribution);
    }
}