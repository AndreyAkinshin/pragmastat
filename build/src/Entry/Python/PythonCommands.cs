using Common;
using Common.Io;
using Entry.Core;
using JetBrains.Annotations;

namespace Entry.Python;

public class PythonCommandBase(string arguments) : RunExecutableCommand
{
    protected override FilePath ExecutableFilePath => Nav.PythonBuildSh;
    protected override string Arguments => arguments;
}

[UsedImplicitly]
public class PythonBuildCommand(): PythonCommandBase("build");

[UsedImplicitly]
public class PythonTestCommand(): PythonCommandBase("test");

[UsedImplicitly]
public class PythonCheckCommand(): PythonCommandBase("check");

[UsedImplicitly]
public class PythonCleanCommand(): PythonCommandBase("clean");