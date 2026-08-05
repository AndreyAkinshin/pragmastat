# How a carrier states a double.
#
# Both carriers render through here so neither can drift from the other.
#
# R has no shortest-round-trip formatter. as.character stops at 15 significant digits, so
# 0.5008354224706334 comes back as 0.500835422470633 and no longer reads as the same double.
# This raises the precision until the text parses back to the value it came from, which is what
# Rust's {}, JavaScript's interpolation and Python's repr each give directly.
#
# scientific = FALSE keeps the whole number visible, matching Rust and Kotlin. Go is the one port
# that switches to an exponent, and does so from 1e+06 upward, so joining it here would mean
# disagreeing with the other five.
format_value <- function(v) {
  if (!is.finite(v)) {
    return(as.character(v))
  }
  for (digits in 1:17) {
    rendered <- format(v, digits = digits, scientific = FALSE, trim = TRUE)
    if (identical(as.numeric(rendered), v)) {
      return(rendered)
    }
  }
  format(v, digits = 17, scientific = FALSE, trim = TRUE)
}
