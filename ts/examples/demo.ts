import {
    center, spread, shift, ratio, disparity,
    centerBounds, shiftBounds, ratioBounds,
    spreadBounds, disparityBounds,
    Rng, Additive, Multiplic, Exp, Power, Uniform
} from '../src';

function main() {
    // --- One-Sample ---

    let x = Array.from({ length: 22 }, (_, i) => i + 1);

    console.log(center(x));             // 11.5
    console.log(centerBounds(x, 1e-3)); // { lower: 6, upper: 17 }
    console.log(spread(x));             // 7
    console.log(spreadBounds(x, 1e-3, "demo")); // { lower: 1, upper: 18 }

    // --- Two-Sample ---

    x = Array.from({ length: 30 }, (_, i) => i + 1);
    let y = Array.from({ length: 30 }, (_, i) => i + 21);

    console.log(shift(x, y));             // -20
    console.log(shiftBounds(x, y, 1e-3)); // { lower: -28, upper: -12 }
    console.log(ratio(x, y));             // 0.4366979828269513
    console.log(ratioBounds(x, y, 1e-3)); // { lower: 0.23255813953488377, upper: 0.6428571428571428 }
    console.log(disparity(x, y));         // -2.2222222222222223
    console.log(disparityBounds(x, y, 1e-3, "demo")); // { lower: -29, upper: -0.4782608695652174 }

    // --- Randomization ---

    let rng = new Rng("demo-uniform");
    console.log(rng.uniformFloat()); // 0.2640554428629759
    console.log(rng.uniformFloat()); // 0.9348534835582796

    rng = new Rng("demo-uniform-int");
    console.log(rng.uniformInt(0, 100)); // 41

    rng = new Rng("demo-sample");
    console.log(JSON.stringify(rng.sample([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3))); // [3,8,9]

    rng = new Rng("demo-resample");
    console.log(JSON.stringify(rng.resample([1, 2, 3, 4, 5], 7))); // [3,1,3,2,4,1,2]

    rng = new Rng("demo-shuffle");
    console.log(JSON.stringify(rng.shuffle([1, 2, 3, 4, 5]))); // [4,2,3,5,1]

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
