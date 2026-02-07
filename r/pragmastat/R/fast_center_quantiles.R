# Fast algorithm for computing specific quantiles of pairwise averages.
# Uses binary search with counting to avoid materializing all N(N+1)/2 pairs.

# Relative tolerance for convergence
.RELATIVE_EPSILON <- 1e-14

# Computes both lower and upper bounds from pairwise averages.
#
# @param sorted Sorted numeric vector
# @param margin_lo 1-based rank for lower bound
# @param margin_hi 1-based rank for upper bound
# @return List with 'lower' and 'upper' components
fast_center_quantile_bounds <- function(sorted, margin_lo, margin_hi) {
  n <- length(sorted)
  total_pairs <- as.numeric(n) * as.numeric(n + 1) / 2

  if (margin_lo < 1) margin_lo <- 1
  if (margin_lo > total_pairs) margin_lo <- total_pairs
  if (margin_hi < 1) margin_hi <- 1
  if (margin_hi > total_pairs) margin_hi <- total_pairs

  lo <- fast_center_find_exact_quantile(sorted, margin_lo)
  hi <- fast_center_find_exact_quantile(sorted, margin_hi)

  if (lo > hi) {
    tmp <- lo
    lo <- hi
    hi <- tmp
  }

  return(list(lower = lo, upper = hi))
}

# Counts pairwise averages <= target value using O(n) two-pointer algorithm.
fast_center_count_pairs <- function(sorted, target) {
  n <- length(sorted)
  count <- 0
  # j is not reset: as i increases, threshold decreases monotonically
  j <- n

  for (i in 1:n) {
    threshold <- 2 * target - sorted[i]

    while (j >= 1 && sorted[j] > threshold) {
      j <- j - 1
    }

    if (j >= i) {
      count <- count + (j - i + 1)
    }
  }

  return(count)
}

# Finds the exact k-th pairwise average using binary search + candidate refinement.
fast_center_find_exact_quantile <- function(sorted, k) {
  n <- length(sorted)
  total_pairs <- as.numeric(n) * as.numeric(n + 1) / 2

  if (n == 1) {
    return(sorted[1])
  }
  if (k == 1) {
    return(sorted[1])
  }
  if (k == total_pairs) {
    return(sorted[n])
  }

  lo <- sorted[1]
  hi <- sorted[n]
  eps <- .RELATIVE_EPSILON

  while (hi - lo > eps * max(1.0, abs(lo), abs(hi))) {
    mid <- (lo + hi) / 2
    count_le <- fast_center_count_pairs(sorted, mid)

    if (count_le >= k) {
      hi <- mid
    } else {
      lo <- mid
    }
  }

  target <- (lo + hi) / 2
  candidates <- numeric(0)

  for (i in 1:n) {
    threshold <- 2 * target - sorted[i]

    # Binary search for insertion point
    left <- i
    right <- n + 1

    while (left < right) {
      m <- (left + right) %/% 2
      if (sorted[m] < threshold - eps) {
        left <- m + 1
      } else {
        right <- m
      }
    }

    if (left <= n && left >= i && abs(sorted[left] - threshold) < eps * max(1.0, abs(threshold))) {
      candidates <- c(candidates, (sorted[i] + sorted[left]) / 2)
    }

    if (left > i) {
      avg_before <- (sorted[i] + sorted[left - 1]) / 2
      if (avg_before <= target + eps) {
        candidates <- c(candidates, avg_before)
      }
    }
  }

  if (length(candidates) == 0) {
    return(target)
  }

  candidates <- sort(candidates)

  for (candidate in candidates) {
    count_at <- fast_center_count_pairs(sorted, candidate)
    if (count_at >= k) {
      return(candidate)
    }
  }

  return(target)
}
