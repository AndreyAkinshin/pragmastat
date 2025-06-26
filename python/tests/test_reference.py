import json
import os
import pytest
from pathlib import Path
from pragmastat import (
    center, spread, volatility, precision,
    med_shift, med_ratio, med_spread, med_disparity
)


def find_repo_root():
    """Find the repository root by looking for build.cmd file."""
    current_dir = Path(__file__).parent
    while current_dir != current_dir.parent:
        if (current_dir / "build.cmd").exists():
            return current_dir
        current_dir = current_dir.parent
    raise RuntimeError("Could not find repository root (build.cmd not found)")


def run_reference_tests(estimator_name, estimator_func, is_two_sample=False):
    """Run reference tests against JSON data files."""
    repo_root = find_repo_root()
    test_data_dir = repo_root / "tests" / estimator_name
    
    json_files = list(test_data_dir.glob("*.json"))
    assert len(json_files) > 0, f"No JSON test files found in {test_data_dir}"
    
    for json_file in json_files:
        with open(json_file, 'r') as f:
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
        
        assert abs(actual_output - expected_output) < 1e-10, \
            f"Failed for test file: {json_file.name}, expected: {expected_output}, got: {actual_output}"


class TestReference:
    
    def test_center_reference(self):
        run_reference_tests("center", center)
    
    def test_spread_reference(self):
        run_reference_tests("spread", spread)
    
    def test_volatility_reference(self):
        run_reference_tests("volatility", volatility)
    
    def test_precision_reference(self):
        run_reference_tests("precision", precision)
    
    def test_med_shift_reference(self):
        run_reference_tests("med-shift", med_shift, is_two_sample=True)
    
    def test_med_ratio_reference(self):
        run_reference_tests("med-ratio", med_ratio, is_two_sample=True)
    
    def test_med_spread_reference(self):
        run_reference_tests("med-spread", med_spread, is_two_sample=True)
    
    def test_med_disparity_reference(self):
        run_reference_tests("med-disparity", med_disparity, is_two_sample=True)