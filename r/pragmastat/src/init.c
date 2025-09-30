#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>

// Forward declarations
SEXP fast_center_c(SEXP values_sexp);
SEXP fast_spread_c(SEXP values_sexp);

// Registration table
static const R_CallMethodDef CallEntries[] = {
    {"fast_center_c", (DL_FUNC) &fast_center_c, 1},
    {"fast_spread_c", (DL_FUNC) &fast_spread_c, 1},
    {NULL, NULL, 0}
};

// Package initialization
void R_init_pragmastat(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
