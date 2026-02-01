# Kotlin

Install from Maven Central Repository via Apache Maven:

```xml
<dependency>
    <groupId>dev.pragmastat</groupId>
    <artifactId>pragmastat</artifactId>
    <version>5.1.0</version>
</dependency>
```

Install from Maven Central Repository via Gradle:

```java
implementation 'dev.pragmastat:pragmastat:5.1.0'
```

Install from Maven Central Repository via Gradle (Kotlin):

```kotlin
implementation("dev.pragmastat:pragmastat:5.1.0")
```

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v5.1.0/kt

Pragmastat on Maven Central Repository: https://central.sonatype.com/artifact/dev.pragmastat/pragmastat/overview

## Demo

```kotlin
package dev.pragmastat.demo

import dev.pragmastat.*
import dev.pragmastat.distributions.*

fun main() {
    // --- Randomization ---

    var rng = Rng(1729)
    println(rng.uniform()) // 0.3943034703296536
    println(rng.uniform()) // 0.5730893757071377

    rng = Rng("experiment-1")
    println(rng.uniform()) // 0.9535207726895857

    rng = Rng(1729)
    println(rng.sample(listOf(0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0), 3)) // [6, 8, 9]

    rng = Rng(1729)
    println(rng.shuffle(listOf(1.0, 2.0, 3.0, 4.0, 5.0))) // [4, 2, 3, 5, 1]

    // --- Distribution Sampling ---

    rng = Rng(1729)
    var dist: Distribution = Uniform(0.0, 10.0)
    println(dist.sample(rng)) // 3.9430347032965365

    rng = Rng(1729)
    dist = Additive(0.0, 1.0)
    println(dist.sample(rng)) // -1.222932972163442

    rng = Rng(1729)
    dist = Exp(1.0)
    println(dist.sample(rng)) // 0.5013761944646019

    rng = Rng(1729)
    dist = Power(1.0, 2.0)
    println(dist.sample(rng)) // 1.284909255071668

    rng = Rng(1729)
    dist = Multiplic(0.0, 1.0)
    println(dist.sample(rng)) // 0.2943655336550937

    // --- Single-Sample Statistics ---

    var x = listOf(0.0, 2.0, 4.0, 6.0, 8.0)

    println(median(x)) // 4
    println(center(x)) // 4
    println(spread(x)) // 4
    println(spread(x.map { it + 10 })) // 4
    println(spread(x.map { it * 2 })) // 8
    println(relSpread(x)) // 1

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

    // --- Confidence Bounds ---

    x = (1..30).map { it.toDouble() }
    y = (21..50).map { it.toDouble() }

    println(pairwiseMargin(30, 30, 1e-4)) // 390
    println(shift(x, y)) // -20
    println(shiftBounds(x, y, 1e-4)) // [-30, -10]
}
```
