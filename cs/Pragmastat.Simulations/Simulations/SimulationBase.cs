using System.Diagnostics;
using System.Text.Json;
using System.Text.Json.Serialization;
using Pragmastat.Internal;
using Pragmastat.Simulations.Misc;
using Spectre.Console;
using Spectre.Console.Cli;

namespace Pragmastat.Simulations.Simulations;

public interface ISimulationRow : IComparable<ISimulationRow>
{
  string Key { get; }
}

public abstract class SimulationBase<TSettings, TInput, TRow> : AsyncCommand<TSettings>
  where TSettings : SimulationBase<TSettings, TInput, TRow>.BaseSettings
  where TRow : ISimulationRow, IComparable<TRow>
{
  public abstract class BaseSettings : CommandSettings
  {
    public abstract string? SampleSizes { get; set; }
    public abstract int SampleCount { get; set; }
    public abstract int Seed { get; set; }
    public abstract int Parallelism { get; set; }
    public abstract bool Overwrite { get; set; }
    public abstract bool Publish { get; set; }
  }

  protected abstract string GetResultFileName();
  protected abstract List<TInput> CreateInputsToProcess(int[] sampleSizes, TSettings settings,
    Dictionary<string, TRow> existingRows);
  protected abstract TRow SimulateRow(TInput input, Action<double> progressCallback);
  protected abstract string FormatRowStats(TRow row);
  protected abstract TRow RoundRow(TRow row, int digits);

  public override async Task<int> ExecuteAsync(CommandContext context, TSettings settings)
  {
    try
    {
      var sampleSizes = ValidateAndParseSampleSizes(settings.SampleSizes);
      var existingRows = await LoadExistingRows(settings);
      var inputs = CreateInputsToProcess(sampleSizes, settings, existingRows);

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

  private async Task RunAllSimulations(TSettings settings, List<TInput> inputs,
    Dictionary<string, TRow> existingRows)
  {
    var globalStopwatch = Stopwatch.StartNew();
    var options = new ParallelOptions { MaxDegreeOfParallelism = settings.Parallelism };
    var progress = new SimulationProgress(inputs.Count);
    var resultWriter = new ResultWriter(settings, GetResultFileName(), existingRows, RoundRow);

    var simulationTask = Parallel.ForEachAsync(Enumerable.Range(0, inputs.Count), options, async (inputIndex, ct) =>
    {
      var input = inputs[inputIndex];
      await Task.Run(() =>
      {
        try
        {
          var row = SimulateRow(input, progressValue => progress.Update(inputIndex, progressValue));
          progress.Complete(inputIndex, row, FormatRowStats(row));
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

  protected int[] ValidateAndParseSampleSizes(string? sampleSizesString)
  {
    var sampleSizes = SampleSizeParser.ParseSampleSizes(sampleSizesString);
    if (sampleSizes.IsEmpty())
      throw new ArgumentException($"Failed to parse sample sizes from '{sampleSizesString}'");
    return sampleSizes;
  }

  private async Task<Dictionary<string, TRow>> LoadExistingRows(TSettings settings)
  {
    string resultFilePath = GetResultFilePath(settings);
    if (!File.Exists(resultFilePath))
      return new Dictionary<string, TRow>();

    string existingJsonContent = await File.ReadAllTextAsync(resultFilePath);
    var existingRowsList = JsonSerializer.Deserialize<List<TRow>>(
      existingJsonContent, GlobalSettings.JsonOptions) ?? [];

    return existingRowsList.ToDictionary(row => row.Key);
  }

  private string GetResultFilePath(TSettings settings)
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
          var elapsed = stopwatch.Elapsed;
          string elapsedStr = elapsed.TotalMinutes >= 1
            ? $"{(int)elapsed.TotalMinutes}m {elapsed.Seconds:D2}s"
            : $"{elapsed.Seconds}s";
          ctx.Status($"Progress: {totalProgress:0.00}%, Completed: {completedTasks}/{totalTasks}, Elapsed: {elapsedStr}");
          ctx.Refresh();

          Thread.Sleep(100);
        }
      });

    await simulationTask;
  }

  private class SimulationProgress
  {
    private readonly object lockObject = new();
    private readonly double[] progresses;
    private readonly TRow?[] completedRows;
    private int completedCount;

    public SimulationProgress(int totalTasks)
    {
      progresses = new double[totalTasks];
      completedRows = new TRow[totalTasks];
    }

    public void Update(int index, double progress)
    {
      lock (lockObject)
      {
        progresses[index] = progress;
      }
    }

    public void Complete(int index, TRow row, string formattedStats)
    {
      lock (lockObject)
      {
        progresses[index] = 1.0;
        completedRows[index] = row;
        completedCount++;

        AnsiConsole.MarkupLine(formattedStats);
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
    private readonly Dictionary<string, TRow> allRows;
    private readonly object lockObject = new();
    private readonly Func<TRow, int, TRow> roundFunc;

    public ResultWriter(TSettings settings, string fileName, Dictionary<string, TRow> rows,
      Func<TRow, int, TRow> roundFunc)
    {
      var resultFileDirectory = GlobalSettings.GetSimulationRoot(settings.Publish);
      if (!Directory.Exists(resultFileDirectory))
        Directory.CreateDirectory(resultFileDirectory);
      resultFilePath = Path.Combine(resultFileDirectory, $"{fileName}.json");
      allRows = new Dictionary<string, TRow>(rows);
      this.roundFunc = roundFunc;
    }

    public async Task WriteRowAsync(TRow row)
    {
      var roundedRow = roundFunc(row, GetRoundingDigits());

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
      List<TRow> rowList;
      lock (lockObject)
      {
        rowList = allRows.Values.Order().ToList();
      }

      var resultJson = JsonSerializer.Serialize(rowList, GlobalSettings.JsonOptions);
      await File.WriteAllTextAsync(resultFilePath, resultJson);
    }

    protected virtual int GetRoundingDigits() => 4;
  }
}

