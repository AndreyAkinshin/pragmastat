#import "/manual/definitions.typ": *

// Uses definitions from parent scope (imported in main document)
#let lang = languages.kt

== #lang.title

Install from Maven Central Repository via Apache Maven:

#raw("<dependency>\n    <groupId>dev.pragmastat</groupId>\n    <artifactId>pragmastat</artifactId>\n    <version>" + version + "</version>\n</dependency>", lang: "xml", block: true)

Install from Maven Central Repository via Gradle:

#raw("implementation 'dev.pragmastat:pragmastat:" + version + "'", lang: "java", block: true)

Install from Maven Central Repository via Gradle (Kotlin):

#raw("implementation(\"dev.pragmastat:pragmastat:" + version + "\")", lang: "kotlin", block: true)

Source code: #link(github-tree + "/kt")

#lang.package

Demo:

#raw(read(lang.demo), lang: lang.code, block: true)
