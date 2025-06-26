using System.Runtime.InteropServices;
using Common.Helpers;

namespace Entry.Web.Retrieve;

public static class RemoteDescriptors
{
    public static RemoteDescriptor CreateHugo() => new()
    {
        // Meta
        Name = "hugo",

        // Download
        UrlTemplate =
            $"https://github.com/gohugoio/hugo/releases/download/" +
            $"v{RemoteDescriptor.VersionPlaceholder}/" +
            $"hugo_{RemoteDescriptor.VersionPlaceholder}_{RemoteDescriptor.OsPlaceholder}-" +
            $"{RemoteDescriptor.ArchPlaceholder}{RemoteDescriptor.ExtPlaceholder}",
        Version = "0.147.8",
        Os = OsHelper.GetOsName("windows", "darwin", "linux"),
        Arch = OsHelper.IsMacOs()
            ? "universal"
            : RuntimeInformation.OSArchitecture switch
            {
                Architecture.Arm64 => "arm64",
                Architecture.X64 => "amd64",
                _ => throw new NotSupportedException(
                    $"Unsupported architecture: {RuntimeInformation.OSArchitecture}")
            },
        Ext = OsHelper.IsWindows() ? ".zip" : ".tar.gz",
        DownloadDestinationFileName = $"hugo{(OsHelper.IsWindows() ? ".zip" : ".tar.gz")}",

        // Extract
        FileNameInArchive = OsHelper.IsWindows() ? "hugo.exe" : "hugo",
        FileNameToExtract = OsHelper.IsWindows() ? "hugo.exe" : "hugo"
    };

    public static RemoteDescriptor CreateTailwind() => new()
    {
        // Meta
        Name = "tailwind",

        // Download
        UrlTemplate =
            $"https://github.com/tailwindlabs/tailwindcss/releases/download/" +
            $"v{RemoteDescriptor.VersionPlaceholder}/" +
            $"tailwindcss-{RemoteDescriptor.OsPlaceholder}-{RemoteDescriptor.ArchPlaceholder}" +
            $"{RemoteDescriptor.ExtPlaceholder}",
        Version = "4.1.10",
        Os = OsHelper.GetOsName("windows", "macos", "linux"),
        Arch = RuntimeInformation.OSArchitecture switch
        {
            Architecture.Arm64 => "arm64",
            Architecture.X64 => "x64",
            _ => throw new NotSupportedException($"Unsupported architecture: {RuntimeInformation.OSArchitecture}")
        },
        Ext = OsHelper.IsWindows() ? ".exe" : "",
        DownloadDestinationFileName = $"tailwind{(OsHelper.IsWindows() ? ".exe" : "")}",
        SetExecutable = true
    };
}