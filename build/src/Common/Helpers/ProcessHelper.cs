using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Text;
using Common.Extensions;
using Common.Io;

namespace Common.Helpers;

public static class ProcessHelper
{
    public static async Task<string> RunAsync(
        FilePath filePath,
        string arguments = "",
        DirectoryPath? workingDirectory = null,
        TimeSpan? timeout = null)
    {
        if (filePath.FullPath.EndsWith(".cmd") && !RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
        {
            arguments = $"-c \"{filePath.FullPath} {arguments.Replace("\"", "'")}\"";
            return await RunAsync("/bin/bash", arguments, workingDirectory, timeout);
        }

        string ret;
        using var process = new Process();
        process.StartInfo = new ProcessStartInfo
        {
            FileName = filePath,
            Arguments = arguments,
            RedirectStandardOutput = true,
            RedirectStandardError = true,
            UseShellExecute = false,
            CreateNoWindow = true,
        };
        if (workingDirectory != null)
            process.StartInfo.WorkingDirectory = workingDirectory;

        try
        {
            var outputBuilder = new StringBuilder();
            process.OutputDataReceived += (_, args) => outputBuilder.AppendLine(args.Data ?? "");
            process.ErrorDataReceived += (_, args) => outputBuilder.AppendLine(args.Data ?? "");
            process.Start();
            process.BeginOutputReadLine();
            await Task.Run(() => process.WaitForExit(timeout ?? Timeout.InfiniteTimeSpan));
            if (process.ExitCode != 0)
                throw new Exception(
                    $"Failed to get output of `{filePath} {arguments}` " +
                    $"(exit code: {process.ExitCode}); " +
                    $"Output:\n{outputBuilder}");

            var output = outputBuilder.ToString();
            if (output.IsNotBlank())
                ret = output;
            else
            {
                throw new Exception($"Failed to get output of `{filePath} {arguments}`.");
            }
        }
        catch (Exception ex)
        {
            throw new Exception($"An error occurred running `{filePath} {arguments}`: {ex.Message}");
        }

        return ret;
    }
}