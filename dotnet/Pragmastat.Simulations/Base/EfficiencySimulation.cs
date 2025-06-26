using Pragmastat.Core;
using Pragmastat.Core.Internal;
using Pragmastat.Distributions;
using Pragmastat.Distributions.Helpers;

namespace Pragmastat.Simulations.Base;

public class EfficiencySimulation<TEstimator>(
    Func<TEstimator, Sample, double> estimate,
    int sampleCount = EfficiencySimulation<TEstimator>.DefaultSampleCount,
    int baseSeed = EfficiencySimulation<TEstimator>.DefaultSeed
)
    where TEstimator : notnull
{
    private const int DefaultSampleCount = 1_000_000;
    private const int DefaultSeed = 1729;

    private readonly Dictionary<string, TEstimator> estimators = new(StringComparer.OrdinalIgnoreCase);
    private readonly Dictionary<string, IContinuousDistribution> distributions = new(StringComparer.OrdinalIgnoreCase);
    private readonly HashSet<int> sampleSizes = new();

    public EfficiencySimulation<TEstimator> AddEstimator(string name, TEstimator estimator)
    {
        if (!estimators.TryAdd(name, estimator))
            throw new ArgumentException($"Estimator '{name}' is already registered");
        return this;
    }

    public EfficiencySimulation<TEstimator> AddDistribution(string name, IContinuousDistribution continuousDistribution)
    {
        if (!distributions.TryAdd(name, continuousDistribution))
            throw new ArgumentException($"Distribution '{name}' is already registered");
        return this;
    }

    public EfficiencySimulation<TEstimator> AddSampleSizes(params int[] sizes)
    {
        foreach (int sampleSize in sizes)
        {
            Assertion.Positive(nameof(sizes), sampleSize);
            sampleSizes.Add(sampleSize);
        }
        return this;
    }

    public class SimulationRow(
        string distribution,
        int sampleSize,
        IReadOnlyDictionary<string, double> relativeEfficiency)
    {
        public string Distribution { get; init; } = distribution;
        public int SampleSize { get; init; } = sampleSize;
        public IReadOnlyDictionary<string, double> RelativeEfficiency { get; init; } = relativeEfficiency;

        public SimulationRow Round(int digits)
        {
            var roundedEfficiency = new Dictionary<string, double>();
            foreach ((string key, double value) in RelativeEfficiency)
                roundedEfficiency[key] = Math.Round(value, digits);
            return new SimulationRow(Distribution, SampleSize, roundedEfficiency);
        }
    }

    public SimulationRow SimulateRow(string distributionName, int sampleSize, Action<double> progressCallback)
    {
        var random = distributions[distributionName].Random(baseSeed + sampleSize);

        var samplingDistributions = new Dictionary<string, double[]>();
        foreach (string estimatorName in estimators.Keys)
            samplingDistributions[estimatorName] = new double[sampleCount];

        for (int i = 0; i < sampleCount; i++)
        {
            var sample = new Sample(random.Next(sampleSize));
            foreach ((string estimatorName, var estimator) in estimators)
                samplingDistributions[estimatorName][i] = estimate(estimator, sample);
            progressCallback((i + 1.0) / sampleCount);
        }

        var mses = new Dictionary<string, double>();
        foreach (string estimatorName in estimators.Keys)
            mses[estimatorName] = Mse(samplingDistributions[estimatorName]);
        double minMse = mses.Values.Min();
        var relativeEfficiency = new Dictionary<string, double>();
        foreach (string estimatorName in estimators.Keys)
            relativeEfficiency[estimatorName] = minMse / mses[estimatorName];

        return new SimulationRow(distributionName, sampleSize, relativeEfficiency);
    }

    [Obsolete]
    public IEnumerable<SimulationRow> Simulate()
    {
        if (distributions.IsEmpty())
            throw new InvalidOperationException("No distributions provided");
        if (estimators.IsEmpty())
            throw new InvalidOperationException("No estimators provided");
        if (sampleSizes.IsEmpty())
            throw new InvalidOperationException("No sample sizes provided");

        foreach ((string distributionName, var distribution) in distributions)
        foreach (int sampleSize in sampleSizes)
        {
            var random = distribution.Random(baseSeed + sampleSize);
            var samplingDistributions = new Dictionary<string, double[]>();
            foreach (string estimatorName in estimators.Keys)
                samplingDistributions[estimatorName] = new double[sampleCount];

            for (int i = 0; i < sampleCount; i++)
            {
                var sample = new Sample(random.Next(sampleSize));
                foreach ((string estimatorName, var estimator) in estimators)
                    samplingDistributions[estimatorName][i] = estimate(estimator, sample);
            }

            var mses = new Dictionary<string, double>();
            foreach (string estimatorName in estimators.Keys)
                mses[estimatorName] = Mse(samplingDistributions[estimatorName]);
            double minMse = mses.Values.Min();
            var relativeEfficiency = new Dictionary<string, double>();
            foreach (string estimatorName in estimators.Keys)
                relativeEfficiency[estimatorName] = minMse / mses[estimatorName];

            var row = new SimulationRow(distributionName, sampleSize, relativeEfficiency);
            yield return row;
        }
    }

    // The mean squared error (MSE)
    private static double Mse(double[] values)
    {
        double mean = values.Average();
        return values.Sum(x => (x - mean) * (x - mean)) / values.Length;
    }
}