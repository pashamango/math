# Collatz Formalization — Module M0 & M1 (PhiWords)

**Status:** Verified Lean 4 Formalization  
**Release:** `v0.1.0`  
**License:** MIT / Apache-2.0  
**Verification Toolchain:** `leanprover/lean4:v4.34.1` (Lake v5.0.0)

---

## ⚠️ Epistemic Boundary & Honest Disclaimer

> [!IMPORTANT]
> **This project is NOT a proof of the Collatz conjecture (the $3x+1$ problem).**
>
> Module M1 (`Collatz.PhiWords`) formalizes the **static algebraic encoding of binary words in base $\phi$ and Fibonacci/Zeckendorf representations**. It establishes structural properties of words without consecutive ones (`no11`), the termination of canonical rewriting ($011 \to 100$), and representation invariants.
>
> **What IS proved in M1:**
> 1. Formal definition and decidability of binary words avoiding the substring $11$ (`no11`).
> 2. Termination of canonical reduction $011 \to 100$ via the strict Lyapunov measure $\text{countOnes}(w)$.
> 3. Normal form characterization: a word is irreducible under reduction if and only if it satisfies `no11`.
> 4. Exact invariance of Fibonacci valuation ($\text{evalFib}$) and algebraic golden-ratio ring valuation ($\text{evalPhi}$ in $\mathbb{Z}[\phi]$) under reduction steps.
> 5. Zeckendorf representation coverage for integers and Bergman representation coverage in $\mathbb{Z}[\phi]$.
>
> **What is NOT proved:**
> 1. No claims whatsoever are made regarding the global convergence of Collatz trajectories.
> 2. No claims of cycle prevention or absence of divergent trajectories under the map $T(n)$.
> 3. The golden ratio $\phi$ in M1 is strictly a positional base for encoding non-adjacent binary sequences; heuristic dynamical claims (such as contraction hypotheses or $3/4$ drift) belong to separate dynamical modules and are **not** asserted as theorems in M1.

---

## Architecture & Layout

This project mirrors the clean, hermetic layout of `ontology/emergent-attractor/` in `pashamango/math`:

```
collatz/
├── lean-toolchain               # Pinned Lean toolchain (leanprover/lean4:v4.34.1)
├── lakefile.lean                 # Dedicated Lake build specification (zero external dependencies)
├── Collatz.lean                  # Library root import
├── Collatz/
│   └── PhiWords.lean             # M1: Canonical words, reduction, Zeckendorf & Bergman theory
├── AxiomAudit.lean               # Exhaustive #print axioms audit for all 64 theorems
├── README.md                     # Epistemic boundary, claims, and build instructions
├── docs/
│   └── CLAIMS.md                 # Theorem Map and formal claim inventory
├── scripts/
│   ├── check_audit.py            # Automated audit gate & axiom verification
│   └── test_audit_gate.py        # Fault injection test suite verifying audit gate integrity
└── verification/
    └── axiom-audit.log           # Full Lean axiom dependency transcript
```

---

## Verification & Clean-Room Replay

The project requires only standard `elan` / Lean 4. It does not require downloading external libraries (such as `mathlib`), guaranteeing instant builds without network dependency:

```bash
# 1. Build from scratch
cd collatz
lake build

# 2. Verify axiom audit across all 64 theorems
lake env lean AxiomAudit.lean > verification/axiom-audit.log
python3 scripts/check_audit.py

# 3. Run fault-injection verification of the audit gate
python3 scripts/test_audit_gate.py
```

### Audit Gate Output
```
{'theorems': 64, 'without_axioms': 49, 'with_foundations': 15}
```
- **Total Theorems Declared & Audited:** 64
- **Sorry / Admit Count:** 0
- **Custom / Unsafe Axioms:** 0
- **Purely Constructive (Zero Axioms):** 49
- **Standard Foundations Only (`propext`, `Quot.sound`, `Classical.choice`):** 15
