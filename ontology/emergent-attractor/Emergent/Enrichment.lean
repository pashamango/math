/-
Copyright (c) 2026 Pavel Cherkashin.
SPDX-License-Identifier: MIT
-/
import Emergent.Modal
import Mathlib.Analysis.Complex.Basic
import Mathlib.Topology.MetricSpace.Contracting
import Mathlib.MeasureTheory.Measure.Dirac.Basic

namespace Emergent
universe u
open Filter MeasureTheory
open scoped Topology

/-- A genuine convergence theorem imported from mathlib, with all its conditions. -/
theorem contraction_attractor {S : Type u} [MetricSpace S] [CompleteSpace S] [Nonempty S]
    (f : S → S) {K : NNReal} (hf : ContractingWith K f) :
    ∃ a, Function.IsFixedPt f a ∧
      (∀ x, Tendsto (fun n : ℕ => f^[n] x) atTop (𝓝 a)) ∧
      (∀ b, Function.IsFixedPt f b → b = a) := by
  exact ⟨hf.fixedPoint f, hf.fixedPoint_isFixedPt,
    hf.tendsto_iterate_fixedPoint, fun _ hb => hf.fixedPoint_unique hb⟩

/-- Convergence means eventual entry into every positive-radius neighborhood.
It does not in general mean exact finite-time hitting of the fixed point. -/
theorem contraction_eventual_neighborhood {S : Type u} [MetricSpace S]
    [CompleteSpace S] [Nonempty S] (f : S → S) {K : NNReal}
    (hf : ContractingWith K f) (x : S) (epsilon : ℝ) (he : 0 < epsilon) :
    ∃ N : ℕ, ∀ n, N ≤ n → dist (f^[n] x) (hf.fixedPoint f) < epsilon := by
  exact (Metric.tendsto_atTop.mp (hf.tendsto_iterate_fixedPoint x)) epsilon he

def quarterTurn (z : ℂ) : ℂ := Complex.I * z

/-- Multiplication by i is not Boolean complementation: its square is -id. -/
theorem quarterTurn_squared (z : ℂ) : quarterTurn (quarterTurn z) = -z := by
  simp [quarterTurn, ← mul_assoc, Complex.I_mul_I]

theorem quarterTurn_four (z : ℂ) :
    quarterTurn (quarterTurn (quarterTurn (quarterTurn z))) = z := by
  rw [quarterTurn_squared, quarterTurn_squared, neg_neg]

/-- Probability one under a chosen measure need not mean modal necessity. -/
theorem almost_sure_not_necessary :
    (Measure.dirac (0 : ℕ)) {n : ℕ | n = 0} = 1 ∧
    ¬ box (fun (_ _ : ℕ) => True) (fun n => n = 0) 0 := by
  refine ⟨Measure.dirac_apply_of_mem rfl, ?_⟩
  intro h
  have bad : (1 : ℕ) = 0 := h 1 True.intro
  omega

end Emergent
