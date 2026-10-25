/-
Copyright (c) 2026 Pavel Cherkashin.
SPDX-License-Identifier: MIT
-/
import Emergent.Dynamics

namespace Emergent.Models

/-- Two worlds with universal (S5) accessibility. -/
def access (_ _ : Bool) : Prop := True
def positivity : Positivity Bool Bool := principal false
def contingent (w : Bool) : Prop := w = false
def actual (x w : Bool) : Prop := x = false ∧ w = false
def step (_ : Bool) (_ : Bool) : Bool := false

theorem model_ultrafilter (w : Bool) : LocalUltrafilter positivity w :=
  principal_ultrafilter false w

theorem model_godlike (w : Bool) : godlike positivity false w :=
  principal_godlike false w

theorem model_global_attraction (w x : Bool) : Reaches (step w) false x := ⟨1, rfl⟩

theorem model_no_collapse : contingent false ∧ ¬ box access contingent false := by
  refine ⟨rfl, ?_⟩
  intro h
  have bad : true = false := h true True.intro
  cases bad

theorem model_positivity_not_rigid :
    ∃ p : Property Bool Bool, positivity p false ∧ ¬ box access (positivity p) false := by
  refine ⟨fun _ w => contingent w, rfl, ?_⟩
  exact model_no_collapse.2

theorem model_no_necessary_actuality :
    ¬ box access (fun w => ∃ x, actual x w ∧ godlike positivity x w) false := by
  intro h
  obtain ⟨_, ⟨_, hw⟩, _⟩ := h true True.intro
  cases hw

/-- One explicit model jointly refutes two unsupported bridges. -/
theorem attraction_ultrafilter_countermodel :
    (∀ w, LocalUltrafilter positivity w) ∧
    (∀ w, godlike positivity false w) ∧
    (∀ w x, Reaches (step w) false x) ∧
    ¬ box access contingent false ∧
    ¬ box access (fun w => ∃ x, actual x w ∧ godlike positivity x w) false :=
  ⟨model_ultrafilter, model_godlike, model_global_attraction,
    model_no_collapse.2, model_no_necessary_actuality⟩

/-- A temporal realization predicate separates future emergence from present existence. -/
def realized (x : Bool) (t : Nat) : Prop := x = false ∧ 0 < t

theorem future_realization_without_present :
    (∀ x : Bool, realized (orbit (step false) 1 x) 1) ∧
    ¬ ∃ x : Bool, realized x 0 := by
  refine ⟨fun _ => ⟨rfl, Nat.zero_lt_succ 0⟩, ?_⟩
  intro ⟨_, _, h⟩
  exact Nat.not_lt_zero 0 h

/-- Distinct toy laws, one common attractor; these are not physical-law models. -/
def toyLaw (fast : Bool) (n : Nat) : Nat := if fast then 0 else n - 1

theorem toy_laws_distinct : toyLaw false 2 ≠ toyLaw true 2 := by decide

theorem toy_laws_reach (fast : Bool) (x : Nat) :
    ∃ n, n ≤ x ∧ orbit (toyLaw fast) n x = 0 := by
  apply rank_reaches (toyLaw fast) 0 id (fun _ h => h)
  intro n hn
  cases fast with
  | false =>
    change n - 1 < n
    omega
  | true =>
    change 0 < n
    omega

/-- A concrete singleton interpretation satisfies all five Scott axioms. -/
def singletonPos : Positivity Unit Unit := principal ()
def singletonR (_ _ : Unit) : Prop := True
def singletonActual (_ _ : Unit) : Prop := True

theorem scott_singleton_model : ScottAxioms singletonR singletonActual singletonPos := by
  constructor
  · intro p w
    exact Iff.rfl
  · intro p q w hp h
    exact h w True.intro () True.intro hp
  · intro w
    exact principal_godlike () w
  · intro p w hp z _
    cases w
    cases z
    exact hp
  · intro w p hp z _
    cases w
    cases z
    exact ⟨(), True.intro, hp.1⟩

end Emergent.Models
