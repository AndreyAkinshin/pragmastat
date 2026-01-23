# Pragmastat Manual

The core part of this project is @manual/,
  which describes the essence of statistical approaches implemented in 7 languages:
  @py/ @ts/ @r/ @cs/ @kt/ @rs/ @go/
The whole manual is quite long, but it's decomposed into multiple short files that can be read individually when needed.
The entry point to the manual is @manual/main.typ
Next, needed sections can be obtained by following INCLUDE directives that use file paths relative to the root of the repository.
It's recommended to start learning the manual with the following essential files:
- @manual/abstract.typ
- @manual/introduction/primer.typ
- @manual/introduction/breaking.typ
- @manual/introduction/definitions.typ

Next, additional sections should be read based on the task.

Practical sections:
- @manual/estimators/ - quick info on each estimator
- @manual/algorithms/ - brief explanation of the used algorithms
- @manual/tests/ - source of truth for reference test case suites
  - @manual/tests/_motivation.typ @manual/tests/_framework.typ - methodology behind testing approach

For theoretical work:
- @manual/methodology/ - detailed notes on the spirit of the manual and changes from the traditional statistics
- @manual/properties/ - analysis of the estimators
- @manual/distributions/ - quick reference on the used distributions
- @manual/studies/ - deep drill into the properties of the estimators
