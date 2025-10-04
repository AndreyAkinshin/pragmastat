<span id="kotlin"></span> <!-- [pdf] DELETE -->

## Kotlin

A package is not available yet.

Source code: https://github.com/AndreyAkinshin/pragmastat/tree/v3.1.17/kotlin

Demo:

```kotlin
package dev.pragmastat.example

import dev.pragmastat.*

fun main() {
    var x = listOf(0.0, 2.0, 4.0, 6.0, 8.0)
    println(center(x)) // 4
    println(center(x.map { it + 10 })) // 14
    println(center(x.map { it * 3 })) // 12

    println(spread(x)) // 4
    println(spread(x.map { it + 10 })) // 4
    println(spread(x.map { it * 2 })) // 8

    println(relSpread(x)) // 1
    println(relSpread(x.map { it * 5 })) // 1

    var y = listOf(10.0, 12.0, 14.0, 16.0, 18.0)
    println(shift(x, y)) // -10
    println(shift(x, x)) // 0
    println(shift(x.map { it + 7 }, y.map { it + 3 })) // -6
    println(shift(x.map { it * 2 }, y.map { it * 2 })) // -20
    println(shift(y, x)) // 10

    x = listOf(1.0, 2.0, 4.0, 8.0, 16.0)
    y = listOf(2.0, 4.0, 8.0, 16.0, 32.0)
    println(ratio(x, y)) // 0.5
    println(ratio(x, x)) // 1
    println(ratio(x.map { it * 2 }, y.map { it * 5 })) // 0.2

    x = listOf(0.0, 3.0, 6.0, 9.0, 12.0)
    y = listOf(0.0, 2.0, 4.0, 6.0, 8.0)
    println(spread(x)) // 6
    println(spread(y)) // 4

    println(avgSpread(x, y)) // 5
    println(avgSpread(x, x)) // 6
    println(avgSpread(x.map { it * 2 }, x.map { it * 3 })) // 15
    println(avgSpread(y, x)) // 5
    println(avgSpread(x.map { it * 2 }, y.map { it * 2 })) // 10

    println(shift(x, y)) // 2
    println(avgSpread(x, y)) // 5

    println(disparity(x, y)) // 0.4
    println(disparity(x.map { it + 5 }, y.map { it + 5 })) // 0.4
    println(disparity(x.map { it * 2 }, y.map { it * 2 })) // 0.4
    println(disparity(y, x)) // -0.4
}
```