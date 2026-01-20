import json
from pathlib import Path
from pragmastat import (
    center,
    spread,
    rel_spread,
    shift,
    ratio,
    avg_spread,
    disparity,
    pairwise_margin,
    shift_bounds,
)


def find_repo_root():
    """Find the repository root by looking for CITATION.cff file."""
    current_dir = Path(__file__).parent
    while current_dir != current_dir.parent:
        if (current_dir / "CITATION.cff").exists():
            return current_dir
        current_dir = current_dir.parent
    raise RuntimeError("Could not find repository root (CITATION.cff not found)")


def run_reference_tests(estimator_name, estimator_func, is_two_sample=False):
    """Run reference tests against JSON data files."""
    repo_root = find_repo_root()
    test_data_dir = repo_root / "tests" / estimator_name

    json_files = list(test_data_dir.glob("*.json"))
    assert len(json_files) > 0, f"No JSON test files found in {test_data_dir}"

    for json_file in json_files:
        with open(json_file, "r") as f:
            test_case = json.load(f)

        if is_two_sample:
            input_x = test_case["input"]["x"]
            input_y = test_case["input"]["y"]
            expected_output = test_case["output"]

            actual_output = estimator_func(input_x, input_y)
        else:
            input_x = test_case["input"]["x"]
            expected_output = test_case["output"]

            actual_output = estimator_func(input_x)

        assert abs(actual_output - expected_output) < 1e-10, (
            f"Failed for test file: {json_file.name}, expected: {expected_output}, got: {actual_output}"
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

            assert abs(result.lower - expected_lower) < 1e-10, (
                f"Failed lower bound for test file: {json_file.name}, expected: {expected_lower}, got: {result.lower}"
            )
            assert abs(result.upper - expected_upper) < 1e-10, (
                f"Failed upper bound for test file: {json_file.name}, expected: {expected_upper}, got: {result.upper}"
            )
