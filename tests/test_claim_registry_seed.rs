//! Integration tests for MIN-83: RP-002 Claim Registry Seeding and Epistemic Typing.
//!
//! Verifies:
//! - Complete seeding of RP-002 mathematical claims (8 canonical claims).
//! - Epistemic type distribution:
//!   - THEOREM: Prime factorization as reversible compiler; Gn->1/2
//!   - CONJECTURE: Golden scaling dynamically privileged; Collatz golden contraction
//!   - KNOWN THEOREM: Li positivity <=> RH
//!   - HEURISTIC: Syracuse log-drift 3/4
//!   - ANALOGY: Perelman transfer
//!   - PROOF TARGET: Collatz descent below start
//! - Strict promotion criteria and anti-silent-promotion enforcement (PRD-26).
//! - Cryptographic hash integrity and verifiable audit trails.
//! - Canonical JSON and Markdown exports.

use amde_navigator::claim::{
    ClaimRegistry, EpistemicClaim, EpistemicType, PromotionError,
};
use amde_navigator::transformation::TransformationKind;

#[test]
fn test_rp002_seed_completeness_and_epistemic_types() {
    let registry = ClaimRegistry::seed_rp002();

    // 8 canonical claims seeded from RP-002
    assert_eq!(registry.epistemic_len(), 8);
    assert!(!registry.is_empty());
    assert!(registry.verify_seed_integrity());

    // 1. THEOREM: Prime Factorization as Reversible Compiler
    let c1 = registry.get_epistemic_claim("CLAIM-RP002-001").expect("Claim 1 must exist");
    assert_eq!(c1.epistemic_type, EpistemicType::Theorem);
    assert_eq!(c1.transformation_kind, Some(TransformationKind::Iso));
    assert!(c1.title.contains("Prime Factorization"));
    assert!(c1.verified);
    assert_eq!(c1.lean_symbol.as_deref(), Some("Amde.PrimeValuation.primeValuationMulEquiv"));

    // 2. THEOREM: Golden Widths Asymptotic Convergence (Gn -> 1/2)
    let c2 = registry.get_epistemic_claim("CLAIM-RP002-002").expect("Claim 2 must exist");
    assert_eq!(c2.epistemic_type, EpistemicType::Theorem);
    assert_eq!(c2.transformation_kind, Some(TransformationKind::Lossy));
    assert!(c2.title.contains("Golden Widths"));
    assert!(c2.verified);
    assert_eq!(c2.lean_symbol.as_deref(), Some("Amde.intersection_goldenSets"));

    // 3. CONJECTURE: Golden Scaling Dynamically Privileged
    let c3 = registry.get_epistemic_claim("CLAIM-RP002-003").expect("Claim 3 must exist");
    assert_eq!(c3.epistemic_type, EpistemicType::Conjecture);
    assert_eq!(c3.transformation_kind, Some(TransformationKind::Approximate));
    assert!(c3.title.contains("Golden Scaling"));
    assert!(!c3.verified);

    // 4. CONJECTURE: Collatz Golden Contraction
    let c4 = registry.get_epistemic_claim("CLAIM-RP002-004").expect("Claim 4 must exist");
    assert_eq!(c4.epistemic_type, EpistemicType::Conjecture);
    assert_eq!(c4.transformation_kind, Some(TransformationKind::Lossy));
    assert!(c4.title.contains("Collatz Golden Contraction"));
    assert!(!c4.verified);

    // 5. KNOWN THEOREM: Li Positivity Criterion Equivalence with RH
    let c5 = registry.get_epistemic_claim("CLAIM-RP002-005").expect("Claim 5 must exist");
    assert_eq!(c5.epistemic_type, EpistemicType::KnownTheorem);
    assert_eq!(c5.transformation_kind, Some(TransformationKind::Approximate));
    assert!(c5.title.contains("Li Positivity"));
    assert!(c5.verified);
    assert!(c5.literature_citation.as_ref().unwrap().contains("Xian-Jin Li"));

    // 6. HEURISTIC: Syracuse Heuristic Log-Drift 3/4
    let c6 = registry.get_epistemic_claim("CLAIM-RP002-006").expect("Claim 6 must exist");
    assert_eq!(c6.epistemic_type, EpistemicType::Heuristic);
    assert_eq!(c6.transformation_kind, Some(TransformationKind::Lossy));
    assert!(c6.title.contains("3/4"));
    assert!(!c6.verified);

    // 7. ANALOGY: Perelman Monotonicity Transfer
    let c7 = registry.get_epistemic_claim("CLAIM-RP002-007").expect("Claim 7 must exist");
    assert_eq!(c7.epistemic_type, EpistemicType::Analogy);
    assert_eq!(c7.transformation_kind, Some(TransformationKind::Adjoint));
    assert!(c7.title.contains("Perelman"));
    assert!(!c7.verified);

    // 8. PROOF TARGET: Collatz Descent Below Start (Finite Stopping Target)
    let c8 = registry.get_epistemic_claim("CLAIM-RP002-008").expect("Claim 8 must exist");
    assert_eq!(c8.epistemic_type, EpistemicType::ProofTarget);
    assert_eq!(c8.transformation_kind, Some(TransformationKind::Lossy));
    assert!(c8.title.contains("Collatz Descent"));
    assert!(!c8.verified);
    assert_eq!(c8.lean_symbol.as_deref(), Some("Amde.DescentCertificate"));
}

#[test]
fn test_cryptographic_hash_integrity_and_tamper_detection() {
    let mut registry = ClaimRegistry::seed_rp002();
    assert!(registry.verify_seed_integrity());

    // Tampering test: modify title without updating sha256
    let claim = registry.get_epistemic_claim_mut("CLAIM-RP002-001").unwrap();
    let original_title = claim.title.clone();
    claim.title = "Tampered Title".to_string();

    // Integrity check must detect alteration
    assert!(!registry.verify_seed_integrity());

    // Restore original title
    let claim = registry.get_epistemic_claim_mut("CLAIM-RP002-001").unwrap();
    claim.title = original_title;
    assert!(registry.verify_seed_integrity());
}

#[test]
fn test_anti_silent_promotion_prohibition() {
    // PRD-26 Rule: "Запрет тихой промоции метафоры в лемму"
    // Leaping over intermediate validation barriers is strictly disallowed.

    // 1. Metaphor -> Lemma Target / Proof Target directly (BANNED)
    let leap_1 = EpistemicType::is_valid_promotion(EpistemicType::Metaphor, EpistemicType::ProofTarget);
    assert!(matches!(leap_1, Err(PromotionError::SilentPromotionDisallowed { .. })));

    // 2. Metaphor -> Theorem directly (BANNED)
    let leap_2 = EpistemicType::is_valid_promotion(EpistemicType::Metaphor, EpistemicType::Theorem);
    assert!(matches!(leap_2, Err(PromotionError::SilentPromotionDisallowed { .. })));

    // 3. Analogy -> Formalization directly (BANNED)
    let leap_3 = EpistemicType::is_valid_promotion(EpistemicType::Analogy, EpistemicType::Formalization);
    assert!(matches!(leap_3, Err(PromotionError::SilentPromotionDisallowed { .. })));

    // 4. Heuristic -> Theorem directly (BANNED)
    let leap_4 = EpistemicType::is_valid_promotion(EpistemicType::Heuristic, EpistemicType::Theorem);
    assert!(matches!(leap_4, Err(PromotionError::SilentPromotionDisallowed { .. })));

    // 5. Conjecture -> Theorem directly (BANNED)
    let leap_5 = EpistemicType::is_valid_promotion(EpistemicType::Conjecture, EpistemicType::Theorem);
    assert!(matches!(leap_5, Err(PromotionError::SilentPromotionDisallowed { .. })));

    // 6. Downgrade via promotion API (BANNED)
    let downgrade = EpistemicType::is_valid_promotion(EpistemicType::Theorem, EpistemicType::Conjecture);
    assert!(matches!(downgrade, Err(PromotionError::DowngradeNotSupported { .. })));
}

#[test]
fn test_strict_promotion_criteria_evidence_gates() {
    let mut claim = EpistemicClaim::new(
        "TEST-CLAIM-001",
        "RP-002",
        "Test Syracuse Descent Hypothesis",
        EpistemicType::ProofTarget,
        Some(TransformationKind::Lossy),
        "N_odd",
        "Descent Certificate",
        "forall n, exists k, S^k(n) < n",
        Some("Amde.DescentCertificate"),
        None,
        false,
        "Target for Lean 4 machine proof",
    );

    assert_eq!(claim.epistemic_type, EpistemicType::ProofTarget);

    // Attempt promotion with incorrect evidence kind (rejected)
    let err = claim.promote(
        EpistemicType::Formalization,
        "informal_handwaving_pdf",
        "proof_draft.pdf",
        "researcher@wolfram",
        "2026-11-02T12:00:00Z",
    );
    assert!(matches!(err, Err(PromotionError::MissingRequiredEvidence { required_kind: "kernel_verified_proof", .. })));
    assert_eq!(claim.epistemic_type, EpistemicType::ProofTarget); // Unchanged

    // Attempt promotion with correct evidence kind (accepted)
    let ok = claim.promote(
        EpistemicType::Formalization,
        "kernel_verified_proof",
        "lake build test/DescentCertificate.lean --hash=7f8a9b",
        "lean-kernel@wolfram",
        "2026-11-02T12:05:00Z",
    );
    assert!(ok.is_ok());
    assert_eq!(claim.epistemic_type, EpistemicType::Formalization);
    assert_eq!(claim.history.len(), 1);
    assert_eq!(claim.history[0].from_type, EpistemicType::ProofTarget);
    assert_eq!(claim.history[0].to_type, EpistemicType::Formalization);
    assert_eq!(claim.history[0].evidence_kind, "kernel_verified_proof");

    // Next promotion to Theorem requires independent cross-agent QA audit
    let err_theorem = claim.promote(
        EpistemicType::Theorem,
        "kernel_verified_proof", // wrong kind, needs independent_qa_audit
        "lake build",
        "builder@wolfram",
        "2026-11-02T12:10:00Z",
    );
    assert!(matches!(err_theorem, Err(PromotionError::MissingRequiredEvidence { required_kind: "independent_qa_audit", .. })));

    let ok_theorem = claim.promote(
        EpistemicType::Theorem,
        "independent_qa_audit",
        "MIN-85-QA-PASS-MUSE-20261027",
        "muse@darwin",
        "2026-11-02T12:15:00Z",
    );
    assert!(ok_theorem.is_ok());
    assert_eq!(claim.epistemic_type, EpistemicType::Theorem);
    assert_eq!(claim.history.len(), 2);
}

#[test]
fn test_end_to_end_ladder_promotion() {
    let mut claim = EpistemicClaim::new(
        "LADDER-001",
        "RP-002",
        "Attractor Contraction Dynamics",
        EpistemicType::Metaphor,
        None,
        "Physical Analogy",
        "Discrete Algebra",
        "Attractor contraction is like physical friction",
        None,
        None,
        false,
        "Intuitive metaphor",
    );

    // 1. Metaphor -> Analogy (gate: structural_mapping)
    claim.promote(
        EpistemicType::Analogy,
        "structural_mapping",
        "SPEC: Map between dissipative system and valuation lattice",
        "agy@wolfram",
        "2026-11-02T10:00:00Z",
    ).expect("Metaphor -> Analogy failed");
    assert_eq!(claim.epistemic_type, EpistemicType::Analogy);

    // 2. Analogy -> Heuristic (gate: quantitative_metric)
    claim.promote(
        EpistemicType::Heuristic,
        "quantitative_metric",
        "NUMERICS: Monte Carlo drift calculation E[Delta I] > 0",
        "agy@wolfram",
        "2026-11-02T10:15:00Z",
    ).expect("Analogy -> Heuristic failed");
    assert_eq!(claim.epistemic_type, EpistemicType::Heuristic);

    // 3. Heuristic -> Conjecture (gate: falsifiable_conjecture_spec)
    claim.promote(
        EpistemicType::Conjecture,
        "falsifiable_conjecture_spec",
        "EXHAUSTIVE: 10^8 integers tested with zero counterexamples",
        "codex@wolfram",
        "2026-11-02T10:30:00Z",
    ).expect("Heuristic -> Conjecture failed");
    assert_eq!(claim.epistemic_type, EpistemicType::Conjecture);

    // 4. Conjecture -> ProofTarget (gate: lean_declaration_spec)
    claim.promote(
        EpistemicType::ProofTarget,
        "lean_declaration_spec",
        "LEAN4: Amde.Attractor.contraction_target signature",
        "muse@darwin",
        "2026-11-02T10:45:00Z",
    ).expect("Conjecture -> ProofTarget failed");
    assert_eq!(claim.epistemic_type, EpistemicType::ProofTarget);

    // 5. ProofTarget -> Formalization (gate: kernel_verified_proof)
    claim.promote(
        EpistemicType::Formalization,
        "kernel_verified_proof",
        "LEAN_KERNEL: zero sorry, Mathlib-checked",
        "lean-kernel@wolfram",
        "2026-11-02T11:00:00Z",
    ).expect("ProofTarget -> Formalization failed");
    assert_eq!(claim.epistemic_type, EpistemicType::Formalization);

    // 6. Formalization -> Theorem (gate: independent_qa_audit)
    claim.promote(
        EpistemicType::Theorem,
        "independent_qa_audit",
        "RECEIPT: PASS signed by Muse & Codex",
        "muse@darwin",
        "2026-11-02T11:15:00Z",
    ).expect("Formalization -> Theorem failed");
    assert_eq!(claim.epistemic_type, EpistemicType::Theorem);

    // Terminal state reached: 6 promotions recorded
    assert_eq!(claim.history.len(), 6);
    assert!(claim.promotion_criteria.is_none());
}

#[test]
fn test_export_formats() {
    let registry = ClaimRegistry::seed_rp002();

    // Markdown Table Export
    let md = registry.export_seed_markdown_table();
    assert!(md.contains("| Claim ID | Epistemic Type | Title | Morphism | Verified | Next Promotion Gate |"));
    assert!(md.contains("CLAIM-RP002-001"));
    assert!(md.contains("CLAIM-RP002-008"));
    assert!(md.contains("THEOREM"));
    assert!(md.contains("CONJECTURE"));
    assert!(md.contains("KNOWN THEOREM"));
    assert!(md.contains("HEURISTIC"));
    assert!(md.contains("ANALOGY"));
    assert!(md.contains("PROOF TARGET"));

    // JSON Export
    let json = registry.export_seed_json();
    assert!(json.contains("\"seed_registry_version\": \"v1.0\""));
    assert!(json.contains("\"benchmark\": \"RP-002\""));
    assert!(json.contains("\"total_epistemic_claims\": 8"));
    assert!(json.contains("\"integrity_verified\": true"));
    assert!(json.contains("CLAIM-RP002-001"));
    assert!(json.contains("CLAIM-RP002-005"));
}
