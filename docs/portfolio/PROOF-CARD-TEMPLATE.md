# Proof Card v1 — [Theorem short name]

**State:** DRAFT | SOURCE_READ | KERNEL_VERIFIED | INDEPENDENTLY_REPLAYED | RELEASED  
**Repository:** `pashamango/math` (private until owner explicitly authorizes public release)  
**Commit:** [full immutable SHA]  
**Source:** [package/path.lean, theorem name]

## Human-readable claim

> [One bounded sentence. Distinguish logical implication from claims about the physical world.]

## Lean 4 theorem signature

```lean
-- Exact theorem signature from the pinned commit only
```

## Assumptions / explicit trust base

- Typeclass and axioms:
- Scott/modal assumptions or fixed-point safety/seed/rank conditions:
- Required nonemptiness/inhabitedness premises:
- `#print axioms` output on independent runner, exact list:
- Trusted definitions and libraries:

## Independent reproducibility

| Check | Toolchain/hash | Command | Exit | Evidence |
| --- | --- | --- | --- | --- |
| Source integrity | [sha256] | [read-only digest] | UNKNOWN | [receipt] |
| Clean Lake build | [Lean mathlib pins] | `lake build` | UNKNOWN | [receipt] |
| Axiom audit | [Lean pin] | `lake env lean AxiomAudit.lean` | UNKNOWN | [receipt] |
| Kernel replay | [checker pin] | [checker command] | UNKNOWN | [receipt] |
| Negative controls | [pin] | [command] | UNKNOWN | [receipt] |
| Rights and provenance | [source license] | [review] | UNKNOWN | [receipt] |

**Only a real run turns UNKNOWN into PASS.** A theorem declaration count is not proof and an interpreter/CI report is not an independent kernel replay.

## Model / countermodel

- Concrete model satisfying assumptions:
- Stronger but false conjecture:
- Explicit counterexample or unresolved counterexample search:
- Applicability boundaries:

For Emergent Attractor consider the distinction `Coherent (leastTheory F)` vs `(leastTheory F).Nonempty`; `F = id` has empty least fixed point, so no unconditional existence follows.

## Why an engineering buyer should care

- Specific task this can audit: [e.g., AI-agent authorization invariant]
- Demonstrable input → verifier → independently replayable acceptance result:
- What remains outside the verifier:
- Time saved / quality comparison: **NOT MEASURED** until benchmarked.

## Publication and rights gate

- Third-party sources, CC BY 4.0 adaptations and NOTICE:
- G0 / G1 / G2 / G3 / G4:
- Reviewer and receipt:
- Public URL / citation: PENDING OWNER APPROVAL

Do not publish a card whose cited proof remains in a private repo. Do not advertise any theorem as proof of empirical divine existence.
