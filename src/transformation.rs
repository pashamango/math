//! Transformation types: ISO, ADJOINT, LOSSY, APPROXIMATE.
//!
//! Based on PRD-26 (AMDE Core Navigator: Typed Transformation Graph).
//!
//! Formal classification of morphisms between Navigator Objects:
//! 1. `ISO`: Bijective, exactly reversible, zero information loss ($\Delta I = 0$), zero metric error ($\epsilon = 0$).
//! 2. `ADJOINT`: Galois connection / section-retraction pair ($f(a) \le b \iff a \le g(b)$) with unit/counit bounds.
//! 3. `LOSSY`: Non-injective projection / truncation, strictly positive information loss ($\Delta I > 0$), non-trivial fiber.
//! 4. `APPROXIMATE`: Metric embedding with certified error bound $d(f(x), \mathrm{target}) \le \epsilon$.

use std::fmt::{self, Display, Formatter};

/// The four canonical transformation kinds in the AMDE Navigator type system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TransformationKind {
    /// Isomorphism: Exact, reversible, zero loss, zero error.
    Iso,
    /// Adjoint pair / Galois connection: preserves canonical upper/lower bounds.
    Adjoint,
    /// Lossy projection / Truncation: information is forgotten, non-injective.
    Lossy,
    /// Metric approximation: numerical evaluation with bounded error.
    Approximate,
}

impl TransformationKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            TransformationKind::Iso => "ISO",
            TransformationKind::Adjoint => "ADJOINT",
            TransformationKind::Lossy => "LOSSY",
            TransformationKind::Approximate => "APPROXIMATE",
        }
    }

    /// Category composition algebra for transformation kinds.
    ///
    /// The composition table follows the information-preservation lattice:
    /// - ISO ∘ ISO = ISO
    /// - ISO ∘ ADJOINT = ADJOINT
    /// - ADJOINT ∘ ISO = ADJOINT
    /// - ADJOINT ∘ ADJOINT = ADJOINT
    /// - ISO ∘ LOSSY = LOSSY
    /// - LOSSY ∘ ISO = LOSSY
    /// - LOSSY ∘ LOSSY = LOSSY
    /// - ADJOINT ∘ LOSSY = LOSSY
    /// - LOSSY ∘ ADJOINT = LOSSY
    /// - Any ∘ APPROXIMATE = APPROXIMATE
    /// - APPROXIMATE ∘ Any = APPROXIMATE
    pub const fn compose(first: TransformationKind, second: TransformationKind) -> TransformationKind {
        match (first, second) {
            (TransformationKind::Iso, TransformationKind::Iso) => TransformationKind::Iso,
            (TransformationKind::Iso, TransformationKind::Adjoint)
            | (TransformationKind::Adjoint, TransformationKind::Iso)
            | (TransformationKind::Adjoint, TransformationKind::Adjoint) => TransformationKind::Adjoint,
            (TransformationKind::Iso, TransformationKind::Lossy)
            | (TransformationKind::Lossy, TransformationKind::Iso)
            | (TransformationKind::Adjoint, TransformationKind::Lossy)
            | (TransformationKind::Lossy, TransformationKind::Adjoint)
            | (TransformationKind::Lossy, TransformationKind::Lossy) => TransformationKind::Lossy,
            (_, TransformationKind::Approximate) | (TransformationKind::Approximate, _) => {
                TransformationKind::Approximate
            }
        }
    }
}

impl Display for TransformationKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Error type when an isomorphism round-trip verification fails.
#[derive(Debug, Clone, PartialEq)]
pub struct RoundtripError<T> {
    pub original: T,
    pub recovered: T,
    pub discrepancy: String,
}

/// Error type when an approximation exceeds its certified tolerance.
#[derive(Debug, Clone, PartialEq)]
pub struct ToleranceError {
    pub actual_error: f64,
    pub max_tolerance: f64,
}

/// Core trait implemented by every typed transformation in Navigator.
pub trait Transformation<From, To> {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn kind(&self) -> TransformationKind;
    fn apply(&self, from: &From) -> To;
}

/// Trait for `ISO` transformations: forward and backward bijection with round-trip proof.
pub trait IsoTransformation<From, To>: Transformation<From, To> {
    fn forward(&self, from: &From) -> To {
        self.apply(from)
    }

    fn backward(&self, to: &To) -> From;

    /// Verifies round-trip: backward(forward(from)) == from.
    fn verify_roundtrip(&self, from: &From) -> Result<(), RoundtripError<From>>
    where
        From: Clone + PartialEq + std::fmt::Debug,
    {
        let transformed = self.forward(from);
        let recovered = self.backward(&transformed);
        if *from == recovered {
            Ok(())
        } else {
            Err(RoundtripError {
                original: from.clone(),
                recovered,
                discrepancy: "Round-trip backward(forward(a)) != a".to_string(),
            })
        }
    }

    /// Verifies dual round-trip: forward(backward(to)) == to.
    fn verify_dual_roundtrip(&self, to: &To) -> Result<(), RoundtripError<To>>
    where
        To: Clone + PartialEq + std::fmt::Debug,
    {
        let back = self.backward(to);
        let recovered = self.forward(&back);
        if *to == recovered {
            Ok(())
        } else {
            Err(RoundtripError {
                original: to.clone(),
                recovered,
                discrepancy: "Dual round-trip forward(backward(b)) != b".to_string(),
            })
        }
    }
}

/// Trait for `ADJOINT` transformations (Galois connections / section-retraction pairs).
pub trait AdjointTransformation<From, To>: Transformation<From, To> {
    /// Left adjoint morphism (e.g. canonical embedding / section).
    fn left_adjoint(&self, from: &From) -> To {
        self.apply(from)
    }

    /// Right adjoint morphism (e.g. canonical retraction / projection).
    fn right_adjoint(&self, to: &To) -> From;

    /// Verifies unit condition: a <= right(left(a)) (or equality for section).
    fn verify_unit<F>(&self, from: &From, leq: F) -> bool
    where
        F: Fn(&From, &From) -> bool,
    {
        let embedded = self.left_adjoint(from);
        let retracted = self.right_adjoint(&embedded);
        leq(from, &retracted)
    }

    /// Verifies counit condition: left(right(b)) <= b.
    fn verify_counit<F>(&self, to: &To, leq: F) -> bool
    where
        F: Fn(&To, &To) -> bool,
    {
        let retracted = self.right_adjoint(to);
        let re_embedded = self.left_adjoint(&retracted);
        leq(&re_embedded, to)
    }
}

/// Trait for `LOSSY` transformations: irreversible projection, quantified loss, and fiber analysis.
pub trait LossyTransformation<From, To>: Transformation<From, To> {
    fn project(&self, from: &From) -> To {
        self.apply(from)
    }

    /// Quantified information loss Delta I >= 0.
    fn information_loss(&self, from: &From) -> f64;

    /// Computes the fiber (preimage subset) within a given search domain.
    fn fiber(&self, target: &To, domain: &[From]) -> Vec<From>
    where
        From: Clone,
        To: PartialEq,
    {
        domain
            .iter()
            .filter(|&item| self.project(item) == *target)
            .cloned()
            .collect()
    }
}

/// Trait for `APPROXIMATE` transformations: numerical evaluation with certified error bounds.
pub trait ApproximateTransformation<From, To>: Transformation<From, To> {
    /// Applies approximation and returns (result, certified_error).
    fn approximate_with_bound(&self, from: &From) -> (To, f64);

    /// Allowed tolerance threshold in target metric.
    fn tolerance(&self) -> f64;

    /// Verifies that actual error <= allowed tolerance.
    fn verify_tolerance(&self, actual_error: f64) -> Result<(), ToleranceError> {
        if actual_error <= self.tolerance() {
            Ok(())
        } else {
            Err(ToleranceError {
                actual_error,
                max_tolerance: self.tolerance(),
            })
        }
    }
}

/// Composed transformation $T_2 \circ T_1: A \to C$.
pub struct ComposedTransformation<A, B, C, T1, T2> {
    pub t1: T1,
    pub t2: T2,
    pub name: String,
    pub description: String,
    pub composed_kind: TransformationKind,
    _phantom: std::marker::PhantomData<fn(A) -> (B, C)>,
}

impl<A, B, C, T1, T2> ComposedTransformation<A, B, C, T1, T2>
where
    T1: Transformation<A, B>,
    T2: Transformation<B, C>,
{
    pub fn new(t1: T1, t2: T2) -> Self {
        let kind = TransformationKind::compose(t1.kind(), t2.kind());
        let name = format!("{} ∘ {}", t2.name(), t1.name());
        let description = format!(
            "Composition of '{}' ({}) followed by '{}' ({}) -> {}",
            t1.name(),
            t1.kind(),
            t2.name(),
            t2.kind(),
            kind
        );
        Self {
            t1,
            t2,
            name,
            description,
            composed_kind: kind,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<A, B, C, T1, T2> Transformation<A, C> for ComposedTransformation<A, B, C, T1, T2>
where
    T1: Transformation<A, B>,
    T2: Transformation<B, C>,
{
    fn name(&self) -> &'static str {
        Box::leak(self.name.clone().into_boxed_str())
    }

    fn description(&self) -> &'static str {
        Box::leak(self.description.clone().into_boxed_str())
    }

    fn kind(&self) -> TransformationKind {
        self.composed_kind
    }

    fn apply(&self, from: &A) -> C {
        let intermediate = self.t1.apply(from);
        self.t2.apply(&intermediate)
    }
}
