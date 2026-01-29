using System.ComponentModel;
using System.Globalization;
using System.Text.Json.Serialization;
using JetBrains.Annotations;
using Pragmastat.Distributions;
using Pragmastat.Internal;
using Pragmastat.Simulations.Misc;
using Spectre.Console.Cli;

namespace Pragmastat.Simulations.Simulations;

[UsedImplicitly]
public class CoverageSimulation : SimulationBase<CoverageSimulation.Settings, CoverageSimulation.Input,
  CoverageSimulation.SimulationRow>
{
  public const string Name = "coverage";

  [UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
  public sealed class Settings : BaseSettings
  {
    [CommandOption("-n|--sample-sizes")]
    [Description("List of sample size (example: `2,3,4,5,10..20,50..100`)")]
    [DefaultValue("2..30")]
    public override string? SampleSizes { get; set; }

    [CommandOption("-m|--sample-count")]
    [Description("How much samples will be used for building sampling distribution")]
    [DefaultValue(10_000_000)]
    public override int SampleCount { get; set; }

    [CommandOption("-d|--distributions")]
    [Description("List of distribution conditions")]
    [DefaultValue("additive,multiplic,exp,uniform")]
    public string? Distributions { get; set; }

    [CommandOption("-r|--misrates")]
    [Description("Comma-separated list of misrates for bounds estimation")]
    [DefaultValue("1e-2,1e-3,1e-4")]
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
    var distributions = ValidateAndParseDistributions(settings.Distributions);
    double[] misrates = ValidateAndParseMisrates(settings.Misrates);

    var inputs = new List<Input>();
    foreach (var distribution in distributions)
      foreach (int sampleSize in sampleSizes)
        foreach (double misrate in misrates)
        {
          var key = $"{distribution.Name}-{sampleSize}-{misrate}";
          if (settings.Overwrite || !existingRows.ContainsKey(key))
          {
            inputs.Add(new Input(distribution, settings.SampleCount, sampleSize, misrate, settings.Seed));
          }
        }

    return inputs;
  }

  protected override SimulationRow SimulateRow(Input input, Action<double> progressCallback)
  {
    (var distribution, int sampleCount, int sampleSize, double misrate, int baseSeed) = input;
    var random = distribution.Value.Random(baseSeed + sampleSize);

    int coverage = 0;
    for (int i = 0; i < sampleCount; i++)
    {
      var x = random.NextSample(sampleSize);
      var y = random.NextSample(sampleSize);
      var bounds = Toolkit.ShiftBounds(x, y, misrate);
      if (bounds.Contains(0))
        coverage++;

      progressCallback((i + 1.0) / sampleCount);
    }

    double observedMisrate = 1.0 - (double)coverage / sampleCount;
    return new SimulationRow(distribution.Name, sampleSize, misrate, observedMisrate);
  }

  protected override string FormatRowStats(SimulationRow row)
  {
    string dist = row.Distribution.PadRight(9);
    string n = row.SampleSize.ToString().PadRight(3);
    string requested = row.RequestedMisrate.ToString("G9").PadRight(11);
    string observed = row.ObservedMisrate.ToString("G9").PadRight(11);
    return $"[green]{dist} N={n}[/] Requested: {requested} Observed: {observed}";
  }

  protected override SimulationRow RoundRow(SimulationRow row, int digits)
  {
    return row with
    {
      RequestedMisrate = Math.Round(row.RequestedMisrate, digits),
      ObservedMisrate = Math.Round(row.ObservedMisrate, digits)
    };
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
    Named<IContinuousDistribution> Distribution,
    int SampleCount,
    int SampleSize,
    double Misrate,
    int BaseSeed);

  [UsedImplicitly(ImplicitUseTargetFlags.WithMembers)] // Fields are used in serialization
  public record SimulationRow(
    string Distribution,
    int SampleSize,
    double RequestedMisrate,
    double ObservedMisrate) : ISimulationRow, IComparable<SimulationRow>
  {
    [JsonIgnore] public string Key => $"{Distribution}-{SampleSize}-{RequestedMisrate}";

    public int CompareTo(SimulationRow? other)
    {
      if (other is null) return 1;

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
