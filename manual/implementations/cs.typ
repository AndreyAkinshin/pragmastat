#import "/manual/definitions.typ": *

// Uses definitions from parent scope (imported in main document)
#let lang = languages.cs

// Escape # in C# for Typst heading
== C\#

Install from NuGet via .NET CLI:

#raw("dotnet add package Pragmastat --version " + version, lang: "bash", block: true)

Install from NuGet via Package Manager Console:

#raw("NuGet\\Install-Package Pragmastat -Version " + version, lang: "ps1", block: true)

Source code: #link(github-tree + "/cs")

#lang.package

Demo:

#raw(read(lang.demo), lang: lang.code, block: true)
