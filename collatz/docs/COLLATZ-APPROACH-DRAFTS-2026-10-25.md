# Academic Outreach Draft Letters: Project Collatz
**Date:** 2026-10-25  
**Project:** Science PM (`MIN-231`) / Project Collatz  
**Status:** DRAFT / PENDING PRINCIPAL APPROVAL  
**Protocol Gate:** 🚨 **STRICTLY BLOCKED / NOTHING HAS BEEN SENT.** 🚨  
*Mandate: No automated sending. Each email is subject to Principal Pavel Cherkashin's manual review and explicit single-tap approval for both the recipient and the exact letter text.*

---

## 1. Outreach Governance & Protocol Guardrails

* **Approved Candidates for Potential Initial Outreach (3 Scholars):**
  1. **Prof. Jeffrey C. Lagarias** (University of Michigan) — Theoretical foundation & 3x+1 bibliographic reference.
  2. **Prof. Jeffrey Shallit** (University of Waterloo) — Automatic sequences, $\varphi$-words, and automated verification (`Walnut`).
  3. **Dr. David Barina** (Brno University of Technology) — Supercomputing computational verification up to $2^{68}$.
* **Deferred Candidates:**
  - Remaining 13 core scholars and 3 appendix scholars are cataloged for institutional context only. No outreach will be prepared or queued without separate explicit direction from Pavel.
* **Core Non-Claims Preserved in All Letters:**
  - Clear statement that the work is an open-source Lean 4 formalization and verification architecture, NOT a proof of the Collatz conjecture.
  - Explicit recognition that the completeness of the backward reachability tree $G_0/G_1$ on $\mathbb{N}^+$ is itself the Collatz conjecture.

---

## 2. Letter Draft 1: Prof. Jeffrey C. Lagarias (Doyen of the 3x+1 Problem)

* **To:** `lagarias@umich.edu`
* **From:** Pavel Cherkashin (`pavel@mindrock.capital`)
* **Subject:** Open-source Lean 4 formalization of Collatz structures (modular reverse tree and phi-words)
* **Status:** Draft / Pending Pavel's single tap (NOT SENT)

```text
Dear Professor Lagarias,

I hope this message finds you well.

I am writing to share an open-source formalization project in Lean 4 centered on the structural mechanics of the 3x+1 problem, developed as part of our computational mathematics initiative at Mindrock Capital / ChitAkasha.

Your foundational surveys, the 2010 AMS volume, and your work with Alex Kontorovich on 2-adic integers have long served as our primary mathematical guideposts. In particular, we have taken to heart your consistent counsel regarding the necessity of mathematical rigor and strict epistemic hygiene in this domain.

To be completely unambiguous: our work does NOT claim to prove the Collatz conjecture. Instead, we have constructed a machine-checked Lean 4 library that formally decomposes the problem into verified modules:
1. Module M1 (PhiWords): Mechanizes binary representations without adjacent ones (no11 / Fibonacci words), establishing canonical reduction via 011 -> 100 with strict bit-1 descent and positional evaluation invariance.
2. Module M2 (ReverseTree): Mechanizes the backward predecessor operator G = G0 U G1, proving mod-6 realizability, cycle-free tree branching, and explicitly formalizing the completeness of the tree on all positive integers as an open hypothesis (reverse_tree_complete_hyp), equivalent to the conjecture itself.
3. Module M3 (CacheCheck): Implements certified path verification checking S * tau trajectory bounds and backward memoization.

All theorems have been verified under the Lean 4 kernel with zero unproven assertions (zero sorry), zero custom axioms, and full automated audit logs.

The code and theorem map are publicly available here:
https://github.com/pashamango/math (collatz package)

Given your unparalleled perspective on the 3x+1 problem, I would be deeply honored and grateful for any brief thoughts, critique, or advice you might have on our formalization structure and positioning.

Thank you very much for your lifelong dedication and scholarship in this fascinating field.

Warm regards,

Pavel Cherkashin
Mindrock Capital / ChitAkasha Research
pavel@mindrock.capital
```

---

## 3. Letter Draft 2: Prof. Jeffrey Shallit (Combinatorics on Words & Automated Verification)

* **To:** `shallit@uwaterloo.ca`
* **From:** Pavel Cherkashin (`pavel@mindrock.capital`)
* **Subject:** Lean 4 formalization of Fibonacci words and 011 -> 100 rewrite termination (Project Collatz)
* **Status:** Draft / Pending Pavel's single tap (NOT SENT)

```text
Dear Professor Shallit,

I am writing to share a recently completed Lean 4 formalization package that builds directly upon the theory of words without adjacent ones and canonical Fibonacci representations.

Your monograph with Jean-Paul Allouche on Automatic Sequences and your recent work on phi-representations and automated proofs with Walnut (arXiv:2305.02672) have been deeply inspiring for our research.

In our open-source math library (developed at Mindrock Capital / ChitAkasha), we have mechanized in Lean 4:
- The language of binary words satisfying the no11 condition (Zeckendorf canonical representations).
- The rewrite relation 011 -> 100, proving that every rewrite strictly decreases the bit-1 count (Lyapunov function) while preserving positional Fibonacci evaluation (evalFib).
- Formal termination and reachability of unique canonical normal forms.
- The interface linking these phi-word reductions to backward branching transitions in arithmetic dynamical systems (the 3x+1 reverse tree).

We explicitly treat the completeness of the backward tree as an open hypothesis and make no claim of proving the Collatz conjecture. Our focus is purely on formalizing the word-theoretic and algorithmic invariants with machine-checked certitude.

The Lean 4 code and complete theorem catalog are open-source:
https://github.com/pashamango/math (collatz/Collatz/PhiWords.lean)

As the pioneer of automated methods in combinatorics on words and phi-expansions, your perspective on our formalization approach would be immensely valuable to us.

Thank you for your foundational contributions to the field.

Sincerely,

Pavel Cherkashin
Mindrock Capital / ChitAkasha Research
pavel@mindrock.capital
```

---

## 4. Letter Draft 3: Dr. David Barina (Computational Collatz Verification)

* **To:** `ibarina@fit.vutbr.cz`
* **From:** Pavel Cherkashin (`pavel@mindrock.capital`)
* **Subject:** Lean 4 trajectory verification and backward path checking (Project Collatz)
* **Status:** Draft / Pending Pavel's single tap (NOT SENT)

```text
Dear Dr. Barina,

I am writing to commend your remarkable computational record verifying the Collatz conjecture up to 2^68 on supercomputing architectures, and to share an open-source formal verification package we have developed in Lean 4.

At Mindrock Capital / ChitAkasha, we have been working on formalizing Collatz trajectory mechanics and backward reachability structures. Inspired by high-performance computational verification methods like yours, our Module M3 (CacheCheck) formalizes:
1. Certified linear path verification checking S * tau trajectory bounds.
2. Formal guarantees on memoized backward predecessor graphs (G0/G1).
3. Bridging machine-checked Lean 4 kernel theorems with certified external execution checkers.

We maintain a strict non-claim policy: our project does NOT claim to prove the conjecture, but rather provides machine-checked foundations for the verification algorithms and dynamical invariants used in computational exploration.

The project repository and verification logs are available at:
https://github.com/pashamango/math (collatz package)

We would be delighted to exchange ideas with you on formalizing computational pruning techniques and trajectory certificates in interactive theorem provers.

Thank you for your outstanding experimental work in computational mathematics.

Best regards,

Pavel Cherkashin
Mindrock Capital / ChitAkasha Research
pavel@mindrock.capital
```

---

## 5. Protocol Verification
- [x] All three drafts explicitly disclaim any proof of the Collatz conjecture.
- [x] Reverse tree completeness is explicitly noted as an open hypothesis.
- [x] Strict gate maintained: NOT SENT; requires Pavel Cherkashin's tap.
