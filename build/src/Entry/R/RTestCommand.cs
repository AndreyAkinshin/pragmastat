using JetBrains.Annotations;

namespace Entry.R;

[UsedImplicitly]
public class RTestCommand : RCommandBase
{
    protected override string GetScriptBody() =>
        $"""
         cd "{Nav.RDir}"
         R -e "devtools::test()"
         """;
}