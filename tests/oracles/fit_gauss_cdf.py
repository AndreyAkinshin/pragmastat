import sys
from decimal import Decimal as D, getcontext
sys.path.insert(0, ".")
getcontext().prec = 80
from gauss_reference import erf_series, erfc_cf
from chebyshev import cheb_fit, horner

def erfc_d(x): return (1 - erf_series(x)) if x < D("2.5") else erfc_cf(x)

# Branch A: erf(t)/t as a polynomial in t^2 on t in [0, 0.5] -> s = t^2 in [0, 0.25]
def fa(s):
    t = s.sqrt()
    return erf_series(t) / t if t != 0 else D(2) / D("1.7724538509055160272981674833411451827975494561223871282138077898529113")
polyA = cheb_fit(fa, D(0), D("0.25"), 12)

# Branch B: erfc(t)*exp(t^2) on [0.5, 4.3]
def fb(t): return erfc_d(t) * (t * t).exp()
polyB = cheb_fit(fb, D("0.5"), D("4.3"), 27)

def fmt(p):
    return [f"{float(c):.17e}" for c in p]

print("A_LO, A_HI = 0.0, 0.25   # in s = t^2")
print("POLY_A =", fmt(polyA))
print()
print("B_LO, B_HI = 0.5, 4.3")
print("POLY_B =", fmt(polyB))
