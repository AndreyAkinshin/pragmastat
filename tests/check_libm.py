#!/usr/bin/env python3
"""Assert no shipped source reaches the platform's transcendental library.

IEEE 754 fixes the four arithmetic operations, the remainder and the square root: every conforming
implementation returns the correctly rounded result, so seven ports agree on them by specification.
It fixes nothing about log, exp, pow or the trigonometric functions. Two libms may each be within
an ulp of the true value and still disagree in the last bit, and this toolkit's central promise is
that the seven ports return the same bits. A call to one of those functions on a path that decides
an answer is therefore not a small risk, it is the whole failure mode, and it is the one that has
actually broken bitwise identity here: `ratio` was `exp(median(log x - log y))` over the platform's
pair, and a one-ulp perturbation moved it on every one of its 180 probed inputs. Every port now
carries its own exponential and logarithm, assembled from operations the specification pins, and
that same perturbation moves ratio on none of them. This file exists so that the next call does
not undo it quietly: nothing else in the repository would notice one.

The gap is not hypothetical either. Two dead `Math.Pow` extensions sat on C#'s estimator path with
no caller at all, and the commit that added this file deleted them.

WHAT ALREADY EXISTED, AND WHY IT IS NOT THIS. tests/conformance_probe.sh asks the same question of
one port. It rewrites Go's four calls into perturbed wrappers, runs the estimators twice, and its
"stray" arm fails when a fifth Go call appears that the perturbation would not have covered. That
arm is a static check over `go/*.go` and nothing else. The other six ports had no equivalent: a
`Math.Exp` could land in C#, Kotlin, TypeScript, Python, Rust or R and every gate in this
repository would stay green, because the fixtures are compared against a stored answer that the
new call would produce identically on the machine that generated them.

The calls that remain are the parametric families, and they are exempt for a stated reason rather
than by a list: see DISTRIBUTION_SOURCES, which derives the exemption from the conformance class
the manifest declares and therefore stops granting it the moment that class changes.

WHAT A STATIC CHECK CAN AND CANNOT SEE, PER LANGUAGE. The list is here because a guard that states
its blind spot is worth more than one that implies it has none.

  * go, py, cs, ts: the platform's functions are reached through a namespace the source has to
    spell (`math.`, `Math.`), so a call is visible in the text. An aliased import would hide one;
    Go's is refused by tests/conformance_probe.sh and the other three cannot alias a namespace this
    way. What is invisible: a call through a variable holding a function reference. No port does
    that, and none has a reason to.
  * rs: the functions are inherent methods on f64, so there is no namespace and the check is a list
    of method names. `x.exp()` on a type that is not f64 would be a false positive; nothing here
    defines such a method, and a false positive is a red run somebody answers, not a silent pass.
  * kt: `.pow` is a method and is read at the call site; the rest arrive as top-level functions and
    are read from the import list, because the toolkit has its own `log(values, subject)` and a
    call-site rule cannot tell the two apart. A wildcard `import kotlin.math.*` makes that reading
    blind, so it is itself a finding: one file had one.
  * r: `^` is a call into libm's pow, and R has no namespace at all. Base R's own `log` and `exp`
    are reachable by their bare names from anywhere. Both are checked; `2^k` is the exception and
    says below why.

The counts are pinned in both directions. Files scanned per port catches a source root that moved
or a filter that widened: a scanner reaching nothing reports success. Exempt call sites found per
port catches the opposite and subtler failure -- a pattern that stopped matching. A scanner that
has gone blind finds no forbidden call *and* no `sqrt`, and only the second number can tell that
apart from a clean port.
"""

import json
import re
import sys
from pathlib import Path

PORTS = ("cs", "go", "kt", "py", "r", "rs", "ts")

# Where each port keeps its shipped library, and which files under it are not shipped. Everything
# under the root is read unless it matches an exclusion, and every exclusion is either named here
# with its reason or derived from the port's own declaration of what its tests are.
SOURCES = {
    # obj/ is the build's own generated tree (AssemblyInfo, the .NET 10 / 6 / netstandard2.0
    # triple), not source anybody wrote.
    "cs": ("cs/Pragmastat", "*.cs", ("obj/*",)),
    "go": ("go", "*.go", ("*_test.go",)),
    "kt": ("kt/src/main/kotlin/dev/pragmastat", "*.kt", ()),
    "py": ("py/pragmastat", "*.py", ()),
    "r": ("r/pragmastat/R", "*.R", ()),
    # The Rust port keeps its unit tests inside src/, both as `#[cfg(test)]` modules inline in a
    # source file and as whole files that lib.rs declares under that attribute. The inline ones are
    # skipped by the attribute while scanning; the whole files are read out of lib.rs rather than
    # listed here, so a test module added tomorrow is excluded because lib.rs says it is a test
    # module, not because somebody remembered to name it in two places. log_function.rs holds one
    # of each shape.
    "rs": ("rs/pragmastat/src", "*.rs", ()),
    "ts": ("ts/src", "*.ts", ()),
}

# Named constants and the operations IEEE 754 or the representation makes exact. sqrt is required
# correctly rounded; abs, copysign and the bit conversions move bits; floor/ceil/trunc/round land on
# integers; ldexp/frexp scale by a power of two; nextafter names a neighbor rather than computing
# one; min/max select. A call to one of these is not a divergence risk, but it IS the evidence that
# the scanner can still see calls at all, which is why they are counted rather than ignored.
EXACT = {
    "abs",
    "bitdecrement",
    "bitincrement",
    "ceil",
    "ceiling",
    "copysign",
    "e",
    "float32bits",
    "float32frombits",
    "float64bits",
    "float64frombits",
    "floor",
    "frexp",
    "fround",
    "inf",
    "isfinite",
    "isinf",
    "isnan",
    "ldexp",
    "ln2",
    "max",
    "maxfloat32",
    "maxfloat64",
    "maxint",
    "maxint16",
    "maxint32",
    "maxint64",
    "maxint8",
    "maxuint16",
    "maxuint32",
    "maxuint64",
    "maxuint8",
    "maxvalue",
    "min",
    "minint",
    "minint16",
    "minint32",
    "minint64",
    "minint8",
    "minvalue",
    "mod",
    "nan",
    "nextafter",
    "pi",
    "round",
    "scaleb",
    "sign",
    "signbit",
    "smallestnonzerofloat32",
    "smallestnonzerofloat64",
    "sqrt",
    "sqrt2",
    "trunc",
}

# The one class of source still allowed to reach a transcendental, and it is not a list of call
# sites. The parametric families draw and invert through log, exp, cos and pow, and the suite that
# compares them is the only one tests/manifest.json still declares `approximate`: the seven ports
# are not promised to agree on those draws bit for bit, so a platform call there breaks no promise.
#
# The exemption is therefore derived rather than written down. It reads the manifest and applies
# only while that class holds, so the day the distributions become exact — which is what porting
# inverse transform sampling does, since it removes the cosine and expresses the power through the
# toolkit's own exp and log — this guard starts refusing those files without anybody editing it.
#
# A per-site list was the alternative and is worse here. It would hold 38 entries today, all
# saying the same thing, and every one of them would have to be deleted by hand at exactly the
# moment the deletion stops being safe to forget.
DISTRIBUTION_SOURCES = {
    "cs": ("Distributions/",),
    # Go keeps the families in the package root rather than a directory, so they are named.
    "go": ("additive.go", "exp.go", "multiplic.go", "power.go", "uniform.go"),
    "kt": ("distributions/",),
    "py": ("distributions/",),
    # R has no directory either; the family files carry a prefix.
    "r": ("dist_",),
    "rs": ("distributions/",),
    "ts": ("distributions/",),
}

DISTRIBUTION_SUITE = "distributions"


def distributions_are_approximate(root):
    """Whether the manifest still declares the distribution suite approximate.

    Read rather than assumed, because this is the whole basis for the exemption below. A manifest
    that stopped declaring the suite at all is a finding, not a license: the guard would otherwise
    fall back to exempting the files for a class nobody claims any more.
    """
    manifest = json.loads((root / "tests" / "manifest.json").read_text())
    suites = manifest.get("suites", {})
    if DISTRIBUTION_SUITE not in suites:
        raise SystemExit(
            f"ERROR: tests/manifest.json declares no {DISTRIBUTION_SUITE!r} suite, so the "
            f"exemption in this file rests on nothing."
        )
    return suites[DISTRIBUTION_SUITE]["conformance"] == "approximate"


def is_distribution_source(port, rel):
    return any(rel.startswith(prefix) or rel == prefix for prefix in DISTRIBUTION_SOURCES[port])

# ── Per-language readers ──────────────────────────────────────────────────────
# Each returns (findings, exempt_hits). A finding is (relative path, line number, spelling, why).

NAMESPACED = re.compile(r"\b(?:math|Math)\.([A-Za-z][A-Za-z0-9_]*)\s*\(")
NAMESPACED_CONST = re.compile(r"\b(?:math|Math)\.([A-Za-z][A-Za-z0-9_]*)\b(?!\s*\()")

# f64's inherent methods. Only the transcendental ones are listed: this is the one language where
# the check cannot be "any call into a namespace", so the forbidden set has to be spelled out.
RS_METHODS = (
    "ln",
    "ln_1p",
    "log",
    "log2",
    "log10",
    "exp",
    "exp2",
    "exp_m1",
    "powf",
    "powi",
    "cbrt",
    "hypot",
    "sin",
    "cos",
    "tan",
    "asin",
    "acos",
    "atan",
    "atan2",
    "sinh",
    "cosh",
    "tanh",
    "asinh",
    "acosh",
    "atanh",
    "to_degrees",
    "to_radians",
)
# The exact-by-construction methods are matched by the SAME pattern, not by a second one beside it.
# Measured, and the reason this is not two regexes: with the forbidden list emptied, a version of
# this file that counted its exempt hits separately still reported success and its pinned counts
# still matched, because nothing the pin watched had changed. A guard whose pin lives on a
# different pattern from the thing it protects can go blind exactly where it matters and stay
# green. One pattern, and the classification happens after the match.
RS_EXACT_METHODS = ("sqrt", "abs", "floor", "ceil", "trunc", "round", "copysign", "signum", "recip")
RS_CALL = re.compile(r"\.(" + "|".join(RS_METHODS + RS_EXACT_METHODS) + r")\s*\(")

# Kotlin reaches these as top-level functions imported from kotlin.math, so the import list is what
# the reader parses: the toolkit has its own `log(values, subject)` and a call-site rule cannot tell
# the two apart. Any imported name that EXACT does not vouch for is a finding, which makes the set
# of forbidden names the complement of the exempt one rather than a second list to keep in step.
KT_IMPORT = re.compile(r"^\s*import\s+kotlin\.math\.([A-Za-z0-9_*]+)\s*$")
KT_QUALIFIED = re.compile(r"\bkotlin\.math\.([A-Za-z][A-Za-z0-9_]*)")
KT_POW = re.compile(r"\.pow\s*\(")

# Base R's own transcendentals, reachable by bare name because R has no namespaces. The toolkit
# spells its own `log_function`/`exp_function`, so the bare names belong to base R.
R_BARE = (
    "log",
    "log2",
    "log10",
    "log1p",
    "logb",
    "exp",
    "expm1",
    "sin",
    "cos",
    "tan",
    "asin",
    "acos",
    "atan",
    "atan2",
    "sinh",
    "cosh",
    "tanh",
    "gamma",
    "lgamma",
    "beta",
    "lbeta",
    "choose",
    "lchoose",
    "pnorm",
    "qnorm",
    "dnorm",
    "pbinom",
    "qbinom",
    "dbinom",
)
# As RS_CALL: one pattern over both classes, so emptying the forbidden list is visible in the pin.
R_EXACT_CALLS = ("sqrt", "abs", "floor", "ceiling", "trunc", "round", "sign")
R_CALL = re.compile(r"(?<![A-Za-z0-9._])(" + "|".join(R_BARE + R_EXACT_CALLS) + r")\s*\(")
# `^` is a call into pow. The base-2 form is the exception: every use here is a power of two, which
# is a scaling by an exponent rather than a computation, and R's own arithmetic dispatches an
# integral exponent to repeated multiplication. r/pragmastat/tests/testthat/test-exp-function.R
# states this and measures it rather than assuming it.
R_POWER = re.compile(r"(?<![A-Za-z0-9._)\]])(?P<base>[A-Za-z0-9._]+)\s*\^")

# `**` in TypeScript and Python, and `Math.pow(2, k)`, are the same operation under three
# spellings. The literal base 2 is the exception, in every language that has one. Scaling by a
# power of two changes only the exponent field, so no rounding decision is made and every
# implementation returns the same bits; the alternative is a hand-rolled ldexp, which is what
# ts/src/signMargin.ts and r/pragmastat/R/sign_margin.R are doing with it. It is an exception
# rather than a rule because the specifications do not spell it out: ECMA-262 calls `**`
# implementation-approximated and R's `^` goes to libm's pow, so exactness here is a property of
# the platform. Each port's exp and log function tests measure it rather than assume it.
EXPONENT = re.compile(r"(?P<base>[A-Za-z0-9._]*)\s*\*\*(?!\*)")
POW2 = re.compile(r"\b(?:math|Math)\.pow\s*\(\s*2\s*,", re.IGNORECASE)


# Comments and string literals are blanked before anything is matched. Not cosmetic: the four
# forbidden names appear in this repository's prose more often than in its code -- every port has a
# comment saying why it does not call exp -- and TypeScript's doc comments open with `/**`, which is
# the exponent operator twice. A first cut of this file reported 300 findings, all of them in
# comments.
LINE_COMMENT = {"cs": "//", "go": "//", "kt": "//", "rs": "//", "ts": "//", "py": "#", "r": "#"}
# Python has no block comment, but its docstrings span lines and hold the same prose, so the triple
# quote is treated as one: every module here opens with a docstring that names the functions this
# file forbids. Kotlin's and Go's raw strings are the same shape and are covered by the same entry.
BLOCK_COMMENT = {"cs": ("/*", "*/"), "go": ("/*", "*/"), "kt": ("/*", "*/"), "rs": ("/*", "*/"), "ts": ("/*", "*/")}
BLOCK_STRING = {"py": ('"""', '"""'), "kt": ('"""', '"""'), "r": None}


def strip(port, lines):
    """Blanks comments and string literals, keeping line numbers and column positions."""
    line_comment = LINE_COMMENT[port]
    blocks = [pair for pair in (BLOCK_COMMENT.get(port), BLOCK_STRING.get(port)) if pair]
    out = []
    open_block = None
    for raw in lines:
        chars = list(raw)
        i = 0
        quote = None
        while i < len(chars):
            rest = "".join(chars[i:])
            if open_block:
                closer = open_block[1]
                if rest.startswith(closer):
                    for j in range(i, i + len(closer)):
                        chars[j] = " "
                    i += len(closer)
                    open_block = None
                    continue
                chars[i] = " "
                i += 1
                continue
            if quote:
                if chars[i] == "\\":
                    chars[i] = " "
                    if i + 1 < len(chars):
                        chars[i + 1] = " "
                    i += 2
                    continue
                if chars[i] == quote:
                    quote = None
                chars[i] = " "
                i += 1
                continue
            opener = next((pair for pair in blocks if rest.startswith(pair[0])), None)
            if opener:
                for j in range(i, i + len(opener[0])):
                    chars[j] = " "
                i += len(opener[0])
                open_block = opener
                continue
            if rest.startswith(line_comment):
                for j in range(i, len(chars)):
                    chars[j] = " "
                break
            if chars[i] in ("'", '"', "`"):
                quote = chars[i]
                chars[i] = " "
                i += 1
                continue
            i += 1
        out.append("".join(chars))
    return out


def scan_namespaced(rel, lines, findings, exempt, allowed):
    """go, py, cs, ts: every call through the platform's math namespace."""
    for number, line in enumerate(lines, start=1):
        pow2 = POW2.search(line)
        if pow2:
            exempt.append((rel, number, "pow(2,"))
            line = line[: pow2.start()] + " " * (pow2.end() - pow2.start()) + line[pow2.end() :]  # noqa: PLW2901
        for name in NAMESPACED.findall(line):
            classify(rel, number, name, findings, exempt, allowed)
        # A named constant is not a call and carries no parentheses.
        for name in NAMESPACED_CONST.findall(line):
            if name.lower() in EXACT:
                exempt.append((rel, number, name))
        for match in EXPONENT.finditer(line):
            if match.group("base") == "2":
                exempt.append((rel, number, "2**"))
            else:
                findings.append((rel, number, "**", "the exponent operator is a call into pow"))


def classify(rel, number, name, findings, exempt, allowed):
    if name.lower() in EXACT:
        exempt.append((rel, number, name))
        return
    if any(name == allowed_name for allowed_name, _ in allowed.get(rel, ())):
        exempt.append((rel, number, name))
        return
    findings.append((rel, number, name, "reaches the platform's library"))


def scan_rs(rel, lines, findings, exempt, allowed):
    """rust: inherent f64 methods, outside `#[cfg(test)]` modules."""
    depth_of_test_module = None
    depth = 0
    pending_cfg_test = False
    for number, line in enumerate(lines, start=1):
        if depth_of_test_module is None:
            if pending_cfg_test and re.match(r"^\s*(?:pub\s+)?mod\s+", line):
                depth_of_test_module = depth
            pending_cfg_test = "#[cfg(test)]" in line
        depth += line.count("{") - line.count("}")
        if depth_of_test_module is not None:
            if depth <= depth_of_test_module:
                depth_of_test_module = None
            continue
        for name in RS_CALL.findall(line):
            classify(rel, number, name, findings, exempt, allowed)


def scan_kt(rel, lines, findings, exempt, allowed):
    """kotlin: the kotlin.math import list, plus `.pow` and any qualified call."""
    for number, line in enumerate(lines, start=1):
        wildcard = KT_IMPORT.match(line)
        if wildcard and wildcard.group(1) == "*":
            findings.append((rel, number, "import kotlin.math.*", "a wildcard import makes the import list unreadable"))
            continue
        if wildcard:
            # Anything that is neither known-exact nor known-transcendental is a finding, not a
            # third silent outcome. Shrinking EXACT then turns those imports red instead
            # of leaving the reader quietly blind, which is the same property RS_CALL's single
            # pattern buys in Rust.
            classify(rel, number, wildcard.group(1), findings, exempt, allowed)
            continue
        for name in KT_QUALIFIED.findall(line):
            classify(rel, number, name, findings, exempt, allowed)
        if KT_POW.search(line):
            classify(rel, number, "pow", findings, exempt, allowed)


def scan_r(rel, lines, findings, exempt, allowed):
    """r: base R's transcendentals by bare name, and `^` for anything but a power of two."""
    for number, line in enumerate(lines, start=1):
        for name in R_CALL.findall(line):
            classify(rel, number, name, findings, exempt, allowed)
        for match in R_POWER.finditer(line):
            if match.group("base") == "2":
                exempt.append((rel, number, "2^"))
            else:
                findings.append((rel, number, match.group("base") + "^", "the exponent operator is a call into pow"))


SCANNERS = {"cs": scan_namespaced, "go": scan_namespaced, "py": scan_namespaced, "ts": scan_namespaced}
SCANNERS.update({"rs": scan_rs, "kt": scan_kt, "r": scan_r})
# The merge above is two tables written by hand into one namespace, so a key repeated across them
# takes the second silently and that port is then read by a scanner that does not know its syntax --
# which finds nothing, and reports a clean port. Asserted rather than trusted: the pair of tables
# has to cover PORTS exactly, no more and no less.
if set(SCANNERS) != set(PORTS):
    raise SystemExit(
        f"check_libm: the two SCANNERS tables cover {sorted(SCANNERS)}, not the seven ports "
        f"{list(PORTS)}. A port with no scanner is not scanned; a port whose key appears in both "
        "tables is scanned by the second, which does not know its syntax and finds nothing."
    )

# Files scanned and exempt call sites found, per port. Pinned in both directions; the docstring
# says what each number catches.
EXPECTED_FILES = {"cs": 71, "go": 28, "kt": 30, "py": 33, "r": 42, "rs": 33, "ts": 32}
# kt's number counts `import kotlin.math.<name>` lines rather than call sites, because that is
# where the Kotlin reader looks; the other six count calls.
EXPECTED_EXACT = {"cs": 18, "go": 123, "kt": 22, "py": 51, "r": 52, "rs": 18, "ts": 85}

# Files skipped because they are a parametric family and the manifest still calls that suite
# approximate. Pinned like the others: an exemption that silently widened would otherwise look
# exactly like a port that has no distributions.
EXPECTED_DISTRIBUTION_FILES = {"cs": 7, "go": 5, "kt": 6, "py": 7, "r": 5, "rs": 7, "ts": 7}


RS_TEST_MODULE = re.compile(r"#\[cfg\(test\)\]\s*\n\s*(?:pub\s+)?mod\s+([a-z_0-9]+)\s*;")


def rs_test_modules(source_root):
    """The whole-file test modules lib.rs declares, as glob patterns to skip."""
    names = RS_TEST_MODULE.findall((source_root / "lib.rs").read_text())
    if not names:
        raise SystemExit("ERROR: rs/pragmastat/src/lib.rs declares no #[cfg(test)] module; the layout moved.")
    missing = [name for name in names if not (source_root / f"{name}.rs").is_file()]
    if missing:
        raise SystemExit(f"ERROR: lib.rs declares test modules with no file: {', '.join(sorted(missing))}")
    return tuple(f"{name}.rs" for name in names)


# A sample of each language holding one call this file must refuse and one it must allow, scanned
# through the same readers before the ports are.
#
# The pinned counts cannot cover this, and that is measured rather than assumed. Deleting `exp` from
# RS_METHODS leaves every number in this file unchanged: no Rust source calls it today, so nothing
# was being counted and nothing stops being counted. The same hole is in the Kotlin import reader and
# in R's bare-name list. It is not in the four namespaced languages, where the forbidden and the
# exempt names share one pattern -- break `math\\.` and 321 exempt hits vanish -- which is the shape
# the other three cannot have, because there the two classes are different names rather than
# different suffixes of one prefix.
#
# So the readers are given a positive sample instead. A guard whose only evidence is that it found
# nothing is indistinguishable from a guard that cannot find anything; this is the smallest input on
# which "found nothing" is a wrong answer.
SELF_TEST = {
    "cs": ("double a = Math.Exp(x);\ndouble b = Math.Sqrt(x);\n", {"Exp"}, 1),
    "go": ("a := math.Log(x)\nb := math.Sqrt(x)\n", {"Log"}, 1),
    # Kotlin's sample carries the wildcard as well, because the import reader and the qualified-name
    # reader overlap on the two ordinary lines: break the first and the second still refuses `ln`.
    # The wildcard is the one thing only the import reader sees, so it is the only line whose loss
    # would otherwise be silent.
    "kt": ("import kotlin.math.ln\nimport kotlin.math.sqrt\nimport kotlin.math.*\n", {"ln", "import kotlin.math.*"}, 1),
    "py": ("a = math.exp(x)\nb = math.sqrt(x)\n", {"exp"}, 1),
    "r": ("a <- exp(x)\nb <- sqrt(x)\n", {"exp"}, 1),
    "rs": ("let a = x.exp();\nlet b = x.sqrt();\n", {"exp"}, 1),
    "ts": ("const a = Math.exp(x);\nconst b = Math.sqrt(x);\n", {"exp"}, 1),
}


def self_test():
    """Fails when a reader has stopped recognizing the language it is responsible for."""
    broken = []
    for port, (sample, wanted, exact_wanted) in SELF_TEST.items():
        found, exempt = [], []
        SCANNERS[port]("<self-test>", strip(port, sample.splitlines()), found, exempt, {})
        names = {name for _, _, name, _ in found}
        if names != wanted:
            broken.append(f"  {port}: refused {sorted(names)} in its own sample, expected {sorted(wanted)}")
        if len(exempt) != exact_wanted:
            broken.append(f"  {port}: allowed {len(exempt)} calls in its own sample, expected {exact_wanted}")
    return broken


def main(repo_root="."):
    root = Path(repo_root)
    broken = self_test()
    if broken:
        print("ERROR: a reader no longer recognizes its own language:", file=sys.stderr)
        print("\n".join(broken), file=sys.stderr)
        print(
            "\nSELF_TEST holds one forbidden and one allowed call per language. A reader that\n"
            "cannot answer that sample reports every port clean for the same reason, and no count\n"
            "in this file can tell the two apart.",
            file=sys.stderr,
        )
        return 1
    exempt_distributions = distributions_are_approximate(root)
    findings = []
    files = {}
    exact_hits = {}
    skipped = {}
    for port in PORTS:
        relative, glob, excluded = SOURCES[port]
        source_root = root / relative
        if not source_root.is_dir():
            print(f"ERROR: {source_root} does not exist; the {port} port layout moved.", file=sys.stderr)
            return 1
        if port == "rs":
            excluded = rs_test_modules(source_root)
        found, exempt = [], []
        scanned = 0
        distribution_files = 0
        for path in sorted(source_root.rglob(glob)):
            rel = str(path.relative_to(source_root))
            if any(path.match(pattern) or rel.startswith(pattern.rstrip("*")) for pattern in excluded):
                continue
            scanned += 1
            if exempt_distributions and is_distribution_source(port, rel):
                distribution_files += 1
                continue
            SCANNERS[port](rel, strip(port, path.read_text().splitlines()), found, exempt, {})
        files[port] = scanned
        exact_hits[port] = len(exempt)
        skipped[port] = distribution_files
        findings.extend((port, *item) for item in found)

    if findings:
        print("ERROR: shipped source reaches the platform's transcendental library:", file=sys.stderr)
        for port, rel, number, name, why in sorted(findings):
            location = f"{SOURCES[port][0]}/{rel}" + (f":{number}" if number else "")
            print(f"  {location}: {name}: {why}", file=sys.stderr)
        print(
            "\nIEEE 754 fixes arithmetic and sqrt and says nothing about the transcendentals, so a\n"
            "call here is a place where two conforming platforms may return different bits and the\n"
            "seven ports stop agreeing. Use the toolkit's own function. The only source exempt\n"
            "from this is the parametric families, and only while tests/manifest.json declares\n"
            "their suite approximate: that exemption is read, not written down here.",
            file=sys.stderr,
        )
        return 1

    drift = []
    for port in PORTS:
        if files[port] != EXPECTED_FILES[port]:
            drift.append(f"  {port}: scanned {files[port]} files, expected {EXPECTED_FILES[port]}")
        if skipped[port] != EXPECTED_DISTRIBUTION_FILES[port]:
            drift.append(
                f"  {port}: skipped {skipped[port]} distribution files, expected "
                f"{EXPECTED_DISTRIBUTION_FILES[port]}"
            )
        if exact_hits[port] != EXPECTED_EXACT[port]:
            drift.append(
                f"  {port}: found {exact_hits[port]} exact-by-construction call sites, expected {EXPECTED_EXACT[port]}"
            )
    if drift:
        print("ERROR: the scan is not the size this file records:", file=sys.stderr)
        print("\n".join(drift), file=sys.stderr)
        print(
            "\nA scanner that has stopped matching one language finds no forbidden call and no\n"
            "sqrt either, and only the second number tells that apart from a clean port. Update\n"
            "the counts in this file if the change was deliberate.",
            file=sys.stderr,
        )
        return 1

    total_files = sum(files.values())
    total_exact = sum(exact_hits.values())
    print(
        f"no shipped source in the seven ports reaches a transcendental the specification leaves "
        f"unfixed ({total_files} files read, {total_exact} exact-by-construction call sites seen)"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1] if len(sys.argv) > 1 else "."))
