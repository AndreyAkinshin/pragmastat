# Kotlin

Install from Maven Central Repository via Apache Maven:

```xml
<dependency>
    <groupId>dev.pragmastat</groupId>
    <artifactId>pragmastat</artifactId>
    <version>6.0.1</version>
</dependency>
```

Install from Maven Central Repository via Gradle:

```java
implementation 'dev.pragmastat:pragmastat:6.0.1'
```

Install from Maven Central Repository via Gradle (Kotlin):

```kotlin
implementation("dev.pragmastat:pragmastat:6.0.1")
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v6.0.1/kt

Pragmastat on Maven Central Repository: https://central.sonatype.com/artifact/dev.pragmastat/pragmastat/overview

## Demo

```kotlin
package dev.pragmastat.demo

import dev.pragmastat.*
import dev.pragmastat.distributions.*

fun main() {
    // --- Randomization ---

    var rng = Rng("demo-uniform")
    println(rng.uniform()) // 0.2640554428629759
    println(rng.uniform()) // 0.9348534835582796

    rng = Rng("demo-sample")
    println(rng.sample(listOf(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0), 3)) // [3, 8, 9]

    rng = Rng("demo-shuffle")
    println(rng.shuffle(listOf(1.0, 2.0, 3.0, 4.0, 5.0))) // [4, 2, 3, 5, 1]

    rng = Rng("demo-resample")
    println(rng.resample(listOf(1.0, 2.0, 3.0, 4.0, 5.0), 7)) // [5, 1, 1, 3, 3, 4, 5]

    // --- Distribution Sampling ---

    rng = Rng("demo-dist-uniform")
    var dist: Distribution = Uniform(0.0, 10.0)
    println(dist.sample(rng)) // 6.54043657816832

    rng = Rng("demo-dist-additive")
    dist = Additive(0.0, 1.0)
    println(dist.sample(rng)) // 0.17410448679568188

    rng = Rng("demo-dist-exp")
    dist = Exp(1.0)
    println(dist.sample(rng)) // 0.6589065267276553

    rng = Rng("demo-dist-power")
    dist = Power(1.0, 2.0)
    println(dist.sample(rng)) // 1.023677535537084

    rng = Rng("demo-dist-multiplic")
    dist = Multiplic(0.0, 1.0)
    println(dist.sample(rng)) // 1.1273244602673853

    // --- Single-Sample Statistics ---

    var x = listOf(1.0, 3.0, 5.0, 7.0, 9.0)

    println(median(x)) // 5
    println(center(x)) // 5
    println(spread(x)) // 4
    println(spread(x.map { it + 10 })) // 4
    println(spread(x.map { it * 2 })) // 8
    println(relSpread(x)) // 0.8

    // --- Two-Sample Comparison ---

    x = listOf(0.0, 3.0, 6.0, 9.0, 12.0)
    var y = listOf(0.0, 2.0, 4.0, 6.0, 8.0)

    println(shift(x, y)) // 2
    println(shift(y, x)) // -2
    println(avgSpread(x, y)) // 5
    println(disparity(x, y)) // 0.4
    println(disparity(y, x)) // -0.4

    x = listOf(1.0, 2.0, 4.0, 8.0, 16.0)
    y = listOf(2.0, 4.0, 8.0, 16.0, 32.0)
    println(ratio(x, y)) // 0.5
    println(ratio(y, x)) // 2

    // --- One-Sample Bounds ---

    x = (1..10).map { it.toDouble() }

    println(signedRankMargin(10, 0.05)) // 18
    println(center(x)) // 5.5
    println(centerBounds(x, 0.05)) // Bounds(lower=3.5, upper=7.5)
    println(medianBounds(x, 0.05)) // Bounds(lower=2.0, upper=9.0)
    println(centerBoundsApprox(x, 0.05)) // Bounds(lower=3.5, upper=7.5) (approximate)

    // --- Two-Sample Bounds ---

    x = (1..30).map { it.toDouble() }
    y = (21..50).map { it.toDouble() }

    println(pairwiseMargin(30, 30, 1e-4)) // 390
    println(shift(x, y)) // -20
    println(shiftBounds(x, y, 1e-4)) // Bounds(lower=-30.0, upper=-10.0)

    x = listOf(1.0, 2.0, 3.0, 4.0, 5.0)
    y = listOf(2.0, 3.0, 4.0, 5.0, 6.0)
    println(ratioBounds(x, y, 0.05)) // Bounds(lower=0.333..., upper=1.5)
}
```
