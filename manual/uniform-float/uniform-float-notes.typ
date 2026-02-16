#import "/manual/definitions.typ": *

Most programming languages expose the primary PRNG operation as `.Next()`, `.random()`, or `rand()` —
  a method name that describes the *mechanism* (advance the internal state and return a value)
  rather than the *result* (a uniformly distributed number on $[0, 1)$).

This toolkit names the operations $UniformFloat$ and $UniformInt$ — combining the distribution name
  with the return type.
The reasons are both pedagogical and practical.
In actual code the method names are language-specific and type-suffixed; see the
  #link(<sec-uniform-float>)[UniformFloat] and #link(<sec-uniform-int>)[UniformInt] pages for the mapping.

*The name communicates the contract.*
Calling $r.UniformFloat()$ immediately tells the reader what distribution the returned value follows
  and what type it produces.
Calling `r.Next()` says only that something comes next;
  the distribution, range, and precision are left to documentation.
In a library that manipulates multiple distributions ($Additive$, $Multiplic$, $Exp$, $Power$, $Uniform$),
  naming the uniform draw explicitly makes it a peer of the other distributions
  rather than a special primitive hidden behind a generic verb.

*The name prevents a category error.*
When `random()` returns a value in $[0, 1)$, users sometimes treat it as "a random number"
  without recognizing that it samples from a specific distribution.
Making the distribution explicit in the name reinforces that $Uniform$ is one choice among many
  and that other distributions require different transformations.

*The name preserves the URL namespace.*
Using $UniformFloat$ for the function frees `/uniform` for the $Uniform$ distribution page,
  avoiding ambiguity between the function (which draws a single value)
  and the distribution family (which defines the parametric model).

*Composition becomes self-documenting.*
When a distribution is built from uniform draws, the code reads naturally as
  "take $UniformFloat$ draws and apply a transformation," keeping the distribution explicit.
Replacing $UniformFloat$ with `.Next()` in these descriptions obscures the mathematical structure.

*Precedent.*
Scientific computing libraries (NumPy's `random.uniform`, R's `runif`, Julia's `rand(Uniform())`)
  already use "uniform" when the distribution matters.
The toolkit follows this convention consistently:
  every random draw is named after its distribution, starting with the simplest one.
