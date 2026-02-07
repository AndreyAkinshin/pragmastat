using System.ComponentModel;
using System.Globalization;
using System.Text.Json.Serialization;
using JetBrains.Annotations;
using Pragmastat.Distributions;
using Pragmastat.Internal;
using Pragmastat.Simulations.Misc;
using Spectre.Console.Cli;

namespace Pragmastat.Simulations.Simulations;

public abstract class CoverageBoundsSimulationBase : SimulationBase<CoverageBoundsSimulationBase.Settings,
  CoverageBoundsSimulationBase.Input, CoverageBoundsSimulationBase.SimulationRow>
{
  [UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
  public sealed class Settings : BaseSettings
  {
    [CommandOption("-n|--sample-sizes")]
    [Description("List of sample sizes (example: `5,10,20,30,50,100`)")]
    [DefaultValue("2..50,60,70,80,90,100")]
    public override string? SampleSizes { get; set; }

    [CommandOption("-m|--sample-count")]
    [Description("Number of samples for building sampling distribution")]
    [DefaultValue(1_000_000)]
    public override int SampleCount { get; set; }

    [CommandOption("-d|--distributions")]
    [Description("List of distributions: additive, uniform, exp, multiplic")]
    [DefaultValue("additive,uniform,exp,multiplic")]
    public string? Distributions { get; set; }

    [CommandOption("-r|--misrates")]
    [Description("Comma-separated list of misrates")]
    [DefaultValue("1e-1,5e-2,1e-2,5e-3,1e-3")]
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

  protected override (List<Input> NewInputs, List<SimulationRow> ReusedRows) CreateInputsToProcess(
    int[] sampleSizes, Settings settings, Dictionary<string, SimulationRow> existingRows)
  {
    var distributions = Registries.Distributions.ParseCommandSeparatedNames(settings.Distributions);
    if (distributions.IsEmpty())
      throw new ArgumentException("No distributions provided");

    double[] misrates = ParseMisrates(settings.Misrates);

    var inputs = new List<Input>();
    var reused = new List<SimulationRow>();
    foreach (var distribution in distributions)
      foreach (int sampleSize in sampleSizes)
        foreach (double misrate in misrates)
        {
          if (!IsValidCombination(distribution.Name, sampleSize, misrate))
            continue;

          var key = $"{distribution.Name}-{sampleSize}-{misrate}";
          if (settings.Overwrite || !existingRows.ContainsKey(key))
            inputs.Add(new Input(distribution, settings.SampleCount, sampleSize, misrate, settings.Seed));
          else
            reused.Add(existingRows[key]);
        }

    return (inputs, reused);
  }

  protected abstract bool IsValidCombination(string distribution, int sampleSize, double misrate);

  protected override SimulationRow CreateErrorRow(Input input, string error)
  {
    return new SimulationRow(input.Distribution.Name, input.SampleSize, input.Misrate, null, error);
  }

  protected override string FormatRowStats(SimulationRow row)
  {
    string dist = row.Distribution.PadRight(9);
    string n = row.SampleSize.ToString().PadRight(3);
    string requested = row.RequestedMisrate.ToString("G4").PadRight(8);

    if (row.Error != null)
      return $"[yellow]{dist} N={n}[/] Req: {requested} Error: {row.Error}";

    string observed = row.ObservedMisrate!.Value.ToString("G4").PadRight(8);
    string ratio = (row.ObservedMisrate.Value / row.RequestedMisrate).ToString("F2");
    return $"[green]{dist} N={n}[/] Req: {requested} Obs: {observed} Ratio: {ratio}";
  }

  protected override SimulationRow RoundRow(SimulationRow row, int digits)
  {
    if (row.Error != null) return row;
    return row with
    {
      RequestedMisrate = Math.Round(row.RequestedMisrate, digits),
      ObservedMisrate = Math.Round(row.ObservedMisrate!.Value, digits)
    };
  }

  protected static double[] ParseMisrates(string? misratesString)
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
    Named<IContinuousDistribution> Distribution,
    int SampleCount,
    int SampleSize,
    double Misrate,
    int BaseSeed);

  [UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
  public record SimulationRow(
    string Distribution,
    int SampleSize,
    double RequestedMisrate,
    [property: JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    double? ObservedMisrate,
    [property: JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    string? Error = null) : ISimulationRow, IComparable<SimulationRow>
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
