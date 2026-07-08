# Regression: shift search bounds x[1]-y[n] and x[m]-y[1] can overflow to
# -/+Inf on extreme finite input, turning the midpoint into NaN and returning
# +-Inf instead of the true finite shift.
test_that("shift does not overflow search bounds on extreme finite input", {
  max_val <- .Machine$double.xmax
  expect_equal(shift(c(-max_val, max_val), c(-max_val, max_val), assume_sorted = TRUE), 0)
  expect_equal(shift(c(0, max_val), c(-max_val, 0), assume_sorted = TRUE), max_val)
})
