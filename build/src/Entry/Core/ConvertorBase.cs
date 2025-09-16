using System.Text.RegularExpressions;
using Common;
using Common.Extensions;
using Common.Io;

namespace Entry.Core;

public abstract class ConvertorBase
{
    protected readonly Navigator Nav = Navigator.Default;

    private readonly Lazy<string> lazyManualVersion;
    public string ManualVersion => lazyManualVersion.Value;

    protected ConvertorBase()
    {
        lazyManualVersion = new Lazy<string>(() =>
        {
            if (Nav.ManualVersionTxt.Exists)
                return Nav.ManualVersionTxt.ReadAllText().Trim();
            return "0.0.0";
        });
    }

    protected abstract string Alias { get; }
    protected abstract FilePath DestMainFile { get; }
    protected abstract DirectoryPath DestImgDir { get; }
    protected abstract bool SupportDarkImages { get; }
    protected abstract Task ConvertReferences();
    protected abstract string InsertImg(string name, string title);
    protected abstract string InsertBegin(string name);
    protected abstract string InsertEnd(string name);

    public async Task ConvertAll()
    {
        // Main
        var dstFile = DestMainFile;
        var srcContent = await Nav.MainMd.ReadAllTextAsync();
        var destContent = await ConvertMain(srcContent);
        await dstFile.WriteAllTextAsync(destContent);
        Term.Info($"Created: {dstFile}");

        // Other
        await ConvertReferences();

        // Images
        DestImgDir.InitBlank();
        foreach (var imgFile in Nav.ImgDir.EnumerateFiles("*.png"))
        {
            if (SupportDarkImages)
            {
                imgFile.CopyTo(DestImgDir);
            }
            else
            {
                if (imgFile.NameWithoutExtension.EndsWith("_light"))
                    imgFile.CopyTo(DestImgDir.File(imgFile.Name.Replace("_light", "")));
            }
        }
    }

    protected virtual async Task<string> ConvertMain(string src)
    {
        return await ConvertText(src);
    }

    public async Task<string> ConvertText(string src)
    {
        var content = await ProcessIncludeDirectives(src);
        content = ProcessImageDirectives(content);
        content = ProcessBeginEndDirectives(content);
        content = ProcessCopyrightDirectives(content);
        var lines = content.Split('\n').ToList();
        var removeMarker = $"<!-- [{Alias}] DELETE -->";
        lines.RemoveAll(line => line.Contains(removeMarker));
        return lines.JoinToString('\n');
    }

    private string ProcessImageDirectives(string content)
    {
        const string imgPattern = @"<!--\s*IMG\s+([^>]+)\s*-->";
        var regex = new Regex(imgPattern, RegexOptions.IgnoreCase);

        return regex.Replace(content, match =>
        {
            var filename = match.Groups[1].Value.Trim();
            return InsertImg(filename, "");
        });
    }

    private string ProcessBeginEndDirectives(string content)
    {
        const string beginPattern = @"<!--\s*BEGIN\s+([^>]+)\s*-->";
        const string endPattern = @"<!--\s*END\s+([^>]+)\s*-->";
        var beginRegex = new Regex(beginPattern, RegexOptions.IgnoreCase);
        var endRegex = new Regex(endPattern, RegexOptions.IgnoreCase);

        var result = beginRegex.Replace(content, match =>
        {
            var name = match.Groups[1].Value.Trim();
            return InsertBegin(name);
        });

        result = endRegex.Replace(result, match =>
        {
            var name = match.Groups[1].Value.Trim();
            return InsertEnd(name);
        });

        return result;
    }

    private Task<string> ProcessIncludeDirectives(string content)
    {
        return Task.FromResult(ProcessIncludeDirectivesRecursive(content, []));
    }

    private string ProcessIncludeDirectivesRecursive(string content, HashSet<string> processedFiles)
    {
        var includePattern = @"<!--\s*INCLUDE\s+([^>]+)\s*-->";
        var regex = new Regex(includePattern, RegexOptions.IgnoreCase);

        var result = content;
        var hasChanges = true;

        // Continue processing until no more INCLUDE directives are found
        while (hasChanges)
        {
            hasChanges = false;
            result = regex.Replace(result, match =>
            {
                var fullPath = Nav.Root.File(match.Groups[1].Value.Trim());

                try
                {
                    if (fullPath.Exists)
                    {
                        // Check for circular inclusion
                        if (!processedFiles.Add(fullPath))
                        {
                            Term.Warning($"Circular inclusion detected for file: `{match.Groups[1].Value.Trim()}`");
                            return
                                $"<!-- ERROR | INCLUDE {match.Groups[1].Value.Trim()} | CIRCULAR INCLUSION DETECTED -->";
                        }

                        // Read the included file content
                        var includedContent = File.ReadAllText(fullPath).Trim();

                        // Recursively process INCLUDE directives in the included content
                        var processedIncludedContent =
                            ProcessIncludeDirectivesRecursive(includedContent, processedFiles);

                        // Remove from processed files after processing (to allow same file to be included in different contexts)
                        processedFiles.Remove(fullPath);

                        hasChanges = true;
                        return processedIncludedContent;
                    }
                    else
                    {
                        Term.Warning($"Failed to find file for including: `{match.Groups[1].Value.Trim()}`");
                        return $"<!-- ERROR | INCLUDE {match.Groups[1].Value.Trim()} | FILE NOT FOUND -->";
                    }
                }
                catch (Exception ex)
                {
                    return $"<!-- ERROR | INCLUDE {match.Groups[1].Value.Trim()} | {ex.Message} -->";
                }
            });
        }

        return result;
    }

    private string ProcessCopyrightDirectives(string content)
    {
        const string copyrightPattern = @"<!--\s*COPYRIGHT\s*-->";
        var regex = new Regex(copyrightPattern, RegexOptions.IgnoreCase);

        return regex.Replace(content, match =>
        {
            return $"Pragmastat v{ManualVersion} (c) 2025 Andrey Akinshin, MIT License";
        });
    }

    protected virtual string ComposeReference(string src)
    {
        return src;
    }
}