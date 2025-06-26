using Common.Io;
using Entry.Core;

namespace Entry.Kotlin;

public class KotlinBuildCommand : RunExecutableCommand
{
    protected override FilePath ExecutableFilePath => Nav.KotlinBuildSh;
    protected override string Arguments => "";
}