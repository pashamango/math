//! Claim Registry and Epistemic Typing for Navigator Transformations and RP-002 Claims.
//!
//! Based on PRD-26 (amde-core-navigator-typed-transformation-graph-claim-registry)
//! and MIN-83 (Claim registry seed from RP-002).
//!
//! Epistemic types:
//! - `THEOREM`: Formally verified mathematical theorem within the framework (Lean 4 + Mathlib).
//! - `KNOWN_THEOREM`: External peer-reviewed theorem with literature citation.
//! - `FORMALIZATION`: Machine-checked proof in interactive theorem prover kernel without sorry/admit.
//! - `PROOF_TARGET` / `LEMMA_TARGET`: Targeted lemma or goal state formulated for machine-assisted proof.
//! - `CONJECTURE`: Mathematically precise, falsifiable statement awaiting formal proof.
//! - `EXPERIMENT`: Empirical validation run or certified numerical evaluation.
//! - `HEURISTIC`: Probabilistic or statistical approximation model reflecting empirical drift.
//! - `ANALOGY`: Structural/methodological correspondence across distinct domains with mapped invariants.
//! - `METAPHOR`: Figurative intuition without operational mapping.
//!
//! Anti-silent-promotion rule:
//! Prohibition of silent promotion (e.g. Metaphor -> Lemma / Theorem).
//! Every promotion requires explicit transition validation and proof/audit evidence tokens.

use crate::transformation::TransformationKind;
use std::fmt::{self, Display, Formatter};
use std::sync::atomic::{AtomicU64, Ordering};

static CLAIM_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Cryptographic SHA-256 implementation (pure standard library, zero dependencies).
pub fn sha256(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];

    let bit_len = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while (msg.len() + 8) % 64 != 0 {
        msg.push(0x00);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i * 4], chunk[i * 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut h_val = h[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h_val
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h_val = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(h_val);
    }

    let mut out = String::with_capacity(64);
    for word in h {
        out.push_str(&format!("{:08x}", word));
    }
    out
}

/// Epistemic status classifying claims in AMDE according to PRD-26 taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EpistemicType {
    /// Pure structural intuition or figurative analogy without operational mapping.
    Metaphor,
    /// Structural/methodological correspondence across distinct domains with mapped invariants.
    Analogy,
    /// Empirical or statistical drift model with measurable approximations.
    Heuristic,
    /// Mathematically precise, falsifiable statement awaiting formal proof.
    Conjecture,
    /// Targeted lemma or machine-assisted formalization goal (Lean 4 specification).
    ProofTarget,
    /// Machine-checked proof in interactive theorem prover kernel (Lean 4 / Mathlib).
    Formalization,
    /// Established theorem within framework with full closed proof, audited and verified.
    Theorem,
    /// External peer-reviewed theorem from published mathematics with authoritative citation.
    KnownTheorem,
    /// Empirical validation run or certified numerical experiment on parameter domain.
    Experiment,
}

impl EpistemicType {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Metaphor => "METAPHOR",
            Self::Analogy => "ANALOGY",
            Self::Heuristic => "HEURISTIC",
            Self::Conjecture => "CONJECTURE",
            Self::ProofTarget => "PROOF_TARGET",
            Self::Formalization => "FORMALIZATION",
            Self::Theorem => "THEOREM",
            Self::KnownTheorem => "KNOWN_THEOREM",
            Self::Experiment => "EXPERIMENT",
        }
    }

    pub const fn display_name(&self) -> &'static str {
        match self {
            Self::Metaphor => "METAPHOR",
            Self::Analogy => "ANALOGY",
            Self::Heuristic => "HEURISTIC",
            Self::Conjecture => "CONJECTURE",
            Self::ProofTarget => "PROOF TARGET",
            Self::Formalization => "FORMALIZATION",
            Self::Theorem => "THEOREM",
            Self::KnownTheorem => "KNOWN THEOREM",
            Self::Experiment => "EXPERIMENT",
        }
    }

    /// Ranks epistemic rigor along the formalization path (1..=7).
    pub const fn rigor_rank(&self) -> u8 {
        match self {
            Self::Metaphor => 1,
            Self::Analogy => 2,
            Self::Heuristic => 3,
            Self::Conjecture => 4,
            Self::ProofTarget => 5,
            Self::Formalization => 6,
            Self::Theorem => 7,
            Self::KnownTheorem => 7,
            Self::Experiment => 3,
        }
    }

    /// Checks if a step transition from `from` to `to` is legally permitted.
    /// Strictly forbids silent promotion leaps across intermediate epistemic barriers.
    pub fn is_valid_promotion(from: Self, to: Self) -> Result<PromotionCriteria, PromotionError> {
        match (from, to) {
            (Self::Metaphor, Self::Analogy) => Ok(PromotionCriteria {
                required_evidence_kind: "structural_mapping",
                description: "Explicit domain mapping between source and target spaces with operational correspondence",
                falsification_barrier: "State space mismatch or undefined operations falsifies analogy",
                verification_gate: "Navigator Object signature alignment check",
            }),
            (Self::Analogy, Self::Heuristic) => Ok(PromotionCriteria {
                required_evidence_kind: "quantitative_metric",
                description: "Quantitative formulation with measurable metrics, empirical distribution, or drift bounds",
                falsification_barrier: "Empirical observation diverging from predicted drift metric",
                verification_gate: "Monte Carlo or deterministic drift evaluation over sample space",
            }),
            (Self::Heuristic, Self::Conjecture) => Ok(PromotionCriteria {
                required_evidence_kind: "falsifiable_conjecture_spec",
                description: "Rigorous mathematical formulation with universal/existential quantification and domain bounds",
                falsification_barrier: "Any counterexample found during numerical exploration falsifies conjecture",
                verification_gate: "Exhaustive counterexample search (N >= 10^6) with zero failures",
            }),
            (Self::Conjecture, Self::ProofTarget) => Ok(PromotionCriteria {
                required_evidence_kind: "lean_declaration_spec",
                description: "Machine-readable theorem signature in Lean 4 with explicit dependencies and proof strategy",
                falsification_barrier: "Type error, circular dependency, or unprovable lemma target",
                verification_gate: "Lean 4 environment typecheck and dependency graph verification",
            }),
            (Self::ProofTarget, Self::Formalization) => Ok(PromotionCriteria {
                required_evidence_kind: "kernel_verified_proof",
                description: "Complete machine-checked proof in Lean 4 kernel with zero 'sorry' or 'admit'",
                falsification_barrier: "Kernel rejection or unproven sorry/admit in proof term",
                verification_gate: "lake build && lean --run KernelReplay.lean confirming standard axioms only",
            }),
            (Self::Formalization, Self::Theorem) => Ok(PromotionCriteria {
                required_evidence_kind: "independent_qa_audit",
                description: "Independent cross-agent audit receipt (PASS) confirming scope, zero shortcuts, and axiomatic closure",
                falsification_barrier: "Hidden assumptions or non-standard foundation axioms",
                verification_gate: "Two-agent peer verification + principal release gate",
            }),
            (from, to) if from == to => Err(PromotionError::AlreadyAtTargetLevel(from)),
            (from, to) if to.rigor_rank() < from.rigor_rank() => Err(PromotionError::DowngradeNotSupported { from, to }),
            (from, to) => Err(PromotionError::SilentPromotionDisallowed {
                from,
                to,
                reason: "Direct or silent promotion skipping required intermediate validation gates is prohibited by PRD-26",
            }),
        }
    }
}

impl Display for EpistemicType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Promotion criteria and verification barriers required to elevate a claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromotionCriteria {
    pub required_evidence_kind: &'static str,
    pub description: &'static str,
    pub falsification_barrier: &'static str,
    pub verification_gate: &'static str,
}

impl PromotionCriteria {
    pub fn for_type(epistemic_type: EpistemicType) -> Option<Self> {
        match epistemic_type {
            EpistemicType::Metaphor => Some(EpistemicType::is_valid_promotion(EpistemicType::Metaphor, EpistemicType::Analogy).unwrap()),
            EpistemicType::Analogy => Some(EpistemicType::is_valid_promotion(EpistemicType::Analogy, EpistemicType::Heuristic).unwrap()),
            EpistemicType::Heuristic => Some(EpistemicType::is_valid_promotion(EpistemicType::Heuristic, EpistemicType::Conjecture).unwrap()),
            EpistemicType::Conjecture => Some(EpistemicType::is_valid_promotion(EpistemicType::Conjecture, EpistemicType::ProofTarget).unwrap()),
            EpistemicType::ProofTarget => Some(EpistemicType::is_valid_promotion(EpistemicType::ProofTarget, EpistemicType::Formalization).unwrap()),
            EpistemicType::Formalization => Some(EpistemicType::is_valid_promotion(EpistemicType::Formalization, EpistemicType::Theorem).unwrap()),
            EpistemicType::Theorem => None, // Terminal state within framework
            EpistemicType::KnownTheorem => None, // External established theorem
            EpistemicType::Experiment => Some(PromotionCriteria {
                required_evidence_kind: "falsifiable_conjecture_spec",
                description: "Formulation of general mathematical conjecture from experimental data",
                falsification_barrier: "Failure to generalize beyond tested parameter domain",
                verification_gate: "Cross-domain parametric consistency check",
            }),
        }
    }
}

/// Error returned when promotion violates epistemic boundaries or evidence requirements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromotionError {
    SilentPromotionDisallowed {
        from: EpistemicType,
        to: EpistemicType,
        reason: &'static str,
    },
    MissingRequiredEvidence {
        required_kind: &'static str,
        reason: &'static str,
    },
    ClaimNotFound(String),
    AlreadyAtTargetLevel(EpistemicType),
    DowngradeNotSupported {
        from: EpistemicType,
        to: EpistemicType,
    },
}

impl Display for PromotionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::SilentPromotionDisallowed { from, to, reason } => {
                write!(f, "Silent promotion disallowed ({} -> {}): {}", from, to, reason)
            }
            Self::MissingRequiredEvidence { required_kind, reason } => {
                write!(f, "Missing required evidence kind '{}': {}", required_kind, reason)
            }
            Self::ClaimNotFound(id) => write!(f, "Claim '{}' not found in registry", id),
            Self::AlreadyAtTargetLevel(t) => write!(f, "Claim is already at target level '{}'", t),
            Self::DowngradeNotSupported { from, to } => {
                write!(f, "Epistemic downgrade ({} -> {}) is not supported via promotion API", from, to)
            }
        }
    }
}

impl std::error::Error for PromotionError {}

/// Immutable audit record of an epistemic promotion event.
#[derive(Debug, Clone, PartialEq)]
pub struct PromotionEvent {
    pub from_type: EpistemicType,
    pub to_type: EpistemicType,
    pub evidence_kind: String,
    pub evidence_ref: String,
    pub verified_by: String,
    pub timestamp_utc: String,
    pub event_hash: String,
}

impl PromotionEvent {
    pub fn new(
        from_type: EpistemicType,
        to_type: EpistemicType,
        evidence_kind: &str,
        evidence_ref: &str,
        verified_by: &str,
        timestamp_utc: &str,
    ) -> Self {
        let payload = format!(
            "from={}|to={}|kind={}|ref={}|by={}|ts={}",
            from_type.as_str(),
            to_type.as_str(),
            evidence_kind,
            evidence_ref,
            verified_by,
            timestamp_utc
        );
        let event_hash = sha256(payload.as_bytes());
        Self {
            from_type,
            to_type,
            evidence_kind: evidence_kind.to_string(),
            evidence_ref: evidence_ref.to_string(),
            verified_by: verified_by.to_string(),
            timestamp_utc: timestamp_utc.to_string(),
            event_hash,
        }
    }
}

/// A formal epistemic claim seeded from RP-002 or registered during mathematical research.
#[derive(Debug, Clone, PartialEq)]
pub struct EpistemicClaim {
    pub claim_id: String,
    pub rp_reference: String,
    pub title: String,
    pub epistemic_type: EpistemicType,
    pub transformation_kind: Option<TransformationKind>,
    pub source_domain: String,
    pub target_domain: String,
    pub mathematical_formulation: String,
    pub lean_symbol: Option<String>,
    pub literature_citation: Option<String>,
    pub verified: bool,
    pub promotion_criteria: Option<PromotionCriteria>,
    pub history: Vec<PromotionEvent>,
    pub notes: String,
    pub sha256_hash: String,
}

impl EpistemicClaim {
    pub fn new(
        claim_id: &str,
        rp_reference: &str,
        title: &str,
        epistemic_type: EpistemicType,
        transformation_kind: Option<TransformationKind>,
        source_domain: &str,
        target_domain: &str,
        mathematical_formulation: &str,
        lean_symbol: Option<&str>,
        literature_citation: Option<&str>,
        verified: bool,
        notes: &str,
    ) -> Self {
        let criteria = PromotionCriteria::for_type(epistemic_type);
        let trans_str = transformation_kind.map(|k| k.as_str()).unwrap_or("NONE");
        let lean_str = lean_symbol.unwrap_or("none");
        let lit_str = literature_citation.unwrap_or("none");

        let raw_payload = format!(
            "id={}|rp={}|title={}|type={}|trans={}|src={}|tgt={}|math={}|lean={}|lit={}|ver={}|notes={}",
            claim_id,
            rp_reference,
            title,
            epistemic_type.as_str(),
            trans_str,
            source_domain,
            target_domain,
            mathematical_formulation,
            lean_str,
            lit_str,
            verified,
            notes
        );
        let sha256_hash = sha256(raw_payload.as_bytes());

        Self {
            claim_id: claim_id.to_string(),
            rp_reference: rp_reference.to_string(),
            title: title.to_string(),
            epistemic_type,
            transformation_kind,
            source_domain: source_domain.to_string(),
            target_domain: target_domain.to_string(),
            mathematical_formulation: mathematical_formulation.to_string(),
            lean_symbol: lean_symbol.map(|s| s.to_string()),
            literature_citation: literature_citation.map(|s| s.to_string()),
            verified,
            promotion_criteria: criteria,
            history: Vec::new(),
            notes: notes.to_string(),
            sha256_hash,
        }
    }

    /// Promotes this claim to a higher epistemic tier.
    /// Strictly verifies that the transition is legal and that required evidence kind is provided.
    pub fn promote(
        &mut self,
        target_type: EpistemicType,
        evidence_kind: &str,
        evidence_ref: &str,
        verified_by: &str,
        timestamp_utc: &str,
    ) -> Result<(), PromotionError> {
        let criteria = EpistemicType::is_valid_promotion(self.epistemic_type, target_type)?;

        if evidence_kind != criteria.required_evidence_kind {
            return Err(PromotionError::MissingRequiredEvidence {
                required_kind: criteria.required_evidence_kind,
                reason: "Provided evidence kind does not match promotion criteria gate",
            });
        }

        let event = PromotionEvent::new(
            self.epistemic_type,
            target_type,
            evidence_kind,
            evidence_ref,
            verified_by,
            timestamp_utc,
        );
        self.history.push(event);
        self.epistemic_type = target_type;
        self.promotion_criteria = PromotionCriteria::for_type(target_type);

        // Recompute cryptographic hash reflecting promotion
        let trans_str = self.transformation_kind.map(|k| k.as_str()).unwrap_or("NONE");
        let lean_str = self.lean_symbol.as_deref().unwrap_or("none");
        let lit_str = self.literature_citation.as_deref().unwrap_or("none");
        let payload = format!(
            "id={}|rp={}|title={}|type={}|trans={}|src={}|tgt={}|math={}|lean={}|lit={}|ver={}|promotions={}",
            self.claim_id,
            self.rp_reference,
            self.title,
            self.epistemic_type.as_str(),
            trans_str,
            self.source_domain,
            self.target_domain,
            self.mathematical_formulation,
            lean_str,
            lit_str,
            self.verified,
            self.history.len()
        );
        self.sha256_hash = sha256(payload.as_bytes());

        Ok(())
    }

    pub fn to_json_snippet(&self) -> String {
        let trans_str = self.transformation_kind.map(|k| k.as_str()).unwrap_or("NONE");
        let lean_json = self.lean_symbol.as_ref().map(|s| format!("\"{}\"", escape_json(s))).unwrap_or_else(|| "null".to_string());
        let lit_json = self.literature_citation.as_ref().map(|s| format!("\"{}\"", escape_json(s))).unwrap_or_else(|| "null".to_string());
        let criteria_json = match &self.promotion_criteria {
            Some(c) => format!(
                "{{\"required_evidence_kind\":\"{}\",\"description\":\"{}\",\"falsification_barrier\":\"{}\",\"verification_gate\":\"{}\"}}",
                c.required_evidence_kind,
                escape_json(c.description),
                escape_json(c.falsification_barrier),
                escape_json(c.verification_gate)
            ),
            None => "null".to_string(),
        };

        format!(
            "    {{\n      \"claim_id\": \"{}\",\n      \"rp_reference\": \"{}\",\n      \"title\": \"{}\",\n      \"epistemic_type\": \"{}\",\n      \"transformation_kind\": \"{}\",\n      \"source_domain\": \"{}\",\n      \"target_domain\": \"{}\",\n      \"formulation\": \"{}\",\n      \"lean_symbol\": {},\n      \"literature_citation\": {},\n      \"verified\": {},\n      \"promotion_criteria\": {},\n      \"sha256\": \"{}\",\n      \"notes\": \"{}\"\n    }}",
            self.claim_id,
            escape_json(&self.rp_reference),
            escape_json(&self.title),
            self.epistemic_type.display_name(),
            trans_str,
            escape_json(&self.source_domain),
            escape_json(&self.target_domain),
            escape_json(&self.mathematical_formulation),
            lean_json,
            lit_json,
            self.verified,
            criteria_json,
            self.sha256_hash,
            escape_json(&self.notes)
        )
    }

    pub fn to_markdown_row(&self) -> String {
        let trans_str = self.transformation_kind.map(|k| k.as_str()).unwrap_or("—");
        let next_step = self.promotion_criteria.as_ref().map(|c| c.required_evidence_kind).unwrap_or("TERMINAL");
        format!(
            "| `{}` | **{}** | `{}` | `{}` | {} | `{}` |",
            self.claim_id,
            self.epistemic_type.display_name(),
            self.title,
            trans_str,
            if self.verified { "✅ YES" } else { "⏳ PENDING" },
            next_step
        )
    }
}

impl Display for EpistemicClaim {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let trans_str = self.transformation_kind.map(|k| k.as_str()).unwrap_or("NONE");
        write!(
            f,
            "[{}] {} ({}) | {} | verified={}",
            self.claim_id,
            self.title,
            self.epistemic_type.display_name(),
            trans_str,
            self.verified
        )
    }
}

/// An immutable verifiable receipt emitted for every executed transformation.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimReceipt {
    pub claim_id: String,
    pub transformation_name: String,
    pub kind: TransformationKind,
    pub epistemic_type: EpistemicType,
    pub source_object: String,
    pub target_object: String,
    pub input_repr: String,
    pub output_repr: String,
    pub verified: bool,
    pub roundtrip_discrepancy: f64,
    pub information_loss: f64,
    pub metric_error: f64,
    pub notes: String,
    pub sha256_hash: String,
}

impl ClaimReceipt {
    pub fn new(
        transformation_name: &str,
        kind: TransformationKind,
        source_object: &str,
        target_object: &str,
        input_repr: &str,
        output_repr: &str,
        verified: bool,
        roundtrip_discrepancy: f64,
        information_loss: f64,
        metric_error: f64,
        notes: &str,
    ) -> Self {
        Self::new_with_epistemic(
            transformation_name,
            kind,
            EpistemicType::Theorem,
            source_object,
            target_object,
            input_repr,
            output_repr,
            verified,
            roundtrip_discrepancy,
            information_loss,
            metric_error,
            notes,
        )
    }

    pub fn new_with_epistemic(
        transformation_name: &str,
        kind: TransformationKind,
        epistemic_type: EpistemicType,
        source_object: &str,
        target_object: &str,
        input_repr: &str,
        output_repr: &str,
        verified: bool,
        roundtrip_discrepancy: f64,
        information_loss: f64,
        metric_error: f64,
        notes: &str,
    ) -> Self {
        let seq = CLAIM_COUNTER.fetch_add(1, Ordering::SeqCst);
        let raw_payload = format!(
            "seq={}|trans={}|kind={}|epistemic={}|src={}|tgt={}|in={}|out={}|ver={}|disc={:.6}|loss={:.6}|err={:.6}",
            seq,
            transformation_name,
            kind,
            epistemic_type.as_str(),
            source_object,
            target_object,
            input_repr,
            output_repr,
            verified,
            roundtrip_discrepancy,
            information_loss,
            metric_error
        );
        let sha256_hash = sha256(raw_payload.as_bytes());
        let claim_id = format!("CLAIM-MIN79-{:04}-{}", seq, &sha256_hash[0..8]);

        Self {
            claim_id,
            transformation_name: transformation_name.to_string(),
            kind,
            epistemic_type,
            source_object: source_object.to_string(),
            target_object: target_object.to_string(),
            input_repr: input_repr.to_string(),
            output_repr: output_repr.to_string(),
            verified,
            roundtrip_discrepancy,
            information_loss,
            metric_error,
            notes: notes.to_string(),
            sha256_hash,
        }
    }

    pub fn to_json_snippet(&self) -> String {
        format!(
            "    {{\n      \"claim_id\": \"{}\",\n      \"transformation\": \"{}\",\n      \"kind\": \"{}\",\n      \"epistemic_type\": \"{}\",\n      \"source\": \"{}\",\n      \"target\": \"{}\",\n      \"input\": \"{}\",\n      \"output\": \"{}\",\n      \"verified\": {},\n      \"roundtrip_discrepancy\": {},\n      \"information_loss\": {:.6},\n      \"metric_error\": {:.8},\n      \"sha256\": \"{}\",\n      \"notes\": \"{}\"\n    }}",
            self.claim_id,
            escape_json(&self.transformation_name),
            self.kind,
            self.epistemic_type.display_name(),
            escape_json(&self.source_object),
            escape_json(&self.target_object),
            escape_json(&self.input_repr),
            escape_json(&self.output_repr),
            self.verified,
            self.roundtrip_discrepancy,
            self.information_loss,
            self.metric_error,
            self.sha256_hash,
            escape_json(&self.notes)
        )
    }
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
}

impl Display for ClaimReceipt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} ({}, {}) | {} -> {} | verified={}",
            self.claim_id,
            self.transformation_name,
            self.kind,
            self.epistemic_type.display_name(),
            self.source_object,
            self.target_object,
            self.verified
        )
    }
}

/// Registry storing all claims produced during Navigator workflows and epistemic seeds.
#[derive(Debug, Default)]
pub struct ClaimRegistry {
    claims: Vec<ClaimReceipt>,
    epistemic_claims: Vec<EpistemicClaim>,
}

impl ClaimRegistry {
    pub fn new() -> Self {
        Self {
            claims: Vec::new(),
            epistemic_claims: Vec::new(),
        }
    }

    /// Seeds the registry with the complete set of RP-002 mathematical claims (MIN-83 / PRD-26).
    pub fn seed_rp002() -> Self {
        let mut registry = Self::new();

        // 1. THEOREM: Prime Factorization as Reversible Compiler
        registry.register_epistemic(EpistemicClaim::new(
            "CLAIM-RP002-001",
            "RP-002",
            "Prime Factorization as Reversible Compiler",
            EpistemicType::Theorem,
            Some(TransformationKind::Iso),
            "(N+, *) Multiplicative Monoid",
            "(DirectSum_{p in P} N, +) Free Commutative Valuation Monoid",
            "n = prod_{p} p^{v_p(n)}; exact monoid isomorphism N+ =~* Multiplicative (Nat.Primes ->0 N), Delta I = 0",
            Some("Amde.PrimeValuation.primeValuationMulEquiv"),
            None,
            true,
            "Fundamental Theorem of Arithmetic formalized as an exact, lossless compiler with verified bijective round-trip.",
        ));

        // 2. THEOREM: Golden Widths Asymptotic Convergence (Gn -> 1/2)
        registry.register_epistemic(EpistemicClaim::new(
            "CLAIM-RP002-002",
            "RP-002",
            "Golden Widths Asymptotic Convergence (Gn -> 1/2)",
            EpistemicType::Theorem,
            Some(TransformationKind::Lossy),
            "(N, discrete) Scale Horizon Index",
            "(Intervals(R), tau) Nested Metric Neighborhoods",
            "G_n = [1/2 - phi^{n+3}/2, 1/2 + phi^{n+3}/2]; lim_{n -> infty} G_n = {1/2}; intersection_{n} G_n = {criticalCoordinate}",
            Some("Amde.intersection_goldenSets"),
            None,
            true,
            "Monotonically contracting nested intervals defined by golden powers phi^{n+3} converging uniquely to critical line coordinate 1/2.",
        ));

        // 3. CONJECTURE: Golden Scaling Dynamically Privileged
        registry.register_epistemic(EpistemicClaim::new(
            "CLAIM-RP002-003",
            "RP-002",
            "Golden Scaling Dynamically Privileged",
            EpistemicType::Conjecture,
            Some(TransformationKind::Approximate),
            "Critical Strip S = {s in C | 0 < Re(s) < 1}",
            "Renormalization Group Attractor Invariants",
            "Critical strip [1 - phi, phi] = [0.381966, 0.618034] of width phi^3 = 2phi - 1 approx 0.236068 around axis 1/2 is privileged invariant attractor under spectral renormalization flow",
            None,
            None,
            false,
            "Conjecture that golden ratio phi = (sqrt(5)-1)/2 dynamically minimizes metric entropy and stabilizes spectral dynamics in the critical strip.",
        ));

        // 4. CONJECTURE: Collatz Golden Contraction
        registry.register_epistemic(EpistemicClaim::new(
            "CLAIM-RP002-004",
            "RP-002",
            "Collatz Golden Contraction",
            EpistemicType::Conjecture,
            Some(TransformationKind::Lossy),
            "2-adic Valuation Slices (N_odd, v_2)",
            "Metric Contraction Factor lambda_eff",
            "On prime-valuation slices, the accelerated Syracuse map S(n) = (3n+1)/2^{v_2(3n+1)} admits effective Lyapunov exponent bounded by golden contraction factor lambda_eff <= phi approx 0.618034",
            None,
            None,
            false,
            "Conjecture that Syracuse dynamics on dyadic filtration slices contracts with golden ratio rate, preventing unbounded divergence.",
        ));

        // 5. KNOWN THEOREM: Li Positivity Criterion Equivalence with RH
        registry.register_epistemic(EpistemicClaim::new(
            "CLAIM-RP002-005",
            "RP-002",
            "Li Positivity Criterion Equivalence with RH",
            EpistemicType::KnownTheorem,
            Some(TransformationKind::Approximate),
            "Completed Riemann xi-function spectrum {rho | xi(rho) = 0}",
            "Li Coefficient Sequence lambda_n in R",
            "RH <=> forall n >= 1, lambda_n = sum_{rho} [1 - (1 - 1/rho)^n] >= 0; equivalent to positive-definiteness of Weil quadratic form",
            Some("Amde.rh_iff_xiQuadraticForm_mem_positiveCone"),
            Some("Xian-Jin Li (1997), 'The Positivity of a Sequence of Numbers and the Riemann Hypothesis', J. Number Theory 65, 325-333; Bombieri & Lagarias (1999)"),
            true,
            "Established mathematical equivalence between Li coefficient nonnegativity and the Riemann Hypothesis.",
        ));

        // 6. HEURISTIC: Syracuse Heuristic Log-Drift 3/4
        registry.register_epistemic(EpistemicClaim::new(
            "CLAIM-RP002-006",
            "RP-002",
            "Syracuse Heuristic Log-Drift 3/4",
            EpistemicType::Heuristic,
            Some(TransformationKind::Lossy),
            "Syracuse Step Orbit Distribution",
            "Logarithmic Ratio E[log_2(S(n)/n)]",
            "Assuming independent uniform parity of (3n+1)/2, expected 2-adic valuation E[v_2] = 2; hence E[log_2(S(n)/n)] approx log_2(3) - 2 approx -0.415037 approx ln(3/4)",
            None,
            Some("Lagarias (1985), 'The 3x + 1 problem and its generalizations', Amer. Math. Monthly 92, 3-23"),
            false,
            "Heuristic statistical drift model explaining the downward geometric pull of Collatz trajectories towards 1.",
        ));

        // 7. ANALOGY: Perelman Monotonicity Transfer
        registry.register_epistemic(EpistemicClaim::new(
            "CLAIM-RP002-007",
            "RP-002",
            "Perelman Monotonicity Transfer to Arithmetic Dynamics",
            EpistemicType::Analogy,
            Some(TransformationKind::Adjoint),
            "Geometric Analysis: Ricci Flow & Perelman W-entropy functional",
            "Arithmetic Markov Dynamics: Valuation Dissipation Delta I & Height Monotonicity",
            "Monotonicity of Perelman W-entropy dW/dt >= 0 under backward heat conjugate flow maps structurally to monotonic height dissipation Delta I >= 0 along Galois connection retractions in valuation lattices",
            None,
            Some("Perelman (2002), 'The entropy formula for the Ricci flow and its geometric applications', arXiv:math/0211159"),
            false,
            "Structural methodological transfer connecting non-collapsing monotonicity theorems in geometric analysis to discrete arithmetic dynamical flows.",
        ));

        // 8. PROOF TARGET: Collatz Descent Below Start (Finite Stopping Target)
        registry.register_epistemic(EpistemicClaim::new(
            "CLAIM-RP002-008",
            "RP-002",
            "Collatz Descent Below Start (Finite Stopping Target)",
            EpistemicType::ProofTarget,
            Some(TransformationKind::Lossy),
            "Odd Natural Numbers n in N_odd, n > 1",
            "Descent Certificate: exists k >= 1 such that S^k(n) < n",
            "forall n in N_odd, n > 1 ==> exists k >= 1, S^k(n) < n; machine-checkable witness certificate for Syracuse orbit descent",
            Some("Amde.DescentCertificate"),
            None,
            false,
            "Target lemma in Lean 4 formalization stack specifying machine-checked proof target that every non-trivial odd integer descends below its starting value.",
        ));

        registry
    }

    pub fn register(&mut self, receipt: ClaimReceipt) {
        self.claims.push(receipt);
    }

    pub fn register_epistemic(&mut self, claim: EpistemicClaim) {
        self.epistemic_claims.push(claim);
    }

    pub fn claims(&self) -> &[ClaimReceipt] {
        &self.claims
    }

    pub fn epistemic_claims(&self) -> &[EpistemicClaim] {
        &self.epistemic_claims
    }

    pub fn get_epistemic_claim(&self, claim_id: &str) -> Option<&EpistemicClaim> {
        self.epistemic_claims.iter().find(|c| c.claim_id == claim_id)
    }

    pub fn get_epistemic_claim_mut(&mut self, claim_id: &str) -> Option<&mut EpistemicClaim> {
        self.epistemic_claims.iter_mut().find(|c| c.claim_id == claim_id)
    }

    /// Promotes an epistemic claim in the registry.
    /// Strictly verifies promotion criteria and prevents silent jumps.
    pub fn promote_claim(
        &mut self,
        claim_id: &str,
        target_type: EpistemicType,
        evidence_kind: &str,
        evidence_ref: &str,
        verified_by: &str,
        timestamp_utc: &str,
    ) -> Result<(), PromotionError> {
        let claim = self
            .get_epistemic_claim_mut(claim_id)
            .ok_or_else(|| PromotionError::ClaimNotFound(claim_id.to_string()))?;

        claim.promote(target_type, evidence_kind, evidence_ref, verified_by, timestamp_utc)
    }

    pub fn len(&self) -> usize {
        self.claims.len()
    }

    pub fn epistemic_len(&self) -> usize {
        self.epistemic_claims.len()
    }

    pub fn is_empty(&self) -> bool {
        self.claims.is_empty() && self.epistemic_claims.is_empty()
    }

    pub fn verify_all(&self) -> bool {
        if self.is_empty() {
            return false;
        }
        let receipts_ok = self.claims.iter().all(|c| c.verified);
        let epistemic_ok = self.epistemic_claims.is_empty() || self.verify_seed_integrity();
        receipts_ok && epistemic_ok
    }

    /// Verifies cryptographic hash integrity across all seeded epistemic claims.
    pub fn verify_seed_integrity(&self) -> bool {
        if self.epistemic_claims.is_empty() {
            return true;
        }

        self.epistemic_claims.iter().all(|c| {
            let trans_str = c.transformation_kind.map(|k| k.as_str()).unwrap_or("NONE");
            let lean_str = c.lean_symbol.as_deref().unwrap_or("none");
            let lit_str = c.literature_citation.as_deref().unwrap_or("none");

            let expected_hash = if c.history.is_empty() {
                let raw_payload = format!(
                    "id={}|rp={}|title={}|type={}|trans={}|src={}|tgt={}|math={}|lean={}|lit={}|ver={}|notes={}",
                    c.claim_id,
                    c.rp_reference,
                    c.title,
                    c.epistemic_type.as_str(),
                    trans_str,
                    c.source_domain,
                    c.target_domain,
                    c.mathematical_formulation,
                    lean_str,
                    lit_str,
                    c.verified,
                    c.notes
                );
                sha256(raw_payload.as_bytes())
            } else {
                let payload = format!(
                    "id={}|rp={}|title={}|type={}|trans={}|src={}|tgt={}|math={}|lean={}|lit={}|ver={}|promotions={}",
                    c.claim_id,
                    c.rp_reference,
                    c.title,
                    c.epistemic_type.as_str(),
                    trans_str,
                    c.source_domain,
                    c.target_domain,
                    c.mathematical_formulation,
                    lean_str,
                    lit_str,
                    c.verified,
                    c.history.len()
                );
                sha256(payload.as_bytes())
            };

            c.sha256_hash == expected_hash
        })
    }

    /// Exports the seeded RP-002 claim registry as a structured Markdown table.
    pub fn export_seed_markdown_table(&self) -> String {
        let mut out = String::from(
            "| Claim ID | Epistemic Type | Title | Morphism | Verified | Next Promotion Gate |\n|:---|:---:|:---|:---:|:---:|:---:|\n",
        );
        for claim in &self.epistemic_claims {
            out.push_str(&claim.to_markdown_row());
            out.push('\n');
        }
        out
    }

    /// Exports the seeded claim registry to canonical JSON.
    pub fn export_seed_json(&self) -> String {
        let mut out = String::from("{\n  \"seed_registry_version\": \"v1.0\",\n  \"benchmark\": \"RP-002\",\n  \"total_epistemic_claims\": ");
        out.push_str(&self.epistemic_claims.len().to_string());
        out.push_str(",\n  \"integrity_verified\": ");
        out.push_str(&self.verify_seed_integrity().to_string());
        out.push_str(",\n  \"epistemic_claims\": [\n");
        let snippets: Vec<String> = self.epistemic_claims.iter().map(|c| c.to_json_snippet()).collect();
        out.push_str(&snippets.join(",\n"));
        out.push_str("\n  ]\n}\n");
        out
    }

    /// Full export including transformation receipts and seeded epistemic claims.
    pub fn export_json(&self) -> String {
        let mut out = String::from("{\n  \"claim_registry_version\": \"v1.1\",\n  \"total_claims\": ");
        out.push_str(&self.claims.len().to_string());
        out.push_str(",\n  \"total_epistemic_claims\": ");
        out.push_str(&self.epistemic_claims.len().to_string());
        out.push_str(",\n  \"all_verified\": ");
        out.push_str(&self.verify_all().to_string());
        out.push_str(",\n  \"claims\": [\n");
        let receipt_snippets: Vec<String> = self.claims.iter().map(|c| c.to_json_snippet()).collect();
        out.push_str(&receipt_snippets.join(",\n"));
        out.push_str("\n  ],\n  \"epistemic_claims\": [\n");
        let epistemic_snippets: Vec<String> = self.epistemic_claims.iter().map(|c| c.to_json_snippet()).collect();
        out.push_str(&epistemic_snippets.join(",\n"));
        out.push_str("\n  ]\n}\n");
        out
    }
}
