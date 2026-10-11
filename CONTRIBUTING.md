# Contributing

This is a reproducible mathematics repository, not an assertion that every research hypothesis is proven. Discuss scope first; keep changes small and traceable.

1. Work in a review branch and link the relevant issue or source.
2. Identify every theorem's assumptions, explicit axioms, dependencies, and any external source adaptation.
3. For Lean projects, pin a `lean-toolchain`, `lakefile.lean`, and `lake-manifest.json` **inside the package**.
4. Run a clean `lake build` and kernel replay; record the exact toolchain and commit digest. Never treat a previous log as proof of a fresh run.
5. Do not add `sorry`, `admit`, `unsafe`, hidden or unaudited global axioms, generated private files or credentials.
6. Require a code review, a source/license review, and independent verification before marking a release done.

Read [release gates](docs/RELEASE_GATES.md) and the [license exceptions](NOTICE.md).
