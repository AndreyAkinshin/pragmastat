using System.ComponentModel;
using Common;
using Common.Helpers;
using Entry.Core;
using JetBrains.Annotations;
using Spectre.Console.Cli;

namespace Entry.Img;

[UsedImplicitly]
[Description("Builds the images")]
public class ImgCommand : AsyncCommand<ImgCommand.Settings>
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
        if (OsHelper.IsWindows())
        {
            Term.Error("img is not supported on Windows yet; call `Rscript generate-images.R` from `img/` manually");
            return -1;
        }

        await Helper.SendExecutionToAsync(settings.OutputScriptPath, Root.ImgBuildSh);
        return 0;
    }
}