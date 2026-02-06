import json
from pathlib import Path
import pytest
from pragmastat import (
    center,
    spread,
    rel_spread,
    shift,
    ratio,
    avg_spread,
    disparity,
    pairwise_margin,
    signed_rank_margin,
    shift_bounds,
    ratio_bounds,
    median_bounds,
    center_bounds,
    Rng,
    Uniform,
    Additive,
    Multiplic,
    Exp,
    Power,
)
from pragmastat.assumptions import AssumptionError


def find_repo_root():
    """Find the repository root by looking for CITATION.cff file."""
    current_dir = Path(__file__).parent
    while current_dir != current_dir.parent:
        if (current_dir / "CITATION.cff").exists():
            return current_dir
        current_dir = current_dir.parent
    raise RuntimeError("Could not find repository root (CITATION.cff not found)")


def run_reference_tests(estimator_name, estimator_func, is_two_sample=False):
    """Run reference tests against JSON data files.

    Tests that raise AssumptionError are skipped, as those cases
    are validated separately in the assumption tests.
    """
    repo_root = find_repo_root()
    test_data_dir = repo_root / "tests" / estimator_name

    json_files = list(test_data_dir.glob("*.json"))
    assert len(json_files) > 0, f"No JSON test files found in {test_data_dir}"

    skipped_count = 0
    executed_count = 0

    for json_file in json_files:
        with open(json_file, "r") as f:
            test_case = json.load(f)

        if is_two_sample:
            input_x = test_case["input"]["x"]
            input_y = test_case["input"]["y"]
            expected_output = test_case["output"]

            try:
                actual_output = estimator_func(input_x, input_y)
            except AssumptionError:
                # Skip cases that violate assumptions - tested separately
                skipped_count += 1
                continue
        else:
            input_x = test_case["input"]["x"]
            expected_output = test_case["output"]

            try:
                actual_output = estimator_func(input_x)
            except AssumptionError:
                # Skip cases that violate assumptions - tested separately
                skipped_count += 1
                continue

        executed_count += 1
        assert abs(actual_output - expected_output) < 1e-9, (
            f"Failed for test file: {json_file.name}, expected: {expected_output}, got: {actual_output}"
        )

    # Ensure at least some tests were actually executed
    assert executed_count > 0, (
        f"All {len(json_files)} tests were skipped due to assumption violations"
    )


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

        assert len(actual) == len(expected), (
            f"Length mismatch for {json_file.name}: {len(actual)} vs {len(expected)}"
        )
        for i, (act, exp) in enumerate(zip(actual, expected)):
            assert abs(act - exp) < 1e-12, (
                f"Failed for {json_file.name}, index {i}: expected {exp}, got {act}"
            )


class TestReference:
    def test_center_reference(self):
        run_reference_tests("center", center)

    def test_spread_reference(self):
        run_reference_tests("spread", spread)

    def test_rel_spread_reference(self):
        run_reference_tests("rel-spread", rel_spread)

    def test_shift_reference(self):
        run_reference_tests("shift", shift, is_two_sample=True)

    def test_ratio_reference(self):
        run_reference_tests("ratio", ratio, is_two_sample=True)

    def test_avg_spread_reference(self):
        run_reference_tests("avg-spread", avg_spread, is_two_sample=True)

    def test_disparity_reference(self):
        run_reference_tests("disparity", disparity, is_two_sample=True)

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
                with pytest.raises(AssumptionError) as exc_info:
                    pairwise_margin(n, m, misrate)
                assert (
                    exc_info.value.violation.id.value
                    == test_case["expected_error"]["id"]
                )
                continue

            expected_output = test_case["output"]
            actual_output = pairwise_margin(n, m, misrate)

            assert actual_output == expected_output, (
                f"Failed for test file: {json_file.name}, expected: {expected_output}, got: {actual_output}"
            )

    def test_shift_bounds_reference(self):
        """Test shift_bounds against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "shift-bounds"

        json_files = list(test_data_dir.glob("*.json"))
        assert len(json_files) > 0, f"No JSON test files found in {test_data_dir}"

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            input_x = test_case["input"]["x"]
            input_y = test_case["input"]["y"]
            misrate = test_case["input"]["misrate"]
            expected_lower = test_case["output"]["lower"]
            expected_upper = test_case["output"]["upper"]

            result = shift_bounds(input_x, input_y, misrate)

            assert abs(result.lower - expected_lower) < 1e-9, (
                f"Failed lower bound for test file: {json_file.name}, expected: {expected_lower}, got: {result.lower}"
            )
            assert abs(result.upper - expected_upper) < 1e-9, (
                f"Failed upper bound for test file: {json_file.name}, expected: {expected_upper}, got: {result.upper}"
            )

    def test_ratio_bounds_reference(self):
        """Test ratio_bounds against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "ratio-bounds"

        if not test_data_dir.exists():
            pytest.skip("ratio-bounds test data directory not found")

        json_files = list(test_data_dir.glob("*.json"))
        if len(json_files) == 0:
            return

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            input_x = test_case["input"]["x"]
            input_y = test_case["input"]["y"]
            misrate = test_case["input"]["misrate"]
            expected_lower = test_case["output"]["lower"]
            expected_upper = test_case["output"]["upper"]

            result = ratio_bounds(input_x, input_y, misrate)

            assert abs(result.lower - expected_lower) < 1e-9, (
                f"Failed lower bound for test file: {json_file.name}, expected: {expected_lower}, got: {result.lower}"
            )
            assert abs(result.upper - expected_upper) < 1e-9, (
                f"Failed upper bound for test file: {json_file.name}, expected: {expected_upper}, got: {result.upper}"
            )

    def test_rng_uniform_reference(self):
        """Test Rng uniform() against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "rng"

        json_files = [
            f
            for f in test_data_dir.glob("*.json")
            if f.name.startswith("uniform-seed-")
        ]
        assert len(json_files) > 0, (
            f"No uniform seed test files found in {test_data_dir}"
        )

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            seed = test_case["input"]["seed"]
            count = test_case["input"]["count"]
            expected = test_case["output"]

            rng = Rng(seed)
            actual = [rng.uniform() for _ in range(count)]

            assert len(actual) == len(expected), (
                f"Length mismatch for {json_file.name}: {len(actual)} vs {len(expected)}"
            )
            for i, (act, exp) in enumerate(zip(actual, expected)):
                assert abs(act - exp) < 1e-15, (
                    f"Failed for {json_file.name}, index {i}: expected {exp}, got {act}"
                )

    def test_rng_uniform_int_reference(self):
        """Test Rng uniform_int() against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "rng"

        json_files = [
            f for f in test_data_dir.glob("*.json") if f.name.startswith("uniform-int-")
        ]
        assert len(json_files) > 0, (
            f"No uniform int test files found in {test_data_dir}"
        )

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

            assert actual == expected, (
                f"Failed for {json_file.name}: expected {expected}, got {actual}"
            )

    def test_rng_string_seed_reference(self):
        """Test Rng with string seeds against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "rng"

        json_files = [
            f
            for f in test_data_dir.glob("*.json")
            if f.name.startswith("uniform-string-")
        ]
        assert len(json_files) > 0, (
            f"No string seed test files found in {test_data_dir}"
        )

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            seed = test_case["input"]["seed"]
            count = test_case["input"]["count"]
            expected = test_case["output"]

            rng = Rng(seed)
            actual = [rng.uniform() for _ in range(count)]

            assert len(actual) == len(expected), (
                f"Length mismatch for {json_file.name}: {len(actual)} vs {len(expected)}"
            )
            for i, (act, exp) in enumerate(zip(actual, expected)):
                assert abs(act - exp) < 1e-15, (
                    f"Failed for {json_file.name}, index {i}: expected {exp}, got {act}"
                )

    def test_rng_uniform_range_reference(self):
        """Test Rng uniform_range() against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "rng"

        json_files = [
            f
            for f in test_data_dir.glob("*.json")
            if f.name.startswith("uniform-range-")
        ]
        assert len(json_files) > 0, (
            f"No uniform range test files found in {test_data_dir}"
        )

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            seed = test_case["input"]["seed"]
            min_val = test_case["input"]["min"]
            max_val = test_case["input"]["max"]
            count = test_case["input"]["count"]
            expected = test_case["output"]

            rng = Rng(seed)
            actual = [rng.uniform_range(min_val, max_val) for _ in range(count)]

            assert len(actual) == len(expected), (
                f"Length mismatch for {json_file.name}: {len(actual)} vs {len(expected)}"
            )
            for i, (act, exp) in enumerate(zip(actual, expected)):
                assert abs(act - exp) < 1e-12, (
                    f"Failed for {json_file.name}, index {i}: expected {exp}, got {act}"
                )

    def test_rng_uniform_bool_reference(self):
        """Test Rng uniform_bool() against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "rng"

        json_files = [
            f
            for f in test_data_dir.glob("*.json")
            if f.name.startswith("uniform-bool-seed-")
        ]
        assert len(json_files) > 0, (
            f"No uniform bool test files found in {test_data_dir}"
        )

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            seed = test_case["input"]["seed"]
            count = test_case["input"]["count"]
            expected = test_case["output"]

            rng = Rng(seed)
            actual = [rng.uniform_bool() for _ in range(count)]

            assert actual == expected, (
                f"Failed for {json_file.name}: expected {expected}, got {actual}"
            )

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
                assert abs(act - exp) < 1e-15, (
                    f"Failed for {json_file.name}, index {i}: expected {exp}, got {act}"
                )

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
                assert abs(act - exp) < 1e-15, (
                    f"Failed for {json_file.name}, index {i}: expected {exp}, got {act}"
                )

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
            lambda input_data: Multiplic(
                input_data["logMean"], input_data["logStdDev"]
            ),
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
        import pytest

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
                assert exc_info.value.violation.id.value == expected_error["id"], (
                    f"Expected error id {expected_error['id']}, got {exc_info.value.violation.id.value}"
                )
                continue

            expected_output = test_case["output"]

            actual_output = signed_rank_margin(n, misrate)

            assert actual_output == expected_output, (
                f"Failed for test file: {json_file.name}, expected: {expected_output}, got: {actual_output}"
            )

    def test_median_bounds_reference(self):
        """Test median_bounds against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "median-bounds"

        json_files = list(test_data_dir.glob("*.json"))
        assert len(json_files) > 0, f"No JSON test files found in {test_data_dir}"

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            input_x = test_case["input"]["x"]
            misrate = test_case["input"]["misrate"]

            # Handle error test cases
            if "expected_error" in test_case:
                with pytest.raises(AssumptionError) as exc_info:
                    median_bounds(input_x, misrate)
                assert (
                    exc_info.value.violation.id.value
                    == test_case["expected_error"]["id"]
                ), (
                    f"Expected error id {test_case['expected_error']['id']}, got {exc_info.value.violation.id.value}"
                )
                continue

            expected_lower = test_case["output"]["lower"]
            expected_upper = test_case["output"]["upper"]

            result = median_bounds(input_x, misrate)

            assert abs(result.lower - expected_lower) < 1e-9, (
                f"Failed lower bound for test file: {json_file.name}, expected: {expected_lower}, got: {result.lower}"
            )
            assert abs(result.upper - expected_upper) < 1e-9, (
                f"Failed upper bound for test file: {json_file.name}, expected: {expected_upper}, got: {result.upper}"
            )

    def test_center_bounds_reference(self):
        """Test center_bounds against reference data."""
        repo_root = find_repo_root()
        test_data_dir = repo_root / "tests" / "center-bounds"

        json_files = list(test_data_dir.glob("*.json"))
        assert len(json_files) > 0, f"No JSON test files found in {test_data_dir}"

        for json_file in json_files:
            with open(json_file, "r") as f:
                test_case = json.load(f)

            input_x = test_case["input"]["x"]
            misrate = test_case["input"]["misrate"]

            # Handle error test cases
            if "expected_error" in test_case:
                with pytest.raises(AssumptionError) as exc_info:
                    center_bounds(input_x, misrate)
                assert (
                    exc_info.value.violation.id.value
                    == test_case["expected_error"]["id"]
                ), (
                    f"Expected error id {test_case['expected_error']['id']}, got {exc_info.value.violation.id.value}"
                )
                continue

            expected_lower = test_case["output"]["lower"]
            expected_upper = test_case["output"]["upper"]

            result = center_bounds(input_x, misrate)

            assert abs(result.lower - expected_lower) < 1e-9, (
                f"Failed lower bound for test file: {json_file.name}, expected: {expected_lower}, got: {result.lower}"
            )
            assert abs(result.upper - expected_upper) < 1e-9, (
                f"Failed upper bound for test file: {json_file.name}, expected: {expected_upper}, got: {result.upper}"
            )
