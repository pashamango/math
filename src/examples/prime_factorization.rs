//! Example 1 (RP-002 Acceptance): Prime Factorization as an Exact Isomorphism (`ISO`).
//!
//! Round-trip isomorphism between the multiplicative monoid $(\mathbb{N}^+, \times)$
//! and the direct sum prime valuation monoid $(\bigoplus_{p \in \mathbb{P}} \mathbb{N}, \oplus)$.
//!
//! Forward: $n \mapsto (v_p(n))_{p \in \mathbb{P}}$
//! Backward: $(v_p)_{p \in \mathbb{P}} \mapsto \prod_{p} p^{v_p}$
//!
//! Contract:
//! - Round-trip identity: $\forall n \ge 1, \mathrm{backward}(\mathrm{forward}(n)) = n$.
//! - Information preservation: $I(n) = \ln(n) = \sum v_p(n) \ln(p)$, $\Delta I = 0$.
//! - Metric error: $\epsilon = 0$ (discrete exact topology).

use crate::object::{InformationFunctional, NavigatorObject, RelationalStructure, Signature, Topology};
use crate::transformation::{IsoTransformation, Transformation, TransformationKind};
use std::collections::BTreeMap;
use std::fmt::{self, Display, Formatter};

/// Natural number carrier $n \in \mathbb{N}^+$.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NaturalNumber(pub u64);

impl NaturalNumber {
    pub fn new(val: u64) -> Result<Self, &'static str> {
        if val == 0 {
            Err("Natural numbers in multiplicative monoid must be positive (n >= 1)")
        } else {
            Ok(Self(val))
        }
    }
}

impl Display for NaturalNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Sparse valuation profile: mapping prime $p \mapsto v_p(n)$.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValuationProfile {
    /// Ordered map from prime $p$ to exponent $v_p \ge 1$.
    pub exponents: BTreeMap<u64, u32>,
}

impl ValuationProfile {
    pub fn new() -> Self {
        Self {
            exponents: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, prime: u64, power: u32) {
        if power > 0 {
            self.exponents.insert(prime, power);
        }
    }

    pub fn get(&self, prime: u64) -> u32 {
        self.exponents.get(&prime).copied().unwrap_or(0)
    }

    /// Evaluates logarithmic information: $\sum v_p \ln(p)$.
    pub fn logarithmic_information(&self) -> f64 {
        self.exponents
            .iter()
            .map(|(&p, &v)| (v as f64) * (p as f64).ln())
            .sum()
    }

    /// Componentwise partial order: $\mathbf{u} \le \mathbf{v} \iff \forall p, u_p \le v_p$.
    pub fn leq(&self, other: &Self) -> bool {
        for (&p, &v) in &self.exponents {
            if v > other.get(p) {
                return false;
            }
        }
        true
    }
}

impl Display for ValuationProfile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.exponents.is_empty() {
            write!(f, "1")
        } else {
            let terms: Vec<String> = self
                .exponents
                .iter()
                .map(|(&p, &v)| {
                    if v == 1 {
                        format!("{}", p)
                    } else {
                        format!("{}^{}", p, v)
                    }
                })
                .collect();
            write!(f, "{}", terms.join(" * "))
        }
    }
}

/// Factorizes $n$ into prime powers using trial division.
pub fn factorize_u64(mut n: u64) -> ValuationProfile {
    assert!(n >= 1, "n must be positive");
    let mut profile = ValuationProfile::new();
    if n == 1 {
        return profile;
    }

    // Factor out 2
    let mut count2 = 0;
    while n % 2 == 0 {
        count2 += 1;
        n /= 2;
    }
    if count2 > 0 {
        profile.insert(2, count2);
    }

    // Factor out odd numbers
    let mut d = 3u64;
    while d.saturating_mul(d) <= n {
        if n % d == 0 {
            let mut count = 0;
            while n % d == 0 {
                count += 1;
                n /= d;
            }
            profile.insert(d, count);
        }
        d += 2;
    }

    if n > 1 {
        profile.insert(n, 1);
    }

    profile
}

/// Reconstructs integer $n = \prod p^{v_p}$.
pub fn reconstruct_u64(profile: &ValuationProfile) -> NaturalNumber {
    let mut product = 1u64;
    for (&p, &v) in &profile.exponents {
        for _ in 0..v {
            product = product
                .checked_mul(p)
                .expect("Overflow in prime reconstruction");
        }
    }
    NaturalNumber(product)
}

/// Formal Navigator Object for $(\mathbb{N}^+, \times)$.
pub fn natural_number_object() -> NavigatorObject<NaturalNumber> {
    NavigatorObject::new(
        "OBJ-NAT-MULT",
        "Natural Numbers Monoid (N+, *)",
        "Domain of positive integers under multiplication",
        RelationalStructure::new(
            "MultiplicativeMonoid",
            "Associative multiplication with identity 1 and divisibility partial order",
            &["associativity", "identity_1", "divisibility_preorder", "fundamental_arithmetic"],
        ),
        Signature::new(
            &["mul(*)", "pow(^)", "gcd", "lcm", "divisible_by(|)"],
            "Algebraic operators on integers",
        ),
        Topology::discrete("DiscreteIntegerMetric", "d(a, b) = 0 iff a == b"),
        InformationFunctional::new(
            "LogarithmicHeight",
            "I(n) = ln(n) representing bits of semantic arithmetic information",
            |n: &NaturalNumber| (n.0 as f64).ln(),
        ),
    )
}

/// Formal Navigator Object for $(\bigoplus_p \mathbb{N}, \oplus)$.
pub fn valuation_profile_object() -> NavigatorObject<ValuationProfile> {
    NavigatorObject::new(
        "OBJ-VAL-PROFILE",
        "Prime Valuation Direct Sum Monoid (Primes ->0 N)",
        "Domain of finitely supported prime valuation exponents",
        RelationalStructure::new(
            "FreeCommutativeMonoid",
            "Free commutative additive monoid generated by the primes with componentwise ordering",
            &["commutativity", "zero_identity", "componentwise_lattice"],
        ),
        Signature::new(
            &["add(+)", "scale(*)", "support", "min_gcd", "max_lcm"],
            "Vector operations on prime exponent profiles",
        ),
        Topology::discrete("DiscreteProfileMetric", "d(u, v) = 0 iff all exponents match"),
        InformationFunctional::new(
            "LogarithmicValuationWeight",
            "I(v) = sum_p v_p * ln(p)",
            |v: &ValuationProfile| v.logarithmic_information(),
        ),
    )
}

/// The typed isomorphism transformation: $\mathbb{N}^+ \leftrightarrow (\mathbb{P} \to_0 \mathbb{N})$.
#[derive(Debug, Clone, Copy, Default)]
pub struct PrimeFactorizationIso;

impl Transformation<NaturalNumber, ValuationProfile> for PrimeFactorizationIso {
    fn name(&self) -> &'static str {
        "PrimeFactorizationIso"
    }

    fn description(&self) -> &'static str {
        "Exact bijection between positive integers and their prime valuation profiles"
    }

    fn kind(&self) -> TransformationKind {
        TransformationKind::Iso
    }

    fn apply(&self, from: &NaturalNumber) -> ValuationProfile {
        factorize_u64(from.0)
    }
}

impl IsoTransformation<NaturalNumber, ValuationProfile> for PrimeFactorizationIso {
    fn backward(&self, to: &ValuationProfile) -> NaturalNumber {
        reconstruct_u64(to)
    }
}
