# Notices, attribution, and license exceptions

**License allocation is preliminary until reviewed.** The repository's [Apache-2.0 LICENSE](LICENSE) is for original code the contributors are entitled to license. It does **not** override rights in third-party source, papers, datasets, or adapted material.

## Emergent Attractor: modal-logic adaptation

The planned file `ontology/emergent-attractor/Emergent/Modal.lean` adapts portions of Christoph Benzmüller's Lean sources accompanying:

Christoph Benzmüller, *Gödel's and Scott's Variants of the Ontological Argument in Lean 4 and TPTP THF*, arXiv:2609.26806v3 (2026).
- Paper and ancillary source: https://arxiv.org/abs/2609.26806
- License for the source release: [Creative Commons Attribution 4.0 International](https://creativecommons.org/licenses/by/4.0/)
- Source references: `Notes/HOMLinHOL.lean`, `Notes/ModalFilter.lean`, `Notes/ScottVariantHOML.lean`

The adaptation uses generic world/entity types, explicit actualist quantification, explicit hypothesis structures instead of global assumptions, and reworked proof details. It is **not** a verbatim copy or a claim of independent invention of Scott's modal argument.

**Before the adapted file is committed or the repository is made public:** preserve attribution in file headers and the package source notice, mark modifications, confirm license compatibility and provenance for each redistributed source, and review the complete package for other third-party works. A license link is not a substitute for documenting this review.

No endorsement by the cited author is implied.
