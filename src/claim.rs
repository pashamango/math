//! Claim Registry and Evidence Certificates for Navigator Transformations.
//!
//! Based on PRD-26 (amde-core-navigator-typed-transformation-graph-claim-registry).
//!
//! Every transformation executed in the Navigator graph emits an immutable `ClaimReceipt`
//! containing cryptographic hashes, verification status, metric bounds, and formal descriptors.

use crate::transformation::TransformationKind;
use std::fmt::{self, Display, Formatter};
use std::sync::atomic::{AtomicU64, Ordering};

static CLAIM_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Cryptographic SHA-256 implementation (pure standard library, zero dependencies).
pub fn sha256(data: &[u8]) -> String {
    // Standard SHA-256 constants
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

/// An immutable verifiable receipt emitted for every executed transformation.
#[derive(Debug, Clone, PartialEq)]
pub struct ClaimReceipt {
    pub claim_id: String,
    pub transformation_name: String,
    pub kind: TransformationKind,
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
        let seq = CLAIM_COUNTER.fetch_add(1, Ordering::SeqCst);
        let raw_payload = format!(
            "seq={}|trans={}|kind={}|src={}|tgt={}|in={}|out={}|ver={}|disc={:.6}|loss={:.6}|err={:.6}",
            seq, transformation_name, kind, source_object, target_object, input_repr, output_repr,
            verified, roundtrip_discrepancy, information_loss, metric_error
        );
        let sha256_hash = sha256(raw_payload.as_bytes());
        let claim_id = format!("CLAIM-MIN79-{:04}-{}", seq, &sha256_hash[0..8]);

        Self {
            claim_id,
            transformation_name: transformation_name.to_string(),
            kind,
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
            "    {{\n      \"claim_id\": \"{}\",\n      \"transformation\": \"{}\",\n      \"kind\": \"{}\",\n      \"source\": \"{}\",\n      \"target\": \"{}\",\n      \"input\": \"{}\",\n      \"output\": \"{}\",\n      \"verified\": {},\n      \"roundtrip_discrepancy\": {},\n      \"information_loss\": {:.6},\n      \"metric_error\": {:.8},\n      \"sha256\": \"{}\",\n      \"notes\": \"{}\"\n    }}",
            self.claim_id,
            escape_json(&self.transformation_name),
            self.kind,
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
            "[{}] {} ({}) | {} -> {} | verified={}",
            self.claim_id,
            self.transformation_name,
            self.kind,
            self.source_object,
            self.target_object,
            self.verified
        )
    }
}

/// Registry storing all claims produced during Navigator workflows.
#[derive(Debug, Default)]
pub struct ClaimRegistry {
    claims: Vec<ClaimReceipt>,
}

impl ClaimRegistry {
    pub fn new() -> Self {
        Self { claims: Vec::new() }
    }

    pub fn register(&mut self, receipt: ClaimReceipt) {
        self.claims.push(receipt);
    }

    pub fn claims(&self) -> &[ClaimReceipt] {
        &self.claims
    }

    pub fn len(&self) -> usize {
        self.claims.len()
    }

    pub fn is_empty(&self) -> bool {
        self.claims.is_empty()
    }

    pub fn verify_all(&self) -> bool {
        !self.claims.is_empty() && self.claims.iter().all(|c| c.verified)
    }

    pub fn export_json(&self) -> String {
        let mut out = String::from("{\n  \"claim_registry_version\": \"v1.0\",\n  \"total_claims\": ");
        out.push_str(&self.claims.len().to_string());
        out.push_str(",\n  \"all_verified\": ");
        out.push_str(&self.verify_all().to_string());
        out.push_str(",\n  \"claims\": [\n");
        let snippets: Vec<String> = self.claims.iter().map(|c| c.to_json_snippet()).collect();
        out.push_str(&snippets.join(",\n"));
        out.push_str("\n  ]\n}\n");
        out
    }
}
