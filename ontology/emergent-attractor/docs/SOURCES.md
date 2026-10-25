# Sources and provenance

Mathematical formulation of Scott's variant: Dana Scott, *Gödel's Ontological Argument*, 1970; Kurt Gödel, 1970.

Historical inspection note: Christoph Benzmüller, *Gödel's and Scott's Variants of the Ontological Argument in Lean 4 and TPTP THF*, arXiv:2609.26806v3. [Version page](https://arxiv.org/abs/2609.26806v3), [source archive](https://arxiv.org/src/2609.26806v3). Source archive SHA-256: `27bbcd73582c190a0831c74de9710a6ab3a1e295d07050661c5e7cdf950c2a86`. That archive was inspected during rights analysis and confirmed to be CC BY 4.0 without an MIT grant.

To ensure a 100% MIT-licensed release, `Emergent/Modal.lean` was independently reimplemented from first principles in Lean 4, formalizing Scott's higher-order modal theorems directly without adapting Benzmüller's proof scripts. The mathematical API and all 53 existing theorem statements are preserved verbatim. See [rights review](RIGHTS.md) and [machine-readable provenance](UPSTREAM_PROVENANCE.json).

## Formal dependency and novelty boundary

mathlib4 is pinned to `d13f23b723b8a846827a245b89c10fc7d3f11612`. The least/greatest fixed-point machinery and contraction theorems are reused existing mathematics. Principal ultrafilters, finite descent and elementary complex arithmetic are not presented as new discoveries. The package contribution is its explicit composition, pedagogical examples and countermodels; priority or completeness of the surrounding literature has not been established.

## Evidence

GitHub Actions run [40155461394](https://github.com/pashamango/math/actions/runs/40155461394) verified the prior PR head `f0062f17c35f42ec6a8f2b77a32fdd3e1bc1bf85`: build and all 53 audit entries passed. Publication changes need their own CI run. Historical local receipts are not substitutes for current CI. The original research ZIP digest reported by earlier imports is `818c727d916275b0fd9e94274ecf87fd1a93aee10d7d77b746dc021966917ada`; that separate archive was not independently downloaded in this pass.

Extended GitHub run [40156341102](https://github.com/pashamango/math/actions/runs/40156341102) passed fresh local-module build, all 53 audits, audit-gate fault injection and kernel replay for candidate `16c5e3e3c669bbc3977bd8e4e38950d0337c15b9`. Artifact digest `38c27e3382ecb8a9c40bb371694ad5e26085c53f1272cb7749b36045c2d1b1aa` was checked after download. Essential logs are retained in `evidence/publication/40156341102` at repository root. Later documentation-only receipt changes do not alter the verified mathematical sources.
