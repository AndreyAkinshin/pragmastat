#import "/manual/definitions.typ": *

The $Compare1$ test suite contains 18 test cases (5 demo + 3 multi-threshold + 1 order + 3 misrate + 3 natural + 3 error).
All tests use seed `"compare1-tests"` for reproducibility.
Each test case output is a JSON object with a `projections` array; each projection has `estimate`, `lower`, `upper`, and `verdict` fields.

*Demo examples* ($n = 10$, $vx = (1, ..., 10)$) --- single threshold, clear verdicts:

- `demo-center-less`: center threshold above the upper bound → $upright("Less")$
- `demo-center-greater`: center threshold below the lower bound → $upright("Greater")$
- `demo-center-inconclusive`: center threshold inside the bounds → $upright("Inconclusive")$
- `demo-spread-less`: spread threshold above the upper bound → $upright("Less")$
- `demo-spread-greater`: spread threshold below the lower bound → $upright("Greater")$

*Multi-threshold* ($n = 10$) --- multiple thresholds per call:

- `multi-center-spread`: one center threshold and one spread threshold → $["Less", "Greater"]$
- `multi-two-centers`: two center thresholds in one call → $["Less", "Greater"]$
- `multi-mixed`: mixed center/spread thresholds → $["Greater", "Less", "Less"]$

*Input order preservation* --- verifies output order matches input order, not canonical order:

- `order-spread-center`: spread threshold listed before center threshold → output[0] = spread projection, output[1] = center projection

*Misrate variation* ($n = 20$, $vx = (1, ..., 20)$, $Center$ threshold at $10$):

3 tests spanning progressively stricter fixture misrates, from narrower to wider bounds.

These validate that smaller misrates produce wider bounds.

*Natural sequences*:

- `natural-10`: $n = 10$, $Center$ threshold at $5.5$
- `natural-15`: $n = 15$, $Center$ threshold at $8$
- `natural-20`: $n = 20$, $Center$ threshold at $10.5$

*Error cases* --- inputs that violate assumptions:

- `error-empty-x`: $vx = ()$ → $"validity"(x)$
- `error-single-x-center`: $|vx| = 1$, $Center$ threshold → $"domain"(x)$ (requires $n >= 2$)
- `error-constant-spread`: $vx = (5, 5, 5, 5, 5, 5)$, $Spread$ threshold → $"sparity"(x)$
