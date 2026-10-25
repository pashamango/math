# Academic Outreach Approach Drafts: Project Collatz
**Date:** 2026-10-25  
**Project:** Science PM (`MIN-231`, `MIN-233`) / Project Collatz  
**Status:** 🚨 **PRELIMINARY DRAFTS — ON HOLD PENDING COLLATZ v0.1.0 RELEASE** 🚨  
**Protocol Gate:** 🚨 **STRICTLY BLOCKED / NOTHING HAS BEEN SENT.** 🚨  

---

## 1. Context & Operational Status (2026-10-25)

* **Development State in Project Collatz:**
  * `MIN-220` (M2: ReverseTree mod-6 characterization, `validPath_yields_no11`, `no11_not_all_realizable`) — **PASS / DONE**.
  * `MIN-219` (M0/M1: Lake scaffold, `PhiWords`, $011 \to 100$ reduction, `evalFib`, `evalPhi`) — **Todo / Rework**.
  * `MIN-221` (M3: CacheCheck, $S \cdot \tau$ linear path checker, backward memoization) — **Todo / Rework**.
  * **Result:** Release `collatz-v0.1.0` has **not** been cut yet; the preprint article is in draft package stage.
* **Science PM Deliverable Status:**
  * Contacts Catalog: `collatz-contacts-2026-10-25.md` (16 core scholars + 3 appendix specialists; verified public profiles).
  * Publication Package: `collatz-paper-draft-package-2026-10-25.md` (Paper architecture, abstract, 81 Lean theorem map, non-claims).
  * Outreach Approach: Direction check letters for 3 primary scholars (Lagarias, Shallit, Barina) — strictly without paper manuscript attached until release.

---

## 2. Inviolable Outreach Send Gate (3 Conditions)

No email, message, DM, or contact may occur until **all three gates are satisfied**:

1. **Gate 1: Release Gate** — Full completion and release of Collatz `v0.1.0` in Project Collatz (M0/M1, M2, and M3 passing all verification checks, zero `sorry`, clean axiom audit, release tag `collatz-v0.1.0` in `pashamango/math`).
2. **Gate 2: Principal Approval Gate** — Explicit single-tap approval by Principal Pavel Cherkashin on the final text for each specific individual.
3. **Gate 3: Identity & Address Re-Verification Gate** — Re-verification of recipient email address on the exact day of dispatch (specifically noting Greenfeld, Laarhoven, and de Weger whose institutional affiliations or emails must be re-confirmed before queuing).

---

## 3. Recipient Categorization

### A. Tier 1: Primary Scholars for Initial Direction Check (3 Scholars)
These three candidates represent the primary academic touchpoints once the release gate opens:
1. **Prof. Jeffrey C. Lagarias** (University of Michigan) — Theoretical foundation, 3x+1 bibliographic surveys, 2-adic dynamics.
2. **Prof. Jeffrey Shallit** (University of Waterloo) — Combinatorics on words, Fibonacci representations (`no11`), automated verification (`Walnut`).
3. **Dr. David Barina** (Brno University of Technology) — Computational verification up to $2^{68}$, trajectory verification algorithms.

### B. Tier 2: Deferred Scholars (13 Core + 3 Appendix — Strictly Cataloged, No Drafts Prepared)
- **Collatz & Number Theory Core:** Terence Tao (UCLA), Marc Chamberland (Grinnell), Alex Kontorovich (Rutgers), Rachel Greenfeld (Northwestern; *re-verify email*), Tomás Oliveira e Silva (Aveiro), Thijs Laarhoven (TU Eindhoven / NXP; *re-verify email*), Benne de Weger (TU Eindhoven; *re-verify email*).
- **Words & Automata:** Jean-Paul Allouche (CNRS / IMJ-PRG), Michel Rigo (Liège), Manon Stipulanti (Liège), Valérie Berthé (CNRS / IRIF).
- **Lean Formalization:** Kevin Buzzard (Imperial College London), Patrick Massot (Paris-Saclay).
- **Appendix (Verification Pending):** Günther J. Wirsching (KU Eichstätt), Shalom Eliahou (ULCO), Ilia Krasikov (Brunel).

---

## 4. Draft Letters for Tier 1 Scholars (Pending Gates)

### Draft 1: Prof. Jeffrey C. Lagarias (Doyen of the 3x+1 Problem)
* **To:** `lagarias@umich.edu`
* **From:** Pavel Cherkashin (`pavel@mindrock.capital`)
* **Subject:** Open-source Lean 4 formalization of Collatz structures (modular reverse tree and phi-words)
* **Status:** 🚨 DRAFT — ON HOLD (NOT SENT)

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

### Draft 2: Prof. Jeffrey Shallit (Combinatorics on Words & Automated Verification)
* **To:** `shallit@uwaterloo.ca`
* **From:** Pavel Cherkashin (`pavel@mindrock.capital`)
* **Subject:** Lean 4 formalization of Fibonacci words and 011 -> 100 rewrite termination (Project Collatz)
* **Status:** 🚨 DRAFT — ON HOLD (NOT SENT)

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

### Draft 3: Dr. David Barina (Computational Collatz Verification)
* **To:** `ibarina@fit.vutbr.cz`
* **From:** Pavel Cherkashin (`pavel@mindrock.capital`)
* **Subject:** Lean 4 trajectory verification and backward path checking (Project Collatz)
* **Status:** 🚨 DRAFT — ON HOLD (NOT SENT)

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

## 5. Protocol Checklist & Invariants

- [x] **No Claims of Conjecture Proof:** Explicitly disclaims proof of Collatz in all documents and draft letters.
- [x] **Completeness Equivalent to Conjecture:** Reverse tree completeness on all $\mathbb{N}^+$ is identified as equivalent to the open conjecture.
- [x] **Direction Check Only:** Letters are designed for direction check without manuscript attachment until release.
- [x] **Zero Outreach Executed:** No emails, letters, DMs, or submissions have been made.
- [x] **Status Held on Hold:** Locked until Collatz v0.1.0 release is tagged and Pavel Cherkashin gives manual approval.
