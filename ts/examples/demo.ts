import {
  center,
  spread,
  shift,
  ratio,
  disparity,
  centerBounds,
  shiftBounds,
  ratioBounds,
  spreadBounds,
  disparityBounds,
  Rng,
  Additive,
  Multiplic,
  Exp,
  Power,
  Uniform,
  Sample,
  Measurement,
  Metric,
  Threshold,
  compare1,
  compare2,
} from '..';

function main(): void {
  // --- One-Sample ---

  let sx = Sample.of(Array.from({ length: 200 }, (_, i) => i + 1));

  console.log(center(sx).value); // 100.5
  console.log(centerBounds(sx, 1e-3).toString()); // [86;115]
  console.log(spread(sx).value); // 59
  console.log(spreadBounds(sx, 1e-3, 'demo').toString()); // [44;87]

  // --- Two-Sample ---

  sx = Sample.of(Array.from({ length: 200 }, (_, i) => i + 1));
  const sy = Sample.of(Array.from({ length: 200 }, (_, i) => i + 101));

  console.log(shift(sx, sy).value); // -100
  console.log(shiftBounds(sx, sy, 1e-3).toString()); // [-120;-80]
  console.log(ratio(sx, sy).value); // 0.5008354224706336
  console.log(ratioBounds(sx, sy, 1e-3).toString()); // [0.4066666666666668;0.5958333333333332]
  console.log(disparity(sx, sy).value); // -1.694915254237288
  console.log(disparityBounds(sx, sy, 1e-3, 'demo').toString()); // [-3.1025641025641026;-0.8494623655913979]

  // --- Confirmatory Comparison ---

  console.log(compare1(sx, [new Threshold(Metric.CENTER, new Measurement(50), 1e-3)])[0].verdict); // greater
  console.log(compare2(sx, sy, [new Threshold(Metric.SHIFT, new Measurement(0), 1e-3)])[0].verdict); // less

  // --- Randomization ---

  let rng = new Rng('demo-uniform');
  console.log(rng.uniformFloat()); // 0.2640554428629759
  console.log(rng.uniformFloat()); // 0.9348534835582796

  rng = new Rng('demo-uniform-int');
  console.log(rng.uniformInt(0, 100)); // 41

  rng = new Rng('demo-sample');
  console.log(JSON.stringify(rng.sample([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3))); // [3,8,9]

  rng = new Rng('demo-resample');
  console.log(JSON.stringify(rng.resample([1, 2, 3, 4, 5], 7))); // [3,1,3,2,4,1,2]

  rng = new Rng('demo-shuffle');
  console.log(JSON.stringify(rng.shuffle([1, 2, 3, 4, 5]))); // [4,2,3,5,1]

  // --- Distributions ---

  rng = new Rng('demo-dist-additive');
  console.log(new Additive(0, 1).sample(rng)); // 0.17410448679568188

  rng = new Rng('demo-dist-multiplic');
  console.log(new Multiplic(0, 1).sample(rng)); // 1.1273244602673853

  rng = new Rng('demo-dist-exp');
  console.log(new Exp(1).sample(rng)); // 0.6589065267276553

  rng = new Rng('demo-dist-power');
  console.log(new Power(1, 2).sample(rng)); // 1.023677535537084

  rng = new Rng('demo-dist-uniform');
  console.log(new Uniform(0, 10).sample(rng)); // 6.54043657816832
}

main();
