using JetBrains.Annotations;

namespace Entry.R;

[UsedImplicitly]
public class RCheckCommand : RCommandBase
{
    protected override string GetScriptBody() =>
        $"""
         cd "{Nav.RDir}"
         R -e "devtools::check()"
         """;
}