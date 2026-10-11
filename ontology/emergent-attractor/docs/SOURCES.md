# Primary sources, attribution and verification boundaries

The original source archive reports retrieval of research materials on 7 October 2026. Links and quoted upstream provenance are reproduced from that archive; **this GitHub import has not independently fetched or validated every remote document.**

## Modal-logic adaptation: third-party CC BY 4.0

**Christoph Benzmüller.** *Gödel's and Scott's Variants of the Ontological Argument in Lean 4 and TPTP THF*. arXiv:2609.26806v3 (2026).
- Article: https://arxiv.org/abs/2609.26806
- Source archive: https://arxiv.org/src/2609.26806
- Original source archive digest (reported by source package): SHA-256 `27bbcd73582c190a0831c74de9710a6ab3a1e295d07050661c5e7cdf950c2a86`.
- Relevant upstream modules: `Notes/HOMLinHOL.lean`, `Notes/ModalFilter.lean`, `Notes/ScottVariantHOML.lean`.
- Upstream source/article release was reported as **Creative Commons Attribution 4.0 International**: https://creativecommons.org/licenses/by/4.0/
- Our adapted file: [`../Emergent/Modal.lean`](../Emergent/Modal.lean).
- Changes described by the source authors: generic type parameters, explicit actualist quantification, `ScottAxioms` as a parameter instead of global axioms, reworked elementary equality proof and removal of copied open/countermodel placeholders.

Keep the original author attribution, source and license links and an explanation of these modifications. The repository does not claim independent invention of Gödel/Scott ontological arguments or affiliation with upstream authors. Before a public release, independently confirm this license for each original source file and the compatibility of all adapted material. The root Apache-2.0 license applies only to rights-cleared original material.

## Interpretive and comparative literature

- Christoph Benzmüller, *A Comment on Modal Collapse and Ultrafilters in Gödel's Ontological Argument*, https://arxiv.org/abs/2608.07578 (motivation for separating positivity and rigidity).
- Christoph Benzmüller, *Proofs Without Nominals: Gödel's Ontological Argument, its Shallow Embedding, and the Open Questions of the Monatshefte Notes*, https://arxiv.org/abs/2609.36279 (context for world-sensitive instantiation).
- Benzmüller and Fuenmayor, *Computer-supported Analysis of Positive Properties, Ultrafilters and Modal Collapse in Variants of Gödel's Ontological Argument*, https://arxiv.org/abs/1910.08955 (Scott/Anderson/Fitting comparison; not imported or independently replayed).
- OpenAI, *Sharing AI progress in mathematics*, 6 October 2026, https://openai.com/index/sharing-ai-progress-in-mathematics/ (format inspiration only; no result used as an axiom).

## Formal dependency

[mathlib4](https://github.com/leanprover-community/mathlib4), pinned Git SHA `d13f23b723b8a846827a245b89c10fc7d3f11612`. Relevant modules include `Mathlib.Order.FixedPoints`, `Mathlib.Topology.MetricSpace.Contracting`, `Mathlib.Analysis.Complex.Basic`, and `Mathlib.MeasureTheory.Measure.Dirac.Basic`. The Knaster–Tarski, contraction, complex number and measure results reused here are **not** new mathematical discoveries.

## Evidence labels

- **SOURCE FACT:** imported Lean text is present on a private GitHub branch; exact Git blob SHA matches the source-only ZIP.
- **STATIC DERIVED FACT:** there are 53 literal `theorem` declarations across seven modules.
- **SOURCE TESTIMONY:** original archive logs assert previous successful local build/kernel replay.
- **NOT YET VERIFIED:** independent clean Lean compilation, live kernel replay, axiom allowance, source copyright/licensing review and public scientific interpretation.

See [root NOTICE](../../../NOTICE.md) for redistribution conditions. Publication requires independent reviewers and Pasha's explicit final approval.
