using Common;
using Common.Io;
using Entry.Core;

namespace Entry.Web;

public class WebConvertor : ConvertorBase
{
    public static readonly WebConvertor Default = new();

    protected override string Alias => "web";
    protected override FilePath DestMainFile => Nav.WebIndexMd;
    protected override DirectoryPath DestImgDir => Nav.WebImgDir;
    protected override bool SupportDarkImages => true;
    protected override bool ShouldNumberHeaders => true;

    protected override async Task<string> ConvertMain(string src)
    {
        var content = await base.ConvertMain(src);
        var definitions = await Nav.DefinitionsTex.ReadAllTextAsync();
        var abstractContent = await Nav.AbstractMd.ReadAllTextAsync();
        var result = Placeholder.Start.ReplaceBy(content,
            $"""
             <div style="display: none;">
             $$
             {definitions.Trim()}
             $$
             </div>

             {abstractContent}
             """
        );
        return await base.ConvertMain(result);
    }

    protected override async Task ConvertReferences()
    {
        var srcDirectory = Nav.ManualDir.SubDirectory("references");
        var dstDirectory = Nav.WebContentDir.SubDirectory("references").EnsureExists();

        foreach (var referenceFile in srcDirectory.EnumerateFiles())
            referenceFile.CopyTo(dstDirectory);

        await dstDirectory.File("_index.md").WriteAllTextAsync(
            """
            ---
            title: References
            ---
            """);

        Term.Info($"Created: {dstDirectory}");
    }

    protected override string InsertImg(string name, string title) => $"{{{{< img {name} >}}}}";
    protected override string InsertBegin(string name)
    {
        return $"<div class=\"{name}\">";
    }

    protected override string InsertEnd(string name)
    {
        return $"</div>";
    }
}