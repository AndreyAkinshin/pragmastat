"""LinearDistModel and NeighborDistModel against their shared dist-linear/dist-neighbor fixtures.

Unlike the five parametric families, this suite's oracle is not one of the seven ports: it is
an independent binary64 + exact-Fraction reimplementation (tests/oracles/dist_reference.py),
written directly from the normative algorithm so a bug shared by every language cannot pass
itself. Every case is compared by binary64 payload, matching the suite's declared exact class.
"""

import json
import math
from pathlib import Path

import pytest

from pragmastat import LinearDistModel, NeighborDistModel
from pragmastat.assumptions import AssumptionError
from pragmastat.rng import Rng
from tests.binary64 import assert_sequence_identical

MODELS = {"linear": LinearDistModel, "neighbor": NeighborDistModel}

QUERY_OPS = ["quantile", "cumulative", "density"]


def _tests_root():
    root = Path(__file__).resolve()
    while not (root / "CITATION.cff").exists():
        assert root != root.parent, "repository root not found"
        root = root.parent
    return root / "tests"


def _parse_special(value):
    """A fixture element is a number, or one of the three strings JSON has no literal for."""
    if value == "NaN":
        return math.nan
    if value == "Infinity":
        return math.inf
    if value == "-Infinity":
        return -math.inf
    return float(value)


def _parse_special_seq(values):
    return [_parse_special(v) for v in values]


def _build(model, x, assume_sorted):
    """Build the model the fixture directory names, from a raw vector only.

    Never the Sample-wrapped measured path: assume_sorted alongside a Sample now raises, and
    this loader only ever passes plain vectors.
    """
    return MODELS[model](x, assume_sorted=assume_sorted)


def _run_query_op(model_obj, op, y):
    if op == "quantile":
        return model_obj.quantile(y)
    if op == "cumulative":
        return model_obj.cumulative(y)
    if op == "density":
        return model_obj.density(y)
    raise AssertionError(f"Unknown dist operation {op!r}")


def _query_cases():
    for model in MODELS:
        for op in QUERY_OPS:
            directory = _tests_root() / f"dist-{model}" / op
            files = sorted(directory.glob("*.json"))
            assert files, f"No JSON test files found in {directory}"
            for path in files:
                yield model, op, path


@pytest.mark.parametrize(("model", "op", "path"), list(_query_cases()), ids=lambda v: getattr(v, "stem", v))
def test_dist_query_reference(model, op, path):
    """Quantile/cumulative/density cases: an expected_error is checked before the operation runs.

    Presence of "expected_error" in the fixture is decided first; only then is the build/query
    attempted and asserted to fail with the matching id/subject. Trying the operation and only
    afterwards checking for "expected_error" would let a real, unexpected bug through as a pass.
    """
    case = json.loads(path.read_text())
    x = _parse_special_seq(case["input"]["x"])
    assume_sorted = case["input"].get("assume_sorted", False)

    if "expected_error" in case:
        expected = case["expected_error"]
        with pytest.raises(AssumptionError) as exc_info:  # noqa: PT012
            model_obj = _build(model, x, assume_sorted)
            _run_query_op(model_obj, op, _parse_special_seq(case["input"]["y"]))
        violation = exc_info.value.violation
        assert violation is not None, f"{path.name}: expected a violation, got a message-only error"
        assert violation.id.value == expected["id"], (
            f"{path.name}: expected error id {expected['id']!r}, got {violation.id.value!r}"
        )
        assert violation.subject == expected["subject"], (
            f"{path.name}: expected error subject {expected['subject']!r}, got {violation.subject!r}"
        )
        return

    y = _parse_special_seq(case["input"]["y"])
    model_obj = _build(model, x, assume_sorted)
    actual = _run_query_op(model_obj, op, y)
    expected = _parse_special_seq(case["output"])
    assert_sequence_identical(actual, expected, f"{model}/{op}/{path.name}")


def _sample_cases():
    for model in MODELS:
        directory = _tests_root() / f"dist-{model}" / "sample"
        files = sorted(directory.glob("*.json"))
        assert files, f"No JSON test files found in {directory}"
        for path in files:
            yield model, path


@pytest.mark.parametrize(("model", "path"), list(_sample_cases()), ids=lambda v: getattr(v, "stem", v))
def test_dist_sample_reference(model, path):
    """No sample case declares an error: an invalid x is covered by the query suites above, and
    drawing from an already-built model cannot fail on its own.
    """
    case = json.loads(path.read_text())
    x = _parse_special_seq(case["input"]["x"])
    model_obj = _build(model, x, assume_sorted=False)
    rng = Rng(case["input"]["seed"])
    actual = model_obj.samples(rng, case["input"]["count"])
    expected = _parse_special_seq(case["output"])
    assert_sequence_identical(actual, expected, f"{model}/sample/{path.name}")


def _landscape_cases():
    directory = _tests_root() / "dist-neighbor" / "landscape"
    files = sorted(directory.glob("*.json"))
    assert files, f"No JSON test files found in {directory}"
    return files


@pytest.mark.parametrize("path", _landscape_cases(), ids=lambda v: getattr(v, "stem", v))
def test_dist_neighbor_landscape_reference(path):
    """Every landscape case builds successfully by construction: the generator only emits one for
    a sample that already passed the query suites' validity checks.
    """
    case = json.loads(path.read_text())
    x = _parse_special_seq(case["input"]["x"])
    model_obj = NeighborDistModel(x, assume_sorted=False)
    landscape = model_obj.landscape()
    expected = case["output"]

    assert landscape.scale_index == expected["k0"], f"{path.name}: scale_index mismatch"
    assert landscape.radius == expected["radius"], f"{path.name}: radius mismatch"
    assert_sequence_identical(
        landscape.boundaries, _parse_special_seq(expected["boundaries"]), f"{path.name} boundaries"
    )
    assert_sequence_identical(
        landscape.cumulative, _parse_special_seq(expected["cumulative"]), f"{path.name} cumulative"
    )
    assert_sequence_identical(landscape.masses, _parse_special_seq(expected["masses"]), f"{path.name} masses")
    assert_sequence_identical(landscape.densities, _parse_special_seq(expected["densities"]), f"{path.name} densities")
    assert list(landscape.sources) == expected["sources"], f"{path.name}: sources mismatch"
