## From Assumptions to Conditions

Traditional statistical practice starts with model assumptions,
  then derives optimal procedures under those assumptions.
This approach prioritizes mathematical convenience over practical application.
Practitioners don't know the distribution in advance,
  so they lack clear guidance on which procedure to choose by default.

Most traditional statistical procedures rely heavily on the $\Additive$ ('Normal') distribution and fail on real data
  because actual measurements contain outliers, exhibit skewness, or follow unknown distributions.
When assumptions fail, procedures designed for those assumptions also fail.

This toolkit starts with procedures and tests how they perform under different distributional conditions.
This approach reverses the traditional workflow: instead of deriving procedures from assumptions,
  we evaluate how each procedure performs across various distributions.
This enables direct comparison and provides clear guidance on procedure selection
  based on known characteristics of the data source.

This procedure-first approach eliminates the need for complex mathematical derivations.
All evaluations can be done numerically through Monte Carlo simulation.
Generate samples from each distribution, apply each procedure, and measure the results.
The numerical evidence directly shows which procedures work best under which conditions.