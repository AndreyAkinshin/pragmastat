#import "/manual/definitions.typ": *

// Uses definitions from parent scope (imported in main document)
#let lang = languages.go

Install from GitHub:

#raw("go get github.com/AndreyAkinshin/pragmastat/go/v" + major + "@v" + version, lang: "bash", block: true)

Source code: #link(github-tree + "/go")

Demo:

#raw(read(lang.demo), lang: lang.code, block: true)
