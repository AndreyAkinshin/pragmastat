using Pragmastat.Simulations.Misc;
using Pragmastat.Simulations.Simulations;
using Spectre.Console.Cli;

namespace Pragmastat.Simulations;

public static class Program
{
  public static async Task<int> Main(string[] args)
  {
    try
    {
      var app = new CommandApp();
      app.Configure(config =>
      {
        config.AddCommand<AvgDriftSimulation>(AvgDriftSimulation.Name);
        config.AddCommand<DispDriftSimulation>(DispDriftSimulation.Name);
        config.AddCommand<CenterBoundsSimulation>(CenterBoundsSimulation.Name);
        config.AddCommand<MedianBoundsSimulation>(MedianBoundsSimulation.Name);
        config.AddCommand<ShiftBoundsSimulation>(ShiftBoundsSimulation.Name);
        config.AddCommand<RatioBoundsSimulation>(RatioBoundsSimulation.Name);
      });
      return await app.RunAsync(args);
    }
    catch (Exception ex)
    {
      Logger.Error($"Application failed to start: {ex.Message}");
      Logger.Error($"Stack trace: {ex.StackTrace}");
      if (ex.InnerException != null)
      {
        Logger.Error($"Inner exception: {ex.InnerException.Message}");
        Logger.Error($"Inner stack trace: {ex.InnerException.StackTrace}");
      }

      return 1;
    }
  }
}
