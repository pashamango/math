# Merge readiness and public-release plan: PR #3 and PR #4

**Review date:** 2026-10-11  
**Review type:** read-only GitHub connector metadata, patch inspection, workflow receipt and root release-policy review.  
**Status:** no merges executed; repository remains private. This is a review artifact, not a CI or kernel replay.

## Snapshot

| Check | PR #3 SSO | PR #4 Emergent Attractor |
| --- | --- | --- |
| URL | https://github.com/pashamango/math/pull/3 | https://github.com/pashamango/math/pull/4 |
| State | Open Draft | Open Draft |
| Head SHA | 387aabec5c6be93a8ece74006622b41cfa2379a7 | 77126f51ec3d342d96dac8d42304a439205b870c |
| Mergeable reported | false | true |
| Base comparison | main ahead 1; feature ahead 2 | feature ahead 14; behind main 0 |
| Changed files | 11 | 14 |
| CI evidence | run 38102306904 completed success | No workflow runs returned for head |
| Reviews / inline threads | 0 / 0 | 0 / 0 |
| Required status API | 403 FORBIDDEN: Resource not accessible by integration | Same 403 |
| Scientific release ready | NO | NO |
| Private merge ready | NO until divergence and contractual gates addressed | NO until clean build, proof/axiom and rights gates addressed |

Important: GitHub's "mergeable" flag is about merge mechanics; it is not scientific validity and not confirmation that protected-branch rules were satisfied.

## PR #3: exact issues

1. ROOT README OVERWRITE: main's README is a multi-package, carefully qualified document (ontology, provenance, LICENSE/NOTICE, private staging, release gates), whereas #3 replaces the initial two-line placeholder with an SSO-centric README. The branch was created from commit 51be1c2... while main advanced to 1a297875..., so the README changes collide. Preserve all existing main sections and add an SSO row and verification instructions; do not replace main's epistemic and publication cautions.
2. DIVERGENCE: main is one commit ahead of the SSO head, and GitHub reports mergeable=false. Reconcile main into the PR head in an isolated worktree/branch with conflict-resolution review. Avoid force-push unless a documented approval and lease are used; never overwrite collaborators' work.
3. CI POSITIVE FACT: SSO verification workflow run ID 38102306904 concluded success. The job python-unittest and its Install package / Verify steps were successful. This validates the current configured tests, NOT full scientific hypotheses or external interoperability.
4. CANDIDATE HARDENING: enforce a deterministic test of the golden vector in another implementation/language; explicit reject reasons for malformed encodings; mutation/version cases; property-based/adversarial cases for delimiter ambiguity, non-NFC strings, floats/NaN, namespace collisions, secret/provenance spoofing and changed-rule semantics. Separate minimum release requirements from follow-on research hypotheses H2–H5.
5. THIRD-PARTY / SUPPLY CHAIN: source tree currently adds a workflow using floating action tags (actions/checkout@v4; actions/setup-python@v5). Root RELEASE_GATES says approval required before workflow introduction and later pins to reviewed immutable SHAs. Review/approve this existing workflow and pin actions before broader public release.
6. PACKAGE POLICY: verify LICENSE, README, Python package metadata, name conflicts, scripts, secret scanning and required tests. The PR currently describes the pilot as alpha and explicitly forbids automatic merge or public release.

**Recommended disposition:** keep Draft until README conflict resolved and release-scope gates are recorded. A green CI receipt is already available, so there is no need to describe CI as unknown.

## PR #4: exact issues

1. BRANCH COMPATIBILITY: base main matches current main ancestor, and GitHub reports mergeable=true. No observed code conflict, but this alone does not authorize a merge.
2. FORMAL CHECKS MISSING: no workflow runs were found for head SHA. In an independent clean environment pin Lean v4.34.1, mathlib d13f23b723b8a846827a245b89c10fc7d3f11612 and package-local manifest. Run lake build, lake env lean AxiomAudit.lean, and leanchecker replay for intended modules when compatible. Archive exit codes, stdout/stderr, binary/toolchain hashes, source SHA, environment manifest and a reviewer identity.
3. AXIOM DISCLOSURE: audit all 53 printed theorem declarations, proof assumptions and any use of classical choice, propositional extensionality and quotients; no sorry/admit/unapproved axioms. A "theorem" token count proves only that declarations exist.
4. RIGHTS / ATTRIBUTION: Modal.lean adapts source attributed to Christoph Benzmüller; SOURCES.md reports CC BY 4.0 but explicitly states independent source/license verification is incomplete. Verify the exact upstream file and terms, keep original attribution and change notices, and reconcile root NOTICE language that still describes the adaptation as planned/pre-commit.
5. TEST TRUST: the original archive's historical build logs are source testimony, not clean-room replication. Do not claim fresh kernel success until a new trusted run is available.
6. CI SAFETY: the PR intentionally excludes unreviewed auto-actions. Owner-reviewed workflow can be introduced with SHA-pinned actions, read-only token, no unsafe pull_request_target and no privileged execution of untrusted PR scripts. Keep release gates intact.

**Recommended disposition:** retain Draft; prioritize independent kernel/axiom replay, license confirmation, and a reviewed reproducible CI workflow.

## Cross-PR sequencing

The two PRs occupy separate package areas, but GitHub main has shared README and release-governance policies. Suggested sequence:
1. Produce exact head-SHA-specific CI evidence for PR #4 on an independent clean runner and complete rights review.
2. Reconcile PR #3 with current main, retaining the root README and adding an SSO package row. Run its existing CI again on the reconciled head, plus external interoperability checks.
3. For each PR, submit a fresh review explaining verified claims and explicitly unresolved work. Only after its private merge gates pass, mark Ready for Review and merge in a separately authorized action.
4. Keep publication distinct from private merge: perform root security/license scans, contributor-policy review, independent reproduction, public-facing claim audit, and owner approval for changing visibility.
5. Make PDP-001 a separate research PR, with a scope-limited pilot. No cross-branch dependency is accepted merely because it shares a name.

## Suggested commands for independent review (instructions, not executed receipts)

PR #3:
    git fetch origin main feat/sso-hash-sphere-pilot
    git switch feat/sso-hash-sphere-pilot
    git merge origin/main
    # Manually resolve README conflict, preserving main's multi-package policy.
    python -m pip install -e .
    python -m unittest discover -s tests -v

PR #4:
    git fetch origin main staging/ontology-lean53-source-v0.1
    git switch staging/ontology-lean53-source-v0.1
    cd ontology/emergent-attractor
    lake build
    lake env lean AxiomAudit.lean > axiom-audit.out
    # Run compatible independent leanchecker replay and review assumptions.

Commands require appropriate local credentials, reviewed toolchain provenance and isolation. Do not run untrusted code with privileged write tokens. Execute without silently bypassing protection; capture all receipts.

## Public reference plan

Before public launch: root README with crisp problem statements and verified proof/test badges; package-level status labels; reproducibility manifest; third-party attribution; stable tagged release; CITATION.cff; research roadmap with PDP-001, SSO and Emergent Attractor; issue templates for theorem/counterexample/test contributions; a CODEOWNERS/policy framework and protected main; public profile / GitHub Pages link to stable release. Keep permissions and private accounts out of public artifacts.

**Governance:** GitHub reports allow_auto_merge=false. Do not enable it globally without defining required checks/rulesets, trusted check producers, and risk tiers. In future, permit unattended merges only for authorized low-risk changes, never for axioms, verifier roots, privileges, dependency policy, private-to-public transitions or releases.

## Verification limitations

The connected GitHub integration was able to read repository metadata, PRs, patches, reviews, comments and workflow runs. It could not read combined commit statuses (403 forbidden); therefore claims about *all* required checks or effective branch-protection settings remain unverified. No independent Python execution or Lean kernel replay was performed as part of this review.

Related formal proposal: docs/research/PDP-001.md.
