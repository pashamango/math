# Release gates — private by default

A draft is not a release. At each package version, record a reproducible evidence ledger.

## G0 Provenance and rights
- Identify source archives, upstream revisions, source digests and original authors.
- File-by-file rights and adapted code notices. Retain CC BY 4.0 attribution and modifications where applicable.
- Approve the LICENSE/NOTICE allocation; scan for secret or private artifacts.

## G1 Package layout
- Independent package directory; toolchain, Lake manifest, README and own verification scripts.
- No root-level Lean toolchain forcing unrelated packages to use the same dependency graph.
- No generated build artifacts in release source.

## G2 Technical verification
- Clean standard environment; `lake exe cache get` (optional) and `lake build` using exact pins.
- Independent `lake env leanchecker` replay for all intended modules and `#print axioms` audit.
- Check actual theorem signatures and assumptions, not just the declaration count.
- Record exit codes, commit SHA, toolchain/hash, logs and reviewer identity.
- No automatic GitHub Actions workflows until owner has authorized execution; later pin Actions to reviewed immutable SHAs.

## G3 Review and scope
- Verify statements against their hypotheses; publish limitations and counterexamples.
- Independent research and license reviews, with author/maintainer veto respected.

## G4 Publish
- Recheck repository is meant to become public. Explicitly authorize visibility change.
- Merge only a reviewed commit; publish release/tag with immutable digest and verified CI.
- Record public URL and provenance in Linear; no status `DONE` without readback.

Current state for `ontology/emergent-attractor`: static source audit reported seven substantive modules and 53 theorem declarations; independent clean-machine verification and license review remain open.
