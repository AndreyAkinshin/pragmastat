using System.ComponentModel;
using System.Text.Json.Serialization;
using JetBrains.Annotations;
using Pragmastat.Distributions;
using Pragmastat.Estimators;
using Pragmastat.Internal;
using Pragmastat.Simulations.Misc;
using Spectre.Console.Cli;

namespace Pragmastat.Simulations.Simulations;

public abstract class DriftSimulationBase : SimulationBase<DriftSimulationBase.Settings, DriftSimulationBase.Input,
  DriftSimulationBase.SimulationRow>
{
  protected virtual NameRegistry<IContinuousDistribution> DistributionRegistry => Registries.Distributions;
  protected abstract NameRegistry<IOneSampleEstimator> EstimatorRegistry { get; }

  [UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
  public sealed class Settings : BaseSettings
  {
    [CommandOption("-n|--sample-sizes")]
    [Description("List of sample size (example: `2,3,4,5,10..20,50..100`)")]
    [DefaultValue("2..100")]
    public override string? SampleSizes { get; set; }

    [CommandOption("-m|--sample-count")]
    [Description("How much samples will be used for building sampling distribution")]
    [DefaultValue(10_000_000)]
    public override int SampleCount { get; set; }

    [CommandOption("-e|--estimators")]
    [Description("List of estimators to evaluate")]
    public string? Estimators { get; set; }

    [CommandOption("-d|--distributions")]
    [Description("List of distribution conditions")]
    [DefaultValue("additive,multiplic,exp,uniform")]
    public string? Distributions { get; set; }

    [CommandOption("-s|--seed")]
    [Description("Seed for generation random numbers")]
    [DefaultValue(1729)]
    public override int Seed { get; set; }

    [CommandOption("-p|--parallelism")]
    [Description("Max degree of parallelism")]
    [DefaultValue(8)]
    public override int Parallelism { get; set; }

    [CommandOption("-o|--overwrite")]
    [Description("Overwrites existing entries")]
    [DefaultValue(false)]
    public override bool Overwrite { get; set; }

    [CommandOption("--publish")]
    [Description("Publishes final results to the root of the simulations folder")]
    [DefaultValue(false)]
    public override bool Publish { get; set; }
  }

  protected override List<Input> CreateInputsToProcess(int[] sampleSizes, Settings settings,
    Dictionary<string, SimulationRow> existingRows)
  {
    var estimators = ValidateAndParseEstimators(settings.Estimators);
    var distributions = ValidateAndParseDistributions(settings.Distributions);

    var inputs = new List<Input>();
    foreach (var distribution in distributions)
      foreach (int sampleSize in sampleSizes)
      {
        var key = $"{distribution.Name}-{sampleSize}";
        if (settings.Overwrite || !existingRows.ContainsKey(key))
        {
          inputs.Add(new Input(distribution, estimators, settings.SampleCount, sampleSize, settings.Seed));
        }
      }

    return inputs;
  }

  protected override SimulationRow SimulateRow(Input input, Action<double> progressCallback)
  {
    (var distribution, var estimators, int sampleCount, int sampleSize, int baseSeed) = input;
    var random = distribution.Value.Random(baseSeed + sampleSize);

    var samplingDistributions = new Dictionary<string, double[]>();
    foreach (var estimator in estimators)
      samplingDistributions[estimator.Name] = new double[sampleCount];

    for (int i = 0; i < sampleCount; i++)
    {
      var sample = new Sample(random.Next(sampleSize));
      foreach ((string estimatorName, var estimator) in estimators)
        samplingDistributions[estimatorName][i] = estimator.Estimate(sample);
      progressCallback((i + 1.0) / sampleCount);
    }

    var drifts = new Dictionary<string, double>();
    foreach (var estimator in estimators)
    {
      var sampling = samplingDistributions[estimator.Name].ToSample();
      drifts[estimator.Name] = Drift(input, sampling);
    }

    return new SimulationRow(distribution.Name, sampleSize, drifts);
  }

  protected override string FormatRowStats(SimulationRow row)
  {
    var rowStats = row.Drifts.Select(kvp => $"{kvp.Key}: {kvp.Value:F4}").JoinToString("  ");
    return $"[green]{row.Distribution} N={row.SampleSize}[/] {rowStats}";
  }

  protected override SimulationRow RoundRow(SimulationRow row, int digits)
  {
    var roundedDrifts = new OrderedDictionary<string, double>();
    foreach ((string key, double value) in row.Drifts)
      roundedDrifts[key] = Math.Round(value, digits);
    return row with { Drifts = roundedDrifts };
  }

  protected abstract double Drift(Input input, Sample sampling);

  private readonly Dictionary<IContinuousDistribution, double> asymptoticSpreadCache = new();

  private static double EstimateAsymptoticSpread(IContinuousDistribution distribution)
  {
    const int samplingSize = 10_000_000;
    return MedianEstimator.Instance.Estimate(distribution.Random().Next(samplingSize).ToSample());
  }

  protected double GetAsymptoticSpread(IContinuousDistribution distribution)
  {
    if (distribution.AsymptoticSpread.HasValue)
      return distribution.AsymptoticSpread.Value;
    if (!asymptoticSpreadCache.ContainsKey(distribution))
      asymptoticSpreadCache[distribution] = EstimateAsymptoticSpread(distribution);
    return asymptoticSpreadCache[distribution];
  }

  private IReadOnlyList<Named<IOneSampleEstimator>> ValidateAndParseEstimators(string? estimatorsString)
  {
    var estimators = EstimatorRegistry.ParseCommandSeparatedNames(estimatorsString);
    if (estimators.IsEmpty())
      throw new ArgumentException("No estimators provided");
    return estimators;
  }

  private IReadOnlyList<Named<IContinuousDistribution>> ValidateAndParseDistributions(string? distributionsString)
  {
    var distributions = DistributionRegistry.ParseCommandSeparatedNames(distributionsString);
    if (distributions.IsEmpty())
      throw new ArgumentException("No distributions provided");
    return distributions;
  }

  public record Input(
    Named<IContinuousDistribution> Distribution,
    IReadOnlyList<Named<IOneSampleEstimator>> Estimators,
    int SampleCount,
    int SampleSize,
    int BaseSeed);

  [UsedImplicitly(ImplicitUseTargetFlags.WithMembers)] // Fields are used in serialization
  public record SimulationRow(
    string Distribution,
    int SampleSize,
    IReadOnlyDictionary<string, double> Drifts) : ISimulationRow, IComparable<SimulationRow>
  {
    [JsonIgnore] public string Key => $"{Distribution}-{SampleSize}";

    public int CompareTo(SimulationRow? other)
    {
      if (other is null) return 1;

      var distributionComparison = string.Compare(Distribution, other.Distribution, StringComparison.Ordinal);
      if (distributionComparison != 0) return distributionComparison;

      return SampleSize.CompareTo(other.SampleSize);
    }

    public int CompareTo(ISimulationRow? other)
    {
      return CompareTo(other as SimulationRow);
    }
  }
}
