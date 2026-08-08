#import "/manual/definitions.typ": *

The `dist-power` test suite contains 5 test cases: five pairs, with shapes from 0.5 to 5.
Each holds a density and a cumulative at every point of a grid drawn from the family's own
  quantile, and a quantile at every hundredth, 1500 values in all.

The comparison is by raw binary64 payload rather than against a tolerance, because nothing on
  these three paths reaches a platform transcendental and the seven ports therefore owe each other
  identical bits.
A tolerance would pass on exactly the divergence the suite exists to catch.

Every port additionally pins the number of values it compared, since a loader that finds four
  suites out of five satisfies every assertion it makes by having less to compare.
