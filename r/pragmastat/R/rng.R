#' Deterministic Random Number Generator
#'
#' A deterministic PRNG based on xoshiro256++ that produces identical sequences
#' across all Pragmastat language implementations when initialized with the same seed.
#'
#' @param seed Integer seed, string seed, or NULL for system time
#' @return An Rng object (R6 class)
#'
#' @examples
#' # Create from integer seed
#' r <- rng(1729)
#' r$uniform()
#'
#' # Create from string seed
#' r <- rng("experiment-1")
#'
#' # Shuffle a vector
#' r <- rng(1729)
#' r$shuffle(1:5)
#'
#' # Sample k elements
#' r <- rng(1729)
#' r$sample(0:9, 3)
#'
#' @export
rng <- function(seed = NULL) {
  Rng$new(seed)
}

#' @export
Rng <- R6::R6Class(
  "Rng",
  private = list(
    inner = NULL
  ),
  public = list(
    #' @description Create a new Rng
    #' @param seed Integer seed, string seed, or NULL for system time
    initialize = function(seed = NULL) {
      if (is.null(seed)) {
        seed_val <- as.numeric(Sys.time()) * 1e9
      } else if (is.character(seed)) {
        # fnv1a_hash returns a u64, pass it directly to preserve precision
        seed_val <- fnv1a_hash(seed)
      } else {
        seed_val <- as.numeric(seed)
      }
      private$inner <- xoshiro256_new(seed_val)
    },

    #' @description Generate a uniform random float in [0, 1)
    #' @return A random value in [0, 1)
    uniform = function() {
      xoshiro256_uniform(private$inner)
    },

    #' @description Generate a uniform random integer in [min, max)
    #'
    #' Uses modulo reduction which introduces slight bias for ranges that don't
    #' evenly divide 2^64. This bias is negligible for statistical simulations
    #' but not suitable for cryptographic applications.
    #'
    #' @param min_val Minimum value (inclusive)
    #' @param max_val Maximum value (exclusive)
    #' @return A random integer in [min, max)
    uniform_int = function(min_val, max_val) {
      xoshiro256_uniform_int(private$inner, min_val, max_val)
    },

    #' @description Return a shuffled copy of the input vector
    #' @param x Input vector to shuffle
    #' @return Shuffled copy of the input
    shuffle = function(x) {
      result <- x
      n <- length(result)

      # Fisher-Yates shuffle (backwards)
      # Note: R uses 1-based indexing, so j is in [1, i] instead of [0, i-1]
      # This is equivalent to other languages' uniform_int(0, i+1) for 0-based arrays
      if (n > 1) {
        for (i in n:2) {
          j <- self$uniform_int(1L, i + 1L)
          temp <- result[i]
          result[i] <- result[j]
          result[j] <- temp
        }
      }

      result
    },

    #' @description Sample k elements from the input vector without replacement
    #' @param x Input vector to sample from
    #' @param k Number of elements to sample (must be non-negative)
    #' @return Vector of k sampled elements
    sample = function(x, k) {
      if (k < 0) stop("k must be non-negative")
      n <- length(x)
      if (k >= n) return(x)

      result <- vector(mode = typeof(x), length = k)
      result_idx <- 0L
      remaining <- k

      for (i in seq_len(n)) {
        if (remaining == 0L) break
        available <- n - i + 1L
        # Probability of selecting this item: remaining / available
        if (self$uniform() * available < remaining) {
          result_idx <- result_idx + 1L
          result[result_idx] <- x[i]
          remaining <- remaining - 1L
        }
      }

      result
    }
  )
)
