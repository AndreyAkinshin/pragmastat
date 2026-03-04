#import "/manual/definitions.typ": *

The $Compare2$ test suite contains 26 test cases (5 demo + 4 multi-threshold + 2 order + 5 misrate + 4 natural + 2 property + 4 error).
All tests use seed `"compare2-tests"` for reproducibility.
Each test case output is a JSON object with a `projections` array; each projection has `estimate`, `lower`, `upper`, and `verdict` fields.

*Demo examples* ($n = m = 30$, $vx = (1, ..., 30)$, $vy = (21, ..., 50)$) --- single threshold, clear verdicts:

- `demo-shift-less`: shift threshold at $0$ with clearly negative shift → $Shift approx -20$, $upright("Less")$
- `demo-shift-greater`: $vx$ and $vy$ swapped → $Shift approx 20$, $upright("Greater")$
- `demo-shift-inconclusive`: $vx = vy$, threshold at $0$ → $upright("Inconclusive")$
- `demo-ratio-less`: $vx = (1, ..., 20)$, $vy = (21, ..., 40)$, ratio threshold at $1$ → $upright("Less")$
- `demo-disparity-less`: $vx = (1, ..., 30)$, $vy = (21, ..., 50)$, disparity threshold at $0$ → $upright("Less")$

*Multi-threshold* ($vx = (1, ..., 30)$, $vy = (21, ..., 50)$):

- `multi-shift-ratio`: combined shift and ratio thresholds
- `multi-shift-disparity`: combined shift and disparity thresholds
- `multi-all-three`: shift, ratio, and disparity together
- `multi-two-shifts`: two different shift thresholds

*Input order preservation* --- verifies output order matches input order, not canonical order:

- `order-disparity-shift`: disparity listed before shift → output[0] = disparity, output[1] = shift
- `order-ratio-shift`: ratio listed before shift → output[0] = ratio, output[1] = shift

*Misrate variation* ($vx = (1, ..., 20)$, $vy = (11, ..., 30)$, $Shift$ threshold at $0$):

5 tests spanning progressively stricter fixture misrates, from narrower to wider bounds.

*Natural sequences* (sizes from ${10, 15}$, achievable fixture misrates):

- `natural-10-10`, `natural-10-15`, `natural-15-10`, `natural-15-15`

*Property validation* ($vx = vy = (1, ..., 20)$):

- `property-shift-identity`: $Shift$ threshold at $0$ → bounds include $0$
- `property-ratio-identity`: $Ratio$ threshold at $1$ → bounds include $1$

*Error cases* --- inputs that violate assumptions:

- `error-empty-x`: $vx = ()$ → $"validity"(x)$
- `error-empty-y`: $vy = ()$ → $"validity"(y)$
- `error-constant-x-disparity`: $vx = (5, 5, ..., 5)$, $Disparity$ threshold → $"sparity"(x)$
- `error-constant-y-disparity`: $vy = (5, 5, ..., 5)$, $Disparity$ threshold → $"sparity"(y)$
