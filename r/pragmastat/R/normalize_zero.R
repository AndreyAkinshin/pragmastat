# Strips the sign off a negative zero and leaves every other double untouched.
#
# When a sample carries both +0 and -0, nothing in the data decides which of
# them lands in the position an estimator selects: comparison cannot separate
# the two, so the sorting algorithm picks. Each port sorts its own way, and they
# disagreed on that pick, e.g. center(c(0, -0, 0, -0, 1)) came back -0 in some
# ports and +0 in others. That breaks the exact conformance class the toolkit
# publishes, which promises the same bits from the same input everywhere.
#
# The sign of a zero carries no statistical meaning here: -0 == 0, and no
# estimate gets less accurate by dropping it. So every estimator normalizes on
# the way out, which closes the whole class of divergence rather than the one
# sample that exposed it. Inputs are left alone: a sample may still contain -0.
#
# isTRUE() rather than a bare comparison so NaN and NA flow through instead of
# turning into a "missing value where TRUE/FALSE needed" error.
normalize_zero <- function(v) {
  if (isTRUE(v == 0)) {
    return(0)
  }
  v
}
