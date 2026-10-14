# MATHLAB-001 — Wolfram Codex first bounded run

**Repo:** PRIVATE `pashamango/math`; coordination `MIN-57`. Read the actual owner-reviewed `docs/RELEASE_GATES.md` and Draft PR [#6](https://github.com/pashamango/math/pull/6), which only contains plans/templates. Work on one designated **Wolfram** host. No sudo/Work-tier upgrades, no model escalation, no changes to existing Governor, Ollama, Colima, Tailscale, SSH or global git configuration.

## Pass 0 — source and egress truth (read-only)

- Confirm local OS/host belongs to Wolfram (ARM64 macOS) and record sanitized receipt with timestamp, no username/Tailnet IPs in public issues.
- Check *an existing authorized private GitHub checkout* for `pashamango/math`, or resolve it with the already-authorized native GitHub access. A local branch absence or private raw URL 404 is **not** proof the repository is deleted. If no authorized authentication exists, stop `SOURCE_READ_BLOCKED` and request it; no browser cookie/token extraction, no repo substitution.
- Pin exact `main`, PR #4 head SHA, pinned `lean-toolchain` and `lake-manifest.json` using native GitHub authenticated object reads. Distinguish a source read from local pull/checkout and never claim clone succeeded without it.
- Run the bounded **public-only** HEAD probe in `docs/operations/TAILNET-OUTBOUND-EGRESS-GATE.md`. HTTPS cert validation mandatory; no secrets; no `tailscale up`, exit node or route changes. This test is independent of private GitHub authorization. If public egress fails, record `NET_EGRESS_READONLY_BLOCKED` and do not work around by reconfiguring the tailnet.

## Pass 1 — capacity and clean Lean verification

- Before any large download/build, inspect current `memory_pressure`, `vm_stat` pageouts over a short interval, swap, free disk, and processes categories. No swap purge or termination of unrelated apps.
- Use a **separate user-owned worktree** for exact PR #4 HEAD with no direct work on protected `main`. Keep `MATH` workload separate from `FORGE` LLM/PentAGI; don't start VM/model solely for formal kernel checking.
- Inspect toolchain availability and package dependency pinned revisions. Lean version requested by PR #4 is `leanprover/lean4:v4.34.1` and mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612`. Use project-local toolchain, no global upgrade. If unavailable, report download size/budget & wait for scoped approval instead of blind install.
- If safe and already authorized, run in `ontology/emergent-attractor`: `lake build`, `lake env lean AxiomAudit.lean`, and `lake env leanchecker Emergent` (check this exact CLI against installed Lean 4.34.1 first). Record full exit status, source commit/toolchain and every unexpected axiom. Historical archive logs do **not** satisfy independent replay.
- Audit claim boundaries: `ScottAxioms` assumptions, `leastTheory` monotone lfp and empty identity example, `SeededExample` nonempty positive seed, `Enrichment` contraction assumptions, `Models` countermodels. Don't assert that a coherent lfp is nonempty without seed; report proof gaps.
- Independently review upstream source license and adapted-code attribution in `Emergent/Modal.lean`; retain CC BY 4.0 credit and Apache allocation where appropriate. Fail closed if source rights cannot be confirmed.

## Pass 2 — single proof portfolio item

- Draft exactly one `PROOF-CARD-TEMPLATE.md` derivative with **verified** theorem statement, explicit axioms, one reproducible positive toy model and one negative countermodel. Do not claim any theorem proves empirical existence; if independent build fails, mark card DRAFT/BLOCKED, never KERNEL_VERIFIED.
- Output `MATHLAB_WOLFRAM_ACCEPTANCE_RECEIPT/v1`: host ID (non-sensitive), GitHub source read, exact PR #4 head SHA, memory/disk trends, safe HTTPS results, actual build/axiom/leanchecker commands and outcomes, outstanding rights, proof-card candidate, next bounded step.
- Git writes: restrict to authorized private review branch and only after passing expected tests; no auto merge, no auto PR #3/#4 promotion and no public visibility changes without G4 owner sign-off.

**Do not confuse the later Lean 4.35 `lake check` tool with the package's currently pinned Lean v4.34.1.** A future Lean upgrade may improve the verification pipeline, but it is a separate reviewed migration with its own mathlib version and regression evidence. Built-in `leanchecker` in v4.34.1 is sufficient for the initial scoped replay if actually installed and compatible.
