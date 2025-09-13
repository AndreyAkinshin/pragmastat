# Pragmastat R Implementation

An R implementation of the Pragmastat statistical toolkit, providing robust statistical estimators for reliable analysis of real-world data.

## Installation

Install from CRAN (when available) or install the development version:

```r
# Install from GitHub (development version)
devtools::install_github("AndreyAkinshin/pragmastat", subdir = "r/pragmastat")

# Or install locally if you have the source
devtools::install()
```

## Usage

```r
library(pragmastat)

# One-sample estimators
data <- c(1.2, 3.4, 2.5, 4.1, 2.8)

center(data)     # Hodges-Lehmann location estimator
spread(data)     # Shamos scale estimator
rel_spread(data) # Relative dispersion measure

# Two-sample estimators
x <- c(5, 6, 7, 8)
y <- c(3, 4, 5, 6)

shift(x, y)    # Median shift between samples
ratio(x, y)    # Median ratio between samples
avg_spread(x, y)   # Pooled spread measure
disparity(x, y) # Effect size measure
```

## Estimators

### One-Sample Estimators

- **center**: Hodges-Lehmann location estimator - robust measure of central tendency
- **spread**: Shamos scale estimator - robust measure of dispersion
- **rel_spread**: Relative dispersion measure - spread normalized by center

### Two-Sample Estimators

- **shift**: Hodges-Lehmann shift estimator - robust measure of location difference
- **ratio**: Robust ratio estimator - median of all pairwise ratios
- **avg_spread**: Pooled spread estimator - combined measure of dispersion
- **disparity**: Effect size measure - normalized difference between samples

## License

MIT License

## Development

### Prerequisites

Ensure you have R and the required development tools installed:

```bash
# Install R development tools (on Ubuntu/Debian)
sudo apt-get install r-base-dev

# Install R development tools (on macOS with Homebrew)
brew install r

# Install devtools package in R
R -e "install.packages('devtools')"
```

### Command-Line Operations

All commands should be run from the package root directory (`r/pragmastat/`).

#### Check Package

Perform comprehensive package checks including code quality, documentation, and examples:

```bash
# Check using devtools 
R -e "devtools::check()"
```

#### Run Tests

Execute the test suite using testthat:

```bash
# Run all tests
R -e "devtools::test()"

# Run tests with detailed output
R -e "testthat::test_dir('tests/testthat')"

# Run specific test file
R -e "testthat::test_file('tests/testthat/test-hodges_lehmann.R')"
```

#### Build Package

Create distributable package files:

```bash
# Build using devtools
R -e "devtools::build()"
```

#### Install Package

Install the package locally for testing:

```bash
# Install using devtools (with dependencies)
R -e "devtools::install()"

# Install in development mode (loads source files)
R -e "devtools::load_all()"
```
