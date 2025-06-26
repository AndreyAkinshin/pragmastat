using Common.Helpers;
using Common.Io;

namespace Common;

public class Navigator(DirectoryPath root)
{
    public static Navigator Default => DefaultNavigator.Value;

    private static readonly Lazy<Navigator> DefaultNavigator =
        new(() => new Navigator(DetectRootPath()));

    private static DirectoryPath DetectRootPath()
    {
        var current = new DirectoryPath(System.Reflection.Assembly.GetExecutingAssembly().Location);
        while (current != null && !current.File("build.cmd").Exists)
            current = current.Parent;
        if (current == null)
            throw new Exception("Failed to find the root directory");
        return current;
    }


    public DirectoryPath Root => root;
    public DirectoryPath BinDir => Root.SubDirectory(".bin");

    public DirectoryPath ManualDir => root.SubDirectory("manual");
    public FilePath MainMd => ManualDir.File("main.md");
    public FilePath AbstractMd => ManualDir.File("abstract.md");
    public FilePath ManualVersionTxt => ManualDir.File("version.txt");
    public DirectoryPath References => ManualDir.SubDirectory("references");

    public DirectoryPath PdfDir => Root.SubDirectory("pdf");
    public DirectoryPath PdfImgDir => PdfDir.SubDirectory("img");
    public FilePath PragmastatRmd => PdfDir.File("pragmastat.Rmd");
    public FilePath PragmastatRmdTemplate => PdfDir.File("pragmastat.Rmd.template");
    public FilePath PdfBuildSh => PdfDir.File("build.sh");
    public FilePath ReferencesBib => PdfDir.File("references.bib");
    public FilePath DefinitionsTex => PdfDir.SubDirectory("tex").File("definitions.tex");

    public DirectoryPath WebDir => Root.SubDirectory("web");
    public DirectoryPath WebContentDir => WebDir.SubDirectory("content");
    public DirectoryPath WebStaticDir => WebDir.SubDirectory("static");
    public DirectoryPath WebImgDir => WebContentDir.SubDirectory("img");
    public FilePath WebIndexMd => WebContentDir.File("_index.md");
    public FilePath WebConfigToml => WebDir.SubDirectory("data").File("config.toml");

    public DirectoryPath ImgDir => Root.SubDirectory("img");
    public FilePath ImgBuildSh => ImgDir.File("build.sh");

    public DirectoryPath DotnetDir => Root.SubDirectory("dotnet");
    public DirectoryPath DotnetArtifacts => DotnetDir.SubDirectory("artifacts");
    public FilePath PragmastatSln => DotnetDir.File("Pragmastat.sln");
    public FilePath DotNetDirectoryBuildProps => DotnetDir.File("Directory.Build.props");

    public FilePath[] DotnetNuGetCsProjFiles =>
    [
        DotnetDir.SubDirectory("Pragmastat.Core").File("Pragmastat.Core.csproj"),
        DotnetDir.SubDirectory("Pragmastat").File("Pragmastat.csproj")
    ];

    public FilePath DotnetUnitTestsCsProj =>
        DotnetDir.SubDirectory("Pragmastat.UnitTests").File("Pragmastat.UnitTests.csproj");

    public FilePath DotnetRefTestsCsProj =>
        DotnetDir.SubDirectory("Pragmastat.ReferenceTests").File("Pragmastat.ReferenceTests.csproj");

    public FilePath DotNetSdk => Root.SubDirectory(".dotnet").File(OsHelper.IsWindows() ? "dotnet.exe" : "dotnet");

    public DirectoryPath ReferenceTestsDir => Root.SubDirectory("tests");

    public DirectoryPath RDir => Root.SubDirectory("r", "pragmastat");
    public DirectoryPath RTestsDir => RDir.SubDirectory("tests");
    public FilePath RDescriptionFile => RDir.File("DESCRIPTION");

    public DirectoryPath PythonDir => Root.SubDirectory("python");
    public FilePath PythonBuildSh => PythonDir.File("build.sh");

    public DirectoryPath RustDir => Root.SubDirectory("rust");
    public FilePath RustBuildSh => RustDir.File("build.sh");

    public DirectoryPath GoDir => Root.SubDirectory("go");
    public FilePath GoBuildSh => GoDir.File("build.sh");

    public DirectoryPath KotlinDir => Root.SubDirectory("kotlin");
    public FilePath KotlinBuildSh => KotlinDir.File("build.sh");

}