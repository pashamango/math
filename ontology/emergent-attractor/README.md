# Emergent Attractor — Lean 4 formal mathematics

**Status: PRIVATE SOURCE REVIEW · 53 theorem declarations · independent kernel replay PENDING.**

This is an independent Lake package inside the [Mindrock Math](../../README.md) monorepo. It explores conditional modal arguments, recursive positivity, lattice fixed points, finite-rank dynamics, convergence, and countermodels. The presence of the word *godlike* in formal definitions does **not** imply a proof of the existence of God, and the package makes no claim about proving the Riemann hypothesis.

## Source modules

| Module | Theorem declarations | Subject |
| --- | ---: | --- |
| `Emergent/Modal.lean` | 13 | Scott-style modal embedding, explicit assumptions, principal ultrafilters |
| `Emergent/Models.lean` | 11 | Finite countermodels, contingent actuality, distinct toy laws |
| `Emergent/Dynamics.lean` | 7 | Iteration, finite-rank reachability, cycles, non-attraction |
| `Emergent/SelfReference.lean` | 7 | Least and greatest fixed points, coherent recursive theories |
| `Emergent/SeededExample.lean` | 6 | A rule that derives properties and an explicit attractor |
| `Emergent/Enrichment.lean` | 5 | Contracting maps, complex quarter-turn, probability-versus-necessity |
| `Emergent/Bridge.lean` | 4 | Conditions connecting recursive positivity to attraction |
| **TOTAL** | **53** | Static declaration count; not independent kernel verification |

Entry point: [`Emergent.lean`](Emergent.lean). The [axiom audit](AxiomAudit.lean) names all 53 declarations for individual `#print axioms` inspection. Imported theorems and auxiliary definitions are separate from these 53 `theorem` declarations.

## Reproduce locally

Requires the pinned Lean 4 toolchain and the package-local Lake dependency lockfile. **Work from this directory**, not from the monorepo root:

```sh
cd ontology/emergent-attractor
lake build
lake env lean AxiomAudit.lean > axiom-audit.out
# Independently compare every reported axiom with the approved Lean axioms.
# If leanchecker is installed in the toolchain:
lake env leanchecker --verbose Emergent
```

**These commands are instructions, not tests already run by this GitHub import.** Verification status must be based on actual fresh logs and kernel outcomes. The original Oct 7 archive includes *self-reported* local successful logs; no independent clean-machine reproduction is currently documented.

## Versions and provenance

- `lean-toolchain`: `leanprover/lean4:v4.34.1`
- `lakefile.lean`: mathlib revision `d13f23b723b8a846827a245b89c10fc7d3f11612`
- `lake-manifest.json`: all nine dependencies pinned to specific Git revisions
- Original research ZIP SHA-256: `818c727d916275b0fd9e94274ecf87fd1a93aee10d7d77b746dc021966917ada`

All seven imported Lean modules and build/audit entrypoints have exact matching Git blob hashes against the original source-only package.

## Licensing and interpretation

`Emergent/Modal.lean` contains an adaptation of work by **Christoph Benzmüller**, credited in the source header. See [source details](docs/SOURCES.md), root [NOTICE](../../NOTICE.md), and [release gates](../../docs/RELEASE_GATES.md). Upstream CC BY 4.0 rights and adaptation notices must be preserved; the root Apache-2.0 license cannot silently relicense the upstream work.

Excluded intentionally from the initial GitHub import: the original automatic Actions workflow (unreviewed triggers), scripts that rewrite verification evidence, historical logs, compiled artifacts and LD_PRELOAD portability adapter. Verified CI, formal reviewer approval, per-file rights confirmation and security scanning remain blockers to any public release.

Coordinating work: [Invariant Foundry MIN-19](https://linear.app/mindrock/issue/MIN-19/ontological-lean-core-provenance-and-clean-room-verification) and [MIN-20](https://linear.app/mindrock/issue/MIN-20/library-product-reusable-lean-package-and-api-proposal).
