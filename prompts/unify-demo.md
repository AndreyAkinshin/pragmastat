# Reference Demo

## Scheme

x = (0, 2, 4, 6, 8)

Center(x) = 4
Center(x + 10) = 14
Center(x * 3) = 12

Spread(x) = 4
Spread(x + 10) = 4
Spread(x * 2) = 8

RelSpread(x) = 1
RelSpread(x * 5) = 1

y = (10, 12, 14, 16, 18)

Shift(x, y) = -10
Shift(x, x) = 0
Shift(x + 7, y + 3) = -6
Shift(x * 2, y * 2) = -20
Shift(y, x) = 10

x = (1, 2, 4, 8, 16)
y = (2, 4, 8, 16, 32)

Ratio(x, y) = 0.5
Ratio(x, x) = 1
Ratio(2 * x, 5 * y) = 0.2

x = (0, 3, 6, 9, 12)
y = (0, 2, 4, 6, 8)

Spread(x) = 6
Spread(y) = 4

AvgSpread(x, y) = 5
AvgSpread(x, x) = 6
AvgSpread(2 * x, 3 * x) = 15
AvgSpread(y, x) = 5
AvgSpread(2 * x, 2 * y) = 10

Shift(x, y) = 2
AvgSpread(x, y) = 5

Disparity(x, y) = 0.4
Disparity(x + 5, y + 5) = 0.4
Disparity(2 * x, 2 * y) = 0.4
Disparity(y, x) = -0.4

## Task

Read their implementations recursively from folders
  @cs/ @go/ @kt/ @py/ @r/ @rs/ @ts/
  and learn the correct API for each language

Apply the above Scheme to demo examples in files:

- @cs/Pragmastat.Demo/Program.cs
- @go/example/main.go
- @kt/src/main/kotlin/dev/pragmastat/example/Main.kt
- @py/examples/demo.py
- @r/pragmastat/inst/examples/demo.R
- @rs/pragmastat/examples/demo.rs
- @ts/examples/demo.ts

Adapt the scheme idiomatically to each language syntax and conventions in naming and API usage.
For each expression, use a printing to console statement.
Build all the projects and fix errors if any.
Run demo examples and check if they produce the correct results.

Also, apply the Scheme to in the "Primer" section of @manual/introduction.md as a set of LaTeX expressions (display style).
Put them one expression per line.
Each group of expressions should be headed by defeinition of the introduced estimator (inline style defenition + brief comment).

## Commands

Execute demos using the following commands from the project root:

**C#:**
```bash
cd cs/Pragmastat.Demo && dotnet run
```

**Go:**
```bash
cd go && go run ./example
```

**Kotlin:**
```bash
cd kt && ./gradlew run
```

**Python:**
```bash
PYTHONPATH=py python3 py/examples/demo.py
```

**R:**
```bash
cd r/pragmastat && Rscript inst/examples/demo.R
```

**Rust:**
```bash
cd rs/pragmastat && cargo run --example demo
```

**TypeScript:**
```bash
cd ts && npx ts-node examples/demo.ts
```