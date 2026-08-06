#import "/manual/definitions.typ": *

The $AdditiveQuantile$ test suite contains 5 test cases holding 547 arguments in total, compared as
  raw binary64 payloads rather than against a tolerance.

- `central`: the band walked densely, 401 arguments, the center itself among them. The rational form is
  smooth, so a transcribed coefficient shows up as a smooth bias across many arguments rather than
  as a spike at one, which is what dense coverage is for.
- `branch-seams`: both seams, approached from each side with three representable steps. The first
  is $abs(p - 1\/2) = 0.425$, where the central form gives way to the tail; the second is $w = 5$,
  which separates the two tail approximants. A port that copied a bound as strict where it should
  be non-strict, or that transposed the two tail branches, produces its first wrong answer here and
  nowhere else.
- `near-tail`, `far-tail`: the two tail branches over the bands they own, the second ending at the
  smallest positive double.
- `textbook`: the quantiles a reader recognizes, and their complements.

Arguments outside the open interval are absent, because their result is an infinity and JSON can
  write neither an infinity nor a NaN.
Each port covers those in its own unit tests, where a language can name its own infinity.

Every port additionally asserts what no stored number can: that the center is exactly zero with a
  positive sign, that the central branch is exactly odd, that the function is monotone from the
  smallest positive double upward, and that it stays finite there.

Two further checks run in one port rather than seven, because accuracy and agreement are different
  properties.
Agreement is what seven ports can disagree about, and the fixture settles it payload by payload.
Accuracy is a property of the function, which the fixture has already shown to be one function, so
  measuring it once is measuring it everywhere.
Those two are a sweep against the high-precision reference with a bound of $16$ units in the last
  place, and a pin on the worst argument found on each branch.
The pin is what has teeth: a bound wide enough to survive a longer sweep cannot notice an error
  that doubles at one argument without crossing it, and the pinned arguments are exactly where the
  function is least comfortable.
Mutating a coefficient by one unit in the last place moves the pin and leaves the bound untouched,
  which is the division of labor intended.
