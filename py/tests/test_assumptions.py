"""Assumption violation conformance tests.

These tests verify that assumption violations are reported correctly and
consistently across all languages. The test data is loaded from shared
JSON files in tests/assumptions/.
"""

import json
from pathlib import Path

import numpy as np
import pytest

from pragmastat import (
    avg_spread,
    center,
    center_bounds,
    disparity,
    median_bounds,
    ratio,
    rel_spread,
    shift,
    signed_rank_margin,
    spread,
)
from pragmastat.assumptions import AssumptionError


def find_repo_root() -> Path:
    """Finds the repository root by looking for CITATION.cff."""
    current = Path(__file__).resolve()
    while current != current.parent:
        if (current / "CITATION.cff").exists():
            return current
        current = current.parent
    raise RuntimeError("Could not find repository root (CITATION.cff not found)")


def parse_value(v) -> float:
    """Parses a JSON value into a float, handling special values."""
    if isinstance(v, (int, float)):
        return float(v)
    if isinstance(v, str):
        if v == "NaN":
            return float("nan")
        if v == "Infinity":
            return float("inf")
        if v == "-Infinity":
            return float("-inf")
        raise ValueError(f"Unknown string value: {v}")
    raise TypeError(f"Unexpected value type: {type(v)}")


def parse_array(arr: list | None) -> np.ndarray:
    """Parses a JSON array into a numpy array."""
    if arr is None:
        return np.array([])
    return np.array([parse_value(v) for v in arr])


def call_function(func_name: str, inputs: dict) -> None:
    """Dispatches to the appropriate estimator function."""
    x = parse_array(inputs.get("x"))
    y = parse_array(inputs.get("y"))
    misrate_raw = inputs.get("misrate")
    misrate = parse_value(misrate_raw) if misrate_raw is not None else None
    n = inputs.get("n")
    seed = inputs.get("seed")

    dispatch = {
        "Center": lambda: center(x),
        "Ratio": lambda: ratio(x, y),
        "RelSpread": lambda: rel_spread(x),
        "Spread": lambda: spread(x),
        "Shift": lambda: shift(x, y),
        "AvgSpread": lambda: avg_spread(x, y),
        "Disparity": lambda: disparity(x, y),
        "MedianBounds": lambda: median_bounds(x, misrate),
        "CenterBounds": lambda: center_bounds(x, misrate),
        "SignedRankMargin": lambda: signed_rank_margin(n, misrate),
    }
    if func_name not in dispatch:
        raise ValueError(f"Unknown function: {func_name}")
    dispatch[func_name]()


def load_assumption_test_cases():
    """Loads all assumption test cases from the shared test data."""
    repo_root = find_repo_root()
    assumptions_dir = repo_root / "tests" / "assumptions"

    # Load manifest
    manifest_path = assumptions_dir / "manifest.json"
    with open(manifest_path) as f:
        manifest = json.load(f)

    test_cases = []
    for suite_entry in manifest["suites"]:
        suite_path = assumptions_dir / suite_entry["file"]
        with open(suite_path) as f:
            suite = json.load(f)

        for case in suite["cases"]:
            test_cases.append(
                pytest.param(
                    case["function"],
                    case["inputs"],
                    case["expected_violation"]["id"],
                    id=f"{suite['suite']}/{case['name']}",
                )
            )

    return test_cases


@pytest.mark.parametrize(
    "func_name,inputs,expected_id",
    load_assumption_test_cases(),
)
def test_assumption_violation(func_name, inputs, expected_id):
    """Tests that the correct assumption violation is raised."""
    with pytest.raises(AssumptionError) as exc_info:
        call_function(func_name, inputs)

    err = exc_info.value
    assert err.violation.id.value == expected_id, (
        f"Expected id={expected_id}, got {err.violation.id.value}"
    )
