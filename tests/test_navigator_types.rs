//! Unit tests for Navigator Object structure $(X, R, \Sigma, \tau, I)$ and Transformation Category Algebra.

use amde_navigator::examples::*;
use amde_navigator::transformation::*;

#[test]
fn test_navigator_object_creation_and_descriptor() {
    let obj = natural_number_object();
    assert_eq!(obj.id, "OBJ-NAT-MULT");
    assert_eq!(obj.relation.name, "MultiplicativeMonoid");
    assert!(obj.topology.is_exact());
    assert_eq!(obj.topology.tolerance(), 0.0);

    let desc = obj.descriptor();
    assert!(desc.to_string().contains("OBJ-NAT-MULT"));
    assert!(desc.to_string().contains("MultiplicativeMonoid"));
}

#[test]
fn test_transformation_kind_composition_algebra() {
    use TransformationKind::*;

    // ISO is the identity of the composition monoid
    assert_eq!(TransformationKind::compose(Iso, Iso), Iso);
    assert_eq!(TransformationKind::compose(Iso, Adjoint), Adjoint);
    assert_eq!(TransformationKind::compose(Adjoint, Iso), Adjoint);
    assert_eq!(TransformationKind::compose(Iso, Lossy), Lossy);
    assert_eq!(TransformationKind::compose(Lossy, Iso), Lossy);
    assert_eq!(TransformationKind::compose(Iso, Approximate), Approximate);
    assert_eq!(TransformationKind::compose(Approximate, Iso), Approximate);

    // ADJOINT compositions
    assert_eq!(TransformationKind::compose(Adjoint, Adjoint), Adjoint);
    assert_eq!(TransformationKind::compose(Adjoint, Lossy), Lossy);
    assert_eq!(TransformationKind::compose(Lossy, Adjoint), Lossy);

    // LOSSY compositions
    assert_eq!(TransformationKind::compose(Lossy, Lossy), Lossy);

    // APPROXIMATE absorbs all kinds
    assert_eq!(TransformationKind::compose(Adjoint, Approximate), Approximate);
    assert_eq!(TransformationKind::compose(Approximate, Adjoint), Approximate);
    assert_eq!(TransformationKind::compose(Lossy, Approximate), Approximate);
    assert_eq!(TransformationKind::compose(Approximate, Lossy), Approximate);
    assert_eq!(TransformationKind::compose(Approximate, Approximate), Approximate);
}

#[test]
fn test_composed_transformation_execution() {
    let iso = PrimeFactorizationIso;
    let trunc = ValuationTruncationLossy::new(2); // Primes 2, 3
    let composed = ComposedTransformation::new(iso, trunc);

    assert_eq!(composed.kind(), TransformationKind::Lossy);

    let n = NaturalNumber::new(120).unwrap(); // 120 = 2^3 * 3^1 * 5^1
    let out = composed.apply(&n);

    assert_eq!(out.primes, vec![2, 3]);
    assert_eq!(out.exponents, vec![3, 1]);
}
