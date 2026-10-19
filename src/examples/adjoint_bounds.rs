//! Example 4: Canonical Adjunction / Galois Connection (`ADJOINT`).
//!
//! Morphism pair between finite-dimensional truncated valuations $\mathbb{N}^K$
//! and full prime valuation profiles $\bigoplus_{p \in \mathbb{P}} \mathbb{N}$.
//!
//! - Left Adjoint (Section / Zero-Extension Embedding):
//!   $L: \mathbf{w} \mapsto \sum_{i=1}^K w_i \mathbf{e}_{p_i}$
//! - Right Adjoint (Retraction / Canonical Truncation):
//!   $R: \mathbf{v} \mapsto (v_{p_1}, \dots, v_{p_K})$
//!
//! Contract:
//! - Adjunction equivalence: $L(\mathbf{w}) \le \mathbf{v} \iff \mathbf{w} \le R(\mathbf{v})$.
//! - Unit identity (Section): $R(L(\mathbf{w})) = \mathbf{w}$.
//! - Counit inequality: $L(R(\mathbf{v})) \le \mathbf{v}$ (componentwise order / divisibility).

use super::prime_factorization::ValuationProfile;
use super::truncation::{TruncatedValuationVector, FIRST_PRIMES};
use crate::transformation::{AdjointTransformation, Transformation, TransformationKind};

/// Adjoint transformation pair connecting truncated and full valuation spaces.
#[derive(Debug, Clone)]
pub struct ValuationAdjointPair {
    pub depth: usize,
    pub prime_basis: Vec<u64>,
}

impl ValuationAdjointPair {
    pub fn new(depth: usize) -> Self {
        assert!(depth <= FIRST_PRIMES.len());
        Self {
            depth,
            prime_basis: FIRST_PRIMES[0..depth].to_vec(),
        }
    }
}

impl Transformation<TruncatedValuationVector, ValuationProfile> for ValuationAdjointPair {
    fn name(&self) -> &'static str {
        "ValuationAdjointEmbedding"
    }

    fn description(&self) -> &'static str {
        "Canonical left-adjoint zero-extension embedding from N^K into full valuation space"
    }

    fn kind(&self) -> TransformationKind {
        TransformationKind::Adjoint
    }

    fn apply(&self, from: &TruncatedValuationVector) -> ValuationProfile {
        let mut profile = ValuationProfile::new();
        for (&p, &v) in from.primes.iter().zip(&from.exponents) {
            profile.insert(p, v);
        }
        profile
    }
}

impl AdjointTransformation<TruncatedValuationVector, ValuationProfile> for ValuationAdjointPair {
    fn right_adjoint(&self, to: &ValuationProfile) -> TruncatedValuationVector {
        let mut exps = Vec::with_capacity(self.depth);
        for &p in &self.prime_basis {
            exps.push(to.get(p));
        }
        TruncatedValuationVector::new(self.prime_basis.clone(), exps)
    }
}
