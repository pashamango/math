//! Typed Navigator and epistemic claim registry reference for AMDE-RP-002.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformKind { Iso, Adjoint, Lossy, Approximate }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimKind { Theorem, Formalization, LemmaTarget, Conjecture, Experiment, Analogy, Metaphor }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object { pub x: String, pub r: String, pub sigma: String, pub tau: String, pub i: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transformation { pub from: String, pub to: String, pub kind: TransformKind }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Claim { pub id: &'static str, pub kind: ClaimKind, pub statement: &'static str }

#[derive(Debug, Default)]
pub struct Navigator { pub objects: Vec<Object>, pub transformations: Vec<Transformation>, pub claims: Vec<Claim> }

impl Navigator {
    pub fn add_transformation(&mut self, t: Transformation) { self.transformations.push(t); }

    /// A claim may be referenced by a transformation only after explicit registration.
    pub fn register_claim(&mut self, claim: Claim) { self.claims.push(claim); }

    /// Metaphors cannot be promoted silently: promotion requires an explicit target kind.
    pub fn promote(&mut self, id: &'static str, target: ClaimKind) -> Result<(), &'static str> {
        let claim = self.claims.iter_mut().find(|c| c.id == id).ok_or("claim not found")?;
        if claim.kind == ClaimKind::Metaphor && target != ClaimKind::Analogy {
            return Err("metaphor promotion requires an explicit review path");
        }
        claim.kind = target;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn blocks_metaphor_to_lemma_promotion() {
        let mut n = Navigator::default();
        n.register_claim(Claim { id: "RP-002-C07", kind: ClaimKind::Metaphor, statement: "navigation as a compass" });
        assert_eq!(n.promote("RP-002-C07", ClaimKind::LemmaTarget), Err("metaphor promotion requires an explicit review path"));
    }
}
