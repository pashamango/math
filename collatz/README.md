# Collatz Formalization — Modules M0, M1 & M2 (PhiWords & ReverseTree)

**Status:** Verified Lean 4 Formalization  
**Release:** `v0.1.0`  
**License:** MIT / Apache-2.0  
**Verification Toolchain:** `leanprover/lean4:v4.34.1` (Lake v5.0.0)

---

## ⚠️ Epistemic Boundary & Honest Disclaimer

> [!IMPORTANT]
> **This project is NOT a proof of the Collatz conjecture (the $3x+1$ problem).**
>
> Module M1 (`Collatz.PhiWords`) formalizes the **static algebraic encoding of binary words in base $\phi$ and Fibonacci/Zeckendorf representations**.
> Module M2 (`Collatz.ReverseTree`) formalizes the **reverse Collatz tree**, the reachability predicate from 1, the structural avoidance of consecutive $G_1$ steps, and the precise mod-6 realizability characterization.
>
> **What IS proved in M1 & M2:**
> 1. **Canonical Words without 11 (`no11`):** Formal definition, decidability, and algebraic properties of binary words avoiding adjacent ones.
> 2. **Canonical Reduction ($011 \to 100$):** Termination of internal rewriting via strict descent of the Lyapunov bit-1 measure $\text{countOnes}(w)$, with exact preservation of length, Fibonacci value ($\text{evalFib}$), and ring valuation in $\mathbb{Z}[\phi]$ ($\text{evalPhi}$).
> 3. **Normal Form Characterization:** A word is irreducible under reduction if and only if it satisfies `no11`.
> 4. **Verified Representations:** Canonical Fibonacci representations for integers $0 \dots 8$ and Bergman base-$\phi$ representations for $\phi^0 \dots \phi^4$.
> 5. **Reverse Operators & Mod-6 Characterization:** Operators $G_0(n) = 2n$ and $G_1(n) = (n-1)/3$ with exact condition $n \equiv 4 \pmod 6$ (`isG1Valid`).
> 6. **ReachableFromOne Predicate:** Inductive definition from root 1 via valid $G_0, G_1$ transitions, with proven soundness of path execution (`evalPath_sound`).
> 7. **Structural Path Lemma (`validPath_yields_no11`):** Every valid reverse path structurally avoids consecutive $G_1$ steps, mapping injectively into `no11` words.
> 8. **Realizability Characterization (Strict Subset, Non-Surjectivity):** Reverse tree paths form a strict subset of `no11` words (`no11_not_all_realizable`). An unconditioned bijection between reverse paths and `no11` words is false; explicit counterexample `[true]` satisfies `no11` but cannot be realized from 1 because $1 \not\equiv 4 \pmod 6$.
> 9. **Conditional Hypothesis:** `CollatzConjecture` is formulated strictly as an explicit hypothesis variable (`Prop := ∀ n > 0, ReachableFromOne n`). All reachability theorems conditioned on it are strictly conditional, with zero claims of unconditional proof.
>
> **What is NOT proved:**
> 1. The Collatz conjecture remains strictly unproven. No claims are made regarding infinite trajectories or global convergence of arbitrary integers to 1.
> 2. Reverse paths do NOT cover all `no11` words; mod-6 congruence restrictions prevent surjectivity.
> 3. No universal Zeckendorf existence theorem for all $\mathbb{N}$ is asserted beyond the proven finite instances.
> 4. The golden ratio $\phi$ in M1 is strictly a positional base for encoding non-adjacent binary sequences; heuristic dynamical drift claims (e.g. $3/4$) belong to separate dynamical modules and are not asserted as theorems here.

---

## Architecture & Layout

This project mirrors the clean, hermetic layout of `ontology/emergent-attractor/` in `pashamango/math`:

```
collatz/
├── lean-toolchain               # Pinned Lean toolchain (leanprover/lean4:v4.34.1)
├── lakefile.lean                 # Dedicated Lake build specification (zero external dependencies)
├── Collatz.lean                  # Library root import (PhiWords + ReverseTree)
├── Collatz/
│   ├── PhiWords.lean             # M1: Canonical words, reduction, Zeckendorf & Bergman theory
│   └── ReverseTree.lean          # M2: Reverse tree, ReachableFromOne, no11 lemma, mod-6 characterization
├── AxiomAudit.lean               # Exhaustive #print axioms audit for all 81 theorems
├── README.md                     # Epistemic boundary, claims, and build instructions
├── docs/
│   └── CLAIMS.md                 # Theorem Map and formal claim inventory (81 theorems)
├── scripts/
│   ├── check_audit.py            # Automated audit gate & axiom verification
│   └── test_audit_gate.py        # Fault injection test suite verifying audit gate integrity
└── verification/
    └── axiom-audit.log           # Full Lean axiom dependency transcript
```

---

## Verification & Clean-Room Replay

The project requires only standard `elan` / Lean 4 (`v4.34.1`). It does not require downloading external libraries (such as `mathlib`), guaranteeing instant builds without network dependency:

```bash
# 1. Build from scratch
cd collatz
lake build

# 2. Verify axiom audit across all 81 theorems
lake env lean AxiomAudit.lean > verification/axiom-audit.log
python3 scripts/check_audit.py

# 3. Run fault-injection verification of the audit gate
python3 scripts/test_audit_gate.py
```

### Audit Gate Output
```
{'theorems': 81, 'without_axioms': 56, 'with_foundations': 25}
```
- **Total Theorems Declared & Audited:** 81
- **Sorry / Admit Count:** 0
- **Custom / Unsafe Axioms:** 0
- **Purely Constructive (Zero Axioms):** 56
- **Standard Core Foundations Only (`propext`, `Quot.sound`, `Classical.choice`):** 25
