//! Example 3 (RP-002 Acceptance): Numerical Xi Evaluation as Metric Approximation (`APPROXIMATE`).
//!
//! Evaluates the Riemann / AMDE dynamical $\xi$-functional:
//! $$\xi(s) = \frac{1}{2} s (s - 1) \pi^{-s/2} \Gamma(s/2) \zeta(s)$$
//! with certified error bound $\epsilon > 0$ and metric tolerance verification.
//!
//! For $s > 1$, $\zeta(s)$ is approximated by partial sum $S_N(s) = \sum_{n=1}^N n^{-s}$
//! with rigorous truncation remainder bound:
//! $$R_N(s) = \sum_{n=N+1}^\infty n^{-s} \le \int_N^\infty x^{-s} dx = \frac{N^{1-s}}{s - 1}$$
//!
//! Contract:
//! - Metric error bound: $| \hat{\xi}(s) - \xi(s) | \le \epsilon$.
//! - Certified tolerance: $\epsilon \le \tau$.
//! - Reflection invariance: $\xi(s) = \xi(1 - s)$ within tolerance bounds.

use crate::object::{InformationFunctional, NavigatorObject, RelationalStructure, Signature, Topology};
use crate::transformation::{ApproximateTransformation, Transformation, TransformationKind};
use std::f64::consts::PI;
use std::fmt::{self, Display, Formatter};

/// Input coordinate $s \in \mathbb{R}$ for $\xi(s)$ evaluation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnalyticCoordinate {
    pub s: f64,
}

impl Display for AnalyticCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "s={:.4}", self.s)
    }
}

/// Numerical output of $\xi(s)$ evaluation with certified error interval.
#[derive(Debug, Clone, PartialEq)]
pub struct NumericalXiResult {
    pub s: f64,
    pub xi_value: f64,
    pub certified_error: f64,
    pub num_terms: usize,
}

impl Display for NumericalXiResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "xi({:.4}) = {:.8} +/- {:.2e} (N={})",
            self.s, self.xi_value, self.certified_error, self.num_terms
        )
    }
}

/// Lanczos gamma function approximation for real arguments $z > 0$.
/// Standard 7-coefficient Lanczos formula with precision ~ 1e-11.
pub fn lanczos_gamma(z: f64) -> f64 {
    if z < 0.5 {
        // Reflection formula: Gamma(z) * Gamma(1 - z) = pi / sin(pi * z)
        let sin_pi_z = (PI * z).sin();
        if sin_pi_z.abs() < 1e-15 {
            return f64::INFINITY;
        }
        return PI / (sin_pi_z * lanczos_gamma(1.0 - z));
    }

    let z = z - 1.0;
    const P: [f64; 9] = [
        0.99999999999980993,
        676.5203681218851,
        -1259.1392167224028,
        771.32342877765313,
        -176.61502916214059,
        12.507343278686905,
        -0.13857109526572012,
        9.9843695780195716e-6,
        1.5056327351493116e-7,
    ];

    let mut x = P[0];
    for (i, &p_val) in P.iter().enumerate().skip(1) {
        x += p_val / (z + i as f64);
    }

    let t = z + 7.5;
    (2.0 * PI).sqrt() * t.powf(z + 0.5) * (-t).exp() * x
}

/// Evaluates partial sum of zeta $\sum_{n=1}^N n^{-s}$ and integral remainder bound.
pub fn partial_zeta_with_bound(s: f64, num_terms: usize) -> (f64, f64) {
    assert!(s > 1.0, "s must be > 1.0 for standard Dirichlet series");
    let mut sum = 0.0;
    for n in 1..=num_terms {
        sum += (n as f64).powf(-s);
    }
    // Remainder bound: integral_N^inf x^(-s) dx = N^(1-s) / (s - 1)
    let n_f = num_terms as f64;
    let remainder = n_f.powf(1.0 - s) / (s - 1.0);
    (sum, remainder)
}

/// Computes $\xi(s) = \frac{1}{2} s (s - 1) \pi^{-s/2} \Gamma(s/2) \zeta(s)$ for $s > 1$.
pub fn compute_xi_with_error(s: f64, num_terms: usize) -> (f64, f64) {
    if s < 1.0 {
        // Use reflection: xi(s) = xi(1 - s)
        let s_dual = 1.0 - s;
        return compute_xi_with_error(s_dual, num_terms);
    }

    let (zeta_approx, zeta_error) = partial_zeta_with_bound(s, num_terms);
    let s_factor = 0.5 * s * (s - 1.0);
    let pi_factor = (-0.5 * s * PI.ln()).exp();
    let gamma_factor = lanczos_gamma(s / 2.0);

    let prefactor = s_factor * pi_factor * gamma_factor;
    let xi_val = prefactor * (zeta_approx + 0.5 * zeta_error);
    let certified_error = (prefactor * 0.5 * zeta_error).abs();

    (xi_val, certified_error)
}

/// Formal Navigator Object for the Analytic Coordinate Space.
pub fn analytic_space_object() -> NavigatorObject<AnalyticCoordinate> {
    NavigatorObject::new(
        "OBJ-ANALYTIC-XI",
        "Analytic Coordinate Domain (R)",
        "Domain of real coordinate arguments s for xi-functional evaluation",
        RelationalStructure::new(
            "OrderedField",
            "Real line with standard ordering and arithmetic field operations",
            &["total_order", "field_axioms", "reflection_symmetry"],
        ),
        Signature::new(
            &["eval_xi", "reflect(1-s)", "deriv", "scale"],
            "Analytic operators on coordinates",
        ),
        Topology::metric(1e-12, "StandardRealMetric", "d(s1, s2) = |s1 - s2|"),
        InformationFunctional::new(
            "CoordinateMagnitude",
            "I(s) = |s|",
            |c: &AnalyticCoordinate| c.s.abs(),
        ),
    )
}

/// Formal Navigator Object for the Numerical Output Space with Tolerance $\tau$.
pub fn numerical_xi_object(tolerance: f64) -> NavigatorObject<NumericalXiResult> {
    NavigatorObject::new(
        "OBJ-NUMERICAL-XI",
        "Numerical Xi Result Space (Float + Tolerance)",
        "Space of verified floating-point approximations with certified error bounds",
        RelationalStructure::new(
            "ApproximateIntervalLattice",
            "Interval arithmetic containment [x - eps, x + eps]",
            &["interval_inclusion", "certified_bound"],
        ),
        Signature::new(
            &["value", "error_bound", "check_tolerance", "interval"],
            "Operations on certified numerical results",
        ),
        Topology::metric(tolerance, "CertifiedErrorMetric", "d(x, true) <= epsilon"),
        InformationFunctional::new(
            "PrecisionInformation",
            "I(res) = -ln(max(error, 1e-16)) measuring bits of verified precision",
            |res: &NumericalXiResult| -(res.certified_error.max(1e-16)).ln(),
        ),
    )
}

/// The typed approximate transformation: $\text{AnalyticCoordinate} \xrightarrow{\mathrm{APPROXIMATE}} \text{NumericalXiResult}$.
#[derive(Debug, Clone)]
pub struct NumericalXiApproximate {
    pub num_terms: usize,
    pub max_tolerance: f64,
}

impl NumericalXiApproximate {
    pub fn new(num_terms: usize, max_tolerance: f64) -> Self {
        Self {
            num_terms,
            max_tolerance,
        }
    }
}

impl Transformation<AnalyticCoordinate, NumericalXiResult> for NumericalXiApproximate {
    fn name(&self) -> &'static str {
        "NumericalXiApproximate"
    }

    fn description(&self) -> &'static str {
        "Evaluates Riemann xi(s) with partial sum and certified integral remainder bound"
    }

    fn kind(&self) -> TransformationKind {
        TransformationKind::Approximate
    }

    fn apply(&self, from: &AnalyticCoordinate) -> NumericalXiResult {
        let (xi_value, certified_error) = compute_xi_with_error(from.s, self.num_terms);
        NumericalXiResult {
            s: from.s,
            xi_value,
            certified_error,
            num_terms: self.num_terms,
        }
    }
}

impl ApproximateTransformation<AnalyticCoordinate, NumericalXiResult> for NumericalXiApproximate {
    fn approximate_with_bound(&self, from: &AnalyticCoordinate) -> (NumericalXiResult, f64) {
        let res = self.apply(from);
        let err = res.certified_error;
        (res, err)
    }

    fn tolerance(&self) -> f64 {
        self.max_tolerance
    }
}
