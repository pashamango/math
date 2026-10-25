# Sources and provenance

Mathematical formulation of Scott's variant: Dana Scott, *Gödel's Ontological Argument*, 1970; Kurt Gödel, 1970.

Historical source reviewed during the earlier implementation: Christoph Benzmüller, *Gödel's and Scott's Variants of the Ontological Argument in Lean 4 and TPTP THF*, arXiv:2609.26806v3. [Version page](https://arxiv.org/abs/2609.26806v3), [source archive](https://arxiv.org/src/2609.26806v3). Archive SHA-256: `27bbcd73582c190a0831c74de9710a6ab3a1e295d07050661c5e7cdf950c2a86`. The article page identifies CC BY 4.0 and says Lean sources are distributed as ancillary files.

The current `Emergent/Modal.lean` was created in a separate task from a specification containing only the API definitions, Scott axioms and theorem statements. No upstream or prior project proof bodies were supplied to the implementer. See the [provenance dossier](PROVENANCE_DOSSIER.md). The paper is cited for mathematical context, not as the source of the current proof terms.

## Formal dependency and novelty boundary

mathlib4 is pinned to `d13f23b723b8a846827a245b89c10fc7d3f11612`. Existing mathlib results are reused and are not claimed as new discoveries.

## Verification

Prior Actions runs verified earlier candidates, not the current source. A new CI run and artifact inspection are required for the current PR head.
