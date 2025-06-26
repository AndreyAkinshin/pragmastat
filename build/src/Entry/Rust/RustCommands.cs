using Common.Io;
using Entry.Core;
using JetBrains.Annotations;

namespace Entry.Rust;

public class RustCommandBase(string arguments) : RunExecutableCommand
{
    protected override FilePath ExecutableFilePath => Nav.RustBuildSh;
    protected override string Arguments => arguments;
}

[UsedImplicitly]
public class RustBuildCommand(): RustCommandBase("build");

[UsedImplicitly]
public class RustTestCommand(): RustCommandBase("test");

[UsedImplicitly]
public class RustCheckCommand(): RustCommandBase("check");

[UsedImplicitly]
public class RustCleanCommand(): RustCommandBase("clean");