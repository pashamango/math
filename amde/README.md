# AMDE-RP-002: Navigator

## Specification

An AMDE object is the tuple **(X, R, Σ, τ, I)**: carrier/data `X`, relations `R`, admissibility/constraints `Σ`, transition semantics `τ`, and invariants/observations `I`. A Navigator is a directed graph of such objects. Every edge has exactly one transformation kind:

| Kind | Contract |
| --- | --- |
| `ISO` | structure-preserving, reversible correspondence |
| `ADJOINT` | directed paired transformation with an explicitly stated adjunction |
| `LOSSY` | intentionally discards information; reversibility is forbidden |
| `APPROXIMATE` | bounded or heuristic approximation; error/bound is required |

Claims are registered with one epistemic kind: `THEOREM`, `FORMALIZATION`, `LEMMA_TARGET`, `CONJECTURE`, `EXPERIMENT`, `ANALOGY`, or `METAPHOR`. Registration is mandatory. A `METAPHOR` cannot silently become a lemma target; an explicit reviewed promotion is required (the reference rejects direct promotion).

## RP-002 seed registry

| ID | Kind | Seed claim |
| --- | --- | --- |
| RP-002-C01 | FORMALIZATION | Objects are represented as `(X,R,Σ,τ,I)`. |
| RP-002-C02 | FORMALIZATION | Navigator edges carry one explicit transformation kind. |
| RP-002-C03 | LEMMA_TARGET | `ISO` preserves the declared structure and admits an inverse. |
| RP-002-C04 | LEMMA_TARGET | `LOSSY` edges do not promise an inverse. |
| RP-002-C05 | CONJECTURE | An `APPROXIMATE` edge requires a stated error model. |
| RP-002-C06 | FORMALIZATION | Every graph claim has one registry entry and epistemic kind. |
| RP-002-C07 | METAPHOR | “Navigation as a compass” is explanatory language only. |
| RP-002-C08 | EXPERIMENT | Dark-math wheel integration is an `APPROXIMATE` backend (MIN-67). |

The seed deliberately contains no untyped claim and makes the boundary between explanatory language and proof obligations executable.
