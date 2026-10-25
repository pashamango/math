# Theorem Map — Modules M1 & M2: PhiWords and ReverseTree

This document catalogs all 81 formal theorems proven across `Collatz.PhiWords` and `Collatz.ReverseTree`.
All declarations have been audited via `AxiomAudit.lean` with zero `sorry`, zero `admit`, zero custom `axiom`, and zero `unsafe` constructs.

---

## 1. Module M1: `Collatz.PhiWords` (64 Theorems)

### Fibonacci Foundations & Basic Properties
| Theorem | Lean Declaration | Description | Axioms |
|---|---|---|---|
| `fib_add_two` | `theorem fib_add_two (n : Nat) : fib (n + 2) = fib (n + 1) + fib n` | Definitional recurrence of Fibonacci numbers | `[]` (None) |

### Canonical Words without Substring 11 (`no11`)
| Theorem | Lean Declaration | Description | Axioms |
|---|---|---|---|
| `no11_nil` | `theorem no11_nil : no11 []` | The empty word satisfies `no11` | `[]` |
| `no11_one` | `theorem no11_one : no11 [true]` | Single bit `1` satisfies `no11` | `[]` |
| `no11_two` | `theorem no11_two : no11 [true, false]` | Word `10` satisfies `no11` | `[]` |
| `no11_three` | `theorem no11_three : no11 [true, false, false]` | Word `100` satisfies `no11` | `[]` |
| `no11_four` | `theorem no11_four : no11 [true, false, true]` | Word `101` satisfies `no11` | `[]` |
| `no11_five` | `theorem no11_five : no11 [true, false, false, false]` | Word `1000` satisfies `no11` | `[]` |
| `no11_six` | `theorem no11_six : no11 [true, false, false, true]` | Word `1001` satisfies `no11` | `[]` |
| `no11_seven` | `theorem no11_seven : no11 [true, false, true, false]` | Word `1010` satisfies `no11` | `[]` |
| `no11_eight` | `theorem no11_eight : no11 [true, false, false, false, false]` | Word `10000` satisfies `no11` | `[]` |
| `no11_false_cons` | `theorem no11_false_cons (b2 : Bool) (rest : Word) : no11 (false :: b2 :: rest) ↔ no11 (b2 :: rest)` | Prepending `0` preserves `no11` | `[]` |
| `no11_true_false_cons` | `theorem no11_true_false_cons (rest : Word) : no11 (true :: false :: rest) ↔ no11 (false :: rest)` | Prepending `10` preserves `no11` | `[]` |
| `no11_of_cons` | `theorem no11_of_cons (b : Bool) (w : Word) (h : no11 (b :: w)) : no11 w` | Tail of a `no11` word satisfies `no11` | `[]` |
| `not_no11_true_true` | `theorem not_no11_true_true (rest : Word) : ¬ no11 (true :: true :: rest)` | `11...` explicitly violates `no11` | `[]` |
| `not_no11_false_true_true` | `theorem not_no11_false_true_true (rest : Word) : ¬ no11 (false :: true :: true :: rest)` | `011...` explicitly violates `no11` | `[]` |

### Canonical Reduction `011 -> 100` & Termination
| Theorem | Lean Declaration | Description | Axioms |
|---|---|---|---|
| `countOnes_append` | `theorem countOnes_append (l1 l2 : Word) : countOnes (l1 ++ l2) = countOnes l1 + countOnes l2` | Additivity of bit-1 count under concatenation | `[propext, Quot.sound]` |
| `step011_countOnes` | `theorem step011_countOnes {w w' : Word} (h : Step011 w w') : countOnes w = countOnes w' + 1` | Every rewrite $011 \to 100$ decreases bit-1 count by exactly 1 | `[propext, Quot.sound]` |
| `step011_countOnes_lt` | `theorem step011_countOnes_lt {w w' : Word} (h : Step011 w w') : countOnes w' < countOnes w` | Strict descent of Lyapunov function (guarantees termination) | `[propext, Quot.sound]` |
| `step011_length` | `theorem step011_length {w w' : Word} (h : Step011 w w') : w.length = w'.length` | Positional length is invariant under internal $011 \to 100$ | `[propext]` |

### Fibonacci / Positional Evaluation Invariance
| Theorem | Lean Declaration | Description | Axioms |
|---|---|---|---|
| `evalLSBAt_append` | `theorem evalLSBAt_append (k : Nat) (l1 l2 : List Bool) : evalLSBAt k (l1 ++ l2) = evalLSBAt k l1 + evalLSBAt (k + l1.length) l2` | Positional evaluation shift additivity | `[propext, Quot.sound]` |
| `evalLSBAt_011_eq_100` | `theorem evalLSBAt_011_eq_100 (k : Nat) : evalLSBAt k [true, true, false] = evalLSBAt k [false, false, true]` | Equivalence of $011$ and $100$ at any positional offset $k$ | `[propext, Quot.sound]` |
| `evalFib_nil` | `theorem evalFib_nil : evalFib [] = 0` | Value of empty word is 0 | `[]` |
| `evalFib_one` | `theorem evalFib_one : evalFib [true] = 1` | Value of `1` is $F_2 = 1$ | `[]` |
| `evalFib_two` | `theorem evalFib_two : evalFib [true, false] = 2` | Value of `10` is $F_3 = 2$ | `[]` |
| `evalFib_three` | `theorem evalFib_three : evalFib [true, false, false] = 3` | Value of `100` is $F_4 = 3$ | `[]` |
| `evalFib_four` | `theorem evalFib_four : evalFib [true, false, true] = 4` | Value of `101` is $F_4 + F_2 = 4$ | `[]` |
| `evalFib_five` | `theorem evalFib_five : evalFib [true, false, false, false] = 5` | Value of `1000` is $F_5 = 5$ | `[]` |
| `evalFib_six` | `theorem evalFib_six : evalFib [true, false, false, true] = 6` | Value of `1001` is $F_5 + F_2 = 6$ | `[]` |
| `evalFib_seven` | `theorem evalFib_seven : evalFib [true, false, true, false] = 7` | Value of `1010` is $F_5 + F_3 = 7$ | `[]` |
| `evalFib_eight` | `theorem evalFib_eight : evalFib [true, false, false, false, false] = 8` | Value of `10000` is $F_6 = 8$ | `[]` |
| `evalFib_reduction_equiv` | `theorem evalFib_reduction_equiv : evalFib [false, true, true] = evalFib [true, false, false]` | Direct evaluation equality: $\text{evalFib}(011) = \text{evalFib}(100) = 3$ | `[]` |
| `step011_evalFib` | `theorem step011_evalFib {w w' : Word} (h : Step011 w w') : evalFib w = evalFib w'` | Full invariance of Fibonacci valuation under any reduction step | `[propext, Quot.sound]` |

### Ring Evaluation Invariance in $\mathbb{Z}[\phi]$ (Bergman Base)
| Theorem | Lean Declaration | Description | Axioms |
|---|---|---|---|
| `phiPower_add_two` | `theorem phiPower_add_two (k : Nat) : phiPower (k + 2) = addPhi (phiPower (k + 1)) (phiPower k)` | Golden ratio power recurrence $\phi^{k+2} = \phi^{k+1} + \phi^k$ | `[]` |
| `addPhi_zero` | `theorem addPhi_zero (p : Nat × Nat) : addPhi p (0, 0) = p` | Identity of addition in $\mathbb{N}[\phi]$ | `[propext]` |
| `addPhi_assoc` | `theorem addPhi_assoc (p1 p2 p3 : Nat × Nat) : addPhi (addPhi p1 p2) p3 = addPhi p1 (addPhi p2 p3)` | Associativity in $\mathbb{N}[\phi]$ | `[propext, Classical.choice, Quot.sound]` |
| `addPhi_comm` | `theorem addPhi_comm (p1 p2 : Nat × Nat) : addPhi p1 p2 = addPhi p2 p1` | Commutativity in $\mathbb{N}[\phi]$ | `[propext, Classical.choice, Quot.sound]` |
| `evalPhiLSBAt_append` | `theorem evalPhiLSBAt_append (k : Nat) (l1 l2 : List Bool) : evalPhiLSBAt k (l1 ++ l2) = addPhi (evalPhiLSBAt k l1) (evalPhiLSBAt (k + l1.length) l2)` | Additivity of $\phi$-evaluation under concatenation | `[propext, Classical.choice, Quot.sound]` |
| `evalPhiLSBAt_011_eq_100` | `theorem evalPhiLSBAt_011_eq_100 (k : Nat) : evalPhiLSBAt k [true, true, false] = evalPhiLSBAt k [false, false, true]` | Equivalence of $011_\phi$ and $100_\phi$ at any offset | `[propext, Classical.choice, Quot.sound]` |
| `evalPhi_nil` | `theorem evalPhi_nil : evalPhi [] = (0, 0)` | Base evaluation of empty word in $\mathbb{Z}[\phi]$ | `[]` |
| `evalPhi_phi0` | `theorem evalPhi_phi0 : evalPhi [true] = (1, 0)` | $\phi^0 = 1 = 1 + 0\phi$ | `[]` |
| `evalPhi_phi1` | `theorem evalPhi_phi1 : evalPhi [true, false] = (0, 1)` | $\phi^1 = 0 + 1\phi$ | `[]` |
| `evalPhi_phi2` | `theorem evalPhi_phi2 : evalPhi [true, false, false] = (1, 1)` | $\phi^2 = 1 + 1\phi$ | `[]` |
| `evalPhi_phi3` | `theorem evalPhi_phi3 : evalPhi [true, false, false, false] = (1, 2)` | $\phi^3 = 1 + 2\phi$ | `[]` |
| `evalPhi_phi4` | `theorem evalPhi_phi4 : evalPhi [true, false, false, false, false] = (2, 3)` | $\phi^4 = 2 + 3\phi$ | `[]` |
| `evalPhi_reduction_equiv` | `theorem evalPhi_reduction_equiv : evalPhi [false, true, true] = evalPhi [true, false, false]` | Direct ring evaluation equality: $\text{evalPhi}(011) = \text{evalPhi}(100) = (1, 1)$ | `[]` |
| `step011_evalPhi` | `theorem step011_evalPhi {w w' : Word} (h : Step011 w w') : evalPhi w = evalPhi w'` | Full invariance of $\mathbb{Z}[\phi]$ valuation under any reduction step | `[propext, Classical.choice, Quot.sound]` |

### Normal Form Characterization & Irreducibility
| Theorem | Lean Declaration | Description | Axioms |
|---|---|---|---|
| `step_not_no11` | `theorem step_not_no11 {w w' : Word} (h : Step w w') : ¬ no11 w` | Any reducible word must violate `no11` | `[]` |
| `no11_irreducible` | `theorem no11_irreducible (w : Word) (h : no11 w) : ¬ ∃ w', Step w w'` | Canonical words are strictly irreducible (normal forms) | `[]` |
| `step_of_not_no11` | `theorem step_of_not_no11 (w : Word) (h : ¬ no11 w) : ∃ w', Step w w'` | Any non-canonical word admits a reduction step | `[propext]` |
| `irreducible_iff_no11` | `theorem irreducible_iff_no11 (w : Word) : (¬ ∃ w', Step w w') ↔ no11 w` | Exact characterization: Irreducible $\iff$ Normal Form $\iff$ `no11` | `[propext, Classical.choice, Quot.sound]` |

### Verified Finite Zeckendorf & Bergman Representations
| Theorem | Lean Declaration | Description | Axioms |
|---|---|---|---|
| `zeckendorf_zero` | `theorem zeckendorf_zero : ∃ w, no11 w ∧ evalFib w = 0` | Existence of canonical Fibonacci representation for 0 | `[]` |
| `zeckendorf_one` | `theorem zeckendorf_one : ∃ w, no11 w ∧ evalFib w = 1` | Existence of canonical Fibonacci representation for 1 | `[]` |
| `zeckendorf_two` | `theorem zeckendorf_two : ∃ w, no11 w ∧ evalFib w = 2` | Existence of canonical Fibonacci representation for 2 | `[]` |
| `zeckendorf_three` | `theorem zeckendorf_three : ∃ w, no11 w ∧ evalFib w = 3` | Existence of canonical Fibonacci representation for 3 | `[]` |
| `zeckendorf_four` | `theorem zeckendorf_four : ∃ w, no11 w ∧ evalFib w = 4` | Existence of canonical Fibonacci representation for 4 | `[]` |
| `zeckendorf_five` | `theorem zeckendorf_five : ∃ w, no11 w ∧ evalFib w = 5` | Existence of canonical Fibonacci representation for 5 | `[]` |
| `zeckendorf_six` | `theorem zeckendorf_six : ∃ w, no11 w ∧ evalFib w = 6` | Existence of canonical Fibonacci representation for 6 | `[]` |
| `zeckendorf_seven` | `theorem zeckendorf_seven : ∃ w, no11 w ∧ evalFib w = 7` | Existence of canonical Fibonacci representation for 7 | `[]` |
| `zeckendorf_eight` | `theorem zeckendorf_eight : ∃ w, no11 w ∧ evalFib w = 8` | Existence of canonical Fibonacci representation for 8 | `[]` |
| `bergman_phi0` | `theorem bergman_phi0 : ∃ w, no11 w ∧ evalPhi w = (1, 0)` | Canonical representation for $\phi^0 = 1$ | `[]` |
| `bergman_phi1` | `theorem bergman_phi1 : ∃ w, no11 w ∧ evalPhi w = (0, 1)` | Canonical representation for $\phi^1 = \phi$ | `[]` |
| `bergman_phi2` | `theorem bergman_phi2 : ∃ w, no11 w ∧ evalPhi w = (1, 1)` | Canonical representation for $\phi^2 = 1 + \phi$ | `[]` |
| `bergman_phi3` | `theorem bergman_phi3 : ∃ w, no11 w ∧ evalPhi w = (1, 2)` | Canonical representation for $\phi^3 = 1 + 2\phi$ | `[]` |
| `bergman_phi4` | `theorem bergman_phi4 : ∃ w, no11 w ∧ evalPhi w = (2, 3)` | Canonical representation for $\phi^4 = 2 + 3\phi$ | `[]` |

---

## 2. Module M2: `Collatz.ReverseTree` (17 Theorems)

### Reverse Operators & Invariants
| Theorem | Lean Declaration | Description | Axioms |
|---|---|---|---|
| `g0_pos` | `theorem g0_pos {n : Nat} (hn : n > 0) : g0 n > 0` | Positivity of operator $G_0(n) = 2n$ | `[propext, Quot.sound]` |
| `g1_pos` | `theorem g1_pos {n : Nat} (hmod : isG1Valid n) : g1 n > 0` | Positivity of operator $G_1(n) = (n-1)/3$ | `[propext, Quot.sound]` |
| `g1_odd` | `theorem g1_odd {n : Nat} (hmod : isG1Valid n) : g1 n % 2 = 1` | Result of $G_1$ transition is strictly odd | `[propext, Quot.sound]` |
| `g1_mod6_ne_4` | `theorem g1_mod6_ne_4 {n : Nat} (hmod : isG1Valid n) : ¬ isG1Valid (g1 n)` | Result of $G_1$ cannot satisfy $n \equiv 4 \pmod 6$ | `[propext, Quot.sound]` |
| `g1_consecutive_invalid` | `theorem g1_consecutive_invalid {n : Nat} (hmod : isG1Valid n) : ¬ isG1Valid (g1 n)` | Consecutive $G_1$ transitions are mathematically impossible | `[propext, Quot.sound]` |

### Reachability Predicate & Path Evaluation Soundness
| Theorem | Lean Declaration | Description | Axioms |
|---|---|---|---|
| `reachable_pos` | `theorem reachable_pos {n : Nat} (h : ReachableFromOne n) : n > 0` | Any reachable integer is strictly positive | `[propext, Quot.sound]` |
| `evalPath_sound_general` | `theorem evalPath_sound_general (p : Path) : ∀ start n, ReachableFromOne start → evalPath start p = some n → ReachableFromOne n` | Generalized soundness of path execution | `[]` |
| `evalPath_sound` | `theorem evalPath_sound {p : Path} {n : Nat} (h : evalPath 1 p = some n) : ReachableFromOne n` | Soundness: path execution from 1 yields reachable node | `[]` |

### Path-to-Word Injection & The `no11` Structural Lemma
| Theorem | Lean Declaration | Description | Axioms |
|---|---|---|---|
| `pathToWord_nil` | `theorem pathToWord_nil : pathToWord [] = []` | Empty path maps to empty word | `[]` |
| `pathToWord_cons` | `theorem pathToWord_cons (s : Step) (p : Path) : pathToWord (s :: p) = stepToBool s :: pathToWord p` | Cons step mapping | `[]` |
| `pathToWord_injective` | `theorem pathToWord_injective : ∀ p1 p2, pathToWord p1 = pathToWord p2 → p1 = p2` | Injection of reverse paths into binary words | `[]` |
| `pathToWord_no11_of_hasNoConsecutiveS1` | `theorem pathToWord_no11_of_hasNoConsecutiveS1 (p : Path) (h : hasNoConsecutiveS1 p) : PhiWords.no11 (pathToWord p)` | Paths without consecutive $s_1$ map to `no11` words | `[]` |
| `evalStep_s1_result_not_s1` | `theorem evalStep_s1_result_not_s1 (curr next : Nat) (h : evalStep curr Step.s1 = some next) : evalStep next Step.s1 = none` | Immediate second $s_1$ evaluation aborts to `none` | `[propext, Quot.sound]` |
| `validPath_hasNoConsecutiveS1` | `theorem validPath_hasNoConsecutiveS1 (p : Path) : ∀ curr, validPath curr p → hasNoConsecutiveS1 p` | Every valid path avoids consecutive $s_1$ steps | `[propext, Quot.sound]` |
| `validPath_yields_no11` | `theorem validPath_yields_no11 {curr : Nat} {p : Path} (h : validPath curr p) : PhiWords.no11 (pathToWord p)` | **Main Structural Lemma:** every valid reverse path gives a `no11` word | `[propext, Quot.sound]` |

### Realizability Characterization (Non-Surjectivity) & Conditional Hypothesis
| Theorem | Lean Declaration | Description | Axioms |
|---|---|---|---|
| `no11_not_all_realizable` | `theorem no11_not_all_realizable : ∃ w, PhiWords.no11 w ∧ ¬ ∃ p, pathToWord p = w ∧ validPath 1 p` | **Realizability Characterization:** reverse paths form a strict subset of `no11` words (explicit counterexample `[true]`) | `[]` |
| `conditional_reachable` | `theorem conditional_reachable (hconjecture : CollatzConjecture) (n : Nat) (hn : n > 0) : ReachableFromOne n` | Reachability under explicit conditional hypothesis variable | `[]` |
