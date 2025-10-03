using System.Text.RegularExpressions;
using Common;
using Common.Io;
using Entry.Core;
using Spectre.Console.Cli;

namespace Entry;

public class UnifyCommand : AsyncCommand<UnifyCommand.Settings>
{
    public class Settings : CommandSettings
    {
    }

    private static readonly Navigator Nav = Navigator.Default;

    public override async Task<int> ExecuteAsync(CommandContext context, Settings settings)
    {
        // Read version from version.txt
        var version = await Nav.ManualVersionTxt.ReadAllTextAsync();
        version = version.Trim();
        
        Console.WriteLine($"Updating version to: {version}");

        // Define file update configurations
        var updates = new[]
        {
            new FileUpdate(
                Nav.DotNetDirectoryBuildProps,
                @"<Version>.*?</Version>",
                $"<Version>{version}</Version>"
            ),
            new FileUpdate(
                Nav.KotlinDir.File("build.gradle.kts"),
                @"version = "".*?""",
                $"version = \"{version}\""
            ),
            new FileUpdate(
                Nav.PythonDir.File("pyproject.toml"),
                @"version = "".*?""",
                $"version = \"{version}\""
            ),
            new FileUpdate(
                Nav.PythonDir.SubDirectory("pragmastat").File("__init__.py"),
                @"__version__ = '.*?'",
                $"__version__ = '{version}'"
            ),
            new FileUpdate(
                Nav.RDescriptionFile,
                @"Version: .*",
                $"Version: {version}"
            ),
            new FileUpdate(
                Nav.RustDir.SubDirectory("pragmastat").File("Cargo.toml"),
                @"(name = ""pragmastat""[\s\S]*?)version = "".*?""",
                $"$1version = \"{version}\""
            ),
            new FileUpdate(
                Nav.Root.SubDirectory("ts").File("package.json"),
                @"""version"": "".*?""",
                $"\"version\": \"{version}\""
            )
        };

        // Apply updates to each file
        foreach (var update in updates)
        {
            await UpdateFileAsync(update);
        }

        // Generate documentation files from templates
        await GenerateDocumentationFiles(version);

        Console.WriteLine("Version unification completed successfully!");
        return 0;
    }

    private async Task UpdateFileAsync(FileUpdate update)
    {
        if (!update.FilePath.Exists)
        {
            Console.WriteLine($"Warning: File {update.FilePath.FullPath} does not exist, skipping.");
            return;
        }

        var content = await update.FilePath.ReadAllTextAsync();
        var updatedContent = Regex.Replace(content, update.Pattern, update.Replacement, RegexOptions.Multiline);

        if (content != updatedContent)
        {
            await update.FilePath.WriteAllTextAsync(updatedContent);
            Console.WriteLine($"Updated: {update.FilePath.FullPath}");
        }
        else
        {
            Console.WriteLine($"No changes needed: {update.FilePath.FullPath}");
        }
    }

    private record FileUpdate(FilePath FilePath, string Pattern, string Replacement);

    private async Task GenerateDocumentationFiles(string version)
    {
        Console.WriteLine("Generating documentation files from templates...");

        // Define language configurations
        var languages = new[]
        {
            new LanguageConfig("dotnet", ".NET", "cs", "csharp"),
            new LanguageConfig("go", "Go", "go", "go"),
            new LanguageConfig("kotlin", "Kotlin", "kotlin", "kotlin"),
            new LanguageConfig("python", "Python", "python", "python"),
            new LanguageConfig("r", "R", "r", "r"),
            new LanguageConfig("rust", "Rust", "rust", "rust"),
            new LanguageConfig("ts", "TypeScript", "typescript", "typescript")
        };

        // Read templates
        var implTemplate = await Nav.ManualImplementationsDir.File("template-impl.md").ReadAllTextAsync();
        var readmeTemplate = await Nav.ManualImplementationsDir.File("template-readme.md").ReadAllTextAsync();

        foreach (var lang in languages)
        {
            await GenerateImplementationFile(lang, implTemplate, version);
            await GenerateReadmeFile(lang, readmeTemplate, version);
        }
    }

    private async Task GenerateImplementationFile(LanguageConfig lang, string template, string version)
    {
        // Read install instructions and process version macro
        var installFile = Nav.ManualImplementationsDir.File($"install-{lang.Slug}.md");
        var installContent = await installFile.ReadAllTextAsync();
        installContent = installContent.Replace("$VERSION$", version);

        // Read demo content
        var demoContent = await GetDemoContent(lang);

        // Replace placeholders
        var content = template
            .Replace("$LANG_SLUG$", lang.Slug)
            .Replace("$LANG_TITLE$", lang.Title)
            .Replace("$LANG_CODE$", lang.CodeLanguage)
            .Replace("$VERSION$", version)
            .Replace("$INSTALL$", installContent.Trim())
            .Replace("$DEMO$", demoContent);

        // Write to implementation file
        var outputFile = Nav.ManualImplementationsDir.File($"{lang.Slug}.md");
        await outputFile.WriteAllTextAsync(content);
        Console.WriteLine($"Generated: {outputFile.FullPath}");
    }

    private async Task GenerateReadmeFile(LanguageConfig lang, string template, string version)
    {
        // Read install instructions and process version macro
        var installFile = Nav.ManualImplementationsDir.File($"install-{lang.Slug}.md");
        var installContent = await installFile.ReadAllTextAsync();
        installContent = installContent.Replace("$VERSION$", version);

        // Read demo content
        var demoContent = await GetDemoContent(lang);

        // Replace placeholders
        var content = template
            .Replace("$LANG_TITLE$", lang.Title)
            .Replace("$LANG_CODE$", lang.CodeLanguage)
            .Replace("$VERSION$", version)
            .Replace("$LANG_SLUG$", lang.Slug)
            .Replace("$INSTALL$", installContent.Trim())
            .Replace("$DEMO$", demoContent);

        // Write to language directory README
        var outputFile = Nav.Root.SubDirectory(lang.Slug).File("README.md");
        if (lang.Slug == "r" || lang.Slug == "rust")
            outputFile = Nav.Root.SubDirectory(lang.Slug, "pragmastat").File("README.md");
        await outputFile.WriteAllTextAsync(content);
        Console.WriteLine($"Generated: {outputFile.FullPath}");
    }

    private async Task<string> GetDemoContent(LanguageConfig lang)
    {
        var content = lang.Slug switch
        {
            "dotnet" => await Nav.DotnetDir.SubDirectory("Pragmastat.Demo").File("Program.cs").ReadAllTextAsync(),
            "go" => await Nav.GoDir.SubDirectory("example").File("main.go").ReadAllTextAsync(),
            "kotlin" => await Nav.KotlinDir.SubDirectory("src").SubDirectory("main").SubDirectory("kotlin").SubDirectory("com").SubDirectory("pragmastat").SubDirectory("example").File("Main.kt").ReadAllTextAsync(),
            "python" => await Nav.PythonDir.SubDirectory("examples").File("demo.py").ReadAllTextAsync(),
            "r" => await Nav.RDir.SubDirectory("inst").SubDirectory("examples").File("demo.R").ReadAllTextAsync(),
            "rust" => await Nav.RustDir.SubDirectory("pragmastat").SubDirectory("examples").File("demo.rs").ReadAllTextAsync(),
            "ts" => await Nav.Root.SubDirectory("ts").SubDirectory("examples").File("demo.ts").ReadAllTextAsync(),
            _ => throw new ArgumentException($"Unknown language: {lang.Slug}")
        };
        
        // Trim trailing whitespace and blank lines
        return content.TrimEnd();
    }

    private record LanguageConfig(string Slug, string Title, string CodeLanguage, string ReadmeCodeLanguage);
}
