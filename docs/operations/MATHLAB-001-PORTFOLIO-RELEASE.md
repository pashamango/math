# MATHLAB-001 — Proof-to-Portfolio / Wolfram Math Laboratory

**Status: PRIVATE DESIGN / REVIEW-ONLY; not a release.** Owner economic priority (2026-10-14): turn `pashamango/math` into a reproducible formal-mathematics portfolio that helps win paid engineering/research/verification work, rather than optimize for investment fundraising. Wolfram is the local lab; Darwin remains independent provenance/ledger host; Grafana Cloud is a separately gated *sanitized external telemetry* candidate. The host-local Wolfram Codex must independently verify its own source access and resources; the ChatGPT GitHub connector is not an SSH session.

## Verified repository baseline (authenticated GitHub read, 2026-10-14)

- This **private** repository `main` at `1a297875049b18603d91450bbb0aadac77dc7350` has multi-package README, Apache 2.0 LICENSE, NOTICE, CONTRIBUTING, SECURITY, and `docs/RELEASE_GATES.md`. Do not replace the main README or change repository visibility.
- PR [#4](https://github.com/pashamango/math/pull/4): **OPEN DRAFT** Emergent Attractor formal Lean 4 library; 7 substantive modules / 53 literal `theorem` declarations, Lean `v4.34.1`, mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612`; `AxiomAudit.lean` prints the axioms of all 53. Static imported source is not independent clean-room kernel replay. File-by-file adapted upstream license/attribution review still open (CC BY 4.0 material attributed to Christoph Benzmüller).
- PR [#3](https://github.com/pashamango/math/pull/3): **OPEN DRAFT** SSO Python hash-sphere pilot. GitHub reports mergeable=false; root README conflicts with newer main; previous SSO CI succeeded but not a cross-platform scientific verification. Merge conflict should preserve main's epistemic/rights policy.
- PR [#5](https://github.com/pashamango/math/pull/5): **OPEN DRAFT** PDP-001 / proof-carrying decomposition research plan and merge-readiness review. Conditional sound composition is a research target, not yet a verified universal task decomposition theorem.
- The repo is **private by default**, and `docs/RELEASE_GATES.md` governs G0 provenance/rights, G1 layout, G2 independent execution/kernel/axioms, G3 review, G4 explicit release/visibility approval.

## Single primary deliverable: ontology proof card + runnable exemplar

A potential user must be able to understand the claim and independently verify it without trusting agent narration:
1. **Exact theorem** as source path, identifier, signature, quantified assumptions; copy no more than necessary into the proof card.
2. **What the theorem does NOT prove:** a formal conditional statement does not demonstrate physical existence, universal AI emergence, metaphysical necessity outside the encoded axioms, or solve unrelated conjectures.
3. **Model and countermodel** demonstrating boundaries. Distinguish `ScottAxioms` / modal conclusions from the independent recursive fixed-point and contraction frameworks. The `leastTheory` of identity is empty; coherence alone does not entail a nonempty fixed point. A new nonemptiness lemma (if desired) must state and discharge an explicit seed or witness assumption, with a negative test that rules out the unsound unconditional formulation.
4. **Pinned toolchain and independent verification receipt:** `lake build`, `lake env lean AxiomAudit.lean`, compatible `leanchecker` replay, all 53 targets with inspected axiom output. Check failed/unknown commands and missing targets; do not use `sorry`, `admit`, unexplained `axiom`, or `native_decide` to conceal the trusted-kernel boundary. Audit `opaque` definitions and custom axioms separately.
5. **Rights:** independently confirm original adapted upstream file license, correct attribution/changes in `NOTICE.md` and package `docs/SOURCES.md`. Root Apache-2.0 cannot silently override adapted CC BY 4.0.
6. **Meaningful demo** for a verifier or engineer: (a) an executable positive example with its assumptions, (b) a counterexample for a stronger false statement, (c) a reproducible, bounded one-command runner; trust boundary and limitations visible in first screenful.
7. Proof-card template at `docs/portfolio/PROOF-CARD-TEMPLATE.md`. A card is `DRAFT` until source and independent check receipts are attached.

## Wolfram execution order: avoid collateral damage

**M0 SOURCE_READ**: verify the intended Wolfram terminal, current disk/RAM/swap trend, toolchain cache and existing *authenticated* repo clone. A 404 to unauthenticated raw GitHub is not proof that a private repository does not exist. Use existing approved credential mechanism, never dump tokens or login config. Do not fetch by unverified mirror. Local output `MATHLAB_SOURCE_READ_RECEIPT/v1`, state `SOURCE_READ | READ_BLOCKED`.

**M1 CLEAN ROOM**: worktree or separate workspace for exact PR #4 SHA, read-only dependency audit, finite download budget and disk capacity assessment before `lake exe cache get` or a large mathlib build. Standard user, no sudo; stop if memory pressure grows. Emit `LEAN_KERNEL_REPLAY_RECEIPT/v1`: source SHA, Lean/Lake mathlib/toolchain SHA/version, actual invocation/exit codes, all 53 axiom inspections and exceptions. Do **not** claim locally verified from historical package logs or a green GitHub merge flag.

**M2 LOGICAL STRENGTH**: inspect full definitions and theorem signatures, identify assumptions that trivially enforce desired result, produce falsifiable countermodels and necessary conditions (especially nonempty seeded `leastTheory`). New Lean changes only on PR #4 review branch, with actual build & replay before scientific claim. No edits to protected main.

**M3 PACKAGE / API**: own Lean package, versioned examples, `README` using a novice-friendly 5-minute quick start, CITATION.cff draft, reproducibility manifest and SHA256, third-party license corrections. Avoid a package-wide proof badge until the exact build head is independently checked. May update PR #4 only after review and scoped authority.

**M4 SSO RECONCILIATION**: fix PR #3 README conflict with main in review branch, rerun pinned Python 3.11 tests, external golden-vector interoperability and invalid-input tests. Keep claims about hash rarity separate from any novel cryptographic security, PoW acceleration or statistical discovery.

**M5 PDP-001**: turn PR #5 conditional composition proposal into one narrow Lean theorem with independently meaningful task semantics, explicit interfaces and capability non-amplification; output synthetic triage receipts. No automatic smart merge of untrusted proofs.

**M6 PUBLIC PORTFOLIO**: only after G0–G3 of a selected release, request **explicit owner approval** to merge and/or change private repo visibility under G4. Publish a stable tagged release with checksums. Then update `pashamango/pashamango` / GitHub Pages (tracked separately by MIN-39), linking **only public, reproducible artifacts**. The CTA should target scoped paid work: proof/CI audits, formal verification of AI-agent contracts, and reproducible computational research. Do not claim actual customer demand or guaranteed rates without evidence.

## Evidence and economic metric

Successful stage 1 = **one** independently verified Lean module and proof card + counterexample + reproducible commands. Stage 2 = integrated source-ready PR #4 (but NOT automatically merged). Stage 3 = one public owner-approved artifact. Stage 4 = portfolio inquiries and paid contract opportunities, tracked as actual conversions (not follower counts or GitHub stars). Prefer a readable 90-second portfolio walkthrough over broad undocumented claims.

Keep measured wall time, local RAM/pageouts, actual external Codex tokens avoided and false-DONE incidence for each local/math and FORGE comparison. No fake savings or claiming machine-proved metaphysics.

**Hard boundaries:** no sudo, no changes to Wolfram Governor/SSH/Tailscale/Ollama/Colima, no new cloud credential in code, no host-side public/private Git write without exact authorized target, no protected main merge, no automated repository visibility change, no developer token escalations or Work/model-tier changes. A failed source/read/build is `BLOCKED`, never `DONE`.
