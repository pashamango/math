/-
Copyright (c) 2026 Pavel Cherkashin.
SPDX-License-Identifier: MIT
-/
/-!
# Explicit Shallow Higher-Order Modal Embedding

This module provides an independent, clean-room shallow embedding of higher-order
modal logic and the mathematical framework of Scott's variant of the Gödelian
ontological argument (Dana Scott, 1970), formulated with explicit world and entity
parameters and actualist quantification.

No global modal axioms (such as S5 or modal collapse) are posited globally;
all properties and accessibility assumptions are stated locally via hypothesis structures.
-/
namespace Emergent
universe u v
abbrev MProp (W : Type u) := W → Prop
abbrev Property (E : Type v) (W : Type u) := E → W → Prop
abbrev Positivity (E : Type v) (W : Type u) := Property E W → W → Prop

def box {W : Type u} (R : W → W → Prop) (q : MProp W) (w : W) : Prop :=
  ∀ z, R w z → q z

def diamond {W : Type u} (R : W → W → Prop) (q : MProp W) (w : W) : Prop :=
  ∃ z, R w z ∧ q z

def negProperty {E : Type v} {W : Type u} (p : Property E W) : Property E W :=
  fun x w => ¬ p x w

def godlike {E : Type v} {W : Type u} (P : Positivity E W) (x : E) (w : W) : Prop :=
  ∀ p, P p w → p x w

def essence {E : Type v} {W : Type u} (R : W → W → Prop)
    (actual : Property E W) (p : Property E W) (x : E) (w : W) : Prop :=
  p x w ∧ ∀ q : Property E W, q x w → ∀ z, R w z → ∀ y, actual y z → p y z → q y z

def necessaryExistence {E : Type v} {W : Type u} (R : W → W → Prop)
    (actual : Property E W) (x : E) (w : W) : Prop :=
  ∀ p, essence R actual p x w → box R (fun z => ∃ y, actual y z ∧ p y z) w

structure ScottAxioms {E : Type v} {W : Type u} (R : W → W → Prop)
    (actual : Property E W) (P : Positivity E W) : Prop where
  a1 : ∀ p w, (¬ P p w) ↔ P (negProperty p) w
  a2 : ∀ p q w, P p w →
    (∀ z, R w z → ∀ y, actual y z → p y z → q y z) → P q w
  a3 : ∀ w, P (godlike P) w
  a4 : ∀ p w, P p w → box R (P p) w
  a5 : ∀ w, P (necessaryExistence R actual) w

variable {E : Type v} {W : Type u} {R : W → W → Prop}
  {actual : Property E W} {P : Positivity E W}

theorem positive_possible (A : ScottAxioms R actual P) (p : Property E W) (w : W)
    (hp : P p w) : diamond R (fun z => ∃ x, actual x z ∧ p x z) w := by
  apply Classical.byContradiction
  intro h_not_diamond
  have h_imp : ∀ z, R w z → ∀ y, actual y z → p y z → negProperty p y z := by
    intro z hz y hy hpy
    intro hp_contr
    exact h_not_diamond ⟨z, hz, y, hy, hp_contr⟩
  have h_neg_pos : P (negProperty p) w := A.a2 p (negProperty p) w hp h_imp
  have h_not_pos : ¬ P p w := (A.a1 p w).mpr h_neg_pos
  exact h_not_pos hp

theorem possible_godlike (A : ScottAxioms R actual P) (w : W) :
    diamond R (fun z => ∃ x, actual x z ∧ godlike P x z) w := by
  exact positive_possible A (godlike P) w (A.a3 w)

theorem witnessed_positive (A : ScottAxioms R actual P) {x : E} {w : W}
    (hg : godlike P x w) {p : Property E W} (hp : p x w) : P p w := by
  apply Classical.byContradiction
  intro h_not_pos
  have h_neg_pos : P (negProperty p) w := (A.a1 p w).mp h_not_pos
  have h_neg_val : negProperty p x w := hg (negProperty p) h_neg_pos
  exact h_neg_val hp

theorem godlike_essence (A : ScottAxioms R actual P) {x : E} {w : W}
    (hg : godlike P x w) : essence R actual (godlike P) x w := by
  constructor
  · exact hg
  · intro q hq z hz y _ hy_godlike
    have h_pos_q_w : P q w := witnessed_positive A hg hq
    have h_box_pos_q : box R (P q) w := A.a4 q w h_pos_q_w
    have h_pos_q_z : P q z := h_box_pos_q z hz
    exact hy_godlike q h_pos_q_z

theorem scott_necessary (A : ScottAxioms R actual P)
    (symm : ∀ w z, R w z → R z w) (w : W) :
    box R (fun z => ∃ x, actual x z ∧ godlike P x z) w := by
  have nec_from_witness : ∀ v : W, (∃ x, actual x v ∧ godlike P x v) →
      box R (fun t => ∃ y, actual y t ∧ godlike P y t) v := by
    intro v ⟨x, _, hx_godlike⟩
    have h_nec_exist : necessaryExistence R actual x v := hx_godlike (necessaryExistence R actual) (A.a5 v)
    exact h_nec_exist (godlike P) (godlike_essence A hx_godlike)
  obtain ⟨z, hwz, hz_witness⟩ := possible_godlike A w
  have hzw : R z w := symm w z hwz
  have h_box_z := nec_from_witness z hz_witness
  have hw_witness : ∃ x, actual x w ∧ godlike P x w := h_box_z w hzw
  exact nec_from_witness w hw_witness

theorem scott_modal_collapse (A : ScottAxioms R actual P)
    (symm : ∀ w z, R w z → R z w) (q : MProp W) (w : W) : q w → box R q w := by
  intro hqw z hz
  have hzw : R z w := symm w z hz
  obtain ⟨x, _, hx_godlike⟩ := scott_necessary A symm z w hzw
  obtain ⟨y, hy_actual, hy_godlike⟩ := scott_necessary A symm w z hz
  have h_ess := (godlike_essence A hx_godlike).2
  let prop_q : Property E W := fun _ t => q t
  have h_prop_q_x : prop_q x w := hqw
  have h_prop_q_y : prop_q y z := h_ess prop_q h_prop_q_x z hz y hy_actual hy_godlike
  exact h_prop_q_y

theorem scott_unique (A : ScottAxioms R actual P) {x y : E} {w : W}
    (hx : godlike P x w) (hy : godlike P y w) : x = y := by
  let eq_x : Property E W := fun z _ => x = z
  have h_eq_x_x : eq_x x w := rfl
  have h_pos_eq_x : P eq_x w := witnessed_positive A hx h_eq_x_x
  exact hy eq_x h_pos_eq_x

/-- A chosen witness defines positivity by evaluation: a principal theory. -/
def principal (g : E) : Positivity E W := fun p w => p g w

theorem principal_godlike (g : E) (w : W) : godlike (principal g) g w :=
  fun _ hp => hp

theorem principal_characterization (g x : E) (w : W) :
    godlike (principal g) x w ↔ x = g := by
  constructor
  · intro hx
    exact hx (fun y _ => y = g) rfl
  · intro hx
    cases hx
    exact principal_godlike g w

theorem principal_complement (g : E) (p : Property E W) (w : W) :
    (¬ principal g p w) ↔ principal g (negProperty p) w := Iff.rfl

/-- 'Everything true of the chosen witness is positive' is definitional here. -/
theorem principal_witnessed (g : E) (p : Property E W) (w : W) :
    p g w ↔ principal g p w := Iff.rfl

/-- An ultrafilter on extensions at a single world, without modal rigidity. -/
def LocalUltrafilter (P : Positivity E W) (w : W) : Prop :=
  P (fun _ _ => True) w ∧ ¬ P (fun _ _ => False) w ∧
  (∀ p q, P p w → P q w → P (fun x z => p x z ∧ q x z) w) ∧
  (∀ p q, (∀ x, p x w → q x w) → P p w → P q w) ∧
  (∀ p, P p w ∨ P (negProperty p) w)

theorem principal_ultrafilter (g : E) (w : W) : LocalUltrafilter (principal g) w :=
  ⟨True.intro, (fun h => h), (fun _ _ hp hq => ⟨hp, hq⟩),
    (fun _ _ h hp => h g hp), (fun p => Classical.em (p g w))⟩

theorem all_positive_has_no_godlike (w : W)
    (hall : ∀ p, P p w) : ¬ ∃ x, godlike P x w := by
  intro ⟨x, hx⟩
  exact hx (fun _ _ => False) (hall _)

end Emergent
