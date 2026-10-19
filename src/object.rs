//! Core mathematical formalization of Navigator Objects: $(X, R, \Sigma, \tau, I)$
//!
//! Based on PRD-26 (AMDE Core Navigator: Typed Transformation Graph & Claim Registry).
//!
//! Each Navigator object $\mathcal{O} = (X, R, \Sigma, \tau, I)$ consists of:
//! - $X$: Carrier state space / domain representation.
//! - $R$: Relational and algebraic structure (e.g. monoid, lattice, order).
//! - $\Sigma$: Signature of transition operators and generators.
//! - $\tau$: Topology / metric / tolerance structure (distance, bounds).
//! - $I$: Information functional / valuation / conserved invariant.

use std::fmt::{self, Display, Formatter};

/// Trait implemented by any state carrier $X$ in the Navigator category.
pub trait Carrier: Clone + std::fmt::Debug + PartialEq {}

impl<T: Clone + std::fmt::Debug + PartialEq> Carrier for T {}

/// Relational and algebraic structure $R$ of the Navigator object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalStructure {
    pub name: &'static str,
    pub description: &'static str,
    pub axioms: &'static [&'static str],
}

impl RelationalStructure {
    pub const fn new(
        name: &'static str,
        description: &'static str,
        axioms: &'static [&'static str],
    ) -> Self {
        Self {
            name,
            description,
            axioms,
        }
    }
}

/// Signature $\Sigma$ of admissible operators and generators acting on carrier $X$.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub generators: &'static [&'static str],
    pub description: &'static str,
}

impl Signature {
    pub const fn new(generators: &'static [&'static str], description: &'static str) -> Self {
        Self {
            generators,
            description,
        }
    }
}

/// Topology / metric structure $\tau$ defining metric distances, tolerances, and bounds.
#[derive(Debug, Clone, PartialEq)]
pub enum TopologyKind {
    /// Discrete topology: distance is 0 if equal, 1 otherwise. Error tolerance is 0.0.
    DiscreteExact,
    /// Metric space with metric distance and certified epsilon tolerance.
    MetricTolerance { epsilon: f64 },
    /// Truncation cutoff topology with prime/index depth $K$.
    TruncationCutoff { depth: usize },
    /// Interval bound topology $[\text{min}, \text{max}]$.
    IntervalBounds { lower: f64, upper: f64 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Topology {
    pub kind: TopologyKind,
    pub metric_name: &'static str,
    pub description: &'static str,
}

impl Topology {
    pub const fn discrete(metric_name: &'static str, description: &'static str) -> Self {
        Self {
            kind: TopologyKind::DiscreteExact,
            metric_name,
            description,
        }
    }

    pub const fn metric(epsilon: f64, metric_name: &'static str, description: &'static str) -> Self {
        Self {
            kind: TopologyKind::MetricTolerance { epsilon },
            metric_name,
            description,
        }
    }

    pub const fn truncation(depth: usize, metric_name: &'static str, description: &'static str) -> Self {
        Self {
            kind: TopologyKind::TruncationCutoff { depth },
            metric_name,
            description,
        }
    }

    pub fn tolerance(&self) -> f64 {
        match self.kind {
            TopologyKind::DiscreteExact => 0.0,
            TopologyKind::MetricTolerance { epsilon } => epsilon,
            TopologyKind::TruncationCutoff { .. } => 0.0,
            TopologyKind::IntervalBounds { lower, upper } => (upper - lower).abs(),
        }
    }

    pub fn is_exact(&self) -> bool {
        matches!(self.kind, TopologyKind::DiscreteExact)
    }
}

/// Information functional $I: X \to \mathbb{R}$ measuring entropy, height, valuation norm, or conserved invariant.
#[derive(Clone)]
pub struct InformationFunctional<X> {
    pub name: &'static str,
    pub description: &'static str,
    pub eval_fn: fn(&X) -> f64,
}

impl<X> InformationFunctional<X> {
    pub const fn new(
        name: &'static str,
        description: &'static str,
        eval_fn: fn(&X) -> f64,
    ) -> Self {
        Self {
            name,
            description,
            eval_fn,
        }
    }

    pub fn evaluate(&self, x: &X) -> f64 {
        (self.eval_fn)(x)
    }
}

impl<X> std::fmt::Debug for InformationFunctional<X> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("InformationFunctional")
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}

/// The formal Navigator Object tuple $\mathcal{O} = (X, R, \Sigma, \tau, I)$.
#[derive(Debug, Clone)]
pub struct NavigatorObject<X: Carrier> {
    pub id: &'static str,
    pub name: &'static str,
    pub domain_description: &'static str,
    pub relation: RelationalStructure,
    pub signature: Signature,
    pub topology: Topology,
    pub information: InformationFunctional<X>,
}

impl<X: Carrier> NavigatorObject<X> {
    pub fn new(
        id: &'static str,
        name: &'static str,
        domain_description: &'static str,
        relation: RelationalStructure,
        signature: Signature,
        topology: Topology,
        information: InformationFunctional<X>,
    ) -> Self {
        Self {
            id,
            name,
            domain_description,
            relation,
            signature,
            topology,
            information,
        }
    }

    pub fn evaluate_information(&self, x: &X) -> f64 {
        self.information.evaluate(x)
    }

    pub fn descriptor(&self) -> ObjectDescriptor {
        ObjectDescriptor {
            id: self.id.to_string(),
            name: self.name.to_string(),
            carrier_space: self.domain_description.to_string(),
            relation: self.relation.name.to_string(),
            axioms: self.relation.axioms.iter().map(|s| s.to_string()).collect(),
            generators: self.signature.generators.iter().map(|s| s.to_string()).collect(),
            topology: format!("{:?} ({})", self.topology.kind, self.topology.metric_name),
            tolerance: self.topology.tolerance(),
            information_functional: self.information.name.to_string(),
        }
    }
}

/// Serializable descriptor summarizing $(X, R, \Sigma, \tau, I)$ for claims and manifests.
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectDescriptor {
    pub id: String,
    pub name: String,
    pub carrier_space: String,
    pub relation: String,
    pub axioms: Vec<String>,
    pub generators: Vec<String>,
    pub topology: String,
    pub tolerance: f64,
    pub information_functional: String,
}

impl Display for ObjectDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Object[{}] '{}': X={}, R={}, Σ={:?}, τ={}, I={}",
            self.id,
            self.name,
            self.carrier_space,
            self.relation,
            self.generators,
            self.topology,
            self.information_functional
        )
    }
}
