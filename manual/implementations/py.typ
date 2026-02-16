#import "/manual/definitions.typ": *

// Uses definitions from parent scope (imported in main document)
#let lang = languages.py

Install from PyPI:

#raw("pip install pragmastat==" + version, lang: "bash", block: true)

Source code: #link(github-tree + "/py")

#lang.package

Demo:

#raw(read(lang.demo), lang: lang.code, block: true)
