#import "/manual/definitions.typ": *

== Introduction

Practitioners who need to apply statistical procedures often struggle
  to choose the right one,
  whether the question concerns the typical value of the data,
  the variability of the measurements,
  or the difference between two groups.
Traditional statistics answers with dozens of methods,
  most of them named after their inventors,
  each valid under its own set of assumptions,
  each requiring further procedures to verify those assumptions.
Textbook recommendations diverge.
An incorrect choice rarely reveals itself:
  the procedure returns a number regardless, and the number looks plausible.
As a result, the answer demands expertise that has little to do with the question.

The struggle reflects how the field developed.
Traditional statistical practice was not designed as a system — it accumulated.
Its methods came from different researchers, different decades, and different problems.
Everyday practice inherited a patchwork of procedures, conventions, and thresholds.
A patchwork offers no coherent interface,
  hence the inconsistent naming and the folklore needed to navigate among the methods.

Choosing a method is only half of the problem:
  many traditional procedures are optimal under theoretical conditions
  that real data rarely satisfies.
Real measurements are often skewed, heavy-tailed, contaminated by outliers, and affected by ties.
On such data, these procedures may quietly produce misleading results.
The traditional toolkit is therefore both difficult to choose from and risky to apply.

Pragmastat addresses both problems.
It is a statistical toolkit designed as a whole:
  a small, closed set of procedures that covers the typical questions
  practitioners ask about data.
The procedures work on real data as it comes,
  without distribution diagnostics, tuning parameters, or lookup tables.
The toolkit reuses well-studied robust estimators:
  it renames, recombines, and refines them into a consistent system.
This manual is the complete specification:
  definitions, algorithms, properties, and reference tests.

The toolkit is available as ready-to-use libraries
  for Python, TypeScript, R, C\#, Kotlin, Rust, and Go.
All seven share a consistent, human-friendly API and produce identical results.
The implementations form the executable part of the manual.

Two principles govern the design of the toolkit: usability and reliability.
The toolkit helps practitioners rather than demanding adaptation:
  every function is named after what it does, follows shared conventions,
  and reports results in the units of the measurements.
Each task has a single recommended default,
  so the choice of method follows from the question itself.
The results, in turn, deserve trust:
  the estimators withstand the outliers and heavy tails common in real data,
  the uncertainty bounds state their error probability explicitly,
  and when the data cannot support a request,
  the function reports a validation error rather than a misleading answer.
The computations are fast and fully deterministic:
  the same input produces the same result in every language and on every platform.

The primary audience is practitioners:
  engineers, researchers, and analysts who need trustworthy numbers for decisions.
Readers interested in theory will find algorithms, asymptotic properties,
  references, and discussions of limitations
  in the estimator sections and the appendix.
The scope is closed by design:
  every procedure is justified by a practical need,
  and tasks beyond it call for other statistical instruments.

The synopsis below lists every function of the toolkit.
Each section of the manual is self-contained and follows the same layout:
  the definition comes first, then an example, then details for readers who need them.
The appendix collects the underlying assumptions, foundations, and methodology.
