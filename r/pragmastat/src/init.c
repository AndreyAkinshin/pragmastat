#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>

// Forward declarations
SEXP center_impl_c(SEXP values_sexp, SEXP assume_sorted_sexp);
SEXP spread_impl_c(SEXP values_sexp, SEXP assume_sorted_sexp);
SEXP shift_impl_c(SEXP x_sexp, SEXP y_sexp, SEXP p_sexp, SEXP assume_sorted_sexp);
SEXP binomial_coefficient_c(SEXP n_sexp, SEXP k_sexp);
SEXP signed_rank_margin_exact_raw_c(SEXP n_sexp, SEXP p_sexp);

// Registration table
static const R_CallMethodDef CallEntries[] = {
    {"center_impl_c", (DL_FUNC) &center_impl_c, 2},
    {"spread_impl_c", (DL_FUNC) &spread_impl_c, 2},
    {"shift_impl_c", (DL_FUNC) &shift_impl_c, 4},
    {"binomial_coefficient_c", (DL_FUNC) &binomial_coefficient_c, 2},
    {"signed_rank_margin_exact_raw_c", (DL_FUNC) &signed_rank_margin_exact_raw_c, 2},
    {NULL, NULL, 0}
};

// Package initialization
void R_init_pragmastat(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
