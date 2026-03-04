#import "/manual/definitions.typ": *

$Compare1$ orchestrates estimation and comparison in two phases:
pre-pass validation and the statistical phase.

*Pre-pass validation*

Before any statistical computation:

1. Reject weighted samples (unsupported).
2. Reject null or empty threshold list.
3. Reject threshold items containing null.
4. Reject thresholds with metrics not in ${Center, Spread}$ (wrong arity).
5. Reject thresholds with non-finite values.

These checks happen before bounds computation, so no statistical work is done on invalid inputs.

*Validate-and-normalize pass*

For each threshold, in input order:

- *Center*: check unit compatibility with $vx$; convert threshold value to $vx$'s unit.
- *Spread*: same as Center.

Bindings that support plain numeric shorthand (Python and R) interpret it directly
on the comparison scale; explicit measurement thresholds are normalized as above.

*Statistical phase (canonical metric order: Center → Spread)*

For each present metric (in canonical order), compute the estimate once and bounds for each threshold entry of that metric:

$ "estimate" = cases(Center(vx) & "if metric" = Center, Spread(vx) & "if metric" = Spread) $

$ "bounds" = cases(CenterBounds(vx, misrate_i) & "if metric" = Center, SpreadBounds(vx, misrate_i, "seed") & "if metric" = Spread and "seed" != "null", SpreadBounds(vx, misrate_i) & "if metric" = Spread and "seed" = "null") $

*Verdict computation*

$ "verdict"_i = cases("Greater" & "if" L_i > t_i, "Less" & "if" U_i < t_i, "Inconclusive" & "otherwise") $

where $[L_i, U_i]$ are the bounds for threshold $i$ and $t_i$ is the normalized threshold value.

*Result ordering*

Results are stored in input order regardless of canonical processing order.
Input `[Spread, Center]` produces output `[spread_projection, center_projection]`.

#source-include("cs/Pragmastat/Internal/CompareEngine.cs", "cs")
