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
  [], a("pragmastat-v" + version + ".md"),
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

// Colophon at bottom of page (no title in PDF - web has separate colophon page)
#v(1fr)
#text(size: 0.9em)[
  #set par(justify: true)

  Andrey Akinshin \
  #link("mailto:" + email)[#email] \
  #link("https://doi.org/" + doi)[DOI: #doi]

  #v(0.8em)
  *Copyright © 2025–2026 Andrey Akinshin*

  #v(0.5em)
  This manual is licensed under the *Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License* (#link("https://creativecommons.org/licenses/by-nc-sa/4.0/")[CC BY-NC-SA 4.0]).
  You are free to share and adapt this material for non-commercial purposes, provided you give appropriate credit, indicate if changes were made, and distribute your contributions under the same license.

  #v(0.5em)
  The accompanying source code and software implementations are licensed under the *MIT License*.
  You are free to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the software, subject to the conditions stated in the license.
  For complete license terms, see the LICENSE file in the source repository.

  #v(0.5em)
  While the information in this manual is believed to be accurate at the date of publication, the author makes no warranty, express or implied, with respect to the material contained herein.
  The author shall not be liable for any errors, omissions, or damages arising from the use of this information.

  #v(0.5em)
  Source code and implementations are available at #link("https://github.com/AndreyAkinshin/pragmastat")[github.com/AndreyAkinshin/pragmastat].

  #v(0.5em)
  Typeset with #link("https://typst.app")[Typst].
  Text refined with LLM assistance.
]

// Table of Contents
#pagebreak()
#text(size: 1.4em, weight: "bold", fill: accent-color)[Contents]
#v(1em)
#outline(title: none, indent: 1.5em)

#include "main.typ"

// Bibliography (Hayagriva YAML format)
#bibliography("references.yaml", style: "apa") <bibliography>
