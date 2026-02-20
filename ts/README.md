# TypeScript

Install from npm:

```bash
npm i pragmastat@10.0.4
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v10.0.4/ts

Pragmastat on npm: https://www.npmjs.com/package/pragmastat

## Demo

```typescript
import {
    center, spread, shift, ratio, disparity,
    centerBounds, shiftBounds, ratioBounds,
    spreadBounds, disparityBounds,
    Rng, Additive, Multiplic, Exp, Power, Uniform
} from 'pragmastat';

function main() {
    // --- One-Sample ---

    let x = Array.from({ length: 20 }, (_, i) => i + 1);

    console.log(center(x));             // 10.5
    console.log(centerBounds(x, 0.05)); // { lower: 7.5, upper: 13.5 }
    console.log(spread(x));             // 6
    console.log(spreadBounds(x, 0.05, "demo")); // { lower: 2, upper: 10 }

    // --- Two-Sample ---

    x = Array.from({ length: 30 }, (_, i) => i + 1);
    let y = Array.from({ length: 30 }, (_, i) => i + 21);

    console.log(shift(x, y));             // -20
    console.log(shiftBounds(x, y, 0.05)); // { lower: -25, upper: -15 }
    console.log(ratio(x, y));             // 0.4366979828269513
    console.log(ratioBounds(x, y, 0.05)); // { lower: 0.31250000000000006, upper: 0.5600000000000003 }
    console.log(disparity(x, y));         // -2.2222222222222223
    console.log(disparityBounds(x, y, 0.05, "demo")); // { lower: -13, upper: -0.8235294117647058 }

    // --- Randomization ---

    let rng = new Rng("demo-uniform");
    console.log(rng.uniformFloat()); // 0.2640554428629759
    console.log(rng.uniformFloat()); // 0.9348534835582796

    rng = new Rng("demo-uniform-int");
    console.log(rng.uniformInt(0, 100)); // 41

    rng = new Rng("demo-sample");
    console.log(rng.sample([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3)); // [3, 8, 9]

    rng = new Rng("demo-resample");
    console.log(rng.resample([1, 2, 3, 4, 5], 7)); // [3, 1, 3, 2, 4, 1, 2]

    rng = new Rng("demo-shuffle");
    console.log(rng.shuffle([1, 2, 3, 4, 5])); // [4, 2, 3, 5, 1]

    // --- Distributions ---

    rng = new Rng("demo-dist-additive");
    console.log(new Additive(0, 1).sample(rng)); // 0.17410448679568188

    rng = new Rng("demo-dist-multiplic");
    console.log(new Multiplic(0, 1).sample(rng)); // 1.1273244602673853

    rng = new Rng("demo-dist-exp");
    console.log(new Exp(1).sample(rng)); // 0.6589065267276553

    rng = new Rng("demo-dist-power");
    console.log(new Power(1, 2).sample(rng)); // 1.023677535537084

    rng = new Rng("demo-dist-uniform");
    console.log(new Uniform(0, 10).sample(rng)); // 6.54043657816832
}

main();
```
