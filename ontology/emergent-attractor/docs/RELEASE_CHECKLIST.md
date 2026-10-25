# Publication checklist

1. Claim map: complete, all 53 signatures indexed; author proposals mapped to code and limitations.
2. Models: singleton Scott interpretation, distinct two-world countermodel and seeded recursive example documented. Vacuity, built-in targets, coherence and modal/temporal distinctions explicit.
3. Separate mathematical review: completed by an independent AI review pass; no external human peer review represented or fabricated. Review anchors original source blobs. Publication edits to six source files add MIT comment headers only.
4. Extended CI: implemented; current candidate must pass build, all-theorem audit, fault injection and kernel replay, with inspected artifacts. Dependency cache is disclosed.
5. Rights: original code MIT; obsolete project license allocations removed. Adapted `Modal.lean` has no verified MIT-only grant. **BLOCKED for MIT-only release.**
6. Editorial package: README, author essay with scope note, models, claims and source notices prepared. Historical assertions of release readiness removed. Citation dates must be assigned only on actual release.
7. Release/tag: **NOT PERFORMED** until item 5 is resolved and current CI receipts archived. Proposed package tag: `emergent-attractor-v0.1.0`, avoiding accidental release of unrelated monorepo work. After merge, require green main CI before tagging; verify tag commit and release assets by readback.

## Exact handoff for the unresolved rights question

`Emergent/Modal.lean` explicitly adapts proof terms from Christoph Benzmüller, arXiv:2609.26806v3, source archive SHA-256 `27bbcd73582c190a0831c74de9710a6ab3a1e295d07050661c5e7cdf950c2a86`. The version page advertises CC BY 4.0; inspected ancillary modules offer no separate MIT grant. Obtain appropriate MIT permission or deliver an independently implemented and provenance-reviewed modal module. Preserve the mathematical API and all 53 existing theorem statements; rerun the complete CI and independent review for any implementation replacement. Changing the header alone is insufficient. Do not contact rights holders without the user's authorization.
