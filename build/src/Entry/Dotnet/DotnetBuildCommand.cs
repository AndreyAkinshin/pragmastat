using Common;
using Entry.Core;
using JetBrains.Annotations;
using Spectre.Console.Cli;

namespace Entry.Dotnet;

[UsedImplicitly]
public class DotnetBuildCommand : AsyncCommand<DotnetBuildCommand.Settings>
{
    [UsedImplicitly]
    public sealed class Settings : CommandSettings
    {
        [CommandOption("--output-script-path")]
        public string OutputScriptPath { get; set; } = "";
    }

    private static readonly Navigator Root = Navigator.Default;

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        await Helper.RunScriptAsync(settings.OutputScriptPath,
            $"""
             "{Root.DotNetSdk}" build "{Root.PragmastatSln}" --configuration Release
             """);
        return 0;
    }
}