# xoshiro256++ PRNG implementation for cross-language reproducibility
#
# Reference: https://prng.di.unimi.it/xoshiro256plusplus.c
#
# This implementation uses a custom 64-bit unsigned integer representation
# as a list with $hi and $lo 32-bit components to support full bitwise operations.

# Unsigned 64-bit integer represented as two 32-bit parts
# All values are treated as unsigned: hi in [0, 2^32-1], lo in [0, 2^32-1]
u64 <- function(hi, lo) {
  # Ensure values are proper unsigned 32-bit (stored as numeric to avoid overflow)
  list(hi = as.numeric(hi) %% 4294967296, lo = as.numeric(lo) %% 4294967296)
}

# Create u64 from a numeric value (seed)
u64_from_numeric <- function(n) {
  n <- as.numeric(n)
  if (n < 0) {
    # Handle negative as two's complement
    # For a negative number -x, two's complement is 2^64 - x
    # We compute this in parts to avoid precision loss
    # -1 -> 0xFFFFFFFFFFFFFFFF (hi=0xFFFFFFFF, lo=0xFFFFFFFF)
    # -2 -> 0xFFFFFFFFFFFFFFFE (hi=0xFFFFFFFF, lo=0xFFFFFFFE)
    pos <- -n  # Make positive
    # Compute 2^64 - pos = (2^32 - 1) * 2^32 + (2^32 - pos) if pos <= 2^32
    # More generally: 2^64 - pos
    if (pos <= 4294967296) {
      # pos fits in lo part
      lo <- 4294967296 - pos
      hi <- 4294967295  # 2^32 - 1
      if (lo == 4294967296) {
        # pos was 0, which shouldn't happen for negative n
        lo <- 0
        hi <- 0
      }
    } else {
      # pos spans both parts
      lo <- (4294967296 - (pos %% 4294967296)) %% 4294967296
      hi <- 4294967295 - floor(pos / 4294967296)
      if ((pos %% 4294967296) != 0) {
        hi <- hi  # No adjustment needed
      } else {
        hi <- hi + 1  # Borrow from hi
      }
      hi <- hi %% 4294967296
    }
    return(u64(hi, lo))
  }
  hi <- floor(n / 4294967296) %% 4294967296
  lo <- n %% 4294967296
  u64(hi, lo)
}

# Parse 8-character hex string to unsigned 32-bit value
# strtoi is limited to signed 32-bit, so we handle large values manually
hex8_to_u32 <- function(hex8) {
  # Parse as two 4-char chunks (16 bits each) to avoid overflow
  hi16 <- strtoi(substr(hex8, 1, 4), base = 16L)
  lo16 <- strtoi(substr(hex8, 5, 8), base = 16L)
  hi16 * 65536 + lo16
}

# Create u64 from hex string
u64_from_hex <- function(hex_str) {
  # Remove 0x prefix if present
  hex_str <- sub("^0x", "", hex_str)
  # Pad to 16 characters
  hex_str <- sprintf("%016s", hex_str)
  hex_str <- gsub(" ", "0", hex_str)
  hi <- hex8_to_u32(substr(hex_str, 1, 8))
  lo <- hex8_to_u32(substr(hex_str, 9, 16))
  u64(hi, lo)
}

# Convert u64 to numeric (for final output, may lose precision for large values)
u64_to_numeric <- function(x) {
  x$hi * 4294967296 + x$lo
}

# Bitwise XOR
u64_xor <- function(a, b) {
  # Use bitwXor on 32-bit parts, handling the fact that R's bitwXor expects integers
  hi <- bitwXor(as.integer(a$hi %% 2147483648), as.integer(b$hi %% 2147483648))
  hi_sign <- bitwXor(as.integer(floor(a$hi / 2147483648)), as.integer(floor(b$hi / 2147483648)))
  hi <- as.numeric(hi) + hi_sign * 2147483648

  lo <- bitwXor(as.integer(a$lo %% 2147483648), as.integer(b$lo %% 2147483648))
  lo_sign <- bitwXor(as.integer(floor(a$lo / 2147483648)), as.integer(floor(b$lo / 2147483648)))
  lo <- as.numeric(lo) + lo_sign * 2147483648

  u64(hi, lo)
}

# Bitwise OR
u64_or <- function(a, b) {
  hi <- bitwOr(as.integer(a$hi %% 2147483648), as.integer(b$hi %% 2147483648))
  hi_sign <- bitwOr(as.integer(floor(a$hi / 2147483648)), as.integer(floor(b$hi / 2147483648)))
  hi <- as.numeric(hi) + hi_sign * 2147483648

  lo <- bitwOr(as.integer(a$lo %% 2147483648), as.integer(b$lo %% 2147483648))
  lo_sign <- bitwOr(as.integer(floor(a$lo / 2147483648)), as.integer(floor(b$lo / 2147483648)))
  lo <- as.numeric(lo) + lo_sign * 2147483648

  u64(hi, lo)
}

# Left shift (k < 64)
u64_shl <- function(x, k) {
  if (k == 0) return(x)
  if (k >= 64) return(u64(0, 0))
  if (k >= 32) {
    # Shift lo into hi, lo becomes 0
    hi <- (x$lo * (2^(k - 32))) %% 4294967296
    return(u64(hi, 0))
  }
  # k < 32
  # hi gets: (hi << k) | (lo >> (32-k))
  # lo gets: lo << k
  shift_mult <- 2^k
  hi <- ((x$hi * shift_mult) %% 4294967296) + floor(x$lo / (2^(32 - k)))
  hi <- hi %% 4294967296
  lo <- (x$lo * shift_mult) %% 4294967296
  u64(hi, lo)
}

# Right shift (logical, k < 64)
u64_shr <- function(x, k) {
  if (k == 0) return(x)
  if (k >= 64) return(u64(0, 0))
  if (k >= 32) {
    # Shift hi into lo, hi becomes 0
    lo <- floor(x$hi / (2^(k - 32)))
    return(u64(0, lo))
  }
  # k < 32
  # lo gets: (lo >> k) | (hi << (32-k))
  # hi gets: hi >> k
  lo <- floor(x$lo / (2^k)) + ((x$hi %% (2^k)) * (2^(32 - k)))
  lo <- lo %% 4294967296
  hi <- floor(x$hi / (2^k))
  u64(hi, lo)
}

# Rotate left
u64_rotl <- function(x, k) {
  u64_or(u64_shl(x, k), u64_shr(x, 64L - k))
}

# Addition (wrapping)
u64_add <- function(a, b) {
  lo <- a$lo + b$lo
  carry <- floor(lo / 4294967296)
  lo <- lo %% 4294967296
  hi <- (a$hi + b$hi + carry) %% 4294967296
  u64(hi, lo)
}

# Multiplication (wrapping, for 64x64 -> lower 64 bits)
u64_mul <- function(a, b) {
  # Split each into 16-bit parts for safe multiplication
  a0 <- a$lo %% 65536
  a1 <- floor(a$lo / 65536)
  a2 <- a$hi %% 65536
  a3 <- floor(a$hi / 65536)

  b0 <- b$lo %% 65536
  b1 <- floor(b$lo / 65536)
  b2 <- b$hi %% 65536
  b3 <- floor(b$hi / 65536)

  # Compute partial products that contribute to lower 64 bits
  # Result bits 0-15: a0*b0
  # Result bits 16-31: a0*b1 + a1*b0
  # Result bits 32-47: a0*b2 + a1*b1 + a2*b0
  # Result bits 48-63: a0*b3 + a1*b2 + a2*b1 + a3*b0

  p0 <- a0 * b0
  p1 <- a0 * b1 + a1 * b0
  p2 <- a0 * b2 + a1 * b1 + a2 * b0
  p3 <- a0 * b3 + a1 * b2 + a2 * b1 + a3 * b0

  # Combine with carries
  lo <- p0 + (p1 %% 65536) * 65536
  carry <- floor(lo / 4294967296) + floor(p1 / 65536)
  lo <- lo %% 4294967296

  hi <- p2 + (p3 %% 65536) * 65536 + carry
  hi <- hi %% 4294967296

  u64(hi, lo)
}

# Modulo (for uniform_int) - a is u64, b is a small positive numeric
# Uses long division approach to avoid precision loss
u64_mod <- function(a, b_numeric) {
  if (b_numeric <= 0) return(0)
  if (b_numeric == 1) return(0)

  # For small moduli, we can compute using the formula:
  # (hi * 2^32 + lo) mod b = ((hi mod b) * (2^32 mod b) + (lo mod b)) mod b
  # This avoids precision loss as long as b < 2^32

  if (b_numeric < 4294967296) {
    hi_mod <- a$hi %% b_numeric
    lo_mod <- a$lo %% b_numeric
    # 2^32 mod b
    pow32_mod <- 4294967296 %% b_numeric
    result <- (hi_mod * pow32_mod + lo_mod) %% b_numeric
    return(result)
  }

  # For larger moduli, use direct conversion (may lose some precision for very large a)
  a_numeric <- u64_to_numeric(a)
  a_numeric %% b_numeric
}

# SplitMix64 PRNG for seed expansion
# seed can be numeric or a u64 list
splitmix64_new <- function(seed) {
  env <- new.env(parent = emptyenv())
  if (is.list(seed) && !is.null(seed$hi) && !is.null(seed$lo)) {
    # Already a u64
    env$state <- seed
  } else {
    env$state <- u64_from_numeric(seed)
  }
  env
}

splitmix64_next <- function(sm) {
  C1 <- u64_from_hex("9e3779b97f4a7c15")
  C2 <- u64_from_hex("bf58476d1ce4e5b9")
  C3 <- u64_from_hex("94d049bb133111eb")

  sm$state <- u64_add(sm$state, C1)
  z <- sm$state
  z <- u64_mul(u64_xor(z, u64_shr(z, 30L)), C2)
  z <- u64_mul(u64_xor(z, u64_shr(z, 27L)), C3)
  u64_xor(z, u64_shr(z, 31L))
}

# xoshiro256++ internal state
xoshiro256_new <- function(seed) {
  sm <- splitmix64_new(seed)
  env <- new.env(parent = emptyenv())
  env$s0 <- splitmix64_next(sm)
  env$s1 <- splitmix64_next(sm)
  env$s2 <- splitmix64_next(sm)
  env$s3 <- splitmix64_next(sm)
  env
}

xoshiro256_next_u64 <- function(xo) {
  result <- u64_add(u64_rotl(u64_add(xo$s0, xo$s3), 23L), xo$s0)

  t <- u64_shl(xo$s1, 17L)

  xo$s2 <- u64_xor(xo$s2, xo$s0)
  xo$s3 <- u64_xor(xo$s3, xo$s1)
  xo$s1 <- u64_xor(xo$s1, xo$s2)
  xo$s0 <- u64_xor(xo$s0, xo$s3)

  xo$s2 <- u64_xor(xo$s2, t)
  xo$s3 <- u64_rotl(xo$s3, 45L)

  result
}

# ========================================================================
# Floating Point Methods
# ========================================================================

xoshiro256_uniform <- function(xo) {
  # Use upper 53 bits for maximum precision
  u64_val <- xoshiro256_next_u64(xo)
  # Shift right by 11 bits
  shifted <- u64_shr(u64_val, 11L)
  # Convert to numeric and scale
  u64_to_numeric(shifted) * (1.0 / 2^53)
}

xoshiro256_uniform_range <- function(xo, min_val, max_val) {
  if (min_val >= max_val) return(min_val)
  min_val + (max_val - min_val) * xoshiro256_uniform(xo)
}

# ========================================================================
# Integer Methods
# ========================================================================

xoshiro256_uniform_int <- function(xo, min_val, max_val) {
  if (min_val >= max_val) return(min_val)
  range_size <- as.numeric(max_val - min_val)
  # Validate range fits in i64 (for cross-language consistency)
  if (range_size > 9223372036854775807) {
    stop("uniform_int: range overflow (max - min exceeds i64)")
  }
  u64_val <- xoshiro256_next_u64(xo)
  # Return as numeric to avoid as.integer() truncation for values > 2^31-1
  # R's numeric (double) can represent integers exactly up to 2^53
  min_val + u64_mod(u64_val, range_size)
}

# ========================================================================
# Boolean Methods
# ========================================================================

xoshiro256_uniform_bool <- function(xo) {
  xoshiro256_uniform(xo) < 0.5
}

# FNV-1a hash
fnv1a_hash <- function(s) {
  FNV_OFFSET_BASIS <- u64_from_hex("cbf29ce484222325")
  FNV_PRIME <- u64_from_hex("00000100000001b3")

  hash <- FNV_OFFSET_BASIS
  bytes <- as.integer(charToRaw(enc2utf8(s)))
  for (b in bytes) {
    hash <- u64_xor(hash, u64(0, b))
    hash <- u64_mul(hash, FNV_PRIME)
  }
  hash
}
