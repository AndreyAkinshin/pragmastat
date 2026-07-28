package pragmastat

import "math"

// gaussCdf computes the standard normal CDF.
//
// Two Chebyshev-fitted Horner chains and one exponential. The coefficients are produced by
// tests/oracles/fit_gauss_cdf.py against a reference good to 36 digits, so they are
// reproducible rather than transcribed.
//
// Every product is pinned with an explicit float64 conversion for the same reason the rest of
// the kernels are: the Go compiler fuses a multiply into an add on every FMA-capable target
// and the other six implementations do not.
func gaussCdf(x float64) float64 {
	t := math.Abs(x) / math.Sqrt2
	if t < 0.5 {
		s := float64(t * t)
		u := float64(8.0*s) - 1.0
		p := -1.2757552949301143e-19
		p = float64(p*u) + 1.2307154179828511e-17
		p = float64(p*u) - 1.0890239994332592e-15
		p = float64(p*u) + 8.774530700097397e-14
		p = float64(p*u) - 6.3744178527620835e-12
		p = float64(p*u) + 4.1270254211564467e-10
		p = float64(p*u) - 2.347229163519518e-08
		p = float64(p*u) + 1.151603779513705e-06
		p = float64(p*u) - 4.762336934468491e-05
		p = float64(p*u) + 0.0016130716680617086
		p = float64(p*u) - 0.04364205888669792
		p = float64(p*u) + 1.0830752376761712
		erf := float64(t * p)
		if x >= 0 {
			return float64(0.5 * (1.0 + erf))
		}
		return float64(0.5 * (1.0 - erf))
	}
	var erfc float64
	if t <= 4.3 {
		u := float64((2.0*t - 4.8) / 3.8)
		p := 2.403093649825437e-09
		p = float64(p*u) - 6.533436159455495e-09
		p = float64(p*u) + 1.334437871983186e-09
		p = float64(p*u) - 2.5055474016226743e-09
		p = float64(p*u) + 5.2376178949357336e-08
		p = float64(p*u) - 1.341394638617228e-07
		p = float64(p*u) + 2.5376572107855777e-07
		p = float64(p*u) - 6.147631059669139e-07
		p = float64(p*u) + 1.561533370779237e-06
		p = float64(p*u) - 3.688982809059467e-06
		p = float64(p*u) + 8.492013869441648e-06
		p = float64(p*u) - 1.9344330869926753e-05
		p = float64(p*u) + 4.3285002216779125e-05
		p = float64(p*u) - 9.489727696113043e-05
		p = float64(p*u) + 0.0002037912849869451
		p = float64(p*u) - 0.0004282777524202283
		p = float64(p*u) + 0.00087969639542425
		p = float64(p*u) - 0.001763698443638436
		p = float64(p*u) + 0.0034462452415540026
		p = float64(p*u) - 0.00655166763664565
		p = float64(p*u) + 0.012094345026186722
		p = float64(p*u) - 0.021629099761798037
		p = float64(p*u) + 0.037371670355588804
		p = float64(p*u) - 0.06218492139115531
		p = float64(p*u) + 0.09925390090168178
		p = float64(p*u) - 0.15121195850373031
		p = float64(p*u) + 0.21849873453703333
		erfc = float64(portableExp(-float64(t*t)) * p)
	}
	if x >= 0 {
		return 1.0 - float64(0.5*erfc)
	}
	return float64(0.5 * erfc)
}
