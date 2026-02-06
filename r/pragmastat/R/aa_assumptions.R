# Assumption validation framework for Pragmastat

ASSUMPTION_IDS <- list(
  VALIDITY = "validity",
  DOMAIN = "domain",
  POSITIVITY = "positivity",
  SPARITY = "sparity"
)

SUBJECTS <- list(
  X = "x",
  Y = "y",
  MISRATE = "misrate"
)

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

check_sparity <- function(values, subject) {
  if (length(values) < 2) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, subject))
  }
  spread_val <- fast_spread(values)
  if (spread_val <= 0) {
    stop(assumption_error(ASSUMPTION_IDS$SPARITY, subject))
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
  log(values)
}
