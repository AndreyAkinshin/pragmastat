using System.ComponentModel;
using System.Diagnostics;
using Common;
using Common.Extensions;
using Common.Helpers;
using Common.Io;
using Entry.Web.Retrieve;
using JetBrains.Annotations;
using Spectre.Console.Cli;

namespace Entry.Web;

[UsedImplicitly]
[Description("Builds and runs the website")]
public class WebCommand : AsyncCommand<WebCommand.Settings>
{
    [UsedImplicitly]
    public sealed class Settings : CommandSettings
    {
        [CommandOption("-i|--init")]
        [Description("Downloads hugo and tailwind to `.bin`")]
        [DefaultValue(false)]
        public bool InitTools { get; set; }

        [CommandOption("-s|--serve")]
        [Description("Serves the hugo website")]
        [DefaultValue(false)]
        public bool Serve { get; set; }

        [CommandOption("--release")]
        public bool Release { get; set; }

        [CommandOption("--convert")]
        [Description("Convert source only without calling hugo")]
        [DefaultValue(false)]
        public bool ConvertOnly { get; set; }

        [CommandOption("--output-script-path")]
        public string OutputScriptPath { get; set; } = "";
    }

    private static readonly Navigator Nav = Navigator.Default;
    private static readonly WebConvertor Convertor = WebConvertor.Default;
    private static string ManualVersion => Convertor.ManualVersion;

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        if (!Nav.BinDir.Exists || settings.InitTools)
        {
            var code = await Init();
            if (code != 0)
                return code;
        }

        if (!settings.ConvertOnly)
            await RunTailwind();

        await Convertor.ConvertAll();

        foreach (var pdfFile in Nav.PdfDir.EnumerateFiles("*.pdf"))
            pdfFile.CopyTo(Nav.WebStaticDir);
        await Nav.WebConfigToml.WriteAllTextAsync(
            $"""
             version = "{ManualVersion}"
             date = "{DateTimeOffset.Now:yyyy-MM-dd}"
             isRelease = {settings.Release.ToString().ToLowerInvariant()}
             """);
        Nav.ImgDir.File("logo.ico").CopyTo(Nav.WebImgDir.File("favicon.ico"));

        if (!settings.ConvertOnly)
        {
            if (settings.Serve)
                await RunHugoServe(context, settings);
            else
                await BuildHugo(context, settings);
        }

        return 0;
    }

    private static async Task<int> Init()
    {
        Term.Info("# Init: Start");
        var stopwatch = Stopwatch.StartNew();

        Navigator.Default.BinDir.Delete();
        var code = await Retriever.RetrieveAsync(
            RemoteDescriptors.CreateHugo(),
            RemoteDescriptors.CreateTailwind()
        );

        Term.Info($"# Init: Finish (Elapsed: {stopwatch.ElapsedMilliseconds / 1000.0}sec)");
        return code;
    }

    private static async Task RunTailwind()
    {
        var workingDirectory = Navigator.Default.WebDir;
        var executable = Navigator.Default.BinDir.File("tailwind" + OsHelper.BinaryExtension);
        var arguments = new[]
        {
            "-i",
            "./assets/css/styles-tailwindcss.css",
            "-o",
            "./assets/css/styles.css",
            "--minify"
        };
        await ProcessHelper.RunAsync(executable, arguments.JoinToString(" "), workingDirectory);
        Term.Info("Created: assets/css/styles.css");
    }

    private static async Task RunHugoServe(CommandContext context, Settings settings)
    {
        var workingDirectory = Navigator.Default.WebDir;
        var executable = Navigator.Default.BinDir.File("hugo" + OsHelper.BinaryExtension);
        var port = "1729";
        List<string> arguments =
        [
            "" +
            "server",
            "--renderToMemory",
            "--port",
            port,
            "--liveReloadPort",
            port,
            "--forceSyncStatic",
            "--gc",
            "--watch",
            "--buildDrafts"
        ];

        var scriptLines = new List<string>
        {
            $"cd {workingDirectory}",
            $"\"{executable}\" {arguments.JoinToString(' ')}"
        };

        await ((FilePath)settings.OutputScriptPath).WriteAllTextAsync(scriptLines.JoinToString(Environment.NewLine));
        await ChmodHelper.SetExecutableSafeAsync(settings.OutputScriptPath);

        if (scriptLines.Count == 1)
            Term.Info($"Running {scriptLines.Single()}");
        else
        {
            Term.Info("Running:");
            foreach (var line in scriptLines)
                Term.Info("  " + line);
        }
    }

    private static async Task BuildHugo(CommandContext context, Settings settings)
    {
        var workingDirectory = Navigator.Default.WebDir;
        var executable = Navigator.Default.BinDir.File("hugo" + OsHelper.BinaryExtension);
        List<string> arguments =
        [
            "--minify"
        ];

        var scriptLines = new List<string>
        {
            $"cd {workingDirectory}",
            $"\"{executable}\" {arguments.JoinToString(' ')}"
        };

        await ((FilePath)settings.OutputScriptPath).WriteAllTextAsync(scriptLines.JoinToString(Environment.NewLine));
        await ChmodHelper.SetExecutableSafeAsync(settings.OutputScriptPath);

        if (scriptLines.Count == 1)
            Term.Info($"Running {scriptLines.Single()}");
        else
        {
            Term.Info("Running:");
            foreach (var line in scriptLines)
                Term.Info("  " + line);
        }
    }
}