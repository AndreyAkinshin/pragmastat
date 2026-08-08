"""Constants shared by the parametric families."""

# sqrt(2 * pi), the normalizing constant of the normal density.
#
# A literal rather than math.sqrt(2 * math.pi): the seven ports must divide by the same double,
# and a runtime square root of a runtime product is one more place for them to differ.
SQRT_2PI = 2.506628274631000241612355239340
