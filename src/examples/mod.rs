//! Reference acceptance examples from RP-002 and AMDE Core Navigator (PRD-26).

pub mod prime_factorization;
pub mod truncation;
pub mod numerical_xi;
pub mod adjoint_bounds;

pub use prime_factorization::{
    factorize_u64, natural_number_object, reconstruct_u64, valuation_profile_object,
    NaturalNumber, PrimeFactorizationIso, ValuationProfile,
};

pub use truncation::{
    truncated_valuation_object, TruncatedValuationVector, ValuationTruncationLossy, FIRST_PRIMES,
};

pub use numerical_xi::{
    analytic_space_object, compute_xi_with_error, numerical_xi_object, AnalyticCoordinate,
    NumericalXiApproximate, NumericalXiResult,
};

pub use adjoint_bounds::ValuationAdjointPair;
