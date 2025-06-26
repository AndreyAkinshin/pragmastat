using Common.Io;

namespace Common.Helpers;

public static class ChmodHelper
{
    public static async Task SetExecutableAsync(FilePath filePath)
    {
        if (OsHelper.IsWindows())
            throw new InvalidOperationException("This operation is available only on *nix");
        await SetExecutableSafeAsync(filePath);
    }

    public static async Task SetExecutableSafeAsync(FilePath filePath)
    {
        if (OsHelper.IsWindows())
            return;

        await ProcessHelper.RunAsync("/bin/chmod", $"+x {filePath}");
    }
}