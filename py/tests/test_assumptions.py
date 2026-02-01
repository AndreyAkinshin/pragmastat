"""Assumption violation conformance tests.

These tests verify that assumption violations are reported correctly and
consistently across all languages. The test data is loaded from shared
JSON files in tests/assumptions/.
"""

import json
from pathlib import Path

import numpy as np
import pytest

from pragmastat import avg_spread, center, disparity, ratio, rel_spread, shift, spread
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


FUNCTION_MAP = {
    "Center": lambda x, _y: center(x),
    "Ratio": lambda x, y: ratio(x, y),
    "RelSpread": lambda x, _y: rel_spread(x),
    "Spread": lambda x, _y: spread(x),
    "Shift": lambda x, y: shift(x, y),
    "AvgSpread": lambda x, y: avg_spread(x, y),
    "Disparity": lambda x, y: disparity(x, y),
}


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
                    case["expected_violation"]["subject"],
                    id=f"{suite['suite']}/{case['name']}",
                )
            )

    return test_cases


@pytest.mark.parametrize(
    "func_name,inputs,expected_id,expected_subject",
    load_assumption_test_cases(),
)
def test_assumption_violation(func_name, inputs, expected_id, expected_subject):
    """Tests that the correct assumption violation is raised."""
    x = parse_array(inputs.get("x"))
    y = parse_array(inputs.get("y"))

    func = FUNCTION_MAP.get(func_name)
    if func is None:
        pytest.fail(f"Unknown function: {func_name}")

    with pytest.raises(AssumptionError) as exc_info:
        func(x, y)

    err = exc_info.value
    assert err.violation.id.value == expected_id, (
        f"Expected id={expected_id}, got {err.violation.id.value}"
    )
    assert err.violation.subject == expected_subject, (
        f"Expected subject={expected_subject}, got {err.violation.subject}"
    )
