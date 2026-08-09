def normalize_zero(value: float) -> float:
    """Replace a negative zero with a positive one.

    A sample holding both ``+0.0`` and ``-0.0`` makes the reported value depend on
    which of the two the sort happened to leave in the selected position.
    Comparison cannot separate them, so that position is settled by the sorting
    algorithm rather than by the data, and each of the seven ports brings its own
    sort: ``center([0.0, -0.0, 0.0, -0.0, 1.0])`` came out of this one as ``-0.0``
    and out of Go as ``+0.0``, against an ``exact`` conformance class that promises
    identical bits from identical inputs.

    The sign of a zero carries no statistical meaning here: ``-0.0 == 0.0``, and no
    estimate loses accuracy by shedding it. Normalizing on the way out of every
    estimator closes the whole class of divergence instead of the one sample that
    exposed it.

    Only outputs are normalized. A sample may still CONTAIN ``-0.0``, and infinities
    and NaN pass through untouched.
    """
    return 0.0 if value == 0 else value
