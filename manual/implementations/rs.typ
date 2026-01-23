#import "/manual/definitions.typ": *

// Uses definitions from parent scope (imported in main document)
#let lang = languages.rs

== #lang.title

Install from crates.io via cargo:

#raw("cargo add pragmastat@" + version, lang: "bash", block: true)

Install from crates.io via `Cargo.toml`:

#raw("[dependencies]\npragmastat = \"" + version + "\"", lang: "toml", block: true)

Source code: #link(github-tree + "/rs")

#lang.package

Demo:

#raw(read(lang.demo), lang: lang.code, block: true)
