using Common;
using JetBrains.Annotations;
using Spectre.Console.Cli;

namespace Entry.Core;

public abstract class RunScriptCommand : AsyncCommand<RunScriptCommand.Settings>
{
    [UsedImplicitly]
    public sealed class Settings : CommandSettings
    {
        [CommandOption("--output-script-path")]
        public string OutputScriptPath { get; set; } = "";
    }

    protected static readonly Navigator Nav = Navigator.Default;

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        await Helper.RunScriptAsync(settings.OutputScriptPath, GetScriptBody());
        return 0;
    }

    protected abstract string GetScriptBody();
}