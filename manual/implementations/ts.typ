#import "/manual/definitions.typ": *

// Uses definitions from parent scope (imported in main document)
#let lang = languages.ts

== #lang.title

Install from npm:

#raw("npm i pragmastat@" + version, lang: "bash", block: true)

Source code: #link(github-tree + "/ts")

#lang.package

Demo:

#raw(read(lang.demo), lang: lang.code, block: true)
