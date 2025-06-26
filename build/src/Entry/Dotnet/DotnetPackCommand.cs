using System.Text;
using Common;
using Common.Extensions;
using Entry.Core;
using JetBrains.Annotations;
using Spectre.Console.Cli;

namespace Entry.Dotnet;

[UsedImplicitly]
public class DotnetPackCommand : AsyncCommand<DotnetPackCommand.Settings>
{
    [UsedImplicitly(ImplicitUseTargetFlags.WithMembers)]
    public sealed class Settings : CommandSettings
    {
        [CommandOption("--release")]
        public bool Release { get; set; }

        [CommandOption("--push")]
        public bool Push { get; set; }

        [CommandOption("--output-script-path")]
        public string OutputScriptPath { get; set; } = "";
    }

    private static readonly Navigator Nav = Navigator.Default;

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        Nav.DotnetArtifacts.InitBlank();
        var script = new StringBuilder();

        var baseArguments =
            $"--configuration Release " +
            $"--include-symbols " +
            $"--include-source " +
            $"-p:SymbolPackageFormat=snupkg " +
            $"--output {Nav.DotnetArtifacts}";
        var labelArguments = settings.Release ? "/p:PrereleaseLabel=" : "";
        foreach (var csProjFile in Nav.DotnetNuGetCsProjFiles)
            script.AppendLine($"\"{Nav.DotNetSdk}\" pack \"{csProjFile}\" {baseArguments} {labelArguments}");

        if (settings.Push)
        {
            var nuGetApiKey = Environment.GetEnvironmentVariable("NUGET_API_KEY");
            if (nuGetApiKey.IsBlank())
            {
                Term.Error("Please, provide `NUGET_API_KEY` environment variable.");
                return -1;
            }

            var nugetArgs = $"--api-key {nuGetApiKey} --source https://api.nuget.org/v3/index.json";
            script.AppendLine($"\"{Nav.DotNetSdk}\" nuget push \"{Nav.DotnetArtifacts}/*.nupkg\" {nugetArgs}");
            script.AppendLine($"\"{Nav.DotNetSdk}\" nuget push \"{Nav.DotnetArtifacts}/*.snupkg\" {nugetArgs}");
        }

        await Helper.RunScriptAsync(settings.OutputScriptPath, script.ToString());
        return 0;
    }
}