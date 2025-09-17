using System.Collections.Specialized;
using System.ComponentModel;
using System.Diagnostics;
using System.Text.Json;
using System.Text.Json.Serialization;
using JetBrains.Annotations;
using Pragmastat.Core;
using Pragmastat.Core.Estimators;
using Pragmastat.Core.Internal;
using Pragmastat.Distributions;
using Pragmastat.Distributions.Randomization;
using Pragmastat.Estimators;
using Pragmastat.Simulations.Misc;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Pragmastat.Simulations.Simulations;

public abstract class DriftSimulationBase : AsyncCommand<DriftSimulationBase.Settings>
{
    protected abstract string GetResultFileName();

    protected virtual NameRegistry<IContinuousDistribution> DistributionRegistry => Registries.Distributions;
    protected abstract NameRegistry<IOneSampleEstimator> EstimatorRegistry { get; }

    [UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
    public sealed class Settings : CommandSettings
    {
        [CommandOption("-n|--sample-sizes")]
        [Description("List of sample size (example: `2,3,4,5,10..20,50..100`)")]
        [DefaultValue("2..100")]
        public string? SampleSizes { get; set; }

        [CommandOption("-m|--sample-count")]
        [Description("How much samples will be used for building sampling distribution")]
        [DefaultValue(10_000_000)]
        public int SampleCount { get; set; }

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
        public int Seed { get; set; }

        [CommandOption("-p|--parallelism")]
        [Description("Max degree of parallelism")]
        [DefaultValue(8)]
        public int Parallelism { get; set; }

        [CommandOption("-o|--overwrite")]
        [Description("Overwrites existing entries")]
        [DefaultValue(false)]
        public bool Overwrite { get; set; }

        [CommandOption("--publish")]
        [Description("Publishes final results to the root of the simulations folder")]
        [DefaultValue(false)]
        public bool Publish { get; set; }
    }

    private readonly Lock progressLock = new();

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        try
        {
            var estimators = ValidateAndParseEstimators(settings.Estimators);
            var distributions = ValidateAndParseDistributions(settings.Distributions);
            var sampleSizes = ValidateAndParseSampleSizes(settings.SampleSizes);

            var existingRows = await LoadExistingRows(settings);
            var inputs = CreateInputsToProcess(distributions, sampleSizes, estimators, settings, existingRows);

            if (inputs.IsEmpty())
            {
                AnsiConsole.MarkupLine("[green]All simulations already exist, nothing to process.[/]");
                return 0;
            }

            await RunAllSimulations(settings, inputs, existingRows);
        }
        catch (Exception ex)
        {
            Logger.Error(ex.Message);
            return -1;
        }

        return 0;
    }

    private async Task RunAllSimulations(Settings settings, List<Input> inputs, Dictionary<string, SimulationRow> existingRows)
    {
        var globalStopwatch = Stopwatch.StartNew();
        var options = new ParallelOptions { MaxDegreeOfParallelism = settings.Parallelism };
        var progress = new SimulationProgress(inputs.Count);
        var resultWriter = new ResultWriter(settings, GetResultFileName(), existingRows);

        var simulationTask = Parallel.ForEachAsync(Enumerable.Range(0, inputs.Count), options, async (inputIndex, ct) =>
        {
            var input = inputs[inputIndex];
            await Task.Run(() =>
            {
                try
                {
                    var row = SimulateRow(input, progressValue => progress.Update(inputIndex, progressValue));
                    progress.Complete(inputIndex, row);
                    _ = Task.Run(async () => await resultWriter.WriteRowAsync(row));
                }
                catch (Exception e)
                {
                    AnsiConsole.WriteException(e);
                }
            }, ct);
        });

        await RunWithProgressDisplay(simulationTask, progress, inputs.Count);
        await resultWriter.FinalizeAsync();

        var elapsedSeconds = globalStopwatch.Elapsed.TotalSeconds.RoundToInt();
        AnsiConsole.MarkupLine($"[green]All simulations finished! Elapsed: {elapsedSeconds}sec[/]");
    }

    private SimulationRow SimulateRow(Input input, Action<double> progressCallback)
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

    private int[] ValidateAndParseSampleSizes(string? sampleSizesString)
    {
        var sampleSizes = SampleSizeParser.ParseSampleSizes(sampleSizesString);
        if (sampleSizes.IsEmpty())
            throw new ArgumentException($"Failed to parse sample sizes from '{sampleSizesString}'");
        return sampleSizes;
    }

    private async Task<Dictionary<string, SimulationRow>> LoadExistingRows(Settings settings)
    {
        string resultFilePath = GetResultFilePath(settings);
        if (!File.Exists(resultFilePath))
            return new Dictionary<string, SimulationRow>();

        string existingJsonContent = await File.ReadAllTextAsync(resultFilePath);
        var existingRowsList = JsonSerializer.Deserialize<List<SimulationRow>>(
            existingJsonContent, GlobalSettings.JsonOptions) ?? [];

        return existingRowsList.ToDictionary(row => row.Key);
    }

    private List<Input> CreateInputsToProcess(
        IReadOnlyList<Named<IContinuousDistribution>> distributions,
        int[] sampleSizes,
        IReadOnlyList<Named<IOneSampleEstimator>> estimators,
        Settings settings,
        Dictionary<string, SimulationRow> existingRows)
    {
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

    private string GetResultFilePath(Settings settings)
    {
        var resultFileDirectory = GlobalSettings.GetSimulationRoot(settings.Publish);
        if (!Directory.Exists(resultFileDirectory))
            Directory.CreateDirectory(resultFileDirectory);
        return Path.Combine(resultFileDirectory, $"{GetResultFileName()}.json");
    }

    private async Task RunWithProgressDisplay(Task simulationTask, SimulationProgress progress, int totalTasks)
    {
        var spinners = new[]
        {
            Spinner.Known.Star, Spinner.Known.Dots, Spinner.Known.Star2,
            Spinner.Known.Pipe, Spinner.Known.GrowVertical, Spinner.Known.CircleHalves, Spinner.Known.Hamburger,
        };
        const int spinnerChangeIntervalInSeconds = 10;

        AnsiConsole.MarkupLine($"[green]Started {totalTasks} simulations...[/]");

        AnsiConsole.Status()
            .Start("Thinking...", ctx =>
            {
                ctx.SpinnerStyle(Style.Parse("green"));
                var stopwatch = Stopwatch.StartNew();

                while (!simulationTask.IsCompleted)
                {
                    var spinnerIndex = stopwatch.Elapsed.Seconds / spinnerChangeIntervalInSeconds % spinners.Length;
                    ctx.Spinner(spinners[spinnerIndex]);

                    var (totalProgress, completedTasks) = progress.GetStatus();
                    ctx.Status($"Progress: {totalProgress:0.00}%, Completed: {completedTasks}/{totalTasks}");
                    ctx.Refresh();

                    Thread.Sleep(100);
                }
            });

        await simulationTask;
    }

    protected record Input(
        Named<IContinuousDistribution> Distribution,
        IReadOnlyList<Named<IOneSampleEstimator>> Estimators,
        int SampleCount,
        int SampleSize,
        int BaseSeed);

    [UsedImplicitly(ImplicitUseTargetFlags.WithMembers)] // Fields are used in serialization
    public record SimulationRow(
        string Distribution,
        int SampleSize,
        IReadOnlyDictionary<string, double> Drifts)
    {
        public SimulationRow Round(int digits)
        {
            var roundedDrifts = new OrderedDictionary<string, double>();
            foreach ((string key, double value) in Drifts)
                roundedDrifts[key] = Math.Round(value, digits);
            return this with { Drifts = roundedDrifts };
        }

        [JsonIgnore]
        public string Key => $"{Distribution}-{SampleSize}";
    }

    private class SimulationProgress
    {
        private readonly object lockObject = new();
        private readonly double[] progresses;
        private readonly SimulationRow?[] completedRows;
        private int completedCount;

        public SimulationProgress(int totalTasks)
        {
            progresses = new double[totalTasks];
            completedRows = new SimulationRow[totalTasks];
        }

        public void Update(int index, double progress)
        {
            lock (lockObject)
            {
                progresses[index] = progress;
            }
        }

        public void Complete(int index, SimulationRow row)
        {
            lock (lockObject)
            {
                progresses[index] = 1.0;
                completedRows[index] = row;
                completedCount++;

                var rowStats = row.Drifts.Select(kvp => $"{kvp.Key}: {kvp.Value:F4}").JoinToString("  ");
                AnsiConsole.MarkupLine($"[green]{row.Distribution} N={row.SampleSize}[/] {rowStats}");
            }
        }

        public (double TotalProgress, int CompletedTasks) GetStatus()
        {
            lock (lockObject)
            {
                var totalProgress = progresses.Average() * 100;
                return (totalProgress, completedCount);
            }
        }
    }

    private class ResultWriter
    {
        private readonly string resultFilePath;
        private readonly Dictionary<string, SimulationRow> allRows;
        private readonly object lockObject = new();

        public ResultWriter(Settings settings, string fileName, Dictionary<string, SimulationRow> rows)
        {
            var resultFileDirectory = GlobalSettings.GetSimulationRoot(settings.Publish);
            if (!Directory.Exists(resultFileDirectory))
                Directory.CreateDirectory(resultFileDirectory);
            resultFilePath = Path.Combine(resultFileDirectory, $"{fileName}.json");
            allRows = new Dictionary<string, SimulationRow>(rows);
        }

        public async Task WriteRowAsync(SimulationRow row)
        {
            var roundedRow = row.Round(4);

            lock (lockObject)
            {
                allRows[roundedRow.Key] = roundedRow;
            }

            await SaveCurrentStateAsync();
        }

        public async Task FinalizeAsync()
        {
            await SaveCurrentStateAsync();
            AnsiConsole.MarkupLine($"[green]Results saved: {resultFilePath}[/]");
        }

        private async Task SaveCurrentStateAsync()
        {
            List<SimulationRow> rowList;
            lock (lockObject)
            {
                rowList = allRows.Values.OrderBy(row => row.Key).ToList();
            }

            var resultJson = JsonSerializer.Serialize(rowList, GlobalSettings.JsonOptions);
            await File.WriteAllTextAsync(resultFilePath, resultJson);
        }
    }
}