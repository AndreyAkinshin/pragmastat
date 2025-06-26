using Common;
using Common.Extensions;
using Common.Helpers;
using Common.Io;

namespace Entry.Core;

public static class Helper
{
    public static async Task RunScriptAsync(string outputScriptPath, string script)
    {
        var callPdfBuildScript =
            $"""
             #!/bin/bash

             {script}
             """;

        await ((FilePath)outputScriptPath).WriteAllTextAsync(callPdfBuildScript);
        await ChmodHelper.SetExecutableSafeAsync(outputScriptPath);
        Term.Info($"Send execution to script via `{outputScriptPath}`");
        Term.Info("The script:");
        Term.Info(script.Split('\n').Select(line => "> " + line).JoinToString('\n'));
    }

    public static async Task SendExecutionToAsync(string outputScriptPath, FilePath buildSh, string arguments = "")
    {
        var callPdfBuildScript =
            $"""
             #!/bin/bash

             exec "{buildSh}" {arguments}
             """;

        await ((FilePath)outputScriptPath).WriteAllTextAsync(callPdfBuildScript);
        await ChmodHelper.SetExecutableSafeAsync(outputScriptPath);
        Term.Info($"Send execution to `{buildSh}` via `{outputScriptPath}`");
    }
}