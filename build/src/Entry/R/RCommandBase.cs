using Common;
using Entry.Core;
using Spectre.Console.Cli;

namespace Entry.R;

public abstract class RCommandBase : RunScriptCommand
{
    public override Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        var srcDir = Nav.ReferenceTestsDir;
        var destDir = Nav.RTestsDir.SubDirectory("tests");
        srcDir.CopyTo(destDir);
        Term.Info($"Copied `{srcDir}` to `{destDir}`");
        return base.ExecuteAsync(context, settings);
    }
}