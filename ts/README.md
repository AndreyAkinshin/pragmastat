# TypeScript

Install from npm:

```bash
npm i pragmastat@5.2.1
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v5.2.1/ts

Pragmastat on npm: https://www.npmjs.com/package/pragmastat

## Demo

```typescript
import {
    median, center, spread, relSpread, shift, ratio, avgSpread, disparity, shiftBounds, ratioBounds, pairwiseMargin,
    Rng, Uniform, Additive, Exp, Power, Multiplic
} from '../src';

function main() {
    // --- Randomization ---

    let rng = new Rng(1729);
    console.log(rng.uniform()); // 0.3943034703296536
    console.log(rng.uniform()); // 0.5730893757071377

    rng = new Rng("experiment-1");
    console.log(rng.uniform()); // 0.9535207726895857

    rng = new Rng(1729);
    console.log(rng.sample([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3)); // [6, 8, 9]

    rng = new Rng(1729);
    console.log(rng.shuffle([1, 2, 3, 4, 5])); // [4, 2, 3, 5, 1]

    // --- Distribution Sampling ---

    rng = new Rng(1729);
    let dist = new Uniform(0, 10);
    console.log(dist.sample(rng)); // 3.9430347032965365

    rng = new Rng(1729);
    let addDist = new Additive(0, 1);
    console.log(addDist.sample(rng)); // -1.222932972163442

    rng = new Rng(1729);
    let expDist = new Exp(1);
    console.log(expDist.sample(rng)); // 0.5013761944646019

    rng = new Rng(1729);
    let powDist = new Power(1, 2);
    console.log(powDist.sample(rng)); // 1.284909255071668

    rng = new Rng(1729);
    let mulDist = new Multiplic(0, 1);
    console.log(mulDist.sample(rng)); // 0.2943655336550937

    // --- Single-Sample Statistics ---

    let x = [0, 2, 4, 6, 8];

    console.log(median(x)); // 4
    console.log(center(x)); // 4
    console.log(spread(x)); // 4
    console.log(spread(x.map(v => v + 10))); // 4
    console.log(spread(x.map(v => v * 2))); // 8
    console.log(relSpread(x)); // 1

    // --- Two-Sample Comparison ---

    x = [0, 3, 6, 9, 12];
    let y = [0, 2, 4, 6, 8];

    console.log(shift(x, y)); // 2
    console.log(shift(y, x)); // -2
    console.log(avgSpread(x, y)); // 5
    console.log(disparity(x, y)); // 0.4
    console.log(disparity(y, x)); // -0.4

    x = [1, 2, 4, 8, 16];
    y = [2, 4, 8, 16, 32];
    console.log(ratio(x, y)); // 0.5
    console.log(ratio(y, x)); // 2

    // --- Confidence Bounds ---

    x = Array.from({ length: 30 }, (_, i) => i + 1);
    y = Array.from({ length: 30 }, (_, i) => i + 21);

    console.log(pairwiseMargin(30, 30, 1e-4)); // 390
    console.log(shift(x, y)); // -20
    console.log(shiftBounds(x, y, 1e-4)); // { lower: -30, upper: -10 }

    x = [1, 2, 3, 4, 5];
    y = [2, 3, 4, 5, 6];
    console.log(ratioBounds(x, y, 0.05)); // { lower: 0.333..., upper: 1.5 }
}

main();
```
