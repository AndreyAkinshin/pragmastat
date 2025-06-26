using System.Runtime.InteropServices;

namespace Common.Helpers;

public static class OsHelper
{
    public static string GetOsName(string windows, string macos, string linux)
    {
        if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            return windows;
        if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
            return macos;
        return linux;
    }

    public static bool IsWindows() => RuntimeInformation.IsOSPlatform(OSPlatform.Windows);
    public static bool IsMacOs() => RuntimeInformation.IsOSPlatform(OSPlatform.OSX);
    public static bool IsLinux() => RuntimeInformation.IsOSPlatform(OSPlatform.Linux);
    public static bool IsUnix() => !IsWindows();

    public static string BinaryExtension => IsWindows() ? ".exe" : "";
}