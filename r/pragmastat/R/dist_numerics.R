# Binary64 primitives shared by the distribution estimation models.
#
# Every operation is written out explicitly so that all language implementations
# evaluate the same graph and produce identical bits. None of these helpers may
# be replaced by a fused, vectorized or library variant without a new algorithm
# identifier.

# Adds delta (+1 or -1) to the 64-bit pattern of a finite non-zero value.
# R has no integer view of a double, so the pattern is stepped through its raw
# little-endian bytes; this reproduces the two's-complement increment the other
# implementations get from doubleToLongBits, because carrying over the byte
# sequence and carrying over the 64-bit word are the same operation.
dist_step_bits <- function(value, delta) {
  bytes <- writeBin(value, raw(), size = 8L, endian = "little")
  i <- 1L
  while (i <= 8L) {
    stepped <- as.integer(bytes[i]) + delta
    if (stepped > 255L) {
      bytes[i] <- as.raw(0L)
    } else if (stepped < 0L) {
      bytes[i] <- as.raw(255L)
    } else {
      bytes[i] <- as.raw(stepped)
      break
    }
    i <- i + 1L
  }
  readBin(bytes, "double", n = 1L, size = 8L, endian = "little")
}

# Next representable binary64 value toward positive infinity.
dist_next_up <- function(value) {
  if (is.na(value) || value == Inf) {
    return(value)
  }
  if (value == 0.0) {
    return(.SMALLEST_POSITIVE_SUBNORMAL)
  }
  dist_step_bits(value, if (value > 0.0) 1L else -1L)
}

# Next representable binary64 value toward negative infinity.
dist_next_down <- function(value) {
  if (is.na(value) || value == -Inf) {
    return(value)
  }
  if (value == 0.0) {
    return(-.SMALLEST_POSITIVE_SUBNORMAL)
  }
  dist_step_bits(value, if (value > 0.0) -1L else 1L)
}

# Overflow-safe midpoint of two finite values.
dist_midpoint <- function(a, b) {
  if (a < 0.0 && 0.0 < b) {
    return(a / 2.0 + b / 2.0)
  }
  a + (b - a) / 2.0
}

# Interpolates between a and b, reproducing both endpoints exactly.
dist_lerp <- function(a, b, t) {
  if (t == 0.0) {
    return(a)
  }
  if (t == 1.0) {
    return(b)
  }
  if (a < 0.0 && 0.0 < b) {
    return((1.0 - t) * a + t * b)
  }
  a + t * (b - a)
}

# Ratio clamped to the unit interval. Callers must have proved 0 <= num <= den
# and den > 0.
dist_unit_ratio <- function(num, den) {
  t <- num / den
  if (is.na(t)) {
    stop("Numerical conformance failure: dist_unit_ratio produced NaN")
  }
  min(1.0, max(0.0, t))
}

# Smallest integer k with k >= ceil(n / k), computed without forming k * k.
dist_ceil_sqrt <- function(n) {
  k <- 1L
  while (k < (n + k - 1L) %/% k) {
    k <- k + 1L
  }
  k
}

# First index i with values[i] >= key, over an ascending vector.
# Both the bounds and the result are zero-based, so the search agrees with the
# other implementations and with the index arithmetic of the model kernels.
dist_lower_bound <- function(values, from, to, key) {
  lo <- from
  hi <- to
  while (lo < hi) {
    mid <- lo + (hi - lo) %/% 2L
    if (values[mid + 1L] < key) {
      lo <- mid + 1L
    } else {
      hi <- mid
    }
  }
  lo
}

# First index i with values[i] > key, over an ascending vector. Zero-based.
dist_upper_bound <- function(values, key) {
  lo <- 0L
  hi <- length(values)
  while (lo < hi) {
    mid <- lo + (hi - lo) %/% 2L
    if (values[mid + 1L] <= key) {
      lo <- mid + 1L
    } else {
      hi <- mid
    }
  }
  lo
}
