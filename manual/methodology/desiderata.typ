#import "/manual/definitions.typ": *

== Desiderata

The toolkit consists of statistical _procedures_ — practical methods that transform raw measurements into actionable insights and decisions.
When practitioners face real-world problems involving data analysis,
  their success depends on selecting the right procedure for each specific situation.
Convenient and efficient procedures have the following _essential attributes_:

- *Usability.*
  Procedures should feel natural to practitioners and minimize opportunities for misuse.
  They should be mathematically elegant yet accessible to readers with standard mathematical backgrounds.
  Implementation should be straightforward across programming languages.
  Like well-designed APIs, these procedures should follow intuitive design principles that reduce cognitive load.
- *Reliability.*
  Procedures should deliver consistent, trustworthy results,
    even in the presence of noise, data corruption, and extreme outliers.
- *Applicability.*
  Procedures should perform well across diverse contexts and sample sizes.
  They should handle the full spectrum of distributions commonly encountered in practice,
    from ideal theoretical models to data that deviates significantly from any assumed distribution.

This manual introduces a unified toolkit that aims to satisfy these properties and provide reliable rule-of-thumb procedures for everyday analytical tasks.
