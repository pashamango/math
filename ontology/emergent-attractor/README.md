# Emergent Attractor — Lean 4

**Status: private structural placeholder; source import and independent review pending.**

The research archive `emergent-attractor-v0.1.zip` (2026-10-07) contains:
- 7 substantive `Emergent/*.lean` modules and 53 literal `theorem` declarations
- pinned Lean 4.34.1 and mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612`
- `lakefile.lean`, `lake-manifest.json`, proof catalogue, axiom audit, and internal logs

This staging directory does **not** yet contain the Lean source files. They will be imported after the adapted-code attribution/license review. The original package logs report successful local build/kernel replay, but an independent clean-environment reproduction has not yet happened.

## Intended layout

```text
ontology/emergent-attractor/
  lean-toolchain
  lakefile.lean
  lake-manifest.json
  Emergent.lean
  Emergent/*.lean
  AxiomAudit.lean
  scripts/
  docs/SOURCES.md
  README.md
```

Use the package directory, rather than the repository root, as the current working directory when building.

## Epistemic limits

A Lean theorem with explicit `ScottAxioms` or other modeling assumptions establishes an implication **under those assumptions**. It does not by itself establish necessary divine existence in reality, solve the Riemann hypothesis, or prove scientifically measurable attractors exist.

Trace original authors and terms in [sources](docs/SOURCES.md), repository [NOTICE](../../NOTICE.md), and [release requirements](../../docs/RELEASE_GATES.md).
