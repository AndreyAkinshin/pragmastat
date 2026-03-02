# Kotlin

Install from Maven Central Repository via Apache Maven:

```xml
<dependency>
    <groupId>dev.pragmastat</groupId>
    <artifactId>pragmastat</artifactId>
    <version>11.0.0</version>
</dependency>
```

Install from Maven Central Repository via Gradle:

```java
implementation 'dev.pragmastat:pragmastat:11.0.0'
```

Install from Maven Central Repository via Gradle (Kotlin):

```kotlin
implementation("dev.pragmastat:pragmastat:11.0.0")
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v11.0.0/kt

Pragmastat on Maven Central Repository: https://central.sonatype.com/artifact/dev.pragmastat/pragmastat/overview

## Demo

```kotlin
package dev.pragmastat.demo

import dev.pragmastat.*
import dev.pragmastat.distributions.*

fun main() {
    // --- One-Sample ---

    val x = Sample.of((1..22).map { it.toDouble() })
    println(center(x)) // Measurement(value=11.5, unit=NumberUnit)
    println(spread(x)) // Measurement(value=7.0, unit=NumberUnit)
    println(centerBounds(x, 1e-3)) // Bounds(lower=6.0, upper=17.0, unit=NumberUnit)
    println(spreadBounds(x, 1e-3, "demo")) // Bounds(lower=1.0, upper=18.0, unit=NumberUnit)

    // --- Two-Sample ---

    val sx = Sample.of((1..30).map { it.toDouble() })
    val sy = Sample.of((21..50).map { it.toDouble() })
    println(shift(sx, sy)) // Measurement(value=-20.0, unit=NumberUnit)
    println(shiftBounds(sx, sy, 1e-3)) // Bounds(lower=-28.0, upper=-12.0, unit=NumberUnit)
    println(ratio(sx, sy)) // Measurement(value=0.436..., unit=RatioUnit)
    println(ratioBounds(sx, sy, 1e-3)) // Bounds(lower=0.232..., upper=0.642..., unit=RatioUnit)
    println(disparity(sx, sy)) // Measurement(value=-2.222..., unit=DisparityUnit)
    println(disparityBounds(sx, sy, 1e-3, "demo")) // Bounds(lower=-29.0, upper=-0.478..., unit=DisparityUnit)

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
```
