# Pragmastat

This is a TypeScript implementation of 'Pragmastat: Pragmatic Statistical Toolkit', which presents a toolkit of statistical procedures that provide reliable results across diverse real-world distributions, with ready-to-use implementations and detailed explanations.

- PDF manual for this version: [pragmastat-v4.0.1.pdf](https://github.com/AndreyAkinshin/pragmastat/releases/download/v4.0.1/pragmastat-v4.0.1.pdf)
- Markdown manual for this version: [pragmastat-v4.0.1.md](https://github.com/AndreyAkinshin/pragmastat/releases/download/v4.0.1/pragmastat-v4.0.1.md)
- Source code for this version: [pragmastat/ts/v4.0.1](https://github.com/AndreyAkinshin/pragmastat/tree/v4.0.1/ts)
- Latest online manual: https://pragmastat.dev
- Manual DOI: [10.5281/zenodo.17236778](https://doi.org/10.5281/zenodo.17236778)

## Installation

Install from npm:

```bash
npm i pragmastat@4.0.1
```

## Demo

```typescript
import { center, spread, relSpread, shift, ratio, avgSpread, disparity, shiftBounds, pairwiseMargin } from '../src';

function main() {
    let x = [0, 2, 4, 6, 8];
    console.log(center(x)); // 4
    console.log(center(x.map(v => v + 10))); // 14
    console.log(center(x.map(v => v * 3))); // 12

    console.log(spread(x)); // 4
    console.log(spread(x.map(v => v + 10))); // 4
    console.log(spread(x.map(v => v * 2))); // 8

    console.log(relSpread(x)); // 1
    console.log(relSpread(x.map(v => v * 5))); // 1

    let y = [10, 12, 14, 16, 18];
    console.log(shift(x, y)); // -10
    console.log(shift(x, x)); // 0
    console.log(shift(x.map(v => v + 7), y.map(v => v + 3))); // -6
    console.log(shift(x.map(v => v * 2), y.map(v => v * 2))); // -20
    console.log(shift(y, x)); // 10

    x = [1, 2, 4, 8, 16];
    y = [2, 4, 8, 16, 32];
    console.log(ratio(x, y)); // 0.5
    console.log(ratio(x, x)); // 1
    console.log(ratio(x.map(v => v * 2), y.map(v => v * 5))); // 0.2

    x = [0, 3, 6, 9, 12];
    y = [0, 2, 4, 6, 8];
    console.log(spread(x)); // 6
    console.log(spread(y)); // 4

    console.log(avgSpread(x, y)); // 5
    console.log(avgSpread(x, x)); // 6
    console.log(avgSpread(x.map(v => v * 2), x.map(v => v * 3))); // 15
    console.log(avgSpread(y, x)); // 5
    console.log(avgSpread(x.map(v => v * 2), y.map(v => v * 2))); // 10

    console.log(shift(x, y)); // 2
    console.log(avgSpread(x, y)); // 5

    console.log(disparity(x, y)); // 0.4
    console.log(disparity(x.map(v => v + 5), y.map(v => v + 5))); // 0.4
    console.log(disparity(x.map(v => v * 2), y.map(v => v * 2))); // 0.4
    console.log(disparity(y, x)); // -0.4

    x = Array.from({ length: 30 }, (_, i) => i + 1);
    y = Array.from({ length: 30 }, (_, i) => i + 21);

    console.log(pairwiseMargin(30, 30, 1e-6)); // 276
    console.log(pairwiseMargin(30, 30, 1e-5)); // 328
    console.log(pairwiseMargin(30, 30, 1e-4)); // 390
    console.log(pairwiseMargin(30, 30, 1e-3)); // 464

    console.log(shift(x, y)); // -20

    console.log(shiftBounds(x, y, 1e-6)); // [-33, -7]
    console.log(shiftBounds(x, y, 1e-5)); // [-32, -8]
    console.log(shiftBounds(x, y, 1e-4)); // [-30, -10]
    console.log(shiftBounds(x, y, 1e-3)); // [-28, -12]
}

main();
```

## The MIT License

Copyright (c) 2025 Andrey Akinshin

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
