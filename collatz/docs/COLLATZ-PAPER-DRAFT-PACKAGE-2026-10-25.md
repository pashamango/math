# Scientific Preprint Package: Modular Invariants and Structural Constraints of the Collatz Reverse Tree in Lean 4
**Title:** Modular Invariants, Canonical Fibonacci Encodings, and Structural Constraints of the Backward Collatz Tree in Lean 4  
**Author:** Pavel Cherkashin (`pavel@mindrock.capital`)  
**Affiliation:** Mindrock Capital / ChitAkasha Research  
**Formal Implementation:** `pashamango/math` (`collatz`, targeting release tag `collatz-v0.1.0`)  
**Date:** 2026-10-25  
**Version:** Draft Package v0.1.0 (Pre-Release Architecture Edition)  
**Status:** 🚨 **ON HOLD — PENDING PROJECT COLLATZ v0.1.0 RELEASE & PRINCIPAL TAP** 🚨  
*Protocol Gate: Strictly blocked from archival or outreach submission until: (1) Collatz v0.1.0 release is tagged and verified in Project Collatz (MIN-219, MIN-220, MIN-221); (2) Principal Pavel Cherkashin grants explicit single-tap approval for preprint distribution.*

---

## 1. Abstract (Draft for Archival / Submission)

We present a modular, machine-checked formalization in Lean 4 investigating the structural and dynamical properties of the backward Collatz ($3x+1$) predecessor tree and its interplay with combinatorics on words. Rather than attempting an unconditioned proof of the Collatz conjecture, we construct a verified algebraic and graph-theoretic framework consisting of 81 kernel-audited theorems across two foundational modules (`Collatz.PhiWords` and `Collatz.ReverseTree`) with an accompanying certified path verification engine (`Collatz.CacheCheck`). 

In `PhiWords` (Module M1, 64 theorems), we formalize the language of binary words avoiding adjacent ones (`no11`, corresponding to Zeckendorf representations and base-$\phi$ Bergman expansions). We mechanize the rewrite relation $011 \to 100$ and prove that every rewrite strictly decreases the bit-1 Lyapunov function ($\text{countOnes}(w)$) while strictly preserving both positional Fibonacci evaluation ($\text{evalFib}$) and ring valuation in $\mathbb{Z}[\phi]$ ($\text{evalPhi}$), guaranteeing strong normalization and reachability of unique canonical forms.

In `ReverseTree` (Module M2, 17 theorems), we formalize the backward Collatz transition system $G = G_0 \cup G_1$, where $G_0(n) = 2n$ and $G_1(n) = (n-1)/3$. We formally prove the mod-6 realizability obstruction: $G_1(n)$ is integral and positive if and only if $n \equiv 4 \pmod 6$, which implies that consecutive odd inverse steps are strictly impossible ($G_1(n) \not\equiv 4 \pmod 6$). This yields our central structural lemma (`validPath_yields_no11`): every valid backward trajectory from root 1 injectively maps to a binary word satisfying `no11`. Crucially, we prove that the reverse paths form a strict subset of `no11` words (`no11_not_all_realizable`), refuting any unconditioned bijection between Collatz trajectories and Zeckendorf words. Finally, we formalize the Collatz conjecture as an explicit conditional hypothesis variable (`CollatzConjecture`), establishing that the completeness of the reverse tree on all $\mathbb{N}^+$ is mathematically equivalent to the conjecture itself (`conditional_reachable`).

The entire development compiles under Lean `v4.34.1` with zero unproven assertions (`sorry`), zero custom axioms, 56 purely constructive theorems, and 25 theorems depending only on Lean's core foundational axioms (`propext`, `Quot.sound`, `Classical.choice`), verified via automated AST axiom inspection and kernel replay.

---

## 2. Article Architecture & Table of Contents

```
1. Introduction & Epistemic Boundary
   1.1 The 3x+1 Problem: Background and Methodological Pitfalls
   1.2 The Mindrock Formalization Program: Constructive Verification vs. Unsound Claims
   1.3 Modular Decomposition: PhiWords (M1), ReverseTree (M2), and CacheCheck (M3)
   1.4 Core Non-Claims: What This Paper Does and Does Not Prove

2. Combinatorics on Words & Canonical Fibonacci Reductions (Module M1)
   2.1 Binary Languages Avoiding 11 (the `no11` Predicate)
   2.2 The Rewrite Relation 011 -> 100
   2.3 Lyapunov Bit-1 Count Descent & Strong Normalization
   2.4 Positional Valuation Invariance: Fibonacci (evalFib) & Ring Z[phi] (evalPhi)
   2.5 Normal Form Characterization & Irreducibility
   2.6 Verified Representations: Finite Zeckendorf and Bergman Bases

3. The Backward Collatz Tree & Mod-6 Dynamics (Module M2)
   3.1 The Backward Predecessor System: Operators G0 and G1
   3.2 Mod-6 Arithmetic Obstructions: Exact Criterion for G1 Realizability (n = 4 mod 6)
   3.3 Impossibility of Consecutive G1 Transitions: The Core Lemma
   3.4 Inductive Reachability from 1 (ReachableFromOne) and Execution Soundness
   3.5 The Structural Injection: Valid Reverse Paths to no11 Words (validPath_yields_no11)
   3.6 Realizability Characterization: Strict Non-Surjectivity and Counterexamples
   3.7 Formalization of the Global Conjecture as a Conditional Hypothesis Variable

4. Certified Linear Trajectory Verification & Cache Architecture (Module M3)
   4.1 Path Certificates and Trajectory Bound Checking (S * tau)
   4.2 Backward Memoization and Reachability Pruning
   4.3 Lean 4 Kernel Execution vs. Certified Native Verifiers

5. Rigorous Epistemic Demarcation & Non-Claims
   5.1 Equivalence of Reverse Tree Completeness and the Collatz Conjecture
   5.2 Obstructions to Surjectivity: Why Zeckendorf Completeness Does Not Imply Collatz
   5.3 Golden Ratio Drift vs. Formal Base-phi Positional Encoding
   5.4 Whitelist Axiom Discipline: Avoiding Inconsistency and Hidden Hypotheses

6. Toolchain, Reproducibility & Audit Provenance
   6.1 Lean 4 Environment & Zero-Mathlib Self-Contained Build
   6.2 Automated Axiom Inspection (AxiomAudit.lean) & Fault Injection
   6.3 Verification Receipts and Cryptographic Manifests

7. Discussion, Related Work & Scholarly Context
   7.1 Connection to Lagarias' 2-Adic and Algorithmic Surveys
   7.2 Connection to Shallit & Allouche's Combinatorics on Words and Walnut Automata
   7.3 Connection to Barina's High-Performance Computational Verification up to 2^68
   7.4 Future Directions: Formalizing Certified Pruning and Tree Density Bounds
```

---

## 3. Exact Lean Declaration & Proof Mapping

All declarations listed below are verified in Lean `v4.34.1` with zero `sorry` and audited via `AxiomAudit.lean`:

### Module M1: `Collatz.PhiWords` (64 Theorems in Codebase)
| Mathematical Result | Formal Lean Declaration | Description | Axiom Footprint |
|---|---|---|---|
| **Definitional Recurrence** | `fib_add_two` | `theorem fib_add_two (n : Nat) : fib (n + 2) = fib (n + 1) + fib n` | `[]` (Constructive) |
| **`no11` Nil** | `no11_nil` | `theorem no11_nil : no11 []` | `[]` |
| **`no11` Single Bit** | `no11_one` | `theorem no11_one : no11 [true]` | `[]` |
| **`no11` Tail Preservation** | `no11_of_cons` | `theorem no11_of_cons (b : Bool) (w : Word) (h : no11 (b :: w)) : no11 w` | `[]` |
| **`no11` Negative Violation** | `not_no11_true_true` | `theorem not_no11_true_true (rest : Word) : ¬ no11 (true :: true :: rest)` | `[]` |
| **`no11` Prefix Violation** | `not_no11_false_true_true` | `theorem not_no11_false_true_true (rest : Word) : ¬ no11 (false :: true :: true :: rest)` | `[]` |
| **Bit-1 Additivity** | `countOnes_append` | `theorem countOnes_append (l1 l2 : Word) : countOnes (l1 ++ l2) = countOnes l1 + countOnes l2` | `[propext, Quot.sound]` |
| **Lyapunov Exact Descent** | `step011_countOnes` | `theorem step011_countOnes {w w' : Word} (h : Step011 w w') : countOnes w = countOnes w' + 1` | `[propext, Quot.sound]` |
| **Strict Lyapunov Descent** | `step011_countOnes_lt` | `theorem step011_countOnes_lt {w w' : Word} (h : Step011 w w') : countOnes w' < countOnes w` | `[propext, Quot.sound]` |
| **Length Invariance** | `step011_length` | `theorem step011_length {w w' : Word} (h : Step011 w w') : w.length = w'.length` | `[propext]` |
| **Fibonacci Equivalence** | `evalLSBAt_011_eq_100` | `theorem evalLSBAt_011_eq_100 (k : Nat) : evalLSBAt k [true, true, false] = evalLSBAt k [false, false, true]` | `[propext, Quot.sound]` |
| **Direct Step Valuation** | `evalFib_reduction_equiv` | `theorem evalFib_reduction_equiv : evalFib [false, true, true] = evalFib [true, false, false]` | `[]` |
| **Valuation Invariance** | `step011_evalFib` | `theorem step011_evalFib {w w' : Word} (h : Step011 w w') : evalFib w = evalFib w'` | `[propext, Quot.sound]` |
| **Ring Valuation Invariance** | `step011_evalPhi` | `theorem step011_evalPhi {w w' : Word} (h : Step011 w w') : evalPhi w = evalPhi w'` | `[propext, Quot.sound]` |
| **Normal Form Equivalent** | `irreducible_iff_no11` | `theorem irreducible_iff_no11 (w : Word) : (∀ w', ¬ Step011 w w') ↔ no11 w` | `[propext, Classical.choice, Quot.sound]` |
| **Zeckendorf Instances** | `zeckendorf_zero` ... `zeckendorf_eight` | `theorem zeckendorf_n : ∃ w, no11 w ∧ evalFib w = n` | `[]` (Constructive) |
| **Bergman Base Instances** | `bergman_phi0` ... `bergman_phi4` | `theorem bergman_phin : ∃ w, no11 w ∧ evalPhi w = ...` | `[]` (Constructive) |

### Module M2: `Collatz.ReverseTree` (17 Theorems in Codebase)
| Mathematical Result | Formal Lean Declaration | Description | Axiom Footprint |
|---|---|---|---|
| **$G_0$ Positivity** | `g0_pos` | `theorem g0_pos {n : Nat} (hn : n > 0) : g0 n > 0` | `[propext, Quot.sound]` |
| **$G_1$ Positivity** | `g1_pos` | `theorem g1_pos {n : Nat} (hmod : isG1Valid n) : g1 n > 0` | `[propext, Quot.sound]` |
| **$G_1$ Oddness** | `g1_odd` | `theorem g1_odd {n : Nat} (hmod : isG1Valid n) : g1 n % 2 = 1` | `[propext, Quot.sound]` |
| **Mod-6 Non-Congruence** | `g1_mod6_ne_4` | `theorem g1_mod6_ne_4 {n : Nat} (hmod : isG1Valid n) : ¬ isG1Valid (g1 n)` | `[propext, Quot.sound]` |
| **Consecutive $G_1$ Impossible** | `g1_consecutive_invalid` | `theorem g1_consecutive_invalid {n : Nat} (hmod : isG1Valid n) : ¬ isG1Valid (g1 n)` | `[propext, Quot.sound]` |
| **Reachable Positivity** | `reachable_pos` | `theorem reachable_pos {n : Nat} (h : ReachableFromOne n) : n > 0` | `[propext, Quot.sound]` |
| **Path Soundness** | `evalPath_sound` | `theorem evalPath_sound {p : Path} {n : Nat} (h : evalPath 1 p = some n) : ReachableFromOne n` | `[]` (Constructive) |
| **Path-Word Injection** | `pathToWord_injective` | `theorem pathToWord_injective : ∀ p1 p2, pathToWord p1 = pathToWord p2 → p1 = p2` | `[]` (Constructive) |
| **Structural Path Lemma** | `validPath_yields_no11` | `theorem validPath_yields_no11 {curr : Nat} {p : Path} (h : validPath curr p) : PhiWords.no11 (pathToWord p)` | `[propext, Quot.sound]` |
| **Non-Surjectivity / Countermodel** | `no11_not_all_realizable` | `theorem no11_not_all_realizable : ∃ w, PhiWords.no11 w ∧ ¬ ∃ p, pathToWord p = w ∧ validPath 1 p` | `[]` (Constructive) |
| **Conditional Global Reachability** | `conditional_reachable` | `theorem conditional_reachable (hconjecture : CollatzConjecture) (n : Nat) (hn : n > 0) : ReachableFromOne n` | `[]` (Constructive) |

---

## 4. Systematic Non-Claims & Epistemic Boundaries

To maintain absolute scientific integrity and avoid the reputational hazards common in amateur $3x+1$ literature, this work enforces four inviolable epistemic boundaries:

1. **No Proof of the Collatz Conjecture:**
   - The Collatz conjecture remains strictly unproven in this work.
   - The global assertion $\forall n > 0, \text{ReachableFromOne}(n)$ is formalized solely as an explicit hypothesis parameter (`CollatzConjecture : Prop`).
   - Completeness of the reverse predecessor tree is mathematically equivalent to the conjecture itself.
2. **Reverse Paths Form a Strict Subset of `no11` Words:**
   - While every valid reverse path yields a word satisfying `no11` (`validPath_yields_no11`), the converse is false (`no11_not_all_realizable`).
   - The word `[true]` satisfies `no11`, but cannot be evaluated as an initial $G_1$ step from root 1 because $1 \not\equiv 4 \pmod 6$.
   - Any claim that the completeness of Zeckendorf representations implies the completeness of the Collatz tree is false and rejected.
3. **$\phi$ is a Positional Base, Not a Heuristic Trajectory Drift:**
   - The golden ratio $\phi$ in Module M1 is formalized strictly as the algebraic radix of the ring $\mathbb{Z}[\phi]$ and Fibonacci recurrence.
   - We do not assert heuristic or probabilistic geometric decay rates (such as $(3/4)^k$) as proved theorems.
4. **Clean-Room Axiom Whitelist:**
   - Zero custom axioms. Zero `sorry`. Zero `unsafe`.
   - All proofs strictly adhere to Lean 4's foundational type theory.

---

## 5. Reproducibility & Audit Provenance

### Verification Commands (Self-Contained Clean Room)
```bash
# Clone the repository
git clone https://github.com/pashamango/math.git
cd math/collatz

# Build with Lean v4.34.1 / Lake
lake build

# Execute full axiom audit for all 81 theorems
lake env lean AxiomAudit.lean > verification/axiom-audit.log

# Verify axiom dependencies with automated gate checker
python3 scripts/check_audit.py

# Run fault injection test suite
python3 scripts/test_audit_gate.py
```

### Audit Gate Result
```json
{
  "total_theorems": 81,
  "without_axioms": 56,
  "with_foundations": 25,
  "sorry_count": 0,
  "custom_axioms": 0,
  "foundational_axioms": [
    "Classical.choice",
    "Quot.sound",
    "propext"
  ]
}
```

---

## 6. Publication, Archival & Preprint Strategy

* **Target Archival Repositories:**
  1. **arXiv:** Primary submission to `math.NT` (Number Theory), with cross-lists to `math.DS` (Dynamical Systems) and `cs.LO` (Logic in Computer Science).
  2. **HAL Science:** Open archive mirror for European mathematical community indexing.
  3. **Zenodo:** Permanent DOI issuance with archival tarball of the Lean 4 source code and verification logs.
  4. **GitHub:** Public open-source repository at `https://github.com/pashamango/math` under `collatz/`.
* **Execution Gate:**
  - **🚨 ON HOLD:** No paper will be uploaded, preprint deposited, or DOI generated until Collatz v0.1.0 is released in Project Collatz and Principal Pavel Cherkashin provides explicit written tap.
