using System.ComponentModel;
using System.Globalization;
using System.Text.Json.Serialization;
using JetBrains.Annotations;
using Pragmastat.Algorithms;
using Pragmastat.Distributions;
using Pragmastat.Functions;
using Pragmastat.Internal;
using Pragmastat.Simulations.Misc;
using Spectre.Console.Cli;

namespace Pragmastat.Simulations.Simulations;

[UsedImplicitly]
public class CoverageSimulation : SimulationBase<CoverageSimulation.Settings, CoverageSimulation.Input,
  CoverageSimulation.SimulationRow>
{
  public const string Name = "bounds-2s-coverage";

  // Distributions that produce only positive values (required for RatioBounds)
  private static readonly HashSet<string> PositiveDistributions =
    new(["Multiplic", "Exp", "Uniform"], StringComparer.OrdinalIgnoreCase);

  [UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
  public sealed class Settings : BaseSettings
  {
    [CommandOption("-n|--sample-sizes")]
    [Description("List of sample size (example: `2,3,4,5,10..20,50..100`)")]
    [DefaultValue("2..50,60,70,80,90,100")]
    public override string? SampleSizes { get; set; }

    [CommandOption("-m|--sample-count")]
    [Description("Number of samples for building sampling distribution")]
    [DefaultValue(10_000)]
    public override int SampleCount { get; set; }

    [CommandOption("-e|--estimators")]
    [Description("List of estimators: shift, ratio")]
    [DefaultValue("shift,ratio")]
    public string? Estimators { get; set; }

    [CommandOption("-d|--distributions")]
    [Description("List of distribution conditions")]
    [DefaultValue("additive,multiplic,exp,uniform")]
    public string? Distributions { get; set; }

    [CommandOption("-r|--misrates")]
    [Description("Comma-separated list of misrates for bounds estimation")]
    [DefaultValue("1e-1,5e-2,1e-2,5e-3,1e-3")]
    public string? Misrates { get; set; }

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

  protected override string GetResultFileName() => Name;

  protected override List<Input> CreateInputsToProcess(int[] sampleSizes, Settings settings,
    Dictionary<string, SimulationRow> existingRows)
  {
    var estimators = ValidateAndParseEstimators(settings.Estimators);
    var distributions = ValidateAndParseDistributions(settings.Distributions);
    double[] misrates = ValidateAndParseMisrates(settings.Misrates);

    var inputs = new List<Input>();
    foreach (string estimator in estimators)
      foreach (var distribution in distributions)
        foreach (int sampleSize in sampleSizes)
          foreach (double misrate in misrates)
          {
            // Skip invalid combinations: ratio requires positive distributions
            if (!IsValidCombination(estimator, distribution.Name))
              continue;

            var key = $"{estimator}-{distribution.Name}-{sampleSize}-{misrate}";
            if (settings.Overwrite || !existingRows.ContainsKey(key))
            {
              inputs.Add(new Input(estimator, distribution, settings.SampleCount, sampleSize, misrate, settings.Seed));
            }
          }

    return inputs;
  }

  private static bool IsValidCombination(string estimator, string distribution)
  {
    // RatioBounds requires positive distributions
    if (estimator == "ratio" && !PositiveDistributions.Contains(distribution))
      return false;

    return true;
  }

  protected override SimulationRow SimulateRow(Input input, Action<double> progressCallback)
  {
    (string estimator, var distribution, int sampleCount, int sampleSize, double misrate, int baseSeed) = input;
    var random = distribution.Value.Random(baseSeed + sampleSize);

    // True value: Shift of identical distributions is 0, Ratio is 1
    bool isRatio = estimator == "ratio";
    double trueValue = isRatio ? 1.0 : 0.0;

    int n = sampleSize, m = sampleSize;
    long total = (long)n * m;

    // Precompute margin and quantile positions once (avoids redundant O(u^2) recurrence per iteration)
    int margin = PairwiseMargin.Instance.Calc(n, m, misrate);
    long halfMargin = Math.Min(margin / 2L, (total - 1) / 2);
    long kLeft = halfMargin;
    long kRight = total - 1 - halfMargin;
    double denominator = total - 1 > 0 ? total - 1 : 1;
    double[] p = [kLeft / denominator, kRight / denominator];

    int coverage = 0;
    for (int i = 0; i < sampleCount; i++)
    {
      var x = random.NextSample(sampleSize);
      var y = random.NextSample(sampleSize);

      double lower, upper;
      if (total == 1)
      {
        double value = isRatio
          ? Math.Exp(Math.Log(x.Values[0]) - Math.Log(y.Values[0]))
          : x.Values[0] - y.Values[0];
        lower = upper = value;
      }
      else if (isRatio)
      {
        var logX = x.Log();
        var logY = y.Log();
        double[] bounds = FastShift.Estimate(logX.SortedValues, logY.SortedValues, p, assumeSorted: true);
        lower = Math.Exp(Math.Min(bounds[0], bounds[1]));
        upper = Math.Exp(Math.Max(bounds[0], bounds[1]));
      }
      else
      {
        double[] bounds = FastShift.Estimate(x.SortedValues, y.SortedValues, p, assumeSorted: true);
        lower = Math.Min(bounds[0], bounds[1]);
        upper = Math.Max(bounds[0], bounds[1]);
      }

      if (lower <= trueValue && trueValue <= upper)
        coverage++;

      if (i % 1000 == 0)
        progressCallback((i + 1.0) / sampleCount);
    }

    double observedMisrate = 1.0 - (double)coverage / sampleCount;
    return new SimulationRow(estimator, distribution.Name, sampleSize, misrate, observedMisrate);
  }

  protected override string FormatRowStats(SimulationRow row)
  {
    string est = row.Estimator.PadRight(6);
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

  private static string[] ValidateAndParseEstimators(string? estimatorsString)
  {
    if (string.IsNullOrWhiteSpace(estimatorsString))
      throw new ArgumentException("No estimators provided");

    var valid = new HashSet<string> { "shift", "ratio" };
    var estimators = estimatorsString.Split(',', StringSplitOptions.RemoveEmptyEntries)
      .Select(s => s.Trim().ToLowerInvariant())
      .Where(s => valid.Contains(s))
      .ToArray();

    if (estimators.Length == 0)
      throw new ArgumentException($"No valid estimators in '{estimatorsString}'");

    return estimators;
  }

  private IReadOnlyList<Named<IContinuousDistribution>> ValidateAndParseDistributions(string? distributionsString)
  {
    var distributions = Registries.Distributions.ParseCommandSeparatedNames(distributionsString);
    if (distributions.IsEmpty())
      throw new ArgumentException("No distributions provided");
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

    if (misrates.IsEmpty())
      throw new ArgumentException($"Failed to parse misrates from '{misratesString}'");

    return misrates.ToArray();
  }

  public record Input(
    string Estimator,
    Named<IContinuousDistribution> Distribution,
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
