"""How a carrier states a double.

Both carriers render through here so neither can drift from the other.

repr already gives Python's shortest round-trip decimal, which is the same digit string Rust's
{} and JavaScript's interpolation produce. The one adjustment is the trailing ".0" that Python
writes on a whole number and the other ports do not, so 86.0 reads as 86.

This deliberately does not reproduce Go, which is the one port that switches to an exponent, and
does so from 1e+06 upward: Go prints 1e+06 where Rust, Kotlin, TypeScript and this all print
1000000. Matching the five would mean changing Go rather than joining it.
"""


def format_value(value: float) -> str:
    """Returns the canonical decimal form of a double."""
    rendered = repr(value)
    return rendered.removesuffix(".0")
