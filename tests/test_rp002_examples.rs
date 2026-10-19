//! Exhaustive acceptance verification for RP-002:
//! - Prime Factorization = ISO
//! - Truncation = LOSSY
//! - Numerical Xi = APPROXIMATE
//! - Valuation Adjunction = ADJOINT

use amde_navigator::examples::*;
use amde_navigator::transformation::{
    AdjointTransformation, ApproximateTransformation, IsoTransformation, LossyTransformation,
};

#[test]
fn test_rp002_prime_factorization_iso() {
    let iso = PrimeFactorizationIso;

    // Test a wide range of positive integers
    let test_suite: Vec<u64> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        12, 16, 25, 30, 60, 84, 100,
        360, 840, 2310, 30030,
        104729, // 10,000th prime
        1_000_000, 1_000_003,
    ];

    for val in test_suite {
        let n = NaturalNumber::new(val).unwrap();
        let profile = iso.forward(&n);
        let recovered = iso.backward(&profile);

        assert_eq!(n, recovered, "Round-trip identity failed for n={}", val);
        assert!(iso.verify_roundtrip(&n).is_ok());
        assert!(iso.verify_dual_roundtrip(&profile).is_ok());

        // Information functional conservation: ln(n) == sum v_p * ln(p)
        let info_nat = (val as f64).ln();
        let info_prof = profile.logarithmic_information();
        assert!(
            (info_nat - info_prof).abs() < 1e-9,
            "Information mismatch for n={}: {} vs {}",
            val,
            info_nat,
            info_prof
        );
    }
}

#[test]
fn test_rp002_truncation_lossy() {
    let trunc = ValuationTruncationLossy::new(3); // Cutoff at primes [2, 3, 5]
    assert_eq!(trunc.prime_basis, vec![2, 3, 5]);

    // Test information loss formula: Delta I = sum_{p > 5} v_p ln(p)
    let n1 = 60; // 2^2 * 3^1 * 5^1 -> primes in basis -> loss = 0
    let p1 = factorize_u64(n1);
    assert_eq!(trunc.information_loss(&p1), 0.0);

    let n2 = 84; // 2^2 * 3^1 * 7^1 -> prime 7 discarded -> loss = ln(7)
    let p2 = factorize_u64(n2);
    let expected_loss2 = (7.0f64).ln();
    assert!(
        (trunc.information_loss(&p2) - expected_loss2).abs() < 1e-10,
        "Loss should equal ln(7)"
    );

    let n3 = 924; // 2^2 * 3 * 7 * 11 -> primes 7, 11 discarded -> loss = ln(7) + ln(11)
    let p3 = factorize_u64(n3);
    let expected_loss3 = (7.0f64).ln() + (11.0f64).ln();
    assert!(
        (trunc.information_loss(&p3) - expected_loss3).abs() < 1e-10,
        "Loss should equal ln(7) + ln(11)"
    );

    // Non-injectivity & Fiber test
    let domain: Vec<ValuationProfile> = vec![
        factorize_u64(12),  // 2^2 * 3
        factorize_u64(60),  // 2^2 * 3 * 5
        factorize_u64(84),  // 2^2 * 3 * 7
        factorize_u64(132), // 2^2 * 3 * 11
        factorize_u64(924), // 2^2 * 3 * 7 * 11
    ];

    let target = trunc.project(&domain[0]); // projection of 12: [2^2, 3^1, 5^0]
    let fiber = trunc.fiber(&target, &domain);

    // 12, 84, 132, 924 all have identical leading components [2^2, 3^1, 5^0]
    assert_eq!(fiber.len(), 4);
    assert!(fiber.contains(&domain[0])); // 12
    assert!(fiber.contains(&domain[2])); // 84
    assert!(fiber.contains(&domain[3])); // 132
    assert!(fiber.contains(&domain[4])); // 924
}

#[test]
fn test_rp002_numerical_xi_approximate() {
    let approx = NumericalXiApproximate::new(2000, 1e-3);

    // Test points s >= 2.0 where standard Dirichlet remainder is sharp
    let points = [2.0, 2.5, 3.0, 4.0, 5.0, 8.0];
    for &s in &points {
        let coord = AnalyticCoordinate { s };
        let (res, err) = approx.approximate_with_bound(&coord);

        assert!(res.xi_value > 0.0, "xi(s) must be positive on real line");
        assert!(err > 0.0, "error bound must be strictly positive");
        assert!(
            approx.verify_tolerance(err).is_ok(),
            "Error {:.2e} exceeds tolerance {:.2e} at s={}",
            err,
            approx.tolerance(),
            s
        );

        // Functional reflection equation: xi(s) == xi(1 - s)
        let coord_reflected = AnalyticCoordinate { s: 1.0 - s };
        let (res_reflected, _) = approx.approximate_with_bound(&coord_reflected);
        let discrepancy = (res.xi_value - res_reflected.xi_value).abs();
        assert!(
            discrepancy < 1e-9,
            "Reflection symmetry failed at s={}: {} vs {}",
            s,
            res.xi_value,
            res_reflected.xi_value
        );
    }
}

#[test]
fn test_adjoint_bounds_galois_connection() {
    let adjoint = ValuationAdjointPair::new(3); // Basis primes: 2, 3, 5

    // 1. Unit condition (Section / Identity): R(L(w)) == w
    let w = TruncatedValuationVector::new(vec![2, 3, 5], vec![4, 2, 1]);
    let embedded = adjoint.left_adjoint(&w);
    let retracted = adjoint.right_adjoint(&embedded);
    assert_eq!(w, retracted, "Unit condition R(L(w)) == w failed");

    // 2. Counit condition: L(R(v)) <= v
    let v = factorize_u64(2310); // 2 * 3 * 5 * 7 * 11
    let re_embedded = adjoint.left_adjoint(&adjoint.right_adjoint(&v));
    assert!(
        re_embedded.leq(&v),
        "Counit condition L(R(v)) <= v failed"
    );

    // 3. Adjunction equivalence: L(w) <= v <=> w <= R(v)
    let test_cases = vec![
        // (w_exponents, full_n, should_be_less)
        (vec![1, 1, 1], 30, true),       // 30 has 2, 3, 5 -> w=[1,1,1] <= [1,1,1]
        (vec![1, 1, 1], 210, true),      // 210 has 2, 3, 5, 7 -> w=[1,1,1] <= [1,1,1]
        (vec![2, 1, 1], 30, false),      // 2^2 does not divide 30 (w_2 = 2 > 1)
        (vec![1, 2, 0], 90, true),       // 90 = 2 * 3^2 * 5 -> w=[1,2,0] <= [1,2,1]
        (vec![1, 2, 2], 90, false),      // 5^2 does not divide 90 (w_5 = 2 > 1)
        (vec![0, 0, 0], 1, true),        // 1 <= 1
        (vec![1, 0, 0], 1, false),       // 2 does not divide 1
    ];

    for (w_exps, full_n, expected) in test_cases {
        let w_vec = TruncatedValuationVector::new(vec![2, 3, 5], w_exps);
        let v_prof = factorize_u64(full_n);

        let left_cond = adjoint.left_adjoint(&w_vec).leq(&v_prof);
        let right_retracted = adjoint.right_adjoint(&v_prof);

        // Componentwise comparison on truncated space
        let mut right_cond = true;
        for i in 0..3 {
            if w_vec.exponents[i] > right_retracted.exponents[i] {
                right_cond = false;
                break;
            }
        }

        assert_eq!(
            left_cond, right_cond,
            "Adjunction equivalence L(w) <= v <=> w <= R(v) failed for w={:?}, n={}",
            w_vec, full_n
        );
        assert_eq!(left_cond, expected);
    }
}
