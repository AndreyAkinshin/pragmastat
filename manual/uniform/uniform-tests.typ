#import "/manual/definitions.typ": *

The `dist-uniform` test suite contains 6 test cases: six intervals, including one entirely below zero and one straddling it.
Each holds a density and a cumulative at every point of a grid drawn from the family's own
  quantile, and a quantile at every hundredth, 1818 values in all.

The comparison is by raw binary64 payload rather than against a tolerance, because nothing on
  these three paths reaches a platform transcendental and the seven ports therefore owe each other
  identical bits.
A tolerance would pass on exactly the divergence the suite exists to catch.

Every port additionally pins the number of values it compared, since a loader that finds four
  suites out of five satisfies every assertion it makes by having less to compare.
