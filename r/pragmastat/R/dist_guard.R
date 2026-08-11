# Refuses an assume_sorted promise made about a Sample.
#
# The static ports give their Sample overload no such parameter, so the
# combination cannot be written at all. R reaches both worlds through one
# function, so the same rule has to be a runtime check: a Sample carries its own
# ascending promise, and the model never consults the flag on that path. The
# argument is missing(assume_sorted) read in the caller, so an explicit FALSE is
# refused too: it is a statement about an input that is not read.
check_no_assume_sorted <- function(omitted) {
  if (!omitted) {
    stop("assume_sorted does not apply to a Sample, which already keeps its values ascending")
  }
}

# Shared sample preparation for the distribution estimation models.
#
# Runs the two assumptions that follow validity in the canonical order:
# domain(x) (at least two observations and a finite representable range) and
# distinctness(x) (no repeated value; -0.0 and +0.0 count as repeated).
# The result is a private ascending copy; R's copy-on-modify semantics keep the
# caller's vector unreachable from the model even when assume_sorted skips the
# sort.
dist_prepare <- function(values, assume_sorted) {
  x <- as.double(values)
  if (!assume_sorted) {
    x <- sort(x)
  }

  n <- length(x)
  if (n < 2L) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$X))
  }

  span <- x[n] - x[1L]
  if (is.na(span) || is.infinite(span)) {
    stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$X))
  }

  if (any(x[-n] == x[-1L])) {
    stop(assumption_error(ASSUMPTION_IDS$DISTINCTNESS, SUBJECTS$X))
  }

  x
}

# Attaches a unit to a query result. R is a vector language and Measurement is
# a value with a unit, so one Measurement carries the whole result vector: the
# unit belongs to the query, not to each element of it.
dist_to_measurement <- function(values, unit) {
  Measurement$new(values, unit)
}

# Converts a measured query vector into the model's own unit.
#
# A query in a compatible but different unit is converted rather than rejected,
# matching the two-sample estimators, which reconcile their inputs instead of
# demanding an exact match. An incompatible unit is a caller error and is
# reported as one.
#
# Validity of the query runs here, before the conversion, so a non-numeric
# Measurement is reported as validity(y) instead of reaching the multiplication.
# The query kernels check the converted vector again, because the conversion
# itself can overflow a finite query to infinity.
dist_to_raw <- function(values, unit) {
  if (!inherits(values, "Measurement")) {
    stop("a measured query must be a Measurement carrying the unit of its values")
  }
  from <- values$unit
  if (!from$is_compatible(unit)) {
    stop(paste0("can't convert ", from$full_name, " to ", unit$full_name))
  }
  check_query_validity(values$value)
  # as.double matches what the query kernels do with a vector that passed
  # validity: it can only see numbers or an empty query by this point.
  as.double(values$value) * conversion_factor(from, unit)
}
