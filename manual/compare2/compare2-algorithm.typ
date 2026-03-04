#import "/manual/definitions.typ": *

$Compare2$ orchestrates estimation and comparison in two phases:
pre-pass validation and the statistical phase.

*Pre-pass validation*

Before any statistical computation:

1. Reject weighted samples for $vx$ and $vy$ (unsupported).
2. Check that $vx$ and $vy$ have compatible units.
3. Reject null or empty threshold list.
4. Reject threshold items containing null.
5. Reject thresholds with metrics not in ${Shift, Ratio, Disparity}$ (wrong arity).
6. Reject thresholds with non-finite values.

These checks happen before bounds computation, so no statistical work is done on invalid inputs.

*Validate-and-normalize pass*

For each threshold, in input order:

- *Shift*: check unit compatibility with $vx$; convert threshold value to the finer of $vx$'s and $vy$'s units.
- *Ratio*: accept unit $Ratio$ or dimensionless (coerce to $Ratio$); threshold value must be $> 0$.
- *Disparity*: accept unit $Disparity$ or dimensionless (coerce to $Disparity$); threshold value must be finite.

Bindings that support plain numeric shorthand (Python and R) interpret it directly
on the working comparison scale; explicit measurement thresholds are normalized as above.

*Statistical phase (canonical metric order: Shift → Ratio → Disparity)*

For each present metric (in canonical order), compute the estimate once and bounds for each threshold entry of that metric:

$ "estimate" = cases(Shift(vx, vy) & "if metric" = Shift, Ratio(vx, vy) & "if metric" = Ratio, Disparity(vx, vy) & "if metric" = Disparity) $

$ "bounds" = cases(ShiftBounds(vx, vy, misrate_i) & "if metric" = Shift, RatioBounds(vx, vy, misrate_i) & "if metric" = Ratio, DisparityBounds(vx, vy, misrate_i, "seed") & "if metric" = Disparity and "seed" != "null", DisparityBounds(vx, vy, misrate_i) & "if metric" = Disparity and "seed" = "null") $

*Verdict computation*

$ "verdict"_i = cases("Greater" & "if" L_i > t_i, "Less" & "if" U_i < t_i, "Inconclusive" & "otherwise") $

where $[L_i, U_i]$ are the bounds for threshold $i$ and $t_i$ is the normalized threshold value.

*Result ordering*

Results are stored in input order regardless of canonical processing order.
Input `[Disparity, Shift]` produces output `[disparity_projection, shift_projection]`.

#source-include("cs/Pragmastat/Internal/CompareEngine.cs", "cs")
