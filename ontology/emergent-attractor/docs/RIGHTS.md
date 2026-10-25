# Rights review

Original project code and author documentation use MIT, per the author's instruction. The root LICENSE is MIT. Dependency licenses remain those of the dependency authors; they are not relicensed by this project.

| Files | Basis | Allocation / status |
| --- | --- | --- |
| `Emergent/{Dynamics,SelfReference,Models,Bridge,Enrichment,SeededExample}.lean`, entrypoints and project scripts | Original project contributions, subject to existing upstream mathematical references | MIT; reuse of mathlib theorems does not claim their invention |
| README, claim map, model analysis and author essay | Project text, with cited upstream ideas | MIT |
| `Emergent/Modal.lean` | Source header explicitly identifies adapted Benzmüller proof terms | Upstream attribution retained; no MIT-only grant verified; **MIT-only release blocker** |
| Pinned mathlib and toolchain | External dependencies, not copied into release source | Their own terms apply |

Downloaded `https://arxiv.org/src/2609.26806v3`; the archive hash exactly matches the previously recorded source hash. Inspected `anc/Notes/HOMLinHOL.lean`, `anc/Notes/ModalFilter.lean`, `anc/Notes/ScottVariantHOML.lean` and `anc/README.md`. These files contain no separate MIT permission. The arXiv version page labels the article CC BY 4.0. That page is evidence of the posted license, not an invented MIT permission for the adaptation. Digests and retrieval URLs are in [UPSTREAM_PROVENANCE.json](UPSTREAM_PROVENANCE.json).

The adaptation changes generic world/entity parameters, actualist quantification, explicit hypothesis structures instead of global axioms, proof presentation and some elementary equality proofs. Attribution and modification history must remain. No endorsement is implied.

To satisfy a strict MIT-only release, obtain an explicit MIT-compatible grant from the rights holder or supply a provenance-reviewed independent implementation. Replacing a license label or deleting attribution does not resolve the blocker. No such grant has been obtained in this session; nobody was contacted.
