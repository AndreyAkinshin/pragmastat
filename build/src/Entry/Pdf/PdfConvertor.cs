using System.Text;
using Common;
using Common.Extensions;
using Common.Io;
using Entry.Core;

namespace Entry.Pdf;

public class PdfConvertor(bool release) : ConvertorBase
{
    protected override string Alias => "pdf";
    protected override FilePath DestMainFile => Nav.PragmastatRmd;
    protected override DirectoryPath DestImgDir => Nav.PdfImgDir;
    protected override bool SupportDarkImages => false;

    protected override async Task<string> ConvertMain(string src)
    {
        var content = await base.ConvertMain(src);
        content = Placeholder.Start.ReplaceBy(content, "\\clearpage");

        // Removing YAML header
        var lines = content.Split('\n').ToList();
        int index = 0, separatorCounter = 0;
        while (index < lines.Count && separatorCounter < 2)
        {
            if (lines[index++].Trim() == "---")
                separatorCounter++;
        }

        lines.RemoveRange(0, index);


        var template = await Nav.PragmastatRmdTemplate.ReadAllTextAsync();

        var abstractContent = await Nav.AbstractMd.ReadAllTextAsync();
        abstractContent = abstractContent.Split('\n')
            .Select(line => "  " + line.TrimStart())
            .JoinToString('\n');
        template = template.Replace("<!-- PLACEHOLDER Abstract -->", abstractContent);

        var editionInfo = release
            ? $"Version {ManualVersion}"
            : $"*Draft of Version {ManualVersion} (`r format(Sys.Date(), '%Y-%m-%d')`)*";
        template = template.Replace("<!-- PLACEHOLDER Version -->", editionInfo);

        var result = template + lines.JoinToString('\n');

        return result;
    }

    protected override async Task ConvertReferences()
    {
        var referenceBib = new StringBuilder();
        foreach (var referenceFile in Nav.References.EnumerateFiles())
        {
            var referenceText = await referenceFile.ReadAllTextAsync();
            var bib = ExtractBib(referenceText);
            referenceBib.AppendLine(bib);
        }

        await Nav.ReferencesBib.WriteAllTextAsync(referenceBib.ToString());
        Term.Info($"Created: {Nav.ReferencesBib}");
    }

    protected override string InsertImg(string name, string title)
    {
        return $"""
                ```{$"{{r {name}, fig.cap=\"{title}\", out.width=\"100%\"}}"}
                knitr::include_graphics("img/{name}.png")
                ```
                """;
    }

    protected override string InsertBegin(string name)
    {
        return $"::: {{.{name} data-latex=\"\"}}\n";
    }

    protected override string InsertEnd(string name)
    {
        return ":::";
    }

    private static string ExtractBib(string referenceMdContent)
    {
        var lines = referenceMdContent.Split('\n').ToList();

        var result = new List<string>();
        bool inBibBlock = false;

        foreach (var line in lines)
        {
            if (line.Trim() == "```bib")
            {
                inBibBlock = true;
                continue;
            }

            if (line.Trim() == "```" && inBibBlock)
            {
                inBibBlock = false;
                continue;
            }

            if (inBibBlock)
            {
                result.Add(line);
            }
        }

        return result.JoinToString('\n');
    }
}