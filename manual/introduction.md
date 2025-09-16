# Introduction

The toolkit consists of statistical *procedures* (only estimators at the moment,
  but it will be extended with other methods, functions, and algorithms).
These procedures process measurements and provide statistical insights or decisions.
The practitioner's main task is choosing the right procedure for each problem.
To build a pragmatic statistical toolkit, consider the following groups of desired properties.

**Usability**

- Be intuitive, with minimal risk of misuse
- Be mathematically elegant and simple, accessible to readers with average mathematical skills
- Be simple to implement in code
  (can be written from scratch in any language without special libraries or complex calculations)
- Have a small number of configuration parameters with reasonable defaults
- Follow the same spirit of usability as computer interfaces and everyday things; see [@norman2013]

**Reliability**

- Provide reliable, precise, and useful results for real-world problems
- Provide consistent results for similar studies despite randomness
- Be robust to corrupted or extreme measurements
- Be location- and scale-invariant to ensure portability

**Applicability**

- Be applicable across diverse contexts
- Work well not only asymptotically but also with small sample sizes
- Work pretty well for the Normal distribution and reasonably well for other popular models:
    Uniform / Beta / Weibull / Student's t / Gumbel /
    Exponential / Cauchy / Pareto / Log-normal / Fréchet / etc.
- Work reliably across a wide range of assumptions:
  - Continuous / Discrete / Continuous-discrete mixtures
  - Uniform / Unimodal / Bimodal / Multimodal
  - Light-tailed / Exponential / Heavy-tailed
  - Symmetric / Right-skewed / Left-skewed
  - Independent / Dependent (correlated measurements)
  - Identically distributed / Heterogeneously distributed (weighted data)
- Tolerate slight deviations from popular assumptions
- Properly handle corner cases

This manual presents a toolkit of rule-of-thumb statistical procedures that naturally complement each other.
