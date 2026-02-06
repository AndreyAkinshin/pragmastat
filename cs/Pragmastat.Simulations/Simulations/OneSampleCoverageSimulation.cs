using System.ComponentModel;
using System.Globalization;
using System.Text.Json.Serialization;
using JetBrains.Annotations;
using Pragmastat.Distributions;
using Pragmastat.Estimators;
using Pragmastat.Functions;
using Pragmastat.Internal;
using Pragmastat.Randomization;
using Pragmastat.Simulations.Misc;
using Spectre.Console.Cli;

namespace Pragmastat.Simulations.Simulations;

[UsedImplicitly]
public class OneSampleCoverageSimulation : SimulationBase<OneSampleCoverageSimulation.Settings,
  OneSampleCoverageSimulation.Input, OneSampleCoverageSimulation.SimulationRow>
{
  public const string Name = "bounds-1s-coverage";

  // Distributions that are symmetric (required for CenterBounds exact coverage)
  private static readonly HashSet<string> SymmetricDistributions =
    new(["Additive", "Uniform"], StringComparer.OrdinalIgnoreCase);

  [UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
  public sealed class Settings : BaseSettings
  {
    [CommandOption("-n|--sample-sizes")]
    [Description("List of sample sizes (example: `5,10,20,30,50,100`)")]
    [DefaultValue("2..50,60,70,80,90,100")]
    public override string? SampleSizes { get; set; }

    [CommandOption("-m|--sample-count")]
    [Description("Number of samples for building sampling distribution")]
    [DefaultValue(100_000)]
    public override int SampleCount { get; set; }

    [CommandOption("-e|--estimators")]
    [Description("List of estimators: center, median, center-approx")]
    [DefaultValue("center,median,center-approx")]
    public string? Estimators { get; set; }

    [CommandOption("-d|--distributions")]
    [Description("List of distributions: additive, uniform, exp, multiplic")]
    [DefaultValue("additive,uniform,exp,multiplic")]
    public string? Distributions { get; set; }

    [CommandOption("-r|--misrates")]
    [Description("Comma-separated list of misrates")]
    [DefaultValue("1e-1,5e-2,1e-2,5e-3,1e-3,1e-4")]
    public string? Misrates { get; set; }

    [CommandOption("-s|--seed")]
    [Description("Seed for random number generation")]
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

  protected override string GetResultFileName() => Name;

  protected override List<Input> CreateInputsToProcess(int[] sampleSizes, Settings settings,
    Dictionary<string, SimulationRow> existingRows)
  {
    var estimators = ValidateAndParseEstimators(settings.Estimators);
    var distributions = ValidateAndParseDistributions(settings.Distributions);
    double[] misrates = ValidateAndParseMisrates(settings.Misrates);

    var inputs = new List<Input>();
    foreach (string estimator in estimators)
      foreach (string distribution in distributions)
        foreach (int sampleSize in sampleSizes)
          foreach (double misrate in misrates)
          {
            // Skip invalid combinations
            if (!IsValidCombination(estimator, distribution, sampleSize, misrate))
              continue;

            var key = $"{estimator}-{distribution}-{sampleSize}-{misrate}";
            if (settings.Overwrite || !existingRows.ContainsKey(key))
            {
              inputs.Add(new Input(estimator, distribution, settings.SampleCount, sampleSize, misrate, settings.Seed));
            }
          }

    return inputs;
  }

  private bool IsValidCombination(string estimator, string distribution, int sampleSize, double misrate)
  {
    // CenterBounds requires weak symmetry - only test symmetric distributions
    if (estimator == "center" && !SymmetricDistributions.Contains(distribution))
      return false;

    // Check minimum achievable misrate for exact methods
    if (estimator is "center" or "median")
    {
      double minMisrate = MinAchievableMisrate.OneSample(sampleSize);
      if (misrate < minMisrate)
        return false;
    }

    // CenterBoundsApprox has its own minimum misrate based on iterations and sample size
    if (estimator == "center-approx")
    {
      const int iterations = 10000;
      double minMisrate = Math.Max(2.0 / iterations, MinAchievableMisrate.OneSample(sampleSize));
      if (misrate < minMisrate)
        return false;
    }

    return true;
  }

  protected override SimulationRow SimulateRow(Input input, Action<double> progressCallback)
  {
    (string estimator, string distribution, int sampleCount, int sampleSize, double misrate, int baseSeed) = input;

    var rng = new Rng($"{estimator}-{distribution}-{sampleSize}-{misrate}-{baseSeed}");
    double trueValue = GetTrueValue(estimator, distribution);

    int coverage = 0;
    for (int i = 0; i < sampleCount; i++)
    {
      var values = GenerateSample(rng, distribution, sampleSize);
      var sample = new Sample(values);
      var bounds = ComputeBounds(estimator, sample, misrate, $"sim-{i}");

      if (bounds.Lower <= trueValue && trueValue <= bounds.Upper)
        coverage++;

      if (i % 10000 == 0)
        progressCallback((i + 1.0) / sampleCount);
    }

    double observedMisrate = 1.0 - (double)coverage / sampleCount;
    return new SimulationRow(estimator, distribution, sampleSize, misrate, observedMisrate);
  }

  private double GetTrueValue(string estimator, string distribution)
  {
    return (estimator, distribution.ToLowerInvariant()) switch
    {
      ("center", "additive") => 0.0,        // Additive(0,1) center = 0
      ("center", "uniform") => 0.0,         // Uniform(-1,1) center = 0
      ("center-approx", "additive") => 0.0,
      ("center-approx", "uniform") => 0.0,
      ("center-approx", "exp") => 0.8392,   // Exp(1) Hodges-Lehmann pseudomedian: (1+2m)e^(-2m) = 0.5
      ("center-approx", "multiplic") => 1.0, // Multiplic center (approximately)
      ("median", "additive") => 0.0,        // Additive(0,1) median = 0
      ("median", "uniform") => 0.0,         // Uniform(-1,1) median = 0
      ("median", "exp") => Math.Log(2),     // Exp(1) median = ln(2)
      ("median", "multiplic") => 1.0,       // Multiplic median = 1
      _ => throw new ArgumentException($"Unknown estimator/distribution: {estimator}/{distribution}")
    };
  }

  private double[] GenerateSample(Rng rng, string distribution, int n)
  {
    return distribution.ToLowerInvariant() switch
    {
      "additive" => GenerateAdditive(rng, n),
      "uniform" => GenerateUniformSymmetric(rng, n),
      "exp" => GenerateExponential(rng, n),
      "multiplic" => GenerateMultiplic(rng, n),
      _ => throw new ArgumentException($"Unknown distribution: {distribution}")
    };
  }

  private double[] GenerateAdditive(Rng rng, int n)
  {
    const int components = 12;
    var values = new double[n];
    for (int i = 0; i < n; i++)
    {
      double sum = 0;
      for (int j = 0; j < components; j++)
        sum += rng.Uniform(-0.5, 0.5);
      values[i] = sum / Math.Sqrt(components / 12.0);
    }
    return values;
  }

  private double[] GenerateUniformSymmetric(Rng rng, int n)
  {
    var values = new double[n];
    for (int i = 0; i < n; i++)
      values[i] = rng.Uniform(-1, 1);
    return values;
  }

  private double[] GenerateExponential(Rng rng, int n)
  {
    var values = new double[n];
    for (int i = 0; i < n; i++)
      values[i] = -Math.Log(1 - rng.Uniform());
    return values;
  }

  private double[] GenerateMultiplic(Rng rng, int n)
  {
    const int components = 12;
    var values = new double[n];
    for (int i = 0; i < n; i++)
    {
      double product = 1.0;
      for (int j = 0; j < components; j++)
        product *= rng.Uniform(0.5, 2.0);
      values[i] = product;
    }
    return values;
  }

  private Bounds ComputeBounds(string estimator, Sample sample, double misrate, string seed)
  {
    return estimator switch
    {
      "center" => Toolkit.CenterBounds(sample, new Probability(misrate)),
      "median" => Toolkit.MedianBounds(sample, new Probability(misrate)),
      "center-approx" => Toolkit.CenterBoundsApprox(sample, new Probability(misrate), seed),
      _ => throw new ArgumentException($"Unknown estimator: {estimator}")
    };
  }

  protected override string FormatRowStats(SimulationRow row)
  {
    string est = row.Estimator.PadRight(12);
    string dist = row.Distribution.PadRight(9);
    string n = row.SampleSize.ToString().PadRight(3);
    string requested = row.RequestedMisrate.ToString("G4").PadRight(8);
    string observed = row.ObservedMisrate.ToString("G4").PadRight(8);
    string ratio = (row.ObservedMisrate / row.RequestedMisrate).ToString("F2");
    return $"[green]{est} {dist} N={n}[/] Req: {requested} Obs: {observed} Ratio: {ratio}";
  }

  protected override SimulationRow RoundRow(SimulationRow row, int digits)
  {
    return row with
    {
      RequestedMisrate = Math.Round(row.RequestedMisrate, digits),
      ObservedMisrate = Math.Round(row.ObservedMisrate, digits)
    };
  }

  private string[] ValidateAndParseEstimators(string? estimatorsString)
  {
    if (string.IsNullOrWhiteSpace(estimatorsString))
      throw new ArgumentException("No estimators provided");

    var valid = new HashSet<string> { "center", "median", "center-approx" };
    var estimators = estimatorsString.Split(',', StringSplitOptions.RemoveEmptyEntries)
      .Select(s => s.Trim().ToLowerInvariant())
      .Where(s => valid.Contains(s))
      .ToArray();

    if (estimators.Length == 0)
      throw new ArgumentException($"No valid estimators in '{estimatorsString}'");

    return estimators;
  }

  private string[] ValidateAndParseDistributions(string? distributionsString)
  {
    if (string.IsNullOrWhiteSpace(distributionsString))
      throw new ArgumentException("No distributions provided");

    var valid = new HashSet<string>(StringComparer.OrdinalIgnoreCase) { "additive", "uniform", "exp", "multiplic" };
    var distributions = distributionsString.Split(',', StringSplitOptions.RemoveEmptyEntries)
      .Select(s => s.Trim())
      .Where(s => valid.Contains(s))
      .ToArray();

    if (distributions.Length == 0)
      throw new ArgumentException($"No valid distributions in '{distributionsString}'");

    return distributions;
  }

  private double[] ValidateAndParseMisrates(string? misratesString)
  {
    if (string.IsNullOrWhiteSpace(misratesString))
      throw new ArgumentException("No misrates provided");

    var misrates = new List<double>();
    string[] parts = misratesString.Split(',', StringSplitOptions.RemoveEmptyEntries);

    foreach (string part in parts)
    {
      string trimmed = part.Trim();
      if (double.TryParse(trimmed, NumberStyles.Float, CultureInfo.InvariantCulture, out double misrate))
      {
        if (misrate > 0 && misrate < 1)
          misrates.Add(misrate);
        else
          throw new ArgumentException($"Misrate must be between 0 and 1, got: {misrate}");
      }
      else
      {
        throw new ArgumentException($"Invalid misrate value: {trimmed}");
      }
    }

    if (misrates.Count == 0)
      throw new ArgumentException($"Failed to parse misrates from '{misratesString}'");

    return misrates.ToArray();
  }

  public record Input(
    string Estimator,
    string Distribution,
    int SampleCount,
    int SampleSize,
    double Misrate,
    int BaseSeed);

  [UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
  public record SimulationRow(
    string Estimator,
    string Distribution,
    int SampleSize,
    double RequestedMisrate,
    double ObservedMisrate) : ISimulationRow, IComparable<SimulationRow>
  {
    [JsonIgnore] public string Key => $"{Estimator}-{Distribution}-{SampleSize}-{RequestedMisrate}";

    public int CompareTo(SimulationRow? other)
    {
      if (other is null) return 1;

      var estimatorComparison = string.Compare(Estimator, other.Estimator, StringComparison.Ordinal);
      if (estimatorComparison != 0) return estimatorComparison;

      var distributionComparison = string.Compare(Distribution, other.Distribution, StringComparison.Ordinal);
      if (distributionComparison != 0) return distributionComparison;

      var sampleSizeComparison = SampleSize.CompareTo(other.SampleSize);
      if (sampleSizeComparison != 0) return sampleSizeComparison;

      return RequestedMisrate.CompareTo(other.RequestedMisrate);
    }

    public int CompareTo(ISimulationRow? other)
    {
      return CompareTo(other as SimulationRow);
    }
  }
}
