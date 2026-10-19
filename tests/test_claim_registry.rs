//! Unit tests for Claim Registry and Cryptographic Verification.

use amde_navigator::claim::{sha256, ClaimReceipt, ClaimRegistry};
use amde_navigator::transformation::TransformationKind;

#[test]
fn test_sha256_known_vectors() {
    // Empty string SHA-256
    assert_eq!(
        sha256(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );

    // "abc" SHA-256
    assert_eq!(
        sha256(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn test_claim_receipt_and_registry() {
    let mut registry = ClaimRegistry::new();
    assert!(registry.is_empty());

    let receipt1 = ClaimReceipt::new(
        "TestIso",
        TransformationKind::Iso,
        "SourceSpace",
        "TargetSpace",
        "input",
        "output",
        true,
        0.0,
        0.0,
        0.0,
        "Test notes",
    );
    assert!(receipt1.verified);
    assert!(receipt1.claim_id.starts_with("CLAIM-MIN79-"));

    registry.register(receipt1);
    assert_eq!(registry.len(), 1);
    assert!(registry.verify_all());

    let json = registry.export_json();
    assert!(json.contains("claim_registry_version"));
    assert!(json.contains("TestIso"));
    assert!(json.contains("all_verified\": true"));
}
