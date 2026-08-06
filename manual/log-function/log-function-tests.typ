#import "/manual/definitions.typ": *

$ LogFunction(x) $

The $LogFunction$ test suite contains 5 test cases holding 1745 arguments in total
  (401 working band + 401 finite range + 401 near one + 30 boundaries + 512 random payloads).
All seven implementations compare the results by binary64 payload rather than by tolerance, since
  a tolerance would accept precisely the divergence this suite exists to detect.

*Working band* — 401 points over $[10^(-6), 10^6]$:

- The magnitudes measurements arrive at, spread geometrically rather than linearly: a linear grid
  over that interval would place almost every point in the same few exponents, which is the one
  thing the reduction distinguishes.

*Finite range* — 401 points over $[5 dot 10^(-324), 1.5 dot 10^308]$:

- Reaches from the smallest denormal to the largest finite double. The denormal end is where the
  exponent field carries no information and the pre-scaling has to run; 13 arguments across the
  suite are subnormal.

*Near one* — 401 points over $[1\/2, 2]$:

- The interval where the result crosses zero. Relative error is largest here, and it is the one
  place where a wrong sign or a lost low bit is invisible in absolute terms. 215 arguments across
  the suite reduce to $k = 0$, where the assembly takes its shorter form.

*Boundaries* — 30 points at the values where the construction changes behavior:

- The ends of the reduced range, $sqrt(2)\/2$ and $sqrt(2)$, each with the representable value on
  either side, so that the recentering branch is exercised on both of its answers
- The powers of two, where the exponent increments
- The smallest normal double and its predecessor, where the pre-scaling turns on

*Random payloads* — 512 uniformly drawn 64-bit patterns:

- The grids above walk the exponent smoothly while barely moving the significand, so a fault in the
  mantissa mask could sit between two grid points and never be evaluated. Drawing whole payloads
  spreads the significand uniformly instead. The draw uses the toolkit's own generator at a fixed
  seed, so the file regenerates byte for byte.

Zero and negative arguments are absent because their results are $-oo$ and NaN, and JSON provides
  no way to express either.
Each implementation keeps its own unit test for those, where a language can name both.

*Why this suite exists separately*

Every ratio fixture reaches this function, which sounds like cover and is not.
Those suites compared with a tolerance for as long as the platform logarithm decided their last
  bits, and a tolerance forgives exactly the last-bit divergence that decides which pairwise
  element a median selects.
Checking the function directly, by payload, is what allowed the ratio suites to become exact.

*Accuracy is checked against mathematics, not against agreement*

Comparing seven implementations against a fixture that one of them generates is a closed loop: a
  reduction wrong in the same way everywhere satisfies all of it.
The mise task `tests:check:function-oracles` compares the generated outputs against a 60-digit
  reference, which is precise enough to determine the correctly rounded binary64 result, and so
  states accuracy rather than agreement.
Over this suite $98.22%$ of results are correctly rounded and none is more than $0.64$ units in the
  last place from the true logarithm.
