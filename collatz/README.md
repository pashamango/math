# Collatz Formalization — Modules M1, M2, M3

**Status:** Verified Lean 4 Formalization  
**Release:** `v0.1.0`  
**License:** MIT / Apache-2.0  
**Verification Toolchain:** `leanprover/lean4:v4.34.1` (Lake v5.0.0)

---

## ⚠️ Epistemic Boundary & Honest Disclaimer

> [!IMPORTANT]
> **This project is NOT a proof of the Collatz conjecture (the $3x+1$ problem).**
>
> 1. **No Global Convergence Claim:** The global reachability of all positive integers from 1 (`CollatzConjecture : Prop := ∀ n > 0, ReachableFromOne n`) is formalized strictly as an **unproven conditional hypothesis variable**. No theorem in this repository claims to prove this hypothesis.
> 2. **τ is NOT a Divergence Detector:** The reciprocal coordinate $\tau = 1/S$ is defined **strictly and exclusively on certified, provably reachable nodes** (`CertifiedNode`). It does not detect divergence or non-divergence of unverified trajectories.
> 3. **Linear Path Checker is NOT a Result about P vs NP:** The $O(|w|)$ certificate verifier (`verifyPath`) is an elementary linear-time verification of an explicit certificate witness path, which is standard in algorithmics and bears no relation to the P vs NP problem.
> 4. **No Mixing of Rewriting with Cycles:** Canonical word rewriting ($011 \to 100$) in $\phi$-words (`Collatz.PhiWords`) formalizes base-$\phi$ positional normal forms and is **strictly separated** from Collatz orbital dynamics or cycles.

---

## Module Overview

The formalization is structured into three self-contained Lean 4 modules:

### 1. Module M1: `Collatz.PhiWords` (64 Theorems)
- Canonical binary words avoiding adjacent ones (`no11`).
- Termination of canonical reduction $011 \to 100$ via the strict Lyapunov measure $\text{countOnes}$.
- Normal form characterization: a word is irreducible under reduction if and only if it satisfies `no11`.
- Exact invariance of Fibonacci valuation ($\text{evalFib}$) and ring valuation in $\mathbb{Z}[\phi]$ ($\text{evalPhi}$).
- Zeckendorf and Bergman representation coverage properties.

### 2. Module M2: `Collatz.ReverseTree` (17 Theorems)
- Reverse Collatz operators $G_0(n) = 2n$ and $G_1(n) = (n - 1) / 3$ under explicit mod-6 condition ($n \equiv 4 \pmod 6$).
- Inductive reachability predicate `ReachableFromOne`.
- Structural path theorem: every valid reverse tree path avoids consecutive $G_1$ transitions, strictly yielding words without adjacent ones (`no11`).
- Injection of valid reverse paths into $\phi$-words.
- Characterization of realizability: proof that reverse paths form a **strict subset** of `no11` words (counterexample `[true]` is valid `no11` but unreachable from 1 due to $1 \not\equiv 4 \pmod 6$).
- Conditional hypothesis `CollatzConjecture`.

### 3. Module M3: `Collatz.CacheCheck` (20 Theorems)
- Rigorous definition of scale $S$ and reciprocal coordinate $\tau = 1/S$ **strictly on provably reachable nodes** (`CertifiedNode`).
- Monotonicity of $\tau$ as a direct lemma of $S$: decreasing scale $S$ corresponds monotonically to non-decreasing $\tau$ (and strictly increasing for $S(v) < S(u)$), with $\tau = 1$ achieved at the root attractor 1.
- Deterministic path checker (`verifyPath`) verifying reverse path certificates with transition cost upper bounded by $O(|w|)$ steps in the unit-cost model.
- Soundness theorem: any node accepted by `verifyPath` is provably reachable (`ReachableFromOne`).
- Cache consistency guard (`checkCache` / `isCacheEntryValid`): admits nodes into the certified cache only upon verification of linear path certificates.

---

## Architecture & Layout

```
collatz/
├── lean-toolchain               # Pinned Lean toolchain (leanprover/lean4:v4.34.1)
├── lakefile.lean                 # Dedicated Lake build specification (zero external dependencies)
├── Collatz.lean                  # Library root import
├── Collatz/
│   ├── PhiWords.lean             # M1: Canonical words, reduction, Zeckendorf & Bergman theory
│   ├── ReverseTree.lean          # M2: Reverse operators, reachability, no11 lemma, realizability
│   └── CacheCheck.lean           # M3: S, tau on certified nodes, monotonicity, linear path checker
├── AxiomAudit.lean               # Exhaustive #print axioms audit across all 101 theorems
├── README.md                     # Epistemic boundary, module architecture, and reproduction
├── docs/
│   └── CLAIMS.md                 # Complete Theorem Map and formal claim inventory
├── scripts/
│   ├── check_audit.py            # Automated audit gate & axiom verification
│   └── test_audit_gate.py        # Fault injection test suite verifying audit gate integrity
└── verification/
    └── axiom-audit.log           # Full Lean axiom dependency transcript
```

---

## Verification & Clean-Room Replay

The project requires only standard Lean 4 / Lake. It has zero external dependencies:

```bash
# 1. Clean build
cd collatz
lake build

# 2. Run automated axiom audit gate
python3 scripts/check_audit.py

# 3. Run fault-injection test suite
python3 scripts/test_audit_gate.py
```

### Audit Gate Results
```
{'theorems': 101, 'without_axioms': 71, 'with_foundations': 30}
PASS: rejected placeholder
PASS: rejected custom axiom
PASS: rejected unsafe
PASS: rejected unlisted theorem
PASS: rejected missing/duplicate rows, foreign axiom and sorryAx; clean baseline accepted
```

- **Total Audited Theorems:** 101
- **Sorry / Admit Count:** 0
- **Custom / Unsafe Axioms:** 0
- **Constructive (Zero Axioms):** 71
- **Standard Core Foundations Only (`propext`, `Quot.sound`):** 30
