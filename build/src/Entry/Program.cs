using Entry.Dotnet;
using Entry.Go;
using Entry.Img;
using Entry.Kotlin;
using Entry.Pdf;
using Entry.Python;
using Entry.R;
using Entry.Rust;
using Entry.Web;
using Pragmastat.Simulations.Simulations;
using Spectre.Console.Cli;

namespace Entry;

internal static class Program
{
    public static async Task<int> Main(string[] args)
    {
        if (args.Length == 1 && args[0].StartsWith("--output-script-path="))
            args = [];

        var app = new CommandApp();
        app.Configure(config =>
        {
            config.SetApplicationName("build.cmd");

            config.AddCommand<WebCommand>("web").WithAlias("w");
            config.AddCommand<PdfCommand>("pdf").WithAlias("p");
            config.AddCommand<ImgCommand>("img").WithAlias("i");

            config.AddBranch<CommandSettings>("dotnet", dotnet =>
            {
                dotnet.AddCommand<DotnetBuildCommand>("build");
                dotnet.AddCommand<DotnetTestCommand>("test");
                dotnet.AddCommand<DotnetPackCommand>("pack");
                dotnet.AddBranch<CommandSettings>("sim",
                    sim =>
                    {
                        sim.AddCommand<CentralTendencyGaussianEfficiency>(CentralTendencyGaussianEfficiency.Name);
                    });
            });

            config.AddBranch<CommandSettings>("r", r =>
            {
                r.AddCommand<RCheckCommand>("check");
                r.AddCommand<RTestCommand>("test");
                r.AddCommand<RBuildCommand>("build");
            });

            config.AddBranch<CommandSettings>("python", r =>
            {
                r.AddCommand<PythonBuildCommand>("build");
                r.AddCommand<PythonTestCommand>("test");
                r.AddCommand<PythonCheckCommand>("check");
                r.AddCommand<PythonCleanCommand>("clean");
            });

            config.AddBranch<CommandSettings>("rust", r =>
            {
                r.AddCommand<RustBuildCommand>("build");
                r.AddCommand<RustTestCommand>("test");
                r.AddCommand<RustCheckCommand>("check");
                r.AddCommand<RustCleanCommand>("clean");
            });

            config.AddBranch<CommandSettings>("go", r =>
            {
                r.AddCommand<GoBuildCommand>("build");
                r.AddCommand<GoTestCommand>("test");
                r.AddCommand<GoAllCommand>("all");
            });

            config.AddCommand<KotlinBuildCommand>("kotlin");
        });
        return await app.RunAsync(args);
    }
}