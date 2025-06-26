using Common.Io;
using Entry.Core;
using JetBrains.Annotations;

namespace Entry.Go;

public class GoCommandBase(string arguments) : RunExecutableCommand
{
    protected override FilePath ExecutableFilePath => Nav.GoBuildSh;
    protected override string Arguments => arguments;
}

[UsedImplicitly]
public class GoBuildCommand(): GoCommandBase("build");

[UsedImplicitly]
public class GoTestCommand(): GoCommandBase("test");

[UsedImplicitly]
public class GoAllCommand(): GoCommandBase("all");
