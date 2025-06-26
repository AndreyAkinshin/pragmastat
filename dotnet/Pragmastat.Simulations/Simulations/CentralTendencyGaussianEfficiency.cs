using System.ComponentModel;
using System.Diagnostics;
using System.Text.Json;
using JetBrains.Annotations;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Internal;
using Pragmastat.Distributions;
using Pragmastat.Estimators;
using Pragmastat.Extended.Estimators;
using Pragmastat.Simulations.Base;
using Pragmastat.Simulations.Helpers;
using Pragmastat.Simulations.Internal;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Pragmastat.Simulations.Simulations;

[UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
public class CentralTendencyGaussianEfficiency : AsyncCommand<CentralTendencyGaussianEfficiency.Settings>
{
    public const string Name = "central-tendency-gaussian-efficiency";

    public sealed class Settings : CommandSettings
    {
        [CommandOption("-n|--sample-sizes")]
        [Description("List of sample size (example: `2,3,4,5,10..20,50..100`)")]
        [DefaultValue("2..100")]
        public string? SampleSizes { get; set; }

        [CommandOption("-m|--sample-count")]
        [Description("How much samples will be used for building sampling distribution")]
        [DefaultValue(1_000_000)]
        public int SampleCount { get; set; }

        [CommandOption("-e|--estimators")]
        [Description("List of estimators to evaluate")]
        [DefaultValue("mean,median,hl")]
        public string? Estimators { get; set; }

        [CommandOption("-s|--seed")]
        [Description("Seed for generation random numbers")]
        [DefaultValue(1729)]
        public int Seed { get; set; }

        [CommandOption("-p|--parallelism")]
        [Description("Max degree of parallelism")]
        [DefaultValue(4)]
        public int Parallelism { get; set; }

        [CommandOption("--publish")]
        [Description("Publishes final results to the root of the simulations folder")]
        [DefaultValue(false)]
        public bool Publish { get; set; }
    }

    private static readonly Dictionary<string, IOneSampleEstimator> KnownEstimators =
        new(StringComparer.OrdinalIgnoreCase)
        {
            { "Mean", MeanEstimator.Instance },
            { "Median", MedianEstimator.Instance },
            { "Center", CenterEstimator.Instance }
        };

    private readonly object progressLock = new();

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var simulation = new EfficiencySimulation<IOneSampleEstimator>(
                (estimator, sample) => estimator.Estimate(sample),
                settings.SampleCount)
            .AddDistribution("Normal", NormalDistribution.Standard);

        // Add Estimators
        string[] estimatorNames = (settings.Estimators ?? "").Split(",", StringSplitOptions.RemoveEmptyEntries);
        if (estimatorNames.IsEmpty())
        {
            Logger.Error("No estimators specified");
            return -1;
        }
        foreach (string estimatorName in estimatorNames)
        {
            if (!KnownEstimators.ContainsKey(estimatorName))
            {
                Logger.Error($"Unknown estimator: {estimatorName}");
                return -1;
            }
            string canonicalName = KnownEstimators.GetOriginalKey(estimatorName);
            var estimator = KnownEstimators[estimatorName];
            simulation.AddEstimator(canonicalName, estimator);
        }

        // Add sample sizes
        int[] sampleSizes = SampleSizeParser.ParseSampleSizes(settings.SampleSizes);
        if (sampleSizes.IsEmpty())
        {
            Logger.Error($"Failed to parse sample sizes from '{settings.SampleSizes}'");
            return -1;
        }
        simulation.AddSampleSizes(sampleSizes);

        // Run
        var options = new ParallelOptions { MaxDegreeOfParallelism = settings.Parallelism };
        double[] progresses = new double[sampleSizes.Length];
        int taskFinished = 0;
        int maxSampleSizeWidth = sampleSizes.Max().ToString().Length;
        var rows = new List<EfficiencySimulation<IOneSampleEstimator>.SimulationRow>();
        var simulationTask = Parallel.ForEachAsync(sampleSizes, options, async (n, ct) =>
        {
            int progressIndex = Array.IndexOf(sampleSizes, n);
            await Task.Run(() =>
            {
                var row = simulation.SimulateRow("Normal", n, progress =>
                {
                    lock (progressLock)
                        progresses[progressIndex] = progress;
                });
                string rowStats = row.RelativeEfficiency
                    .Select(kvp => $"{kvp.Key}: {kvp.Value * 100,5:0.0}%")
                    .JoinToString("  ");
                string nStr = n.ToString().PadRight(maxSampleSizeWidth);
                lock (progressLock)
                {
                    rows.Add(row);
                    progresses[progressIndex] = 1.0;
                    taskFinished++;
                    AnsiConsole.MarkupLine($"[green]N={nStr}[/] {rowStats}");
                }
            }, ct);
        });

        var spinners = new[]
        {
            Spinner.Known.Star,
            Spinner.Known.Dots,
            Spinner.Known.Star2,
            Spinner.Known.Pipe,
            Spinner.Known.GrowVertical,
            Spinner.Known.CircleHalves,
            Spinner.Known.Hamburger,
        };
        const int spinnerChangeIntervalInSeconds = 10;

        AnsiConsole.MarkupLine($"[green]Started {sampleSizes.Length} simulations...[/]");
        AnsiConsole.Status()
            .Start("Thinking...", ctx =>
            {
                ctx.SpinnerStyle(Style.Parse("green"));

                var stopwatch = Stopwatch.StartNew();
                while (taskFinished < sampleSizes.Length)
                {
                    int spinnerIndex = stopwatch.Elapsed.Seconds / spinnerChangeIntervalInSeconds % spinners.Length;
                    ctx.Spinner(spinners[spinnerIndex]);

                    lock (progressLock)
                    {
                        double totalProgress = progresses.Average() * 100;
                        ctx.Status($"Progress: {totalProgress:0.00}%, " +
                                   $"Completed Simulations: {taskFinished}/{sampleSizes.Length}  ");
                        ctx.Refresh();
                    }
                    Thread.Sleep(100);
                }
            });
        await simulationTask;
        AnsiConsole.MarkupLine("[green]All simulations has been finished![/]");

        rows.Sort((a, b) => a.SampleSize.CompareTo(b.SampleSize));
        var roundedRows = rows.Select(r => r.Round(4)).ToList();

        string resultFileDirectory = GlobalSettings.GetSimulationRoot(settings.Publish);
        if (!Directory.Exists(resultFileDirectory))
            Directory.CreateDirectory(resultFileDirectory);
        string resultJson = JsonSerializer.Serialize(roundedRows, GlobalSettings.JsonOptions);
        string resultFilePath = Path.Combine(resultFileDirectory, $"{Name}.json");
        await File.WriteAllTextAsync(resultFilePath, resultJson);
        AnsiConsole.MarkupLine($"[green]Results are saved in: {resultFilePath}[/]");

        return 0;
    }
}