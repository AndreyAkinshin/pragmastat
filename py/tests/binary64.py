"""One spelling of "these two binary64 values are identical", shared by every suite.

An exact comparison is a comparison of the raw 64-bit payloads, not of the
numbers. The two differ in both directions, and neither predicate is uniformly
stronger:

* ``-0.0 == +0.0`` is true, so ``==`` passes a pair of results that do not carry
  the same bits. A sign of zero is reachable whenever a result comes out of a
  difference or an average of symmetric values, which is most of what these
  estimators compute.
* ``float("nan") == float("nan")`` is false, so ``==`` fails a pair of results
  that DO carry the same bits.

The claim these suites make is bit-identity across the seven implementations, so
the payload is the thing to compare. The point is not that any particular case
is currently reachable: these predicates exist to catch what nobody predicted.

Every failure message prints both payloads in hex next to the decimal values.
A one-ULP report is only worth having if it is readable, and a sign-of-zero
report is unreadable without the bits.
"""

import struct


def payload(value):
    """Return the raw binary64 payload of ``value`` as a 64-bit unsigned integer."""
    return struct.unpack("<Q", struct.pack("<d", float(value)))[0]


def fmt(value):
    """Render a float next to its payload, so a one-ULP or sign-of-zero gap is unmistakable."""
    return f"{value!r} (0x{payload(value):016X})"


def identical(actual, expected):
    """Whether two binary64 values carry the same bits."""
    return payload(actual) == payload(expected)


def assert_identical(actual, expected, what):
    """Assert two binary64 values carry the same bits."""
    assert identical(actual, expected), f"{what}: expected {fmt(expected)}, got {fmt(actual)}"


def assert_sequence_identical(actual, expected, what):
    """Assert two sequences agree element by element on the raw binary64 payloads."""
    assert len(actual) == len(expected), f"{what}: length mismatch, expected {len(expected)}, got {len(actual)}"
    for i, (act, exp) in enumerate(zip(actual, expected, strict=True)):
        assert_identical(act, exp, f"{what}, index {i}")


def assert_bounds_identical(actual, expected, what):
    """Assert both ends of two :class:`~pragmastat.Bounds` carry the same bits."""
    assert_identical(actual.lower, expected.lower, f"{what} (lower)")
    assert_identical(actual.upper, expected.upper, f"{what} (upper)")
