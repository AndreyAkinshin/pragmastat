using Common;
using Common.Extensions;
using Common.Io;

namespace Entry.Web.Retrieve;

public class RemoteDescriptor
{
    public const string VersionPlaceholder = "$VERSION";
    public const string OsPlaceholder = "$OS";
    public const string ArchPlaceholder = "$ARCH";
    public const string ExtPlaceholder = "$EXT";

    private DirectoryPath DestinationDirectory => Navigator.Default.BinDir;

    // Meta
    public string Name { get; init; } = "";

    // Download
    public string UrlTemplate { get; init; } = "";
    public string Version { get; init; } = "";
    public string Os { get; init; } = "";
    public string Arch { get; init; } = "";
    public string Ext { get; init; } = "";
    public string DownloadDestinationFileName { get; init; } = "";
    public bool SetExecutable { get; init; }

    // Extract
    public string FileNameInArchive { get; init; } = "";
    public string FileNameToExtract { get; init; } = "";

    // Computed
    public string Url => UrlTemplate
        .Replace(VersionPlaceholder, Version)
        .Replace(OsPlaceholder, Os)
        .Replace(ArchPlaceholder, Arch)
        .Replace(ExtPlaceholder, Ext);

    public FilePath DownloadDestinationFilePath => DestinationDirectory.File(DownloadDestinationFileName);
    public FilePath ExtractDestinationFilePath => DestinationDirectory.File(FileNameToExtract);
    public FilePath BinaryFilePath => FileNameToExtract.IsNotBlank()
        ? ExtractDestinationFilePath
        : DownloadDestinationFilePath;
}