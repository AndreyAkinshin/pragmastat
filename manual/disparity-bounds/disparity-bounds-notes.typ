#import "/manual/definitions.typ": *

==== Width Convergence

The table below shows how $"Width" = U - L$ narrows as $N$ grows,
for $vx = vy = (1, 1 + 1\/(N-1), ..., 2)$ ($N$ evenly spaced points on $[1, 2]$)
and $misrate = 10^(-3)$.
Dashes indicate $N$ too small to achieve the target misrate.

#include "disparity-bounds-width-table.typ"

#image("/img/bounds-width-disparity_light.png")
