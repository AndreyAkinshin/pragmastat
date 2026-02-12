import {
    center, spread, relSpread, shift, ratio, avgSpread, disparity,
    centerBounds, shiftBounds, ratioBounds,
    Rng, Uniform, Additive, Exp, Power, Multiplic
} from '../src';

function main() {
    // --- Randomization ---

    let rng = new Rng("demo-uniform");
    console.log(rng.uniform()); // 0.2640554428629759
    console.log(rng.uniform()); // 0.9348534835582796

    rng = new Rng("demo-sample");
    console.log(rng.sample([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3)); // [3, 8, 9]

    rng = new Rng("demo-shuffle");
    console.log(rng.shuffle([1, 2, 3, 4, 5])); // [4, 2, 3, 5, 1]

    rng = new Rng("demo-resample");
    console.log(rng.resample([1, 2, 3, 4, 5], 7)); // [5, 1, 1, 3, 3, 4, 5]

    // --- Distribution Sampling ---

    rng = new Rng("demo-dist-uniform");
    let dist = new Uniform(0, 10);
    console.log(dist.sample(rng)); // 6.54043657816832

    rng = new Rng("demo-dist-additive");
    let addDist = new Additive(0, 1);
    console.log(addDist.sample(rng)); // 0.17410448679568188

    rng = new Rng("demo-dist-exp");
    let expDist = new Exp(1);
    console.log(expDist.sample(rng)); // 0.6589065267276553

    rng = new Rng("demo-dist-power");
    let powDist = new Power(1, 2);
    console.log(powDist.sample(rng)); // 1.023677535537084

    rng = new Rng("demo-dist-multiplic");
    let mulDist = new Multiplic(0, 1);
    console.log(mulDist.sample(rng)); // 1.1273244602673853

    // --- Single-Sample Statistics ---

    let x = [1, 3, 5, 7, 9];

    console.log(center(x)); // 5
    console.log(spread(x)); // 4
    console.log(spread(x.map(v => v + 10))); // 4
    console.log(spread(x.map(v => v * 2))); // 8
    console.log(relSpread(x)); // 0.8

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

    // --- One-Sample Bounds ---

    x = Array.from({ length: 10 }, (_, i) => i + 1);

    console.log(center(x)); // 5.5
    console.log(centerBounds(x, 0.05)); // { lower: 3.5, upper: 7.5 }

    // --- Two-Sample Bounds ---

    x = Array.from({ length: 30 }, (_, i) => i + 1);
    y = Array.from({ length: 30 }, (_, i) => i + 21);

    console.log(shift(x, y)); // -20
    console.log(shiftBounds(x, y, 1e-4)); // { lower: -30, upper: -10 }

    x = [1, 2, 3, 4, 5];
    y = [2, 3, 4, 5, 6];
    console.log(ratioBounds(x, y, 0.05)); // { lower: 0.333..., upper: 1.5 }
}

main();
