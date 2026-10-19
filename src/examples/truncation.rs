//! Example 2 (RP-002 Acceptance): Truncation as an Irreversible Projection (`LOSSY`).
//!
//! Truncates the infinite-dimensional prime valuation profile to a finite $K$-dimensional
//! subspace corresponding to the first $K$ primes: $\mathbb{P}_{\le K} = \{p_1, \dots, p_K\}$.
//!
//! Projection: $\pi_K: (v_p)_{p \in \mathbb{P}} \mapsto (v_{p_1}, \dots, v_{p_K})$
//!
//! Contract:
//! - Non-injective: $\exists \mathbf{u} \ne \mathbf{v}$ such that $\pi_K(\mathbf{u}) = \pi_K(\mathbf{v})$.
//! - Information loss: $\Delta I = I(\mathbf{v}) - I_K(\pi_K(\mathbf{v})) = \sum_{p > p_K} v_p \ln(p) \ge 0$.
//! - Fiber / Preimage: $\pi_K^{-1}(\mathbf{w}) = \{\mathbf{v} \mid \pi_K(\mathbf{v}) = \mathbf{w}\}$ is non-trivial.

use super::prime_factorization::ValuationProfile;
use crate::object::{InformationFunctional, NavigatorObject, RelationalStructure, Signature, Topology};
use crate::transformation::{LossyTransformation, Transformation, TransformationKind};
use std::fmt::{self, Display, Formatter};

/// Standard list of first 10 primes for reference truncation.
pub const FIRST_PRIMES: [u64; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

/// Truncated valuation vector retaining only exponents for the first $K$ primes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TruncatedValuationVector {
    pub primes: Vec<u64>,
    pub exponents: Vec<u32>,
}

impl TruncatedValuationVector {
    pub fn new(primes: Vec<u64>, exponents: Vec<u32>) -> Self {
        assert_eq!(primes.len(), exponents.len());
        Self { primes, exponents }
    }

    /// Dimension / cutoff depth $K$.
    pub fn depth(&self) -> usize {
        self.primes.len()
    }

    /// Evaluates truncated logarithmic information: $\sum_{i=1}^K w_i \ln(p_i)$.
    pub fn information(&self) -> f64 {
        self.primes
            .iter()
            .zip(&self.exponents)
            .map(|(&p, &v)| (v as f64) * (p as f64).ln())
            .sum()
    }
}

impl Display for TruncatedValuationVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let parts: Vec<String> = self
            .primes
            .iter()
            .zip(&self.exponents)
            .map(|(&p, &v)| format!("{}^{}", p, v))
            .collect();
        write!(f, "[{}]", parts.join(", "))
    }
}

/// Formal Navigator Object for the truncated space $\mathbb{N}^K$.
pub fn truncated_valuation_object(depth: usize) -> NavigatorObject<TruncatedValuationVector> {
    assert!(depth <= FIRST_PRIMES.len(), "Depth exceeds prime table");
    NavigatorObject::new(
        "OBJ-TRUNC-VAL",
        "Truncated Prime Valuation Space (N^K)",
        "Finite K-dimensional space of leading prime valuations",
        RelationalStructure::new(
            "FiniteComponentwiseLattice",
            "Finite-dimensional product order on N^K with meet (min) and join (max)",
            &["bounded_dimension", "componentwise_partial_order"],
        ),
        Signature::new(
            &["component_add", "component_min", "component_max", "dimension"],
            "Operations restricted to the first K prime channels",
        ),
        Topology::truncation(depth, "TruncationCutoffMetric", "Finite horizon prefix topology"),
        InformationFunctional::new(
            "TruncatedLogarithmicWeight",
            "I_K(w) = sum_{i=1}^K w_i * ln(p_i)",
            |w: &TruncatedValuationVector| w.information(),
        ),
    )
}

/// The typed lossy transformation: $\bigoplus_p \mathbb{N} \xrightarrow{\mathrm{LOSSY}} \mathbb{N}^K$.
#[derive(Debug, Clone)]
pub struct ValuationTruncationLossy {
    pub depth: usize,
    pub prime_basis: Vec<u64>,
}

impl ValuationTruncationLossy {
    pub fn new(depth: usize) -> Self {
        assert!(depth <= FIRST_PRIMES.len(), "Depth exceeds prime table");
        Self {
            depth,
            prime_basis: FIRST_PRIMES[0..depth].to_vec(),
        }
    }
}

impl Transformation<ValuationProfile, TruncatedValuationVector> for ValuationTruncationLossy {
    fn name(&self) -> &'static str {
        "ValuationTruncationLossy"
    }

    fn description(&self) -> &'static str {
        "Projects infinite prime valuation profile onto finite leading K-prime components, discarding higher primes"
    }

    fn kind(&self) -> TransformationKind {
        TransformationKind::Lossy
    }

    fn apply(&self, from: &ValuationProfile) -> TruncatedValuationVector {
        let mut exps = Vec::with_capacity(self.depth);
        for &p in &self.prime_basis {
            exps.push(from.get(p));
        }
        TruncatedValuationVector::new(self.prime_basis.clone(), exps)
    }
}

impl LossyTransformation<ValuationProfile, TruncatedValuationVector> for ValuationTruncationLossy {
    fn information_loss(&self, from: &ValuationProfile) -> f64 {
        // Delta I = sum_{p > p_K} v_p * ln(p)
        let loss: f64 = from
            .exponents
            .iter()
            .filter(|(&p, _)| !self.prime_basis.contains(&p))
            .map(|(&p, &v)| (v as f64) * (p as f64).ln())
            .sum();
        if loss.abs() < 1e-12 {
            0.0
        } else {
            loss
        }
    }
}
