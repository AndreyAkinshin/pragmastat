using System.Diagnostics;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Text;

namespace DotNetBootstrap;

internal static class Program
{
  private static readonly Stopwatch GlobalStopwatch = new();
  private static readonly List<string> LogMessages = new();

  private static void Main(string[] args)
  {
    GlobalStopwatch.Start();

    string[] bootstrapArgs, targetArgs;
    if (args.Contains("--"))
    {
      var index = Array.IndexOf(args, "--");
      bootstrapArgs = args.Take(index).ToArray();
      targetArgs = args.Skip(index + 1).ToArray();
    }
    else
    {
      bootstrapArgs = args;
      targetArgs = Array.Empty<string>();
    }

    string target = "", slnDir = "", publishDir = "", targetScriptFile = "";
    bool debugMode = false, forceMode = false;

    try
    {
      for (var i = 0; i < bootstrapArgs.Length; i++)
      {
        switch (bootstrapArgs[i])
        {
          case "--debug":
            debugMode = true;
            break;
          case "--force":
            forceMode = true;
            break;
          case "--sln-dir":
            slnDir = i < bootstrapArgs.Length - 1 ? bootstrapArgs[i + 1] : "";
            break;
          case "--target-script-file":
            targetScriptFile = i < bootstrapArgs.Length - 1 ? bootstrapArgs[i + 1] : "";
            break;
          default:
            target = bootstrapArgs[i];
            break;
        }
      }

      if (debugMode)
      {
        DebugWriteLine(debugMode, $"Bootstrap Arguments ({bootstrapArgs.Length}):");
        for (int i = 0; i < bootstrapArgs.Length; i++)
          DebugWriteLine(debugMode, $"  {i}: {bootstrapArgs[i]}");
        DebugWriteLine(debugMode, $"Target Arguments ({targetArgs.Length}):");
        for (int i = 0; i < targetArgs.Length; i++)
          DebugWriteLine(debugMode, $"  {i}: {targetArgs[i]}");
      }

      if (target == "")
        throw new Exception("Target csproj is not specified");
      if (!File.Exists(target))
        throw new Exception($"Target csproj '{target}' does not exist");
      if (!target.ToLowerInvariant().EndsWith(".csproj"))
        throw new Exception($"Target csproj '{target}' is not a csproj file");

      var projDir = Directory.GetParent(target)?.FullName;
      if (projDir == null)
        throw new Exception($"Failed to get directory of '{target}'");

      if (slnDir == "")
        slnDir = projDir;

      var rid = RuntimeInformation.RuntimeIdentifier;
      publishDir = Path.Combine(projDir, "bin", "Bootstrap-" + rid);
      var targetName = Path.GetFileNameWithoutExtension(target);
      var targetExecutableFile = Path.Combine(publishDir, targetName + ".dll");

      var sdkFile = GetDotNetSdkFile();

      DebugWriteLine(debugMode, "Arguments are parsed");
      if (debugMode)
      {
        DebugWriteLine(debugMode, "Input:");
        DebugWriteLine(debugMode, $"  Target           : {target}");
        DebugWriteLine(debugMode, $"  TargetName       : {targetName}");
        DebugWriteLine(debugMode, $"  TargetExecutable : {targetExecutableFile}");
        DebugWriteLine(debugMode, $"  TargetScript     : {targetScriptFile}");
        DebugWriteLine(debugMode, $"  ProjDir          : {projDir}");
        DebugWriteLine(debugMode, $"  SlnDir           : {slnDir}");
        DebugWriteLine(debugMode, $"  PublishDir       : {publishDir}");
        DebugWriteLine(debugMode, $"  SdkFile          : {sdkFile}");
      }

      var publishMode = forceMode;

      if (!Directory.Exists(publishDir))
      {
        DebugWriteLine(debugMode, $"Creating publish directory '{publishDir}'");
        Directory.CreateDirectory(publishDir);
        DebugWriteLine(debugMode, "Publish directory is created");
      }

      var srcFileEntries = GetSrcFileEntries(slnDir);
      DebugWriteLine(debugMode, "Source files are collected");

      var currentSnapshot = FormatSnapshot(srcFileEntries);
      DebugWriteLine(debugMode, "Snapshot is formatted");

      var snapshotFilePath = Path.Combine(publishDir, "snapshot.txt");
      var previousSnapshot = File.Exists(snapshotFilePath) ? File.ReadAllText(snapshotFilePath) : "";
      DebugWriteLine(debugMode, "Previous snapshot is loaded");

      if (previousSnapshot != currentSnapshot)
      {
        DebugWriteLine(debugMode, "Existing snapshot is outdated");
        publishMode = true;
      }
      else
        DebugWriteLine(debugMode, "Existing snapshot is up-to-date");

      if (publishMode)
      {
        if (Directory.Exists(publishDir))
        {
          DebugWriteLine(debugMode, $"Deleting publish directory '{publishDir}'...");
          Directory.Delete(publishDir, true);
        }

        Console.WriteLine($"Bootstrapping {targetName}...");
        RunCommand(sdkFile,
          $"publish --configuration Release --output \"{publishDir}\" \"{target}\" /p:PublishReadyToRun=true",
          debugMode);

        DebugWriteLine(debugMode, $"Writing snapshot to {snapshotFilePath}");
        WriteAllText(debugMode, snapshotFilePath, currentSnapshot);
      }
      else
      {
        DebugWriteLine(debugMode, "Everything is ready, no boostrap needed");
      }

      if (targetScriptFile != "")
        SaveTargetScript(targetScriptFile, sdkFile, targetExecutableFile, targetArgs, debugMode);

      DebugWriteLine(debugMode, "Finish");
    }
    catch (Exception e)
    {
      if (!debugMode)
      {
        Console.WriteLine("Full DotNetBootstrap log:");
        foreach (var logMessage in LogMessages)
          Console.WriteLine($"  {logMessage}");
      }

      ErrorWriteLine(e);
      try
      {
        if (publishDir != "" && Directory.Exists(publishDir))
        {
          Console.WriteLine($"Deleting cache directory '{publishDir}'...");
          Directory.Delete(publishDir, true);
        }
      }
      catch (Exception e2)
      {
        ErrorWriteLine(e2);
      }

      Environment.Exit(1);
    }
  }

  private static void SaveTargetScript(string targetScriptFile, string sdkFile, string targetExecutableFile,
    string[] targetArgs, bool debugMode)
  {
    DebugWriteLine(debugMode, $"Creating target script file '{targetScriptFile}'");
    var builder = new StringBuilder();
    builder.Append('\"');
    builder.Append(sdkFile);
    builder.Append('\"');
    builder.Append(' ');
    builder.Append('\"');
    builder.Append(targetExecutableFile);
    builder.Append('\"');
    foreach (var targetArg in targetArgs)
    {
      builder.Append(' ');
      builder.Append('\"');
      builder.Append(targetArg);
      builder.Append('\"');
    }

    WriteAllText(debugMode, targetScriptFile, builder.ToString());
  }

  private static List<FileEntry> GetSrcFileEntries(string srcDir)
  {
    var srcFileEntries = new List<FileEntry>();

    foreach (var fullFilePath in Directory.EnumerateFiles(srcDir, "*", SearchOption.AllDirectories))
    {
      var relativeFilePath = fullFilePath[srcDir.Length..].TrimStart('\\', '/');
      var relativeFilePathParts = relativeFilePath.Split('\\', '/');
      if (relativeFilePathParts.Contains("bin") ||
          relativeFilePathParts.Contains("obj") ||
          relativeFilePathParts.Contains(".idea"))
        continue;

      var fileExtension = Path.GetExtension(fullFilePath);
      if (fileExtension != ".cs" && fileExtension != ".csproj" && fileExtension != ".sln" && fileExtension != ".json" && fileExtension != ".user" && fileExtension != ".Config")
        continue;

      var fileTimestamp = File.GetLastWriteTime(fullFilePath);
      var fileSize = new FileInfo(fullFilePath).Length;

      srcFileEntries.Add(new FileEntry(relativeFilePath, fileTimestamp, fileSize));
    }

    srcFileEntries.Sort();
    return srcFileEntries;
  }

  private static string FormatSnapshot(List<FileEntry> fileEntries)
  {
    if (fileEntries.Count == 0)
      return "";

    var maxFileLength = fileEntries.Max(x => x.RelativePath.Length) + 1;
    var maxFileSize = fileEntries.Max(x => x.FileSize);
    var expectedBuilderLength = fileEntries.Count * (maxFileLength + 33 + 1 + maxFileSize.ToString().Length + 2);
    var builder = new StringBuilder(expectedBuilderLength);
    foreach (var fileEntry in fileEntries)
    {
      builder.Append(fileEntry.RelativePath);
      builder.Append(' ', maxFileLength - fileEntry.RelativePath.Length);
      builder.Append(fileEntry.Timestamp.ToString("O"));
      builder.Append(' ');
      builder.Append(fileEntry.FileSize);
      builder.AppendLine();
    }

    return builder.ToString();
  }

  private static void RunCommand(string fileName, string arguments, bool debugMode)
  {
    var initialFileName = fileName;
    var initialArguments = arguments;

    DebugWriteLine(debugMode, $"Running '{fileName} {arguments}'");
    if (!RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
    {
      arguments = $"-c \"{fileName} {arguments.Replace("\"", "'")}\"";
      fileName = "/bin/bash";
      DebugWriteLine(debugMode, $"Patched command-line for Unix: '{fileName} {arguments}'");
    }

    var processStartInfo = new ProcessStartInfo
    {
      FileName = fileName,
      Arguments = arguments,
      UseShellExecute = false,
      CreateNoWindow = true,
      RedirectStandardOutput = true,
      RedirectStandardError = true
    };
    var process = new Process { StartInfo = processStartInfo };
    process.OutputDataReceived += (_, args) => AutoWriteLine(args.Data);
    process.ErrorDataReceived += (_, args) => ErrorWriteLine(args.Data);
    process.Start();
    process.BeginOutputReadLine();
    process.BeginErrorReadLine();

    var timeout = TimeSpan.FromMinutes(5);
    var hasExited = process.WaitForExit(timeout);
    if (!hasExited)
    {
      process.Kill(true);
      hasExited = process.WaitForExit(timeout);
      if (hasExited)
        process.WaitForExit(); // Finish reading output
      throw new Exception($"Failed to execute '{fileName}' in {timeout.TotalMinutes} minutes");
    }

    process.WaitForExit(); // Finish reading output

    if (process.ExitCode != 0)
      throw new Exception($"Process '{initialFileName} {initialArguments}' exited with code {process.ExitCode}");
  }

  private static void DebugWriteLine(bool debugMode, string message)
  {
    if (!debugMode)
    {
      LogMessages.Add(message);
      return;
    }

    var elapsed = GlobalStopwatch.ElapsedMilliseconds.ToString().PadLeft(5);
    Console.ForegroundColor = ConsoleColor.Gray;
    Console.WriteLine($"[DEBUG] [{elapsed}ms] " + message);
    Console.ResetColor();
  }

  private static void AutoWriteLine(string? message)
  {
    if (message?.Contains("error", StringComparison.OrdinalIgnoreCase) ?? false)
      ErrorWriteLine(message);
    else
      Console.WriteLine(message);
  }

  private static void ErrorWriteLine(Exception e) => ErrorWriteLine(e.ToString());

  private static void ErrorWriteLine(string? message)
  {
    Console.ForegroundColor = ConsoleColor.Red;
    Console.Error.WriteLine(message ?? "");
    Console.ResetColor();
  }

  private static string GetDotNetSdkFile()
  {
    const string rootMarker = "build.cmd";
    var directory = Assembly.GetExecutingAssembly().Location;
    while (directory != null && !File.Exists(Path.Combine(directory, rootMarker)))
      directory = Directory.GetParent(directory)?.FullName;
    if (directory == null)
      throw new Exception($"Failed to find {rootMarker}");
    return Path.Combine(directory, ".dotnet", "dotnet");
  }

  private static void WriteAllText(bool debugMode, string filePath, string text)
  {
    DebugWriteLine(debugMode, $"Writing text to '{filePath}'...");

    var parentDirectoryPath = Directory.GetParent(filePath)?.FullName;
    if (parentDirectoryPath == null)
      throw new Exception($"Failed to get directory of '{filePath}'");

    if (!Directory.Exists(parentDirectoryPath))
    {
      DebugWriteLine(debugMode, $"Creating directory '{parentDirectoryPath}'...");
      Directory.CreateDirectory(parentDirectoryPath);
    }

    File.WriteAllText(filePath, text);
  }
}

public record FileEntry(string RelativePath, DateTime Timestamp, long FileSize) : IComparable<FileEntry>
{
  public int CompareTo(FileEntry? other)
  {
    if (ReferenceEquals(this, other)) return 0;
    if (ReferenceEquals(null, other)) return 1;
    return string.Compare(RelativePath, other.RelativePath, StringComparison.Ordinal);
  }
}