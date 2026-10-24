# Mindrock Math

A collection of reproducible formal mathematics, proof experiments, countermodels, and reusable Lean libraries.

> **Private staging — not yet a public release.** No theorem here establishes physical existence, the truth of its modeling premises, or an unrelated open conjecture.

## Packages

| Area | Package | Scope | Release gate |
| --- | --- | --- | --- |
| [Ontology](ontology/) | [Emergent Attractor](ontology/emergent-attractor/) | Modal logic, recursive positivity, fixed points, finite-rank reachability and countermodels | Independent Lean replay and attribution review pending |
| [Verification](docs/SSO.md) | [SSO](src/pashamango/math/sso/) | Scientific Objects on a Hash Sphere: canonical JSON framing, SHA-256 rarity, affine cycles | Python unittest suite passing; golden vectors pinned |

Future independent areas may include `number-theory/`, `dynamics/`, `probability/`, and `formal-methods/`. Dependencies and Lean toolchains belong to each package rather than to a shared root `lakefile`.

## Epistemic contract

Every package must distinguish (1) machine-checked *conditional* theorem statements, (2) assumptions and nonstandard axioms, (3) separately checked compilation/kernel evidence, and (4) scientific interpretation or speculation. Reports of successful builds are evidence, not independent re-execution.

The initial Emergent Attractor research archive reports **7 substantive Lean modules and 53 theorem declarations**. A read-only file audit has checked this count; a fresh clean-environment kernel replay remains pending.

## License and upstream research

Original code is released under the [MIT License](LICENSE), **except where otherwise noted**. Adapted upstream source material retains the applicable third-party license (including CC BY 4.0), authorship credit and change notices. See [NOTICE.md](NOTICE.md) and each package's `docs/SOURCES.md`. Do not infer that a root license relicenses third-party works.

## Release and contributions

Read [release gates](docs/RELEASE_GATES.md), [CONTRIBUTING](CONTRIBUTING.md), and [SECURITY](SECURITY.md). Releases require pinned dependencies, independent build evidence, an axiom audit, license review, and a checked artifact hash. The repository is kept private until these gates and an explicit publication decision are satisfied.

Project coordination: [Invariant Foundry](https://linear.app/mindrock/project/invariant-foundry-research-to-products-c7c4dc27611f). Research provenance: [Scheduler Commons #168](https://github.com/pashamango/minutta-rp-pr-cycle-lab/issues/168) (permission-restricted).

This repository is not affiliated with, or endorsed by, upstream researchers.
