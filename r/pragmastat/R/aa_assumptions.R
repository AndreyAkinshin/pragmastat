# Assumption validation framework for Pragmastat
#
# Canonical priority order:
#   validity > domain > positivity > {sparity, distinctness} > cumulance
# Sparity and distinctness share a priority level: no input can violate both.

ASSUMPTION_IDS <- list(
  VALIDITY = "validity",
  DOMAIN = "domain",
  POSITIVITY = "positivity",
  SPARITY = "sparity",
  DISTINCTNESS = "distinctness",
  CUMULANCE = "cumulance"
)

SUBJECTS <- list(
  X = "x",
  Y = "y",
  MISRATE = "misrate"
)

DEFAULT_MISRATE <- 1e-3

assumption_error <- function(id, subject) {
  violation <- list(id = id, subject = subject)
  message <- paste0(id, "(", subject, ")")
  cond <- structure(
    list(
      message = message,
      call = sys.call(-1),
      violation = violation
    ),
    class = c("assumption_error", "error", "condition")
  )
  cond
}

check_validity <- function(values, subject) {
  if (length(values) == 0) {
    stop(assumption_error(ASSUMPTION_IDS$VALIDITY, subject))
  }
  if (any(is.na(values) | is.nan(values) | is.infinite(values))) {
    stop(assumption_error(ASSUMPTION_IDS$VALIDITY, subject))
  }
}

check_positivity <- function(values, subject) {
  if (any(values <= 0)) {
    stop(assumption_error(ASSUMPTION_IDS$POSITIVITY, subject))
  }
}

# Validity for a Distribution Estimation fitting vector.
# Beyond the shared checks, the value must actually be numeric. R coerces a
# factor to its integer codes without complaint, which would silently
# reconstruct a distribution over label positions instead of over measurements.
check_dist_validity <- function(values, subject) {
  if (!is.numeric(values)) {
    stop(assumption_error(ASSUMPTION_IDS$VALIDITY, subject))
  }
  check_validity(values, subject)
}

# Validity for a Distribution Estimation query vector.
# A query vector is a list of questions asked of a model, not a sample, so an
# empty vector satisfies the assumption vacuously. Every element that is present
# must be finite, and the vector must be numeric for the same reason as above.
# Violations always report subject "y".
check_query_validity <- function(values) {
  if (!is.numeric(values) && length(values) > 0) {
    stop(assumption_error(ASSUMPTION_IDS$VALIDITY, SUBJECTS$Y))
  }
  if (length(values) > 0 && any(!is.finite(values))) {
    stop(assumption_error(ASSUMPTION_IDS$VALIDITY, SUBJECTS$Y))
  }
}

# Every queried cumulance must lie in the closed unit interval.
# NaN is rejected here as well, matching the negated conjunction the other
# implementations spell out; query validity normally catches it first.
check_cumulance <- function(values) {
  if (any(is.na(values) | values < 0.0 | values > 1.0)) {
    stop(assumption_error(ASSUMPTION_IDS$CUMULANCE, SUBJECTS$Y))
  }
}

is_assumption_error <- function(e) {
  inherits(e, "assumption_error")
}

# Log-transform values with positivity check.
# Throws assumption_error if any value is non-positive.
log_transform <- function(values, subject) {
  if (any(values <= 0)) {
    stop(assumption_error(ASSUMPTION_IDS$POSITIVITY, subject))
  }
  vapply(values, log_function, numeric(1))
}
