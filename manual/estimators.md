# Estimators

In this section, we consider one-sample and two-sample estimators.
These estimators can be used directly to obtain basic insights from the samples, or as a building block in composite procedures.
Estimator evaluation focuses on these key properties:

- *Gaussian efficiency*: performance quality under normal distribution conditions.
  High efficiency produces accurate estimates with fewer measurements under normality.
- *Breakdown point*: which portion of the sample can be replaced by corrupted measurements
    without impacting the estimation.
- *Invariance properties* (allow higher portability)
  - Shift-invariance: result is the same if all measurements are increased by the same constant
  - Scale-invariance: result is the same if all measurements are multiplied by the same constant
- *Domain*: Supported set of measurement values

<!-- One-sample -->

<!-- INCLUDE manual/estimators/center.md -->

<!-- INCLUDE manual/estimators/spread.md -->

<!-- INCLUDE manual/estimators/rel-spread.md -->

<!-- Two-sample -->

<!-- INCLUDE manual/estimators/shift.md -->

<!-- INCLUDE manual/estimators/ratio.md -->

<!-- INCLUDE manual/estimators/avg-spread.md -->

<!-- INCLUDE manual/estimators/disparity.md -->

