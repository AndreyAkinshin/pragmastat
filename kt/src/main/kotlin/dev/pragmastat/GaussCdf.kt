package dev.pragmastat

import kotlin.math.abs

/**
 * Computes the standard normal CDF using ACM Algorithm 209.
 *
 * Calculates (1/sqrt(2*pi)) * integral from -infinity to x of e^(-u^2/2) du
 * by means of polynomial approximations due to A. M. Murray of Aberdeen University.
 *
 * See: http://dl.acm.org/citation.cfm?id=367664
 *
 * @param x Value in range (-infinity, +infinity)
 * @return Area under the Standard Normal Curve from -infinity to x
 */
internal fun gaussCdf(x: Double): Double {
    val z: Double

    if (abs(x) < 1e-9) {
        z = 0.0
    } else {
        val y = abs(x) / 2.0
        if (y >= 3.0) {
            z = 1.0
        } else if (y < 1.0) {
            val w = y * y
            z = (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (0.000124818987 * w - 0.001075204047) * w +
                                            0.005198775019
                                    ) * w -
                                        0.019198292004
                                ) * w +
                                    0.059054035642
                            ) * w -
                                0.151968751364
                        ) * w +
                            0.319152932694
                    ) * w -
                        0.531923007300
                ) * w + 0.797884560593
            ) * y * 2.0
        } else {
            val y2 = y - 2.0
            z = (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (
                                            (
                                                (
                                                    (
                                                        (
                                                            (
                                                                -0.000045255659 * y2 +
                                                                    0.000152529290
                                                            ) * y2 -
                                                                0.000019538132
                                                        ) * y2 -
                                                            0.000676904986
                                                    ) * y2 +
                                                        0.001390604284
                                                ) * y2 -
                                                    0.000794620820
                                            ) * y2 -
                                                0.002034254874
                                        ) * y2 +
                                            0.006549791214
                                    ) * y2 -
                                        0.010557625006
                                ) * y2 +
                                    0.011630447319
                            ) * y2 -
                                0.009279453341
                        ) * y2 +
                            0.005353579108
                    ) * y2 -
                        0.002141268741
                ) * y2 +
                    0.000535310849
            ) * y2 + 0.999936657524
        }
    }

    return if (x > 0.0) (z + 1.0) / 2.0 else (1.0 - z) / 2.0
}
