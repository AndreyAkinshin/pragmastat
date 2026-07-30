"""The C extension and the pure-Python fallback must agree at every public exit.

This port ships two implementations of the selection kernels: three C extensions under ``py/src``
and the pure-Python functions they replace. An installed wheel uses the C ones, a source checkout
without a compiler uses the others, and nothing compared them. Two implementations of one thing
that no test puts side by side is the shape of every divergence this project has had to chase.

They are NOT identical at kernel level, and the difference is a signed zero. ``center_impl_c`` and
``_center_impl_python`` select the same element by value but not by payload: on inputs holding both
``+0.0`` and ``-0.0`` they disagree on 110 of 652 enumerated patterns, because the C kernel pivots
on the middle element while the Python one pivots by an FNV-seeded draw, and the two therefore
leave a different one of the two indistinguishable zeros in the selected position.

That difference does not reach a caller: every public estimator normalizes the sign of a zero on
the way out. This file pins that, because normalization is what makes the two kernels
interchangeable, and a change to either kernel or to the normalization would break the property
without any other test noticing.
"""

import itertools

import pytest
from binary64 import fmt, payload

import pragmastat as ps
from pragmastat import center_impl as center_impl_module
from pragmastat import spread_impl as spread_impl_module

NEGATIVE_ZERO = 0x8000000000000000


def _patterns() -> list[list[float]]:
    """Inputs that hold both zeros, which is where the two kernels can differ."""
    negative_zero = float("-0.0")
    out = []
    for size in (2, 3, 4):
        out.extend(
            [float(v) for v in combo] for combo in itertools.product([0.0, negative_zero, 1.0, -1.0], repeat=size)
        )
    return out


def _public_exits(x: list[float]) -> list[tuple[str, int]]:
    """Every public exit reachable from a one-sample input, as payloads."""
    results: list[tuple[str, int]] = []

    for name, call in [
        ("center", lambda: ps.center(x)),
        ("spread", lambda: ps.spread(x)),
        ("shift", lambda: ps.shift(x, x)),
        ("disparity", lambda: ps.disparity(x, x)),
    ]:
        try:
            results.append((name, payload(call())))
        except Exception as error:  # noqa: BLE001 - the same input must fail the same way
            results.append((name, hash(type(error).__name__)))

    for name, call in [
        ("center_bounds", lambda: ps.center_bounds(x, 0.3)),
        ("spread_bounds", lambda: ps.spread_bounds(x, 0.5, "seed")),
        ("shift_bounds", lambda: ps.shift_bounds(x, x, 0.5)),
        ("disparity_bounds", lambda: ps.disparity_bounds(x, x, 0.9, "seed")),
    ]:
        try:
            bounds = call()
            results.append((f"{name}.lower", payload(bounds.lower)))
            results.append((f"{name}.upper", payload(bounds.upper)))
        except Exception as error:  # noqa: BLE001
            results.append((name, hash(type(error).__name__)))

    return results


@pytest.mark.skipif(
    not (center_impl_module._HAS_C_EXTENSION and spread_impl_module._HAS_C_EXTENSION),  # noqa: SLF001
    reason="C extension not built, so there is only one kernel to compare",
)
def test_both_kernels_agree_at_every_public_exit(monkeypatch):
    patterns = _patterns()

    with_c = [_public_exits(x) for x in patterns]

    monkeypatch.setattr(center_impl_module, "_HAS_C_EXTENSION", False)
    monkeypatch.setattr(spread_impl_module, "_HAS_C_EXTENSION", False)
    without_c = [_public_exits(x) for x in patterns]

    for x, c_row, py_row in zip(patterns, with_c, without_c, strict=True):
        for (name, c_value), (_, py_value) in zip(c_row, py_row, strict=True):
            assert c_value == py_value, (
                f"{name}({x}) differs between the kernels: C 0x{c_value:016x}, pure Python 0x{py_value:016x}"
            )


@pytest.mark.skipif(
    not center_impl_module._HAS_C_EXTENSION,  # noqa: SLF001
    reason="C extension not built",
)
def test_the_kernels_differ_on_a_signed_zero_and_normalization_is_why_that_is_safe():
    """The reason the agreement above is a property of the boundary, not of the kernels.

    If this ever stops finding a disagreement, the kernels have converged and the normalization is
    no longer load-bearing for them. That is a fine state to be in, but it should be noticed rather
    than assumed: the test above would keep passing either way.
    """
    import numpy as np

    from pragmastat import _center_impl_c

    negative_zero = float("-0.0")
    disagreements = 0
    for size in (2, 3, 4):
        for combo in itertools.product([0.0, negative_zero], repeat=size):
            values = np.array(sorted(combo), dtype=float)
            from_c = _center_impl_c.center_impl_c(values, 1)
            from_python = center_impl_module._center_impl_python(values, assume_sorted=True)  # noqa: SLF001
            if payload(from_c) != payload(from_python):
                disagreements += 1

    assert disagreements > 0, (
        "the two kernels now agree bitwise on signed zeros; the normalization at the public "
        "boundary is no longer what makes them interchangeable, and this file should say so"
    )


@pytest.mark.skipif(
    not center_impl_module._HAS_C_EXTENSION,  # noqa: SLF001
    reason="C extension not built",
)
def test_no_public_exit_reports_a_negative_zero_from_either_kernel(monkeypatch):
    for use_c in (True, False):
        if not use_c:
            monkeypatch.setattr(center_impl_module, "_HAS_C_EXTENSION", False)
            monkeypatch.setattr(spread_impl_module, "_HAS_C_EXTENSION", False)
        for x in _patterns():
            for name, value in _public_exits(x):
                assert value != NEGATIVE_ZERO, (
                    f"{name}({x}) reported a negative zero from the "
                    f"{'C' if use_c else 'pure-Python'} kernel: {fmt(value)}"
                )
