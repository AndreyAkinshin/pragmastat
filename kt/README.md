# Kotlin

Install from Maven Central Repository via Apache Maven:

```xml
<dependency>
    <groupId>dev.pragmastat</groupId>
    <artifactId>pragmastat</artifactId>
    <version>12.0.0</version>
</dependency>
```

Install from Maven Central Repository via Gradle:

```java
implementation 'dev.pragmastat:pragmastat:12.0.0'
```

Install from Maven Central Repository via Gradle (Kotlin):

```kotlin
implementation("dev.pragmastat:pragmastat:12.0.0")
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v12.0.0/kt

Pragmastat on Maven Central Repository: https://central.sonatype.com/artifact/dev.pragmastat/pragmastat/overview

## Demo

```kotlin
package dev.pragmastat.demo

import dev.pragmastat.*
import dev.pragmastat.distributions.*

fun main() {
    // --- One-Sample ---

    val x = Sample.of((1..200).map { it.toDouble() })
    println(center(x)) // 100.5
    println(spread(x)) // 59.0
    println(centerBounds(x, 1e-3)) // Bounds(lower=86.0, upper=115.0, unit=...)
    println(spreadBounds(x, 1e-3, "demo")) // Bounds(lower=44.0, upper=87.0, unit=...)

    // --- Two-Sample ---

    val sx = Sample.of((1..200).map { it.toDouble() })
    val sy = Sample.of((101..300).map { it.toDouble() })
    println(shift(sx, sy)) // -100.0
    println(shiftBounds(sx, sy, 1e-3)) // Bounds(lower=-120.0, upper=-80.0, unit=...)
    println(ratio(sx, sy)) // 0.5008354224706336
    println(ratioBounds(sx, sy, 1e-3)) // Bounds(lower=0.4066..., upper=0.5958..., unit=...)
    println(disparity(sx, sy)) // -1.694915254237288
    println(disparityBounds(sx, sy, 1e-3, "demo")) // Bounds(lower=-3.1025..., upper=-0.8494..., unit=...)

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
