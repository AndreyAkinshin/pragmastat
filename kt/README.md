# Kotlin

Install from Maven Central Repository via Apache Maven:

```xml
<dependency>
    <groupId>dev.pragmastat</groupId>
    <artifactId>pragmastat</artifactId>
    <version>10.0.6</version>
</dependency>
```

Install from Maven Central Repository via Gradle:

```java
implementation 'dev.pragmastat:pragmastat:10.0.6'
```

Install from Maven Central Repository via Gradle (Kotlin):

```kotlin
implementation("dev.pragmastat:pragmastat:10.0.6")
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v10.0.6/kt

Pragmastat on Maven Central Repository: https://central.sonatype.com/artifact/dev.pragmastat/pragmastat/overview

## Demo

```kotlin
package dev.pragmastat.demo

import dev.pragmastat.*
import dev.pragmastat.distributions.*

fun main() {
    // --- One-Sample ---

    var x = (1..20).map { it.toDouble() }

    println(center(x)) // 10.5
    println(centerBounds(x, 0.05)) // Bounds(lower=7.5, upper=13.5)
    println(spread(x)) // 6.0
    println(spreadBounds(x, 0.05, "demo")) // Bounds(lower=2.0, upper=10.0)

    // --- Two-Sample ---

    x = (1..30).map { it.toDouble() }
    var y = (21..50).map { it.toDouble() }

    println(shift(x, y)) // -20
    println(shiftBounds(x, y, 0.05)) // Bounds(lower=-25.0, upper=-15.0)
    println(ratio(x, y)) // 0.43669798282695127
    println(ratioBounds(x, y, 0.05)) // Bounds(lower=0.31250000000000006, upper=0.5599999999999999)
    println(disparity(x, y)) // -2.2222222222222223
    println(disparityBounds(x, y, 0.05, "demo")) // Bounds(lower=-13.0, upper=-0.8235294117647058)

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
