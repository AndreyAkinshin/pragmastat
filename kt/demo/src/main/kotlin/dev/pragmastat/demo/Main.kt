package dev.pragmastat.demo

import dev.pragmastat.*
import dev.pragmastat.distributions.*

fun main() {
    // --- One-Sample ---

    val x = Sample.of((1..200).map { it.toDouble() })
    println(center(x)) // 100.5
    println(spread(x)) // 59
    println(centerBounds(x, Probability(1e-3))) // [86;115]
    println(spreadBounds(x, Probability(1e-3), "demo")) // [44;87]

    // --- Two-Sample ---

    val sx = Sample.of((1..200).map { it.toDouble() })
    val sy = Sample.of((101..300).map { it.toDouble() })
    println(shift(sx, sy)) // -100
    println(shiftBounds(sx, sy, Probability(1e-3))) // [-120;-80]
    println(ratio(sx, sy)) // 0.5008354224706334
    println(ratioBounds(sx, sy, Probability(1e-3))) // [0.4066666666666668;0.5958333333333332]
    println(disparity(sx, sy)) // -1.694915254237288
    println(disparityBounds(sx, sy, Probability(1e-3), "demo")) // [-3.1025641025641026;-0.8494623655913979]

    // --- Confirmatory Comparison ---

    println(compare1(sx, listOf(Threshold(Metric.CENTER, Measurement(50.0), Probability(1e-3))))[0].verdict.toId()) // greater
    println(compare2(sx, sy, listOf(Threshold(Metric.SHIFT, Measurement(0.0), Probability(1e-3))))[0].verdict.toId()) // less

    // --- Custom units ---

    val ns = MeasurementUnit("ns", "Time", "ns", "Nanosecond", 1)
    val us = MeasurementUnit("us", "Time", "us", "Microsecond", 1000)
    val ms = MeasurementUnit("ms", "Time", "ms", "Millisecond", 1_000_000)

    val registry = UnitRegistry.standard()
    registry.register(ns)
    registry.register(us)
    registry.register(ms)

    val timeSample = Sample.of(listOf(100.0, 200.0, 300.0, 400.0, 500.0), ns)
    println(center(timeSample)) // 300 ns
    println(center(timeSample.convertTo(us))) // 0.3 us

    // --- Randomization ---

    var rng = Rng("demo-uniform")
    println(rng.uniformDouble()) // 0.2640554428629759
    println(rng.uniformDouble()) // 0.9348534835582796

    rng = Rng("demo-uniform-int")
    println(rng.uniformInt(0, 100)) // 41

    rng = Rng("demo-sample")
    println(rng.sample(listOf(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0), 3)) // [3.0, 8.0, 9.0]

    rng = Rng("demo-resample")
    println(rng.resample(listOf(1.0, 2.0, 3.0, 4.0, 5.0), 7)) // [3.0, 1.0, 3.0, 2.0, 4.0, 1.0, 2.0]

    rng = Rng("demo-shuffle")
    println(rng.shuffle(listOf(1.0, 2.0, 3.0, 4.0, 5.0))) // [4.0, 2.0, 3.0, 5.0, 1.0]

    // --- Distributions ---

    rng = Rng("demo-dist-additive")
    println(Additive(0.0, 1.0).sample(rng)) // 0.17410448679568188

    rng = Rng("demo-dist-multiplic")
    println(Multiplic(0.0, 1.0).sample(rng)) // 1.1273244602673853

    rng = Rng("demo-dist-exp")
    println(Exp(1.0).sample(rng)) // 0.6589065267276553

    rng = Rng("demo-dist-power")
    println(Power(1.0, 2.0).sample(rng)) // 1.023677535537084

    rng = Rng("demo-dist-uniform")
    println(Uniform(0.0, 10.0).sample(rng)) // 6.54043657816832
}
