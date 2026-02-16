#import "@preview/basic-report:0.4.0": *
#import "definitions.typ": *

#let doi = "10.5281/zenodo.17236778"
#let email = "andrey.akinshin@gmail.com"
#let accent-color = rgb("#008B8B")

#show: it => basic-report(
  doc-category: [Technical Manual],
  doc-title: "Pragmastat: Pragmatic Statistical Toolkit",
  author: "Andrey Akinshin ∙ " + email,
  affiliation: [Version #version ∙ #link("https://doi.org/" + doi)[DOI: #doi]],
  language: "en",
  heading-color: accent-color,
  show-outline: false,
  it
)

// Page styling: wider content area
#set page(margin: (x: 2cm, y: 2.5cm))

// Center all tables, keep them on a single page, use smaller font
#show table: it => align(center, block(breakable: false, text(size: 0.85em, it)))

// Each chapter starts on a new page
#show heading.where(level: 1): it => {
  pagebreak(weak: true)
  it
}

// Render citations without parentheses: "Hodges & Lehmann, 1963" instead of "(Hodges & Lehmann, 1963)"
#set cite(form: "prose")

// Make citations clickable links to the bibliography section
#show cite: it => link(<bibliography>)[#it]

// Lists: no bullet markers, indented like standard definition list continuation
#set list(marker: none, indent: 0em, body-indent: 2em)

// Abstract (no title in PDF - web has separate abstract page)
#include "abstract.typ"

// Artifacts table
#let release-base = github-repo + "/releases/download/v" + version + "/"
#let a(name) = link(release-base + name)[#name]
#v(1.5em)
#table(
  columns: (auto, 1fr),
  stroke: none,
  row-gutter: 0.3em,
  column-gutter: 1.5em,
  align: (right, left),
  [*Documentation*], a("pragmastat-v" + version + ".pdf"),
  [], a("web-v" + version + ".zip"),
  [*Implementations*], a("py-v" + version + ".zip"),
  [], a("ts-v" + version + ".zip"),
  [], a("r-v" + version + ".zip"),
  [], a("cs-v" + version + ".zip"),
  [], a("kt-v" + version + ".zip"),
  [], a("rs-v" + version + ".zip"),
  [], a("go-v" + version + ".zip"),
  [*Reference data*], a("tests-v" + version + ".zip"),
  [], a("sim-v" + version + ".zip"),
  [*Source code*], link(github-repo + "/archive/refs/tags/v" + version + ".zip")[pragmastat-#(version)\.zip],
)

// Colophon (before TOC in PDF; in web version it goes into Appendix)
#v(1.5em)
#include "colophon/colophon.typ"

// Table of Contents
#pagebreak()
#text(size: 1.4em, weight: "bold", fill: accent-color)[Contents]
#v(1em)
#outline(title: none, indent: 1.5em, depth: 2)

// Synopsis (unnumbered in PDF, starts on new page)
#pagebreak()
#{
  set heading(numbering: none)
  include "synopsis/synopsis.typ"
}

#include "main.typ"

// Appendix (in web version, these are separate pages grouped under "Appendix")
= Appendix

#{
  set heading(offset: 1)
  include "assumptions/assumptions.typ"
  include "foundations/foundations.typ"
  include "methodology/methodology.typ"
}

// Bibliography (Hayagriva YAML format)
#{
  set heading(offset: 1)
  bibliography("references.yaml", style: "apa")
} <bibliography>
