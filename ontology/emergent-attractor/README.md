# Emergent Attractor

A Lean 4 library of conditional modal arguments, recursive positivity, fixed points, dynamics and countermodels. **53 theorem declarations across seven source modules.**

[Claims and hypotheses](docs/CLAIMS.md) · [All signatures](docs/SIGNATURES.md) · [Models](docs/MODELS.md) · [Separate AI review](docs/INDEPENDENT_REVIEW.md) · [Author's essay](docs/ESSAY.md)

## What the library establishes

The reference Scott branch derives possibility, necessity, uniqueness and modal collapse under explicit assumptions. A separate recursive branch combines safe monotone closure with supplied targets and natural-number rank descent. Concrete countermodels show why fixedness, attraction, probability one and necessity must be distinguished. A seeded example makes a nonconstant closure rule and finite dynamics explicit. Standard mathlib contraction results provide a separate asymptotic convergence theorem.

These results do not identify a formal object with a real deity or derive physical laws. The two proposals in the essay are an interpretive research program; their precise relationship to the existing formalization is documented in the claim map. All substantive modal, closure, rank and contraction hypotheses are part of each theorem's contract. A kernel-axiom whitelist does not discharge them.

## Verification

Toolchain: `leanprover/lean4:v4.34.1`. mathlib revision: `d13f23b723b8a846827a245b89c10fc7d3f11612`; remaining Git dependencies are locked in `lake-manifest.json`.

```sh
cd ontology/emergent-attractor
lake exe cache get
lake build
mkdir -p verification
lake env lean AxiomAudit.lean > verification/axiom-audit.log
python3 scripts/check_audit.py verification/axiom-audit.log
python3 scripts/test_audit_gate.py
lake env leanchecker --verbose Emergent
```

GitHub [PR #8](https://github.com/pashamango/math/pull/8) runs this process on Ubuntu, rebuilds project modules after deleting local package outputs, and retains source digests, versions, checkout SHA and separate logs. Dependency binaries may be restored from mathlib's cache; this is not a full source rebuild of all dependencies. `leanchecker` uses Lean's kernel to replay project declarations against imported environments; it is not an independently implemented verifier. GitHub's PR checkout is a synthetic merge commit, recorded in the artifact.

Prior run [40155461394](https://github.com/pashamango/math/actions/runs/40155461394) passed 1973 build jobs and all 53 audit entries (16 axiom-free; 37 using only `propext`, `Classical.choice`, `Quot.sound`). Extended run [40156341102](https://github.com/pashamango/math/actions/runs/40156341102) passed fresh package build, all 53 audits, eight audit-gate corruption cases and kernel replay of all eight project modules. Essential logs and the inspected artifact receipt are archived in [Git evidence](../../evidence/publication/40156341102/). Artifacts expire after 90 days; essential logs for the extended run have been retained in Git.

## License and release status

Original project contributions are **MIT**. The adapted modal module retains upstream attribution; a MIT-only grant has not been verified. [Rights review](docs/RIGHTS.md) records the remaining blocker with source digests. **Release candidate under review; no new MIT-only release is authorized by the evidence.**

The separate review is an independent AI review pass, not external human peer review. [Release checklist](docs/RELEASE_CHECKLIST.md) records open and completed gates. [Sources](docs/SOURCES.md) distinguish borrowed mathematics, historical testimony and current verification.
