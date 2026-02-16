#import "/manual/definitions.typ": *

The $Resample$ function selects $k$ elements from a sample of size $n$ with replacement.

The algorithm generates $k$ independent uniform random integers in $[0, n)$
  using the #link(<sec-rng>)[Rng] generator, and collects the corresponding elements:

#block(inset: (left: 1em))[
```
result = new array of size k
for i from 0 to k-1:
    j = uniform_int(0, n)
    result[i] = x[j]
```
]

Each selection is independent with equal probability $1\/n$ for every element.
The same element may appear multiple times in the output.
Time complexity is $O(k)$ with $O(k)$ additional space for the result array.
