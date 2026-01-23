#import "/manual/definitions.typ": *

// Uses definitions from parent scope (imported in main document)
#let lang = languages.r

== #lang.title

Install from GitHub:

#raw("install.packages(\"remotes\") # If 'remotes' is not installed\nremotes::install_github(\"AndreyAkinshin/pragmastat\",\n                        subdir = \"r/pragmastat\", ref = \"v" + version + "\")\nlibrary(pragmastat)", lang: "r", block: true)

Source code: #link(github-tree + "/r")

Demo:

#raw(read(lang.demo), lang: lang.code, block: true)
