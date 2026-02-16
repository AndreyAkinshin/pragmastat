#import "/manual/definitions.typ": *

The $Shuffle$ function uses the Fisher-Yates algorithm (see @fisher1938, @knuth1997),
  also known as the Knuth shuffle,
  with the #link(<sec-rng>)[Rng] generator for random decisions:

#block(inset: (left: 1em))[
```
for i from n-1 down to 1:
    j = uniform_int(0, i+1)
    swap(array[i], array[j])
```
]

This produces a uniformly random permutation in $O(n)$ time with $O(n)$ additional space (the input is copied).
The algorithm is unbiased: each of the $n!$ permutations has equal probability.
