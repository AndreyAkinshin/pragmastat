/**
 * Constants shared by the parametric families.
 */

/**
 * sqrt(2 * pi), the normalizing constant of the normal density.
 *
 * A literal rather than Math.sqrt(2 * Math.PI): the seven ports must divide by the
 * same double, and a runtime square root of a runtime product is one more place for
 * them to differ.
 */
export const SQRT_2PI = 2.50662827463100024161235523934;
