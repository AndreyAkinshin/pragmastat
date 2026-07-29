#import "/manual/definitions.typ": *

$ AdditiveCumulative(z) $

The function has no shared fixture directory of its own.
Three arrangements cover it instead, each reaching a property the others do not.

*Through the margins*.
Every fixture of #link(<sec-pairwise-margin>)[$PairwiseMargin$] and
  #link(<sec-signed-rank-margin>)[$SignedRankMargin$] whose sample size exceeds the
  exact-computation limit evaluates this function, and those fixtures compare by binary64 payload
  rather than by tolerance.
This is the coverage that matters most, because it exercises the function at the values the
  estimators actually reach, including the fixtures placed exactly on an Edgeworth decision
  boundary where the returned integer changes between one representable misrate and the next.

*Against a reference*.
A fixture of 999 quantiles of the standard $Additive$ distribution records the values the generating
  implementation produces, which pins the approximation itself rather than its effect on a margin.
This fixture is read by the generating implementation alone, so it constrains that one rather than
  the agreement between the seven.

*Under perturbation*.
The conformance experiment described in the methodology chapter recomputes every estimator with
  each library call returning the neighboring representable value, and requires the margins that
  depend on this function not to move.
The only library call remaining on that path is an exponential the toolkit supplies itself, so the
  requirement holds by construction rather than by measurement.

A separate suite of arguments in isolation was considered and not built.
Such a suite would pin the function at points no estimator visits while adding nothing at the
  points they do, and a fixture the code never reaches is a maintenance cost rather than a
  guarantee.
The exponential beneath it is a different case and does have its own suite, described in
  #link(<sec-exp-function>)[$ExpFunction$], because that is the layer where the seven
  implementations could disagree.
