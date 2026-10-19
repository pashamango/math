//! # AMDE Core Navigator: Type System Specification & Reference Implementation
//!
//! **Linear Ticket**: `MIN-79`  
//! **PRD Link**: `PRD-26 (amde-core-navigator-typed-transformation-graph-claim-registry)`  
//! **Acceptance Reference**: `RP-002`  
//!
//! ## Overview
//!
//! The Navigator models mathematical spaces and computational frames as formal objects
//! $\mathcal{O} = (X, R, \Sigma, \tau, I)$:
//! - **$X$ (Carrier)**: Domain state space (e.g. $\mathbb{N}^+$, $\bigoplus \mathbb{N}$, $\mathbb{N}^K$, $\mathbb{R}$).
//! - **$R$ (Relation)**: Algebraic and relational structure (e.g. multiplicative monoid, lattice, order).
//! - **$\Sigma$ (Signature)**: Admissible transition operators and dynamic generators.
//! - **$\tau$ (Topology)**: Metric distance, tolerance bounds, and truncation horizon.
//! - **$I$ (Information)**: Invariant functional measuring entropy, height, or valuation norm.
//!
//! ## Typed Transformations
//!
//! Morphisms between Navigator objects are typed into four canonical categories:
//!
//! 1. **`ISO` (Isomorphism)**:
//!    Exact, bijective, zero information loss ($\Delta I = 0$), zero metric error ($\epsilon = 0$).  
//!    *RP-002 Canonical Example*: Prime Factorization $\mathbb{N}^+ \leftrightarrow \bigoplus \mathbb{N}$.
//!
//! 2. **`ADJOINT` (Adjunction / Galois Connection)**:
//!    Morphism pair $(L, R)$ satisfying $L(w) \le v \iff w \le R(v)$ with unit/counit bounds.  
//!    *Canonical Example*: Canonical embedding / retraction between truncated and full valuations.
//!
//! 3. **`LOSSY` (Lossy Projection / Truncation)**:
//!    Non-injective forgetful map, positive information loss ($\Delta I > 0$), non-trivial fiber.  
//!    *RP-002 Canonical Example*: Valuation Truncation $\bigoplus \mathbb{N} \to \mathbb{N}^K$.
//!
//! 4. **`APPROXIMATE` (Metric Approximation)**:
//!    Numerical embedding with certified error bound $d(f(x), \mathrm{target}) \le \epsilon \le \tau$.  
//!    *RP-002 Canonical Example*: Numerical $\xi(s)$ evaluation with remainder bound.
//!
//! ## Claim Registry
//!
//! Every transition generates an immutable `ClaimReceipt` cryptographically signed with
//! SHA-256 for auditing and kernel verification.

pub mod claim;
pub mod examples;
pub mod object;
pub mod transformation;

pub use claim::{sha256, ClaimReceipt, ClaimRegistry};
pub use object::{
    Carrier, InformationFunctional, NavigatorObject, ObjectDescriptor, RelationalStructure,
    Signature, Topology, TopologyKind,
};
pub use transformation::{
    AdjointTransformation, ApproximateTransformation, ComposedTransformation, IsoTransformation,
    LossyTransformation, RoundtripError, ToleranceError, Transformation, TransformationKind,
};
