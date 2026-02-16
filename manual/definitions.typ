// Pragmastat Definitions
// Shared definitions for PDF generation and web conversion
// These are imported into the main document and implementation pages

#import "version.typ": version

// ==========================================================================
// Shared URLs
// ==========================================================================
#let github-repo = "https://github.com/AndreyAkinshin/pragmastat"
#let github-tree = github-repo + "/tree/v" + version
#let docs-site = "https://pragmastat.dev"

// ==========================================================================
// Language Configurations
// Paths are absolute from project root (use --root . with typst)
// ==========================================================================
#let languages = (
  cs: (
    title: "C#",
    code: "cs",
    demo: "/cs/Pragmastat.Demo/Program.cs",
    package: "Pragmastat on NuGet: https://www.nuget.org/packages/Pragmastat/",
  ),
  go: (
    title: "Go",
    code: "go",
    demo: "/go/demo/main.go",
    package: none,
  ),
  kt: (
    title: "Kotlin",
    code: "kotlin",
    demo: "/kt/src/main/kotlin/dev/pragmastat/demo/Main.kt",
    package: "Pragmastat on Maven Central Repository: https://central.sonatype.com/artifact/dev.pragmastat/pragmastat/overview",
  ),
  py: (
    title: "Python",
    code: "python",
    demo: "/py/examples/demo.py",
    package: "Pragmastat on PyPI: https://pypi.org/project/pragmastat/",
  ),
  r: (
    title: "R",
    code: "r",
    demo: "/r/pragmastat/inst/examples/demo.R",
    package: none,
  ),
  rs: (
    title: "Rust",
    code: "rust",
    demo: "/rs/pragmastat/examples/demo.rs",
    package: "Pragmastat on crates.io: https://crates.io/crates/pragmastat",
  ),
  ts: (
    title: "TypeScript",
    code: "typescript",
    demo: "/ts/examples/demo.ts",
    package: "Pragmastat on npm: https://www.npmjs.com/package/pragmastat",
  ),
)

// ==========================================================================
// Estimators (math operators)
// ==========================================================================
#let Center = math.op("Center")
#let Spread = math.op("Spread")
#let Shift = math.op("Shift")
#let Ratio = math.op("Ratio")
#let RelSpread = math.op("RelSpread")
#let AvgSpread = math.op("AvgSpread")
#let Disparity = math.op("Disparity")
#let PairwiseMargin = math.op("PairwiseMargin")
#let ShiftBounds = math.op("ShiftBounds")
#let RatioBounds = math.op("RatioBounds")
#let CenterBounds = math.op("CenterBounds")
#let SpreadBounds = math.op("SpreadBounds")
#let AvgSpreadBounds = math.op("AvgSpreadBounds")
#let DisparityBounds = math.op("DisparityBounds")
#let SignMargin = math.op("SignMargin")
#let SignedRankMargin = math.op("SignedRankMargin")
#let Dominance = math.op("Dominance")

// ==========================================================================
// Traditional Estimators
// ==========================================================================
#let Median = math.op("Median")
#let Mean = math.op("Mean")
#let StdDev = math.op("StdDev")
#let Var = math.op("Var")
#let IQR = math.op("IQR")
#let MAD = math.op("MAD")
#let Mode = math.op("Mode")
#let Quantile = math.op("Quantile")
#let median = math.op("median")

// ==========================================================================
// Distributions (underlined operators to distinguish from estimators)
// ==========================================================================
#let Additive = math.underline(math.op("Additive"))
#let Multiplic = math.underline(math.op("Multiplic"))
#let Exp = math.underline(math.op("Exp"))
#let Power = math.underline(math.op("Power"))
#let Uniform = math.underline(math.op("Uniform"))

// ==========================================================================
// Utilities (randomization)
// ==========================================================================
#let Rng = math.op("Rng")
#let UniformInt = math.op("UniformInt")
#let UniformFloat = math.op("UniformFloat")
#let Sample = math.op("Sample")
#let Resample = math.op("Resample")
#let Shuffle = math.op("Shuffle")

// ==========================================================================
// Vectors (bold)
// ==========================================================================
#let vx = math.bold("x")
#let vy = math.bold("y")
#let vz = math.bold("z")
#let vw = math.bold("w")
#let vd = math.bold("d")

// ==========================================================================
// Drift and Efficiency
// ==========================================================================
#let Drift = math.op("Drift")
#let AvgDrift = math.op("AvgDrift")
#let DispDrift = math.op("DispDrift")

// ==========================================================================
// Distribution Parameters (upright text in math mode)
// ==========================================================================
#let pmean = math.upright("mean")
#let pstddev = math.upright("stdDev")
#let plogmean = math.upright("logMean")
#let plogstddev = math.upright("logStdDev")
#let pmin = math.upright("min")
#let pmax = math.upright("max")
#let pshape = math.upright("shape")
#let prate = math.upright("rate")

// ==========================================================================
// Asymptotic Constants
// ==========================================================================
#let cmad = math.attach(math.italic("c"), b: math.upright("mad"))
#let cspr = math.attach(math.italic("c"), b: math.upright("spr"))

// ==========================================================================
// Mathematical Relations
// ==========================================================================
#let approxdist = $tilde upright("approx")$  // approximately distributed as

// ==========================================================================
// Special Symbols
// ==========================================================================
#let misrate = math.upright("misrate")
#let EE = math.bb("E")
#let RR = math.bb("R")
#let NN = math.bb("N")

// ==========================================================================
// Source Code Include
// ==========================================================================
// Usage: #source-include("path/to/file.cs", "cs")
// Reads a source file and displays it as a code block
#let source-include(path, lang) = {
  raw(read("/" + path), lang: lang, block: true)
}
