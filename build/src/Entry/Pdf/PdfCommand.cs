using System.ComponentModel;
using Common;
using Common.Helpers;
using Entry.Core;
using JetBrains.Annotations;
using Spectre.Console.Cli;

namespace Entry.Pdf;

[UsedImplicitly]
[Description("Builds the pdf")]
public class PdfCommand : AsyncCommand<PdfCommand.Settings>
{
    [UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
    public sealed class Settings : CommandSettings
    {
        [CommandOption("--only-compose")]
        public bool OnlyCompose { get; set; }

        [CommandOption("--release")]
        public bool Release { get; set; }

        [CommandOption("--output-script-path")]
        public string OutputScriptPath { get; set; } = "";
    }

    private static readonly Navigator Root = Navigator.Default;

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var converter = new PdfConvertor(settings.Release);
        Term.Info("Building pdf...");
        await converter.ConvertAll();
        if (settings.OnlyCompose)
            return 0;

        if (OsHelper.IsWindows())
        {
            Term.Error("pdf is not supported on Windows yet; call knitr manually");
            return -1;
        }

        await Helper.SendExecutionToAsync(
            settings.OutputScriptPath,
            Root.PdfBuildSh,
            settings.Release ? "--release" : "");
        return 0;
    }
}