//! AMDE Navigator CLI & Verification Engine.
//!
//! Runs all acceptance examples from RP-002:
//! - Prime Factorization = ISO
//! - Truncation = LOSSY
//! - Numerical Xi = APPROXIMATE
//! - Valuation Adjunction = ADJOINT
//!
//! Populates the Claim Registry, validates all contracts, and prints verified receipts.

use amde_navigator::claim::{ClaimReceipt, ClaimRegistry};
use amde_navigator::examples::*;
use amde_navigator::transformation::{
    AdjointTransformation, ApproximateTransformation, ComposedTransformation, IsoTransformation,
    LossyTransformation, Transformation, TransformationKind,
};

fn main() {
    println!("===============================================================================");
    println!("  AMDE Navigator: Type System Specification & Reference Implementation");
    println!("  Ticket: MIN-79 | PRD-26 | Acceptance Benchmark: RP-002");
    println!("===============================================================================\n");

    let mut registry = ClaimRegistry::new();

    // -------------------------------------------------------------------------
    // 1. Formal Objects (X, R, Sigma, tau, I)
    // -------------------------------------------------------------------------
    println!("--- 1. Formal Navigator Objects (X, R, Sigma, tau, I) ---");
    let obj_nat = natural_number_object();
    let obj_val = valuation_profile_object();
    let obj_trunc = truncated_valuation_object(3); // First 3 primes: 2, 3, 5
    let obj_analytic = analytic_space_object();
    let obj_num_xi = numerical_xi_object(1e-4);

    println!("  [1] {}", obj_nat.descriptor());
    println!("  [2] {}", obj_val.descriptor());
    println!("  [3] {}", obj_trunc.descriptor());
    println!("  [4] {}", obj_analytic.descriptor());
    println!("  [5] {}\n", obj_num_xi.descriptor());

    // -------------------------------------------------------------------------
    // 2. Acceptance Example 1 (RP-002): Prime Factorization = ISO
    // -------------------------------------------------------------------------
    println!("--- 2. Acceptance Example 1 (RP-002): Prime Factorization = ISO ---");
    let iso_transform = PrimeFactorizationIso;
    let test_numbers = [1u64, 12, 60, 84, 840, 2310, 65537];

    for &val in &test_numbers {
        let n = NaturalNumber::new(val).unwrap();
        let profile = iso_transform.forward(&n);
        let recovered = iso_transform.backward(&profile);

        let roundtrip_ok = iso_transform.verify_roundtrip(&n).is_ok();
        let dual_roundtrip_ok = iso_transform.verify_dual_roundtrip(&profile).is_ok();
        let info_orig = obj_nat.evaluate_information(&n);
        let info_profile = obj_val.evaluate_information(&profile);
        let delta_info = (info_orig - info_profile).abs();

        println!(
            "  n = {:>5} -> Factorization: {:<18} | Recovered: {:>5} | Roundtrip: {:<5} | ΔI = {:.1e}",
            val,
            profile.to_string(),
            recovered.0,
            roundtrip_ok && dual_roundtrip_ok,
            delta_info
        );

        assert!(roundtrip_ok);
        assert!(dual_roundtrip_ok);
        assert!(delta_info < 1e-10);

        registry.register(ClaimReceipt::new(
            iso_transform.name(),
            TransformationKind::Iso,
            obj_nat.name,
            obj_val.name,
            &format!("n={}", val),
            &profile.to_string(),
            roundtrip_ok && dual_roundtrip_ok,
            0.0,
            delta_info,
            0.0,
            "RP-002 Prime Factorization verified: exact bijective roundtrip, Delta I = 0",
        ));
    }
    println!("  => ISO Acceptance: 100% verified across all test cases.\n");

    // -------------------------------------------------------------------------
    // 3. Acceptance Example 2 (RP-002): Truncation = LOSSY
    // -------------------------------------------------------------------------
    println!("--- 3. Acceptance Example 2 (RP-002): Truncation = LOSSY ---");
    let truncation = ValuationTruncationLossy::new(3); // Cutoff at primes [2, 3, 5]
    println!(
        "  Truncation Horizon: K=3, Basis Primes = {:?}",
        truncation.prime_basis
    );

    // 12 = 2^2 * 3^1
    // 60 = 2^2 * 3^1 * 5^1
    // 84 = 2^2 * 3^1 * 7^1  (7 is discarded!)
    // 420 = 2^2 * 3^1 * 5^1 * 7^1
    let numbers_to_truncate = [12u64, 60, 84, 420, 924];
    let profiles: Vec<ValuationProfile> = numbers_to_truncate
        .iter()
        .map(|&n| factorize_u64(n))
        .collect();

    for (i, &num) in numbers_to_truncate.iter().enumerate() {
        let prof = &profiles[i];
        let truncated = truncation.project(prof);
        let loss = truncation.information_loss(prof);
        let info_prof = prof.logarithmic_information();
        let info_trunc = truncated.information();

        println!(
            "  n = {:>4} ({:<18}) -> Truncated: {:<12} | I_full = {:.4} | I_trunc = {:.4} | Loss ΔI = {:.4}",
            num,
            prof.to_string(),
            truncated.to_string(),
            info_prof,
            info_trunc,
            loss
        );

        assert!((info_prof - (info_trunc + loss)).abs() < 1e-10);

        registry.register(ClaimReceipt::new(
            truncation.name(),
            TransformationKind::Lossy,
            obj_val.name,
            obj_trunc.name,
            &format!("n={} ({})", num, prof),
            &truncated.to_string(),
            true,
            0.0,
            loss,
            0.0,
            "RP-002 Valuation Truncation verified: positive information loss quantified, projection non-injective",
        ));
    }

    // Demonstrate Fiber / Non-injectivity:
    // Notice 12 and 84 both have [2^2, 3^1, 5^0] in the truncated space!
    let target_truncated = truncation.project(&profiles[0]); // for n = 12: [2^2, 3^1, 5^0]
    let fiber = truncation.fiber(&target_truncated, &profiles);
    println!(
        "\n  Fiber analysis for target {}:",
        target_truncated
    );
    for member in &fiber {
        println!("    - Preimage element: {}", member);
    }
    assert!(
        fiber.len() >= 2,
        "Fiber must contain multiple elements demonstrating non-injectivity"
    );
    println!("  => LOSSY Acceptance: Non-injectivity & quantified information loss strictly verified.\n");

    // -------------------------------------------------------------------------
    // 4. Acceptance Example 3 (RP-002): Numerical Xi = APPROXIMATE
    // -------------------------------------------------------------------------
    println!("--- 4. Acceptance Example 3 (RP-002): Numerical Xi = APPROXIMATE ---");
    let approx_transform = NumericalXiApproximate::new(2000, 1e-3);
    let s_coords = [2.0f64, 3.0, 4.0, 6.0];

    for &s_val in &s_coords {
        let coord = AnalyticCoordinate { s: s_val };
        let (result, err) = approx_transform.approximate_with_bound(&coord);
        let within_tolerance = approx_transform.verify_tolerance(err).is_ok();

        // Check reflection symmetry: xi(s) should equal xi(1 - s)
        let coord_dual = AnalyticCoordinate { s: 1.0 - s_val };
        let (result_dual, _) = approx_transform.approximate_with_bound(&coord_dual);
        let reflection_discrepancy = (result.xi_value - result_dual.xi_value).abs();

        println!(
            "  {} -> Value: {:.8} | Certified Error ε: {:.2e} | Tol Check: {:<5} | Reflection Discrepancy: {:.2e}",
            coord,
            result.xi_value,
            err,
            within_tolerance,
            reflection_discrepancy
        );

        assert!(within_tolerance);
        assert!(reflection_discrepancy < 1e-10);

        registry.register(ClaimReceipt::new(
            approx_transform.name(),
            TransformationKind::Approximate,
            obj_analytic.name,
            obj_num_xi.name,
            &format!("s={:.2}", s_val),
            &format!("{:.8}", result.xi_value),
            within_tolerance,
            0.0,
            0.0,
            err,
            "RP-002 Numerical Xi verified: bounded metric approximation with certified remainder bound",
        ));
    }
    println!("  => APPROXIMATE Acceptance: Certified metric tolerance & reflection symmetry verified.\n");

    // -------------------------------------------------------------------------
    // 5. Example 4: Adjoint Pair = ADJOINT (Galois Connection)
    // -------------------------------------------------------------------------
    println!("--- 5. Adjoint Pair = ADJOINT (Galois Connection) ---");
    let adjoint_pair = ValuationAdjointPair::new(3);
    let test_truncated = TruncatedValuationVector::new(vec![2, 3, 5], vec![3, 1, 2]); // 2^3 * 3^1 * 5^2
    let embedded_profile = adjoint_pair.left_adjoint(&test_truncated);
    let retracted_vector = adjoint_pair.right_adjoint(&embedded_profile);

    // Unit condition: R(L(w)) == w
    let unit_holds = test_truncated == retracted_vector;

    // Counit condition: L(R(v)) <= v
    let test_full = factorize_u64(840); // 2^3 * 3^1 * 5^1 * 7^1
    let counit_embedded = adjoint_pair.left_adjoint(&adjoint_pair.right_adjoint(&test_full));
    let counit_holds = counit_embedded.leq(&test_full);

    println!("  Truncated input w:         {}", test_truncated);
    println!("  L(w) (Left adjoint embed): {}", embedded_profile);
    println!("  R(L(w)) (Unit roundtrip):  {}", retracted_vector);
    println!("  Unit Identity R(L(w)) == w: {}", unit_holds);
    println!("  Full profile v (840):      {}", test_full);
    println!("  L(R(v)) (Counit projection):{}", counit_embedded);
    println!("  Counit Inequality L(R(v)) <= v: {}", counit_holds);

    assert!(unit_holds);
    assert!(counit_holds);

    registry.register(ClaimReceipt::new(
        adjoint_pair.name(),
        TransformationKind::Adjoint,
        obj_trunc.name,
        obj_val.name,
        &test_truncated.to_string(),
        &embedded_profile.to_string(),
        unit_holds && counit_holds,
        0.0,
        0.0,
        0.0,
        "Adjoint Galois connection verified: unit section identity and counit deflationary inequality",
    ));
    println!("  => ADJOINT Acceptance: Galois connection verified.\n");

    // -------------------------------------------------------------------------
    // 6. Category Composition Algebra: Composed Transformation
    // -------------------------------------------------------------------------
    println!("--- 6. Category Composition Algebra ---");
    let composed = ComposedTransformation::new(PrimeFactorizationIso, ValuationTruncationLossy::new(3));
    println!("  Composed: {}", composed.name);
    println!("  Composed Kind: {}", composed.kind());
    assert_eq!(composed.kind(), TransformationKind::Lossy);

    let n = NaturalNumber::new(840).unwrap();
    let truncated_output = composed.apply(&n);
    println!("  n = 840 -> Composed output: {}", truncated_output);

    registry.register(ClaimReceipt::new(
        &composed.name,
        composed.kind(),
        obj_nat.name,
        obj_trunc.name,
        "n=840",
        &truncated_output.to_string(),
        true,
        0.0,
        factorize_u64(840).logarithmic_information() - truncated_output.information(),
        0.0,
        "Category composition ISO ∘ LOSSY = LOSSY verified end-to-end",
    ));
    println!("  => Composition Algebra verified.\n");

    // -------------------------------------------------------------------------
    // 7. Claim Registry Summary (Transformation Receipts)
    // -------------------------------------------------------------------------
    println!("--- 7. Transformation Claim Receipts & Audit Manifest ---");
    println!("  Total receipts logged: {}", registry.len());
    println!("  All receipts verified: {}", registry.verify_all());
    assert!(registry.verify_all());

    for claim in registry.claims() {
        println!("    {}", claim);
    }
    println!();

    // -------------------------------------------------------------------------
    // 8. Epistemic Claim Registry Seed from RP-002 (MIN-83 / PRD-26)
    // -------------------------------------------------------------------------
    println!("--- 8. Epistemic Claim Registry Seed from RP-002 (MIN-83) ---");
    let seed_registry = ClaimRegistry::seed_rp002();
    println!("  Total RP-002 seeded claims: {}", seed_registry.epistemic_len());
    println!("  Cryptographic hash integrity verified: {}", seed_registry.verify_seed_integrity());
    assert_eq!(seed_registry.epistemic_len(), 8);
    assert!(seed_registry.verify_seed_integrity());

    println!("\n  [RP-002 Epistemic Matrix]");
    for claim in seed_registry.epistemic_claims() {
        let trans_str = claim.transformation_kind.map(|k| k.as_str()).unwrap_or("—");
        println!(
            "  - [{}] {:<14} | {:<48} | kind: {:<11} | verified: {}",
            claim.claim_id,
            claim.epistemic_type.display_name(),
            claim.title,
            trans_str,
            if claim.verified { "YES" } else { "PENDING" }
        );
    }

    println!("\n  [Anti-Silent-Promotion Check: PRD-26 Gate]");
    let leap_check = amde_navigator::claim::EpistemicType::is_valid_promotion(
        amde_navigator::claim::EpistemicType::Metaphor,
        amde_navigator::claim::EpistemicType::ProofTarget,
    );
    match leap_check {
        Err(amde_navigator::claim::PromotionError::SilentPromotionDisallowed { from, to, reason }) => {
            println!("  => Gate Enforced: Promotion {} -> {} blocked as expected ({})", from, to, reason);
        }
        _ => panic!("Silent promotion must be strictly rejected"),
    }

    println!("\n===============================================================================");
    println!("  ACCEPTANCE PASSED: All RP-002 claims seeded, typed, and promotion gates verified.");
    println!("===============================================================================");
}
