using JetBrains.Annotations;

namespace Entry.R;

[UsedImplicitly]
public class RBuildCommand : RCommandBase
{
    protected override string GetScriptBody() =>
        $"""
         cd "{Nav.RDir}"
         R -e "devtools::build()"
         """;
}