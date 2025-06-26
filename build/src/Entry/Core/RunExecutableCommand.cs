using Common.Io;

namespace Entry.Core;

public abstract class RunExecutableCommand : RunScriptCommand
{
    protected abstract FilePath ExecutableFilePath { get; }
    protected abstract string Arguments { get; }

    protected override string GetScriptBody() =>
        $"""
         cd "{ExecutableFilePath.Parent?.FullPath}"
         {ExecutableFilePath.FullPath} {Arguments}
         """;
}