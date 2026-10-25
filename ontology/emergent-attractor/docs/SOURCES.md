# Sources and provenance

Mathematical formulation of Scott's variant: Dana Scott, *Gödel's Ontological Argument*, 1970; Kurt Gödel, 1970.

Historical source reviewed during the earlier implementation: Christoph Benzmüller, *Gödel's and Scott's Variants of the Ontological Argument in Lean 4 and TPTP THF*, arXiv:2609.26806v3. [Version page](https://arxiv.org/abs/2609.26806v3), [source archive](https://arxiv.org/src/2609.26806v3). Archive SHA-256: `27bbcd73582c190a0831c74de9710a6ab3a1e295d07050661c5e7cdf950c2a86`. The article page identifies CC BY 4.0 and says Lean sources are distributed as ancillary files.

The current `Emergent/Modal.lean` was created in a separate task from a specification containing only the API definitions, Scott axioms and theorem statements. No upstream or prior project proof bodies were supplied to the implementer. See the [provenance dossier](PROVENANCE_DOSSIER.md). The paper is cited for mathematical context, not as the source of the current proof terms.

## Formal dependency and novelty boundary

mathlib4 is pinned to `d13f23b723b8a846827a245b89c10fc7d3f11612`. Existing mathlib results are reused and are not claimed as new discoveries.

## Verification

Current candidate verification: PR head `53e275114990631e7309a988483f5444fc3d3c9d` passed [Actions run 40165513435](https://github.com/pashamango/math/actions/runs/40165513435). The [verification artifact](https://github.com/pashamango/math/actions/runs/40165513435/artifacts/12588072185) was read back and its SHA-256 (`6c378d8223e1b73e2349a8da691ea0c339376626fffefc914a5c488dfee318f3`) and embedded source hashes were verified.
