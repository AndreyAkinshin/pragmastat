using System.IO.Compression;
using Common;
using Common.Extensions;
using Common.Helpers;
using Spectre.Console;

namespace Entry.Web.Retrieve;

public static class Retriever
{
    private static readonly object Sync = new();

    public static async Task<int> RetrieveAsync(params RemoteDescriptor[] descriptors)
    {
        // Download
        using HttpClient client = new();
        await AnsiConsole.Progress()
            .Columns([
                new TaskDescriptionColumn { Alignment = Justify.Left },
                new ProgressBarColumn(),
                new PercentageColumn(),
                new SpinnerColumn()
            ])
            .StartAsync(async ctx =>
            {
                var localClient = client;
                await Task.WhenAll(descriptors.Select(async descriptor =>
                {
                    Term.Info(descriptor.Url);
                    var task = ctx.AddTask($"Download {descriptor.Name}",
                        new ProgressTaskSettings { AutoStart = false });
                    await Download(localClient, task, descriptor);
                }));
            });

        // Extract
        await AnsiConsole.Progress()
            .Columns([
                new TaskDescriptionColumn(),
                new ProgressBarColumn(),
                new PercentageColumn(),
                new SpinnerColumn()
            ])
            .StartAsync(async ctx =>
            {
                await Task.WhenAll(descriptors
                    .Where(request => request.FileNameToExtract.IsNotEmpty())
                    .Select(async request =>
                    {
                        var task = ctx.AddTask($"Extract {request.Name}",
                                new ProgressTaskSettings { AutoStart = false })
                            .IsIndeterminate();
                        await Extract(task, request);
                    }));
            });

        // Report
        foreach (var descriptor in descriptors)
            AnsiConsole.MarkupLine($"[green]Downloaded[/] {descriptor.BinaryFilePath}");

        return 0;
    }

    private static async Task Download(HttpClient client, ProgressTask task, RemoteDescriptor descriptor)
    {
        try
        {
            using var response = await client.GetAsync(descriptor.Url, HttpCompletionOption.ResponseHeadersRead);
            response.EnsureSuccessStatusCode();

            task.MaxValue(response.Content.Headers.ContentLength ?? 0);
            task.StartTask();

            var filePath = descriptor.DownloadDestinationFilePath;
            lock (Sync)
            {
                filePath.Parent?.EnsureExists();
            }

            AnsiConsole.MarkupLine($"Starting download of [u]{descriptor.Name}[/] ({task.MaxValue} bytes) ");

            await using var contentStream = await response.Content.ReadAsStreamAsync();
            await using var fileStream =
                new FileStream(filePath, FileMode.Create, FileAccess.Write, FileShare.None, 8192, true);
            var buffer = new byte[8192];
            while (true)
            {
                var read = await contentStream.ReadAsync(buffer);
                if (read == 0)
                {
                    AnsiConsole.MarkupLine($"Download of [u]{descriptor.Name}[/] [green]completed![/]");
                    break;
                }

                task.Increment(read);
                await fileStream.WriteAsync(buffer.AsMemory(0, read));
            }

            if (descriptor.FileNameInArchive.IsEmpty() && descriptor.SetExecutable)
                await filePath.SetExecutableAsync();
        }
        catch (Exception ex)
        {
            AnsiConsole.MarkupLine($"[red]Error:[/] {ex}");
        }
    }

    private static async Task Extract(ProgressTask task, RemoteDescriptor request)
    {
        var archiveFilePath = request.DownloadDestinationFilePath;
        var extractFileName = request.FileNameToExtract;
        var extractFilePath = request.ExtractDestinationFilePath;

        if (archiveFilePath.FullPath.EndsWith(".zip"))
        {
            using var archive = ZipFile.OpenRead(archiveFilePath);
            var entry = archive.GetEntry(extractFileName);
            if (entry == null) return;

            await using var entryStream = entry.Open();
            await using var extractFileStream = File.Create(extractFilePath);
            await entryStream.CopyToAsync(extractFileStream);
            await archiveFilePath.DeleteAsync();
            task.Value = task.MaxValue;
            task.StopTask();
            return;
        }

        if (archiveFilePath.FullPath.EndsWith(".tar.gz") && OsHelper.IsUnix())
        {
            var command = "tar";
            var arguments = $"-xzf {archiveFilePath} " +
                            $"-C {Path.GetDirectoryName(extractFilePath)} " +
                            $"{Path.GetFileName(extractFileName)}";
            await ProcessHelper.RunAsync(command, arguments);
            await archiveFilePath.DeleteAsync();
            task.Value = task.MaxValue;
            task.StopTask();
            return;
        }

        throw new NotSupportedException($"Unsupported archive type: {archiveFilePath}");
    }
}