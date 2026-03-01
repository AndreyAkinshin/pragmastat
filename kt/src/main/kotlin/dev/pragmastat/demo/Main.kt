package dev.pragmastat.demo

import dev.pragmastat.*
import dev.pragmastat.distributions.*

fun main() {
    // --- One-Sample (free functions) ---

    var xv = (1..22).map { it.toDouble() }

    println(center(xv)) // 11.5
    println(centerBounds(xv, 1e-3)) // Bounds(lower=6.0, upper=17.0)
    println(spread(xv)) // 7.0
    println(spreadBounds(xv, 1e-3, "demo")) // Bounds(lower=1.0, upper=18.0)

    // --- Two-Sample (free functions) ---

    xv = (1..30).map { it.toDouble() }
    var yv = (21..50).map { it.toDouble() }

    println(shift(xv, yv)) // -20.0
    println(shiftBounds(xv, yv, 1e-3)) // Bounds(lower=-28.0, upper=-12.0)
    println(ratio(xv, yv)) // 0.43669798282695127
    println(ratioBounds(xv, yv, 1e-3)) // Bounds(lower=0.23255813953488377, upper=0.6428571428571428)
    println(disparity(xv, yv)) // -2.2222222222222223
    println(disparityBounds(xv, yv, 1e-3, "demo")) // Bounds(lower=-29.0, upper=-0.4782608695652174)

    // --- Sample-based API ---

    val x = Sample.of((1..22).map { it.toDouble() })
    println(center(x)) // Measurement(value=11.5, unit=NumberUnit)
    println(spread(x)) // Measurement(value=7.0, unit=NumberUnit)
    println(centerBounds(x, 1e-3)) // Bounds(lower=6.0, upper=17.0, unit=NumberUnit)

    val sx = Sample.of((1..30).map { it.toDouble() })
    val sy = Sample.of((21..50).map { it.toDouble() })
    println(shift(sx, sy)) // Measurement(value=-20.0, unit=NumberUnit)
    println(ratio(sx, sy)) // Measurement(value=0.436..., unit=RatioUnit)
    println(disparity(sx, sy)) // Measurement(value=-2.222..., unit=DisparityUnit)

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
