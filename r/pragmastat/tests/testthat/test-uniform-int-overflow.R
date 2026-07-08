# Regression: u64_mod lost precision for moduli >= 2^32 (it fell back to a
# lossy double conversion). It is now exact up to 2^52 via mulmod. Oracle values
# come from Python's exact big-integer arithmetic on 0x123456789abcdef0.
test_that("u64_mod is exact for moduli at and above 2^32", {
  a <- u64_from_hex("123456789abcdef0")
  expect_equal(u64_mod(a, 2^40), 517992144624)
  expect_equal(u64_mod(a, 2^45), 24707247955696)
  expect_equal(u64_mod(a, 2^30), 448585456)
})

# Contract: ranges above 2^52 cannot be sampled exactly with double-based
# arithmetic and are rejected, consistently with the other languages.
test_that("uniform_int rejects ranges above 2^52", {
  rng <- Rng$new(1)
  expect_error(rng$uniform_int(0, 2^53), "2\\^52")
})
