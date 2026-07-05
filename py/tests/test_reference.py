import json
from pathlib import Path

import pytest

from pragmastat import (
    DISPARITY_UNIT,
    NUMBER_UNIT,
    RATIO_UNIT,
    Additive,
    Exp,
    Measurement,
    MeasurementUnit,
    Metric,
    Multiplic,
    Power,
    Rng,
    Sample,
    Threshold,
    Uniform,
    center,
    center_bounds,
    compare1,
    compare2,
    disparity,
    disparity_bounds,
    ratio,
    ratio_bounds,
    shift,
    shift_bounds,
    spread,
    spread_bounds,
)
from pragmastat.assumptions import AssumptionError
from pragmastat.estimators import (
    _avg_spread as avg_spread,
)
from pragmastat.estimators import (
    _avg_spread_bounds as avg_spread_bounds,
)
from pragmastat.pairwise_margin import pairwise_margin
from pragmastat.signed_rank_margin import signed_rank_margin


def _assert_violation(err: AssumptionError, expected: dict, context: str = "", skip_subject: bool = False) -> None:
    """Assert that an AssumptionError has the expected violation fields.

    ``skip_subject`` is used on the Sample entry point for two-sample estimators:
    Sample construction validates the y argument under subject "x" (construction
    can't know it's arg2), so for such fixtures we compare only the error id there.
    The raw path still asserts subject fully.

    This is analogous to the Rust ``is_sample_creation && expected.subject == "y"``
    predicate, but Python additionally gates on ``id == "validity"`` (see
    :func:`_is_sample_construction_y_error`): only sample-CONSTRUCTION validity
    errors mislabel the y argument's subject. Post-construction errors
    (positivity/sparity) are raised by the ESTIMATOR, which reports "y"
    positionally, so their subject IS asserted even on the Sample path.
    """
    assert err.violation is not None, f"Expected violation, got message-only error: {err}{context}"
    assert err.violation.id.value == expected["id"], (
        f"Expected error id '{expected['id']}', got '{err.violation.id.value}'{context}"
    )
    if "subject" in expected and not skip_subject:
        assert err.violation.subject == expected["subject"], (
            f"Expected error subject '{expected['subject']}', got '{err.violation.subject}'{context}"
        )


def _is_sample_construction_y_error(test_case: dict) -> bool:
    """Whether the Sample path can't report the expected subject for this fixture.

    The subject is skipped on the Sample entry point ONLY for sample-CONSTRUCTION
    validity errors that expect subject "y": construction validates the y argument
    under subject "x" (it can't know it's arg2), so the "y" subject is unreachable
    there. This mirrors C#/Kotlin/R/TS, which gate on the validity id specifically.

    Positivity/sparity errors on the y argument are raised by the ESTIMATOR (not
    construction), which reports "y" positionally, so their subject IS asserted.
    """
    if "expected_error" not in test_case:
        return False
    expected = test_case["expected_error"]
    return expected.get("subject") == "y" and expected.get("id") == "validity"


def find_repo_root():
    """Find the repository root by looking for CITATION.cff file."""
    current_dir = Path(__file__).parent
    while current_dir != current_dir.parent:
        if (current_dir / "CITATION.cff").exists():
            return current_dir
        current_dir = current_dir.parent
    raise RuntimeError("Could not find repository root (CITATION.cff not found)")


def _load_fixtures(estimator_name):
    """Load all JSON fixtures for an estimator/bounds directory (shared loader)."""
    repo_root = find_repo_root()
    test_data_dir = repo_root / "tests" / estimator_name

    json_files = list(test_data_dir.glob("*.json"))
    assert len(json_files) > 0, f"No JSON test files found in {test_data_dir}"

    fixtures = []
    for json_file in json_files:
        with open(json_file, "r") as f:
            fixtures.append((json_file.name, json.load(f)))
    return fixtures


# --- Entry-point builders -----------------------------------------------------
#
# Each estimator is exercised through BOTH the Sample API and the raw
# native-array API. An "entry" is a (label, callable) pair; the callable takes a
# test case and returns the comparable result (a number, or a (lower, upper)
# tuple for bounds). This catches Sample-adapter bugs that a raw-only harness
# would miss.


def _normalize_point(result):
    """Reduce a point-estimator result to a plain float (Measurement or float)."""
    return result.value if isinstance(result, Measurement) else result


def _point_entries(estimator_func, is_two_sample):
    """Build [(label, call_fn), ...] for a one/two-sample point estimator."""

    def sample_call(test_case):
        sx = Sample(test_case["input"]["x"])
        if is_two_sample:
            sy = Sample(test_case["input"]["y"])
            return _normalize_point(estimator_func(sx, sy))
        return _normalize_point(estimator_func(sx))

    def raw_call(test_case):
        x = test_case["input"]["x"]
        if is_two_sample:
            y = test_case["input"]["y"]
            return _normalize_point(estimator_func(x, y))
        return _normalize_point(estimator_func(x))

    return [("raw", raw_call), ("sample", sample_call)]


def _sample_only_point_entries(estimator_func, is_two_sample):
    """Internal estimators (avg_spread) have no raw entry; run Sample-only."""
    return _point_entries(estimator_func, is_two_sample)[1:]


def run_reference_tests(estimator_name, entries):
    """Run point-estimator reference tests through every provided entry point."""
    for fixture_name, test_case in _load_fixtures(estimator_name):
        is_y_validity_error = _is_sample_construction_y_error(test_case)
        for label, call_fn in entries:
            context = f" for {fixture_name} [{label}]"
            if "expected_error" in test_case:
                expected_error = test_case["expected_error"]
                skip_subject = label == "sample" and is_y_validity_error
                with pytest.raises(AssumptionError) as exc_info:
                    call_fn(test_case)
                _assert_violation(exc_info.value, expected_error, context, skip_subject=skip_subject)
                continue

            expected_output = test_case["output"]
            actual_output = call_fn(test_case)
            assert abs(actual_output - expected_output) < 1e-9, (
                f"Failed{context}: expected: {expected_output}, got: {actual_output}"
            )


def _bounds_entries(bounds_func, is_two_sample, **call_kwargs):
    """Build [(label, call_fn), ...] for a one/two-sample bounds estimator.

    The call_fn returns a (lower, upper) tuple from the resulting Bounds.
    """

    def sample_call(test_case):
        misrate = test_case["input"]["misrate"]
        sx = Sample(test_case["input"]["x"])
        if is_two_sample:
            sy = Sample(test_case["input"]["y"])
            result = bounds_func(sx, sy, misrate, **call_kwargs)
        else:
            result = bounds_func(sx, misrate, **call_kwargs)
        return result.lower, result.upper

    def raw_call(test_case):
        misrate = test_case["input"]["misrate"]
        x = test_case["input"]["x"]
        if is_two_sample:
            y = test_case["input"]["y"]
            result = bounds_func(x, y, misrate, **call_kwargs)
        else:
            result = bounds_func(x, misrate, **call_kwargs)
        return result.lower, result.upper

    return [("raw", raw_call), ("sample", sample_call)]


def _sample_only_bounds_entries(bounds_func, is_two_sample, **call_kwargs):
    """Internal bounds (avg_spread_bounds) have no raw entry; run Sample-only."""
    return _bounds_entries(bounds_func, is_two_sample, **call_kwargs)[1:]


def run_bounds_reference_tests(estimator_name, entries_builder):
    """Run bounds reference tests through every provided entry point.

    ``entries_builder(seed)`` returns the list of entry points; seed is read
    per-fixture (some bounds estimators accept a deterministic seed).
    """
    for fixture_name, test_case in _load_fixtures(estimator_name):
        seed = test_case["input"].get("seed")
        entries = entries_builder(seed)
        is_y_validity_error = _is_sample_construction_y_error(test_case)
        for label, call_fn in entries:
            context = f" for {fixture_name} [{label}]"
            if "expected_error" in test_case:
                expected_error = test_case["expected_error"]
                skip_subject = label == "sample" and is_y_validity_error
                with pytest.raises(AssumptionError) as exc_info:
                    call_fn(test_case)
                _assert_violation(exc_info.value, expected_error, context, skip_subject=skip_subject)
                continue

            expected_lower = test_case["output"]["lower"]
            expected_upper = test_case["output"]["upper"]
            actual_lower, actual_upper = call_fn(test_case)
            assert abs(actual_lower - expected_lower) < 1e-9, (
                f"Failed lower bound{context}: expected: {expected_lower}, got: {actual_lower}"
            )
            assert abs(actual_upper - expected_upper) < 1e-9, (
                f"Failed upper bound{context}: expected: {expected_upper}, got: {actual_upper}"
            )


def _parse_sample_values(raw_values):
    """Convert JSON values (which may contain 'NaN', 'Infinity', '-Infinity') to floats."""
    result = []
    for v in raw_values:
        if isinstance(v, str):
            if v == "NaN":
                result.append(float("nan"))
            elif v == "Infinity":
                result.append(float("inf"))
            elif v == "-Infinity":
                result.append(float("-inf"))
            else:
                result.append(float(v))
        else:
            result.append(float(v))
    return result


def run_distribution_tests(dist_name, dist_factory):
    """Run distribution reference tests against JSON data files."""
    repo_root = find_repo_root()
    test_data_dir = repo_root / "tests" / "distributions" / dist_name

    json_files = list(test_data_dir.glob("*.json"))
    assert len(json_files) > 0, f"No JSON test files found in {test_data_dir}"

    for json_file in json_files:
        with open(json_file, "r") as f:
            test_case = json.load(f)

        input_data = test_case["input"]
        expected = test_case["output"]
        rng = Rng(input_data["seed"])
        dist = dist_factory(input_data)
        actual = [dist.sample(rng) for _ in range(input_data["count"])]

        assert len(actual) == len(expected), f"Length mismatch for {json_file.name}: {len(actual)} vs {len(expected)}"
        for i, (act, exp) in enumerate(zip(actual, expected)):
            assert abs(act - exp) < 1e-12, f"Failed for {json_file.name}, index {i}: expected {exp}, got {act}"


class TestReference:
    def test_center_reference(self):
        run_reference_tests("center", _point_entries(center, is_two_sample=False))

    def test_spread_reference(self):
        run_reference_tests("spread", _point_entries(spread, is_two_sample=False))

    def test_shift_reference(self):
        run_reference_tests("shift", _point_entries(shift, is_two_sample=True))

    def test_ratio_reference(self):
        run_reference_tests("ratio", _point_entries(ratio, is_two_sample=True))

    def test_avg_spread_reference(self):
        # avg_spread is an internal estimator with no public raw entry: Sample-only.
        run_reference_tests("avg-spread", _sample_only_point_entries(avg_spread, is_two_sample=True))

    def test_disparity_reference(self):
        run_reference_tests("disparity", _point_entries(disparity, is_two_sample=True))

    def test_pairwise_margin_reference(self):
        """Test pairwise_margin against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "pairwise-margin"

        json_files = list(test_data_dir.glob("*.json"))
        assert len(json_files) > 0, f"No JSON test files found in {test_data_dir}"

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            n = test_case["input"]["n"]
            m = test_case["input"]["m"]
            misrate = test_case["input"]["misrate"]

            # Handle error test cases
            if "expected_error" in test_case:
                expected_error = test_case["expected_error"]
                with pytest.raises(AssumptionError) as exc_info:
                    pairwise_margin(n, m, misrate)
                _assert_violation(exc_info.value, expected_error, f" for {json_file.name}")
                continue

            expected_output = test_case["output"]
            actual_output = pairwise_margin(n, m, misrate)

            assert actual_output == expected_output, (
                f"Failed for test file: {json_file.name}, expected: {expected_output}, got: {actual_output}"
            )

    def test_shift_bounds_reference(self):
        run_bounds_reference_tests(
            "shift-bounds",
            lambda _seed: _bounds_entries(shift_bounds, is_two_sample=True),
        )

    def test_ratio_bounds_reference(self):
        run_bounds_reference_tests(
            "ratio-bounds",
            lambda _seed: _bounds_entries(ratio_bounds, is_two_sample=True),
        )

    def test_rng_uniform_reference(self):
        """Test Rng uniform_float() against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "rng"

        json_files = [f for f in test_data_dir.glob("*.json") if f.name.startswith("uniform-seed-")]
        assert len(json_files) > 0, f"No uniform seed test files found in {test_data_dir}"

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            seed = test_case["input"]["seed"]
            count = test_case["input"]["count"]
            expected = test_case["output"]

            rng = Rng(seed)
            actual = [rng.uniform_float() for _ in range(count)]

            assert len(actual) == len(expected), (
                f"Length mismatch for {json_file.name}: {len(actual)} vs {len(expected)}"
            )
            for i, (act, exp) in enumerate(zip(actual, expected)):
                assert abs(act - exp) < 1e-15, f"Failed for {json_file.name}, index {i}: expected {exp}, got {act}"

    def test_rng_uniform_int_reference(self):
        """Test Rng uniform_int() against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "rng"

        json_files = [f for f in test_data_dir.glob("*.json") if f.name.startswith("uniform-int-")]
        assert len(json_files) > 0, f"No uniform int test files found in {test_data_dir}"

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            seed = test_case["input"]["seed"]
            min_val = test_case["input"]["min"]
            max_val = test_case["input"]["max"]
            count = test_case["input"]["count"]
            expected = test_case["output"]

            rng = Rng(seed)
            actual = [rng.uniform_int(min_val, max_val) for _ in range(count)]

            assert actual == expected, f"Failed for {json_file.name}: expected {expected}, got {actual}"

    def test_rng_string_seed_reference(self):
        """Test Rng with string seeds against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "rng"

        json_files = [f for f in test_data_dir.glob("*.json") if f.name.startswith("uniform-string-")]
        assert len(json_files) > 0, f"No string seed test files found in {test_data_dir}"

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            seed = test_case["input"]["seed"]
            count = test_case["input"]["count"]
            expected = test_case["output"]

            rng = Rng(seed)
            actual = [rng.uniform_float() for _ in range(count)]

            assert len(actual) == len(expected), (
                f"Length mismatch for {json_file.name}: {len(actual)} vs {len(expected)}"
            )
            for i, (act, exp) in enumerate(zip(actual, expected)):
                assert abs(act - exp) < 1e-15, f"Failed for {json_file.name}, index {i}: expected {exp}, got {act}"

    def test_rng_uniform_float_range_reference(self):
        """Test Rng uniform_float_range() against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "rng"

        json_files = [f for f in test_data_dir.glob("*.json") if f.name.startswith("uniform-range-")]
        assert len(json_files) > 0, f"No uniform range test files found in {test_data_dir}"

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            seed = test_case["input"]["seed"]
            min_val = test_case["input"]["min"]
            max_val = test_case["input"]["max"]
            count = test_case["input"]["count"]
            expected = test_case["output"]

            rng = Rng(seed)
            actual = [rng.uniform_float_range(min_val, max_val) for _ in range(count)]

            assert len(actual) == len(expected), (
                f"Length mismatch for {json_file.name}: {len(actual)} vs {len(expected)}"
            )
            for i, (act, exp) in enumerate(zip(actual, expected)):
                assert abs(act - exp) < 1e-12, f"Failed for {json_file.name}, index {i}: expected {exp}, got {act}"

    def test_rng_uniform_bool_reference(self):
        """Test Rng uniform_bool() against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "rng"

        json_files = [f for f in test_data_dir.glob("*.json") if f.name.startswith("uniform-bool-seed-")]
        assert len(json_files) > 0, f"No uniform bool test files found in {test_data_dir}"

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            seed = test_case["input"]["seed"]
            count = test_case["input"]["count"]
            expected = test_case["output"]

            rng = Rng(seed)
            actual = [rng.uniform_bool() for _ in range(count)]

            assert actual == expected, f"Failed for {json_file.name}: expected {expected}, got {actual}"

    def test_shuffle_reference(self):
        """Test Rng shuffle() against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "shuffle"

        json_files = list(test_data_dir.glob("*.json"))
        assert len(json_files) > 0, f"No shuffle test files found in {test_data_dir}"

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            seed = test_case["input"]["seed"]
            x = test_case["input"]["x"]
            expected = test_case["output"]

            rng = Rng(seed)
            actual = rng.shuffle(x)

            assert len(actual) == len(expected), (
                f"Length mismatch for {json_file.name}: {len(actual)} vs {len(expected)}"
            )
            for i, (act, exp) in enumerate(zip(actual, expected)):
                assert abs(act - exp) < 1e-15, f"Failed for {json_file.name}, index {i}: expected {exp}, got {act}"

    def test_sample_reference(self):
        """Test Rng sample() against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "sample"

        json_files = list(test_data_dir.glob("*.json"))
        assert len(json_files) > 0, f"No sample test files found in {test_data_dir}"

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            seed = test_case["input"]["seed"]
            x = test_case["input"]["x"]
            k = test_case["input"]["k"]
            expected = test_case["output"]

            rng = Rng(seed)
            actual = rng.sample(x, k)

            assert len(actual) == len(expected), (
                f"Length mismatch for {json_file.name}: {len(actual)} vs {len(expected)}"
            )
            for i, (act, exp) in enumerate(zip(actual, expected)):
                assert abs(act - exp) < 1e-15, f"Failed for {json_file.name}, index {i}: expected {exp}, got {act}"

    def test_resample_reference(self):
        """Test Rng resample() against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "resample"

        json_files = list(test_data_dir.glob("*.json"))
        assert len(json_files) > 0, f"No resample test files found in {test_data_dir}"

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            seed = test_case["input"]["seed"]
            x = test_case["input"]["x"]
            k = test_case["input"]["k"]
            expected = test_case["output"]

            rng = Rng(seed)
            actual = rng.resample(x, k)

            assert len(actual) == len(expected), (
                f"Length mismatch for {json_file.name}: {len(actual)} vs {len(expected)}"
            )
            for i, (act, exp) in enumerate(zip(actual, expected)):
                assert abs(act - exp) < 1e-15, f"Failed for {json_file.name}, index {i}: expected {exp}, got {act}"

    def test_uniform_distribution_reference(self):
        run_distribution_tests(
            "uniform",
            lambda input_data: Uniform(input_data["min"], input_data["max"]),
        )

    def test_additive_distribution_reference(self):
        run_distribution_tests(
            "additive",
            lambda input_data: Additive(input_data["mean"], input_data["stdDev"]),
        )

    def test_multiplic_distribution_reference(self):
        run_distribution_tests(
            "multiplic",
            lambda input_data: Multiplic(input_data["logMean"], input_data["logStdDev"]),
        )

    def test_exp_distribution_reference(self):
        run_distribution_tests(
            "exp",
            lambda input_data: Exp(input_data["rate"]),
        )

    def test_power_distribution_reference(self):
        run_distribution_tests(
            "power",
            lambda input_data: Power(input_data["min"], input_data["shape"]),
        )

    def test_sample_negative_k_raises(self):
        """Test that sample with negative k raises ValueError."""
        rng = Rng("test-sample-validation")
        with pytest.raises(ValueError):
            rng.sample([1, 2, 3], -1)

    def test_signed_rank_margin_reference(self):
        """Test signed_rank_margin against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "signed-rank-margin"

        json_files = list(test_data_dir.glob("*.json"))
        assert len(json_files) > 0, f"No JSON test files found in {test_data_dir}"

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            n = test_case["input"]["n"]
            misrate = test_case["input"]["misrate"]

            # Handle error test cases
            if "expected_error" in test_case:
                expected_error = test_case["expected_error"]
                with pytest.raises(AssumptionError) as exc_info:
                    signed_rank_margin(n, misrate)
                _assert_violation(exc_info.value, expected_error, f" for {json_file.name}")
                continue

            expected_output = test_case["output"]

            actual_output = signed_rank_margin(n, misrate)

            assert actual_output == expected_output, (
                f"Failed for test file: {json_file.name}, expected: {expected_output}, got: {actual_output}"
            )

    def test_center_bounds_reference(self):
        run_bounds_reference_tests(
            "center-bounds",
            lambda _seed: _bounds_entries(center_bounds, is_two_sample=False),
        )

    def test_spread_bounds_reference(self):
        run_bounds_reference_tests(
            "spread-bounds",
            lambda seed: _bounds_entries(spread_bounds, is_two_sample=False, seed=seed),
        )

    def test_avg_spread_bounds_reference(self):
        # avg_spread_bounds is internal (no public raw entry): Sample-only.
        repo_root = find_repo_root()
        if not (repo_root / "tests" / "avg-spread-bounds").exists():
            pytest.skip("avg-spread-bounds test data directory not found")
        run_bounds_reference_tests(
            "avg-spread-bounds",
            lambda seed: _sample_only_bounds_entries(avg_spread_bounds, is_two_sample=True, seed=seed),
        )

    def test_disparity_bounds_reference(self):
        repo_root = find_repo_root()
        if not (repo_root / "tests" / "disparity-bounds").exists():
            pytest.skip("disparity-bounds test data directory not found")
        run_bounds_reference_tests(
            "disparity-bounds",
            lambda seed: _bounds_entries(disparity_bounds, is_two_sample=True, seed=seed),
        )


class TestSampleConstruction:
    """Tests from tests/sample-construction/ cross-language test data."""

    def test_valid_single(self):
        s = Sample([42.0])
        assert s.size == 1
        assert s.is_weighted is False

    def test_valid_multiple(self):
        s = Sample([1.0, 2.0, 3.0, 4.0, 5.0])
        assert s.size == 5
        assert s.is_weighted is False

    def test_valid_weighted(self):
        s = Sample([1.0, 2.0, 3.0], weights=[0.5, 0.3, 0.2])
        assert s.size == 3
        assert s.is_weighted is True

    def test_error_empty(self):
        with pytest.raises(AssumptionError) as exc_info:
            Sample([])
        assert exc_info.value.violation is not None
        assert exc_info.value.violation.id.value == "validity"
        assert exc_info.value.violation.subject == "x"

    def test_error_nan(self):
        with pytest.raises(AssumptionError) as exc_info:
            Sample([1.0, float("nan"), 3.0])
        assert exc_info.value.violation is not None
        assert exc_info.value.violation.id.value == "validity"
        assert exc_info.value.violation.subject == "x"

    def test_error_inf(self):
        with pytest.raises(AssumptionError) as exc_info:
            Sample([1.0, float("inf"), 3.0])
        assert exc_info.value.violation is not None
        assert exc_info.value.violation.id.value == "validity"
        assert exc_info.value.violation.subject == "x"

    def test_error_neg_inf(self):
        with pytest.raises(AssumptionError) as exc_info:
            Sample([1.0, float("-inf"), 3.0])
        assert exc_info.value.violation is not None
        assert exc_info.value.violation.id.value == "validity"
        assert exc_info.value.violation.subject == "x"


class TestUnitPropagation:
    """Tests from tests/unit-propagation/ cross-language test data."""

    def test_center_preserves_unit(self):
        s = Sample([1, 2, 3, 4, 5], unit=NUMBER_UNIT)
        result = center(s)
        assert isinstance(result, Measurement)
        assert abs(result.value - 3) < 1e-9
        assert result.unit == NUMBER_UNIT

    def test_spread_preserves_unit(self):
        s = Sample([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], unit=NUMBER_UNIT)
        result = spread(s)
        assert isinstance(result, Measurement)
        assert result.unit == NUMBER_UNIT

    def test_shift_preserves_unit(self):
        sx = Sample([1, 2, 3, 4, 5], unit=NUMBER_UNIT)
        sy = Sample([6, 7, 8, 9, 10], unit=NUMBER_UNIT)
        result = shift(sx, sy)
        assert isinstance(result, Measurement)
        assert result.unit == NUMBER_UNIT

    def test_ratio_returns_ratio_unit(self):
        sx = Sample([1, 2, 3, 4, 5], unit=NUMBER_UNIT)
        sy = Sample([6, 7, 8, 9, 10], unit=NUMBER_UNIT)
        result = ratio(sx, sy)
        assert isinstance(result, Measurement)
        assert result.unit == RATIO_UNIT

    def test_disparity_returns_disparity_unit(self):
        sx = Sample([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], unit=NUMBER_UNIT)
        sy = Sample([11, 12, 13, 14, 15, 16, 17, 18, 19, 20], unit=NUMBER_UNIT)
        result = disparity(sx, sy)
        assert isinstance(result, Measurement)
        assert result.unit == DISPARITY_UNIT

    def test_weighted_rejected(self):
        s = Sample([1, 2, 3], weights=[0.5, 0.3, 0.2])
        with pytest.raises(AssumptionError, match="weighted samples are not supported"):
            center(s)


class TestBoundsUnitReattachment:
    """Bounds estimators re-attach the correct unit on the Sample path and stay
    unitless (NUMBER_UNIT) on the raw native-array path.

    Sample-path units propagate as:
      - center_bounds / spread_bounds -> x.unit
      - shift_bounds                  -> finer(x, y)
      - ratio_bounds                  -> RATIO_UNIT
      - disparity_bounds              -> DISPARITY_UNIT

    A custom non-default unit (seconds/milliseconds) is used throughout so these
    assertions fail if an implementation hardcodes NUMBER_UNIT instead of
    propagating; the shift case pairs two different units to pin the actual
    finer(x, y) selection.
    """

    SEC = MeasurementUnit("s", "Time", "s", "Second", 1_000_000_000)
    MS = MeasurementUnit("ms", "Time", "ms", "Millisecond", 1_000_000)

    MISRATE = 0.3
    SEED = "bounds-unit"
    # Strictly positive so ratio is defined; 8 elements is large enough for the
    # 0.3 misrate used by every bounds estimator here (incl. disparity, which
    # splits its budget across shift and avg-spread sub-bounds).
    X = [5.0, 1.0, 8.0, 3.0, 2.0, 7.0, 4.0, 6.0]
    Y = [12.0, 9.0, 15.0, 10.0, 13.0, 11.0, 16.0, 14.0]

    # --- Sample path: ratio/disparity re-attach their dedicated units ---

    def test_ratio_bounds_sample_unit_is_ratio(self):
        sx = Sample(self.X, unit=self.SEC)
        sy = Sample(self.Y, unit=self.SEC)
        assert ratio_bounds(sx, sy, self.MISRATE).unit == RATIO_UNIT

    def test_disparity_bounds_sample_unit_is_disparity(self):
        sx = Sample(self.X, unit=self.SEC)
        sy = Sample(self.Y, unit=self.SEC)
        assert disparity_bounds(sx, sy, self.MISRATE, seed=self.SEED).unit == DISPARITY_UNIT

    # --- Sample path: center/spread propagate x.unit, shift the finer(x, y) ---

    def test_center_bounds_sample_unit_is_x_unit(self):
        sx = Sample(self.X, unit=self.SEC)
        assert center_bounds(sx, self.MISRATE).unit == self.SEC

    def test_spread_bounds_sample_unit_is_x_unit(self):
        sx = Sample(self.X, unit=self.SEC)
        assert spread_bounds(sx, self.MISRATE, seed=self.SEED).unit == self.SEC

    def test_shift_bounds_sample_unit_is_finer(self):
        # ms (base_units=1e6) is finer than s (base_units=1e9).
        sx = Sample(self.X, unit=self.SEC)
        sy = Sample(self.Y, unit=self.MS)
        assert shift_bounds(sx, sy, self.MISRATE).unit == self.MS

    # --- Raw (native array) bounds are unitless (NUMBER_UNIT) ---

    def test_raw_center_bounds_is_unitless(self):
        assert center_bounds(self.X, self.MISRATE).unit == NUMBER_UNIT

    def test_raw_spread_bounds_is_unitless(self):
        assert spread_bounds(self.X, self.MISRATE, seed=self.SEED).unit == NUMBER_UNIT

    def test_raw_shift_bounds_is_unitless(self):
        assert shift_bounds(self.X, self.Y, self.MISRATE).unit == NUMBER_UNIT

    def test_raw_ratio_bounds_is_unitless(self):
        assert ratio_bounds(self.X, self.Y, self.MISRATE).unit == NUMBER_UNIT

    def test_raw_disparity_bounds_is_unitless(self):
        assert disparity_bounds(self.X, self.Y, self.MISRATE, seed=self.SEED).unit == NUMBER_UNIT


class TestRawBoundsMisrateDomain:
    """The raw (native-array, float misrate) bounds API rejects out-of-[0,1] and
    NaN misrate with a domain/misrate AssumptionError.

    Covered directly via the raw path (Python has no Probability wrapper; every
    entry point takes a plain float misrate) for a one-sample (center_bounds) and
    a two-sample (shift_bounds) estimator.
    """

    X = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
    Y = [9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]
    BAD_MISRATES = [2.0, -0.1, float("nan")]

    def _assert_domain_misrate(self, err: AssumptionError) -> None:
        assert err.violation is not None
        assert err.violation.id.value == "domain"
        assert err.violation.subject == "misrate"

    @pytest.mark.parametrize("misrate", BAD_MISRATES)
    def test_center_bounds_raw_rejects(self, misrate):
        with pytest.raises(AssumptionError) as exc_info:
            center_bounds(self.X, misrate)
        self._assert_domain_misrate(exc_info.value)

    @pytest.mark.parametrize("misrate", BAD_MISRATES)
    def test_shift_bounds_raw_rejects(self, misrate):
        with pytest.raises(AssumptionError) as exc_info:
            shift_bounds(self.X, self.Y, misrate)
        self._assert_domain_misrate(exc_info.value)


class TestRatioBoundsErrorPriority:
    """The order in which ratio_bounds reports assumption errors: the misrate
    domain check runs before the positivity check on the values, so an invalid
    misrate wins even when x is also non-positive.

    Both entry points are exercised; positivity is raised by the estimator (not
    Sample construction), so the Sample path reports the true subject too.
    """

    @staticmethod
    def _assert_violation(err: AssumptionError, id_: str, subject: str, context: str) -> None:
        assert err.violation is not None, f"Expected violation, got message-only error: {err}{context}"
        assert err.violation.id.value == id_, context
        assert err.violation.subject == subject, context

    @pytest.mark.parametrize("entry", ["raw", "sample"])
    def test_domain_before_positivity(self, entry):
        # misrate=-0.1 is invalid (domain), x=-1 is non-positive (positivity):
        # domain(misrate) must take priority over positivity(x).
        x, y = [-1.0], [1.0]
        if entry == "sample":
            x, y = Sample(x), Sample(y)
        with pytest.raises(AssumptionError) as exc_info:
            ratio_bounds(x, y, -0.1)
        self._assert_violation(exc_info.value, "domain", "misrate", f" [{entry}]")

    @pytest.mark.parametrize("entry", ["raw", "sample"])
    def test_positivity_when_misrate_valid(self, entry):
        # Valid misrate but non-positive x -> positivity(x).
        x, y = [-1.0, -2.0, -3.0], [1.0, 2.0, 3.0]
        if entry == "sample":
            x, y = Sample(x), Sample(y)
        with pytest.raises(AssumptionError) as exc_info:
            ratio_bounds(x, y, 0.5)
        self._assert_violation(exc_info.value, "positivity", "x", f" [{entry}]")


class TestCenterMidpointSymmetry:
    """The n==2 center midpoint must be order-symmetric (the 0.5*a+0.5*b fix).

    assume_sorted=True is required so the midpoint sees the raw order (the
    normalizing sort would otherwise hide the asymmetry). The OLD a+(b-a)*0.5
    formula yields -3.4000000000000004 for the reversed order; the fixed formula
    must produce the EXACT same bits for both orders.
    """

    def test_center_n2_midpoint_order_symmetric(self):
        forward = center([-5.0, -1.8], assume_sorted=True)
        reversed_ = center([-1.8, -5.0], assume_sorted=True)
        assert forward == reversed_  # exact bit equality, not approximate
        assert forward == -3.4


class TestCompare1:
    """Tests from tests/compare1/ cross-language test data."""

    def test_compare1_reference(self):
        """Test compare1 against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "compare1"

        json_files = list(test_data_dir.glob("*.json"))
        assert len(json_files) > 0, f"No JSON test files found in {test_data_dir}"

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            input_data = test_case["input"]
            x_values = input_data["x"]
            seed = input_data.get("seed")
            thresholds_data = input_data["thresholds"]

            # Build thresholds
            thresholds = []
            for t_data in thresholds_data:
                metric = Metric(t_data["metric"])
                thresholds.append(Threshold(metric, t_data["value"], t_data["misrate"]))

            # Handle error test cases
            if "expected_error" in test_case:
                expected_error = test_case["expected_error"]
                # Sample creation itself may raise AssumptionError (e.g., empty x)
                with pytest.raises(AssumptionError) as exc_info:  # noqa: PT012
                    sx = Sample(x_values)
                    compare1(sx, thresholds, seed=seed)
                _assert_violation(exc_info.value, expected_error, f" for {json_file.name}")
                continue

            # Normal test case
            expected_projections = test_case["output"]["projections"]

            sx = Sample(x_values)
            projections = compare1(sx, thresholds, seed=seed)

            assert len(projections) == len(expected_projections), (
                f"Projection count mismatch for {json_file.name}: {len(projections)} vs {len(expected_projections)}"
            )

            for i, (actual, expected) in enumerate(zip(projections, expected_projections)):
                context = f" for {json_file.name}, projection {i}"
                assert abs(actual.estimate.value - expected["estimate"]) < 1e-9, (
                    f"Estimate mismatch{context}: expected {expected['estimate']}, got {actual.estimate.value}"
                )
                assert abs(actual.bounds.lower - expected["lower"]) < 1e-9, (
                    f"Lower bound mismatch{context}: expected {expected['lower']}, got {actual.bounds.lower}"
                )
                assert abs(actual.bounds.upper - expected["upper"]) < 1e-9, (
                    f"Upper bound mismatch{context}: expected {expected['upper']}, got {actual.bounds.upper}"
                )
                assert actual.verdict.value == expected["verdict"], (
                    f"Verdict mismatch{context}: expected {expected['verdict']}, got {actual.verdict.value}"
                )

    def test_compare1_supports_measurement_threshold_units(self):
        ms = MeasurementUnit("ms", "Time", "ms", "Millisecond", 1_000_000)
        ns = MeasurementUnit("ns", "Time", "ns", "Nanosecond", 1)
        sx = Sample(list(range(1, 11)), unit=ms)
        thresholds = [Threshold(Metric.CENTER, Measurement(3_000_000, ns), 0.05)]

        [projection] = compare1(sx, thresholds)

        assert abs(projection.estimate.value - 5.5) < 1e-9
        assert projection.estimate.unit == ms
        assert projection.bounds.unit == ms
        assert projection.verdict.value == "greater"


class TestCompare2:
    """Tests from tests/compare2/ cross-language test data."""

    def test_compare2_reference(self):
        """Test compare2 against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "compare2"

        json_files = list(test_data_dir.glob("*.json"))
        assert len(json_files) > 0, f"No JSON test files found in {test_data_dir}"

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            input_data = test_case["input"]
            x_values = input_data["x"]
            y_values = input_data["y"]
            seed = input_data.get("seed")
            thresholds_data = input_data["thresholds"]

            # Build thresholds
            thresholds = []
            for t_data in thresholds_data:
                metric = Metric(t_data["metric"])
                thresholds.append(Threshold(metric, t_data["value"], t_data["misrate"]))

            # Handle error test cases
            if "expected_error" in test_case:
                expected_error = test_case["expected_error"]
                # Sample creation itself may raise a validity AssumptionError (empty/NaN
                # x or y). Construction reports the y argument under subject "x" (it can't
                # know it's arg2), so only for construction *validity* errors expecting "y"
                # do we assert id only; post-construction errors (sparity/...) report "y"
                # positionally and are asserted in full.
                skip_subject = _is_sample_construction_y_error(test_case)
                with pytest.raises(AssumptionError) as exc_info:  # noqa: PT012
                    sx = Sample(x_values)
                    sy = Sample(y_values)
                    compare2(sx, sy, thresholds, seed=seed)
                _assert_violation(exc_info.value, expected_error, f" for {json_file.name}", skip_subject=skip_subject)
                continue

            # Normal test case
            expected_projections = test_case["output"]["projections"]

            sx = Sample(x_values)
            sy = Sample(y_values)
            projections = compare2(sx, sy, thresholds, seed=seed)

            assert len(projections) == len(expected_projections), (
                f"Projection count mismatch for {json_file.name}: {len(projections)} vs {len(expected_projections)}"
            )

            for i, (actual, expected) in enumerate(zip(projections, expected_projections)):
                context = f" for {json_file.name}, projection {i}"
                assert abs(actual.estimate.value - expected["estimate"]) < 1e-9, (
                    f"Estimate mismatch{context}: expected {expected['estimate']}, got {actual.estimate.value}"
                )
                assert abs(actual.bounds.lower - expected["lower"]) < 1e-9, (
                    f"Lower bound mismatch{context}: expected {expected['lower']}, got {actual.bounds.lower}"
                )
                assert abs(actual.bounds.upper - expected["upper"]) < 1e-9, (
                    f"Upper bound mismatch{context}: expected {expected['upper']}, got {actual.bounds.upper}"
                )
                assert actual.verdict.value == expected["verdict"], (
                    f"Verdict mismatch{context}: expected {expected['verdict']}, got {actual.verdict.value}"
                )

    def test_compare2_supports_measurement_threshold_units(self):
        ms = MeasurementUnit("ms", "Time", "ms", "Millisecond", 1_000_000)
        ns = MeasurementUnit("ns", "Time", "ns", "Nanosecond", 1)
        sx = Sample(list(range(1, 31)), unit=ms)
        sy = Sample([value * 1_000_000 for value in range(21, 51)], unit=ns)
        thresholds = [Threshold(Metric.SHIFT, Measurement(-14, ms), 0.05)]

        [projection] = compare2(sx, sy, thresholds)

        assert abs(projection.estimate.value + 20_000_000) < 1e-9
        assert projection.estimate.unit == ns
        assert projection.bounds.unit == ns
        assert projection.verdict.value == "less"
