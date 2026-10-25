# Rights and provenance record

Project-authored source code and documentation, including the current `Emergent/Modal.lean`, are offered under [MIT](../../../LICENSE). External dependencies retain their own terms and are not relicensed here.

## Why the modal module was replaced

The previous modal proof module identified Scott proof terms adapted from Christoph Benzmüller's arXiv:2609.26806v3. Its article page identifies CC BY 4.0 and says Lean sources accompany the article as ancillary files. The earlier PR #9 rewrite was not clean-room and is not the basis for the current MIT implementation.

## Process used for the current candidate

- The specifier had seen the previous source and did not act as implementer.
- The specifier passed only formal definitions, Scott axiom structure and exact theorem statements. No prior theorem proof bodies, source comments or repository checkout were included.
- A separate AI implementer was started with no inherited conversation history and no source access; it returned the current module and attested it did not access source files.
- The source/output hashes and limits of this record are in [PROVENANCE_DOSSIER.md](PROVENANCE_DOSSIER.md).

The current candidate does not use upstream Lean proof bodies. This record documents the implementation process, not a legal opinion. The project owner accepts responsibility for proceeding on this basis.
