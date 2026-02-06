# Minimum achievable misrate functions for bounds validation.

# Computes the minimum achievable misrate for one-sample bounds.
# For a sample of size n, the minimum achievable misrate is 2^(1-n),
# which corresponds to the probability of the most extreme configuration
# in the Wilcoxon signed-rank distribution.
#
# @param n Sample size (must be positive)
# @return Minimum achievable misrate
min_achievable_misrate_one_sample <- function(n) {
  if (n <= 0) stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$X))
  2^(1 - n)
}

# Computes the minimum achievable misrate for two-sample Mann-Whitney based bounds.
#
# @param n Size of first sample (must be positive)
# @param m Size of second sample (must be positive)
# @return Minimum achievable misrate
min_achievable_misrate_two_sample <- function(n, m) {
  if (n <= 0) stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$X))
  if (m <= 0) stop(assumption_error(ASSUMPTION_IDS$DOMAIN, SUBJECTS$Y))
  2 / choose(n + m, n)
}
