# Reference Demo

## Scheme

Reference test values demonstrating expected behavior:

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

### Step 1: Learn the APIs

Explore the implementation folders to understand each language's API:
  @cs/ @go/ @kt/ @py/ @r/ @rs/ @ts/

Focus on function names, calling conventions, and type requirements.

### Step 2: Implement Demo Programs

Apply the above Scheme to demo examples in files:

- @cs/Pragmastat.Demo/Program.cs
- @go/demo/main.go
- @kt/src/main/kotlin/dev/pragmastat/demo/Main.kt
- @py/examples/demo.py
- @r/pragmastat/inst/examples/demo.R
- @rs/pragmastat/examples/demo.rs
- @ts/examples/demo.ts

**Requirements:**

- Adapt the scheme idiomatically to each language's syntax and conventions
- Use proper naming (e.g., `relSpread` vs `RelSpread` vs `rel_spread`)
- Print each expression result to console (one per line)
- Preserve the order of expressions as shown in the Scheme

**Expected Output Format:**

- Numeric results may vary slightly due to language differences (e.g., `4` vs `4.0`)
- R prefixes output with `[1]`
- All languages should produce equivalent numeric values

### Step 3: Build and Verify

Build all projects and fix any compilation/syntax errors:

```bash
./build.sh demo
```

Verify that each demo produces output matching the Scheme values (allowing for formatting differences).

### Step 4: LaTeX Documentation

Apply the Scheme to the "Primer" section of @manual/introduction/primer.md as LaTeX expressions.

**Format:**
- Use display-style math (`$$...$$`) for expressions
- One expression per line
- Prefix each group with an inline estimator definition and brief description

## Commands

Execute demos using the following commands from the project root:

**Individual language demos:**

```bash
./build.sh cs demo    # C#
./build.sh go demo    # Go
./build.sh kt demo    # Kotlin
./build.sh py demo    # Python
./build.sh r demo     # R
./build.sh rs demo    # Rust
./build.sh ts demo    # TypeScript
```

**Run all demos at once:**
```bash
./build.sh demo
```
