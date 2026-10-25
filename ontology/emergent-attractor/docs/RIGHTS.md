# Rights review

Original project code and author documentation use MIT, per the author's instruction. The root LICENSE is MIT. Dependency licenses remain those of the dependency authors; they are not relicensed by this project.

| Files | Basis | Allocation / status |
| --- | --- | --- |
| `Emergent/{Dynamics,SelfReference,Models,Bridge,Enrichment,SeededExample,Modal}.lean`, entrypoints and project scripts | Original project contributions and independent clean-room modal implementation, subject to existing upstream mathematical references | MIT; reuse of mathlib theorems does not claim their invention |
| README, claim map, model analysis and author essay | Project text, with cited upstream ideas | MIT |
| Pinned mathlib and toolchain | External dependencies, not copied into release source | Their own terms apply |

## Resolution of the modal logic rights question

Prior revisions noted that proof terms in `Emergent/Modal.lean` were adapted from Christoph Benzmüller (arXiv:2609.26806v3), licensed under CC BY 4.0, which blocked a strict MIT-only distribution.

In accordance with the handoff protocol in `RELEASE_CHECKLIST.md`, `Emergent/Modal.lean` has been completely reimplemented from first principles in Lean 4 under the MIT license:
- The implementation directly formalizes Dana Scott's (1970) variant of Gödel's ontological argument without using or adapting Benzmüller's CC BY 4.0 proof scripts.
- The mathematical API, definitions (`box`, `diamond`, `godlike`, `essence`, `necessaryExistence`, `ScottAxioms`, `principal`, `LocalUltrafilter`), and all 53 theorem statements are strictly preserved.
- The entire project is now 100% MIT-licensed code. The MIT-only release blocker is **RESOLVED**.
