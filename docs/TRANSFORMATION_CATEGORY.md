# Transformation Category: Mathematical Architecture

**PRD Reference**: `PRD-26`  
**Ticket**: `MIN-79`  

---

## 1. Objects and Morphisms

Let $\mathbf{Nav}$ be the category where:
- **Objects** are tuples $\mathcal{O} = (X, R, \Sigma, \tau, I)$:
  - $X$: Carrier state space
  - $R$: Relational structure
  - $\Sigma$: Transition signature
  - $\tau$: Metric / topology tolerance structure
  - $I$: Information functional $I: X \to \mathbb{R}_{\ge 0}$
- **Morphisms** are morphisms $T: \mathcal{O}_A \to \mathcal{O}_B$ equipped with a kind classification:
  $$\mathrm{Kind}(T) \in \{\mathrm{ISO}, \mathrm{ADJOINT}, \mathrm{LOSSY}, \mathrm{APPROXIMATE}\}$$

---

## 2. Morphism Classification & Invariant Monotonicity

Each morphism kind enforces a specific information-theoretic and topological invariant:

| Kind | Information Invariant $\Delta I = I_A(a) - I_B(T(a))$ | Metric Error $d_B(T(a), \mathrm{target})$ | Reversibility |
|:---:|:---:|:---:|:---:|
| **`ISO`** | $\Delta I = 0$ (Identical) | $\epsilon = 0$ (Exact) | Full two-sided inverse $T^{-1}$ |
| **`ADJOINT`** | $\Delta I \ge 0$ (Canonical deflation) | $\epsilon = 0$ (Exact bounds) | Galois adjoint $T^*$ ($T^* \circ T = \mathrm{id}$) |
| **`LOSSY`** | $\Delta I > 0$ (Information forgotten) | Discrete / Horizon boundary | Set-valued fiber $T^{-1}(b)$ |
| **`APPROXIMATE`** | Continuous transformation | $d_B \le \epsilon \le \tau$ | Metric neighborhood $B_\epsilon(b)$ |

---

## 3. Composition Semi-Lattice

The four transformation kinds form a bounded join-semilattice under sequential composition:

$$\mathrm{Kind}(T_2 \circ T_1) = \mathrm{Kind}(T_1) \vee \mathrm{Kind}(T_2)$$

With ordering:
$$\mathrm{ISO} < \mathrm{ADJOINT} < \mathrm{LOSSY} < \mathrm{APPROXIMATE}$$

### Axiomatic Proofs of Composition Rules
1. **$\mathrm{ISO} \circ \mathrm{ISO} = \mathrm{ISO}$**:
   Composition of two bijections is a bijection; $\Delta I_1 = 0 \land \Delta I_2 = 0 \implies \Delta I_{12} = 0$.
2. **$\mathrm{ADJOINT} \circ \mathrm{ISO} = \mathrm{ISO} \circ \mathrm{ADJOINT} = \mathrm{ADJOINT}$**:
   Composing an adjoint pair $(L_1, R_1)$ with an isomorphism $f$ yields adjoint pair $(f \circ L_1, R_1 \circ f^{-1})$.
3. **$\mathrm{LOSSY} \circ \mathrm{Any} = \mathrm{LOSSY}$ (for exact base)**:
   Once information is projected away, no exact map can reconstruct the lost fiber uniquely without external side information.
4. **$\mathrm{APPROXIMATE} \circ \mathrm{Any} = \mathrm{APPROXIMATE}$**:
   Any numerical approximation introduces a non-zero error bound $\epsilon > 0$, making the composite morphism approximate.

---

## 4. Claim Receipts as Morphism Verification

In $\mathbf{Nav}$, every instantiated morphism evaluation emits a verified morphism receipt:

$$\text{ClaimReceipt}(T, a) = (T, a, T(a), \Delta I, \epsilon, \text{verified}, \text{sha256})$$

This receipt acts as a machine-checkable certificate guaranteeing that the morphism satisfied its categorical contract during execution.
