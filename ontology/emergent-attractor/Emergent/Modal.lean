/-
Copyright (c) 2026 Pavel Cherkashin.
SPDX-License-Identifier: MIT
-/
namespace Emergent
universe u v
abbrev MProp (W : Type u) := W → Prop
abbrev Property (E : Type v) (W : Type u) := E → W → Prop
abbrev Positivity (E : Type v) (W : Type u) := Property E W → W → Prop

def box {W : Type u} (R : W → W → Prop) (q : MProp W) (w : W) : Prop := ∀ z, R w z → q z
def diamond {W : Type u} (R : W → W → Prop) (q : MProp W) (w : W) : Prop := ∃ z, R w z ∧ q z
def negProperty {E : Type v} {W : Type u} (p : Property E W) : Property E W := fun x w => ¬ p x w
def godlike {E : Type v} {W : Type u} (P : Positivity E W) (x : E) (w : W) : Prop := ∀ p, P p w → p x w
def essence {E : Type v} {W : Type u} (R : W → W → Prop) (actual : Property E W) (p : Property E W) (x : E) (w : W) : Prop := p x w ∧ ∀ q : Property E W, q x w → ∀ z, R w z → ∀ y, actual y z → p y z → q y z
def necessaryExistence {E : Type v} {W : Type u} (R : W → W → Prop) (actual : Property E W) (x : E) (w : W) : Prop := ∀ p, essence R actual p x w → box R (fun z => ∃ y, actual y z ∧ p y z) w

structure ScottAxioms {E : Type v} {W : Type u} (R : W → W → Prop) (actual : Property E W) (P : Positivity E W) : Prop where
  a1 : ∀ p w, (¬ P p w) ↔ P (negProperty p) w
  a2 : ∀ p q w, P p w → (∀ z, R w z → ∀ y, actual y z → p y z → q y z) → P q w
  a3 : ∀ w, P (godlike P) w
  a4 : ∀ p w, P p w → box R (P p) w
  a5 : ∀ w, P (necessaryExistence R actual) w

variable {E : Type v} {W : Type u} {R : W → W → Prop} {actual : Property E W} {P : Positivity E W}
theorem positive_possible (A : ScottAxioms R actual P) (p : Property E W) (w : W) (hp : P p w) : diamond R (fun z => ∃ x, actual x z ∧ p x z) w := by
  classical
  by_contra h
  have hno : ¬ ∃ z, R w z ∧ ∃ x, actual x z ∧ p x z := by
    simpa [diamond] using h
  have htransfer :
      ∀ z, R w z → ∀ y, actual y z → p y z → negProperty p y z := by
    intro z hr y ha hpy
    exact fun hpy' => hno ⟨z, hr, y, ha, hpy⟩
  have hneg : P (negProperty p) w := A.a2 p (negProperty p) w hp htransfer
  exact (A.a1 p w).mpr hneg hp

theorem possible_godlike (A : ScottAxioms R actual P) (w : W) : diamond R (fun z => ∃ x, actual x z ∧ godlike P x z) w := by
  exact positive_possible A (godlike P) w (A.a3 w)

theorem witnessed_positive (A : ScottAxioms R actual P) {x : E} {w : W} (hg : godlike P x w) {p : Property E W} (hp : p x w) : P p w := by
  classical
  by_contra hnot
  have hneg : P (negProperty p) w := (A.a1 p w).mp hnot
  exact (hg (negProperty p) hneg) hp

theorem godlike_essence (A : ScottAxioms R actual P) {x : E} {w : W} (hg : godlike P x w) : essence R actual (godlike P) x w := by
  constructor
  · exact hg (godlike P) (A.a3 w)
  · intro q hqx z hR y hyactual hyg
    have hqpos : P q w := witnessed_positive A hg hqx
    have hqposz : P q z := (A.a4 q w hqpos) z hR
    exact hyg q hqposz

theorem scott_necessary (A : ScottAxioms R actual P) (symm : ∀ w z, R w z → R z w) (w : W) : box R (fun z => ∃ x, actual x z ∧ godlike P x z) w := by
  intro z hwz
  have hne : P (necessaryExistence R actual) z := A.a5 z
  have hpossible := positive_possible A (godlike P) z (A.a3 z)
  rcases hpossible with ⟨u, hzu, x, hxactual, hxg⟩
  have hne_u : P (necessaryExistence R actual) u :=
    (A.a4 (necessaryExistence R actual) z hne) u hzu
  have hnex : necessaryExistence R actual x u :=
    witnessed_positive A hxg hne_u
  have hbox := hnex (godlike P) (godlike_essence A hxg)
  have huz : R u z := symm z u hzu
  exact hbox z huz

theorem scott_modal_collapse (A : ScottAxioms R actual P) (symm : ∀ w z, R w z → R z w) (q : MProp W) (w : W) : q w → box R q w := by
  intro hqw z hwz
  have hsc_z := scott_necessary A symm z
  have hzw : R z w := symm w z hwz
  rcases hsc_z w hzw with ⟨xw, hxw_actual, hxw_g⟩
  have hqw_pos : P (fun (_ : E) (t : W) => q t) w := by
    apply witnessed_positive A hxw_g
    exact hqw
  have hqw_pos_z : P (fun (_ : E) (t : W) => q t) z :=
    (A.a4 (fun (_ : E) (t : W) => q t) w hqw_pos) z hwz
  have hsc_w := scott_necessary A symm w
  rcases hsc_w z hwz with ⟨xz, hxz_actual, hxz_g⟩
  exact hxz_g (fun (_ : E) (t : W) => q t) hqw_pos_z

theorem scott_unique (A : ScottAxioms R actual P) {x y : E} {w : W} (hx : godlike P x w) (hy : godlike P y w) : x = y := by
  classical
  by_contra hxy
  have hneq : x ≠ y := fun h => hxy h
  have hpos : P (fun a (_ : W) => a = x) w := by
    apply witnessed_positive A hx
    rfl
  have hyx : y = x := hy (fun a (_ : W) => a = x) hpos
  exact hneq hyx.symm

/-- A chosen witness defines positivity by evaluation: a principal theory. -/
def principal (g : E) : Positivity E W := fun p w => p g w
theorem principal_godlike (g : E) (w : W) : godlike (principal g) g w := by
  intro p hp
  exact hp
theorem principal_characterization (g x : E) (w : W) : godlike (principal g) x w ↔ x = g := by
  constructor
  · intro hg
    have h := hg (fun a (_ : W) => a = g) rfl
    exact h
  · intro h
    subst x
    exact principal_godlike g w
theorem principal_complement (g : E) (p : Property E W) (w : W) : (¬ principal g p w) ↔ principal g (negProperty p) w := by
  rfl
/-- 'Everything true of the chosen witness is positive' is definitional here. -/
theorem principal_witnessed (g : E) (p : Property E W) (w : W) : p g w ↔ principal g p w := by
  rfl
/-- An ultrafilter on extensions at a single world, without modal rigidity. -/
def LocalUltrafilter (P : Positivity E W) (w : W) : Prop := P (fun _ _ => True) w ∧ ¬ P (fun _ _ => False) w ∧ (∀ p q, P p w → P q w → P (fun x z => p x z ∧ q x z) w) ∧ (∀ p q, (∀ x, p x w → q x w) → P p w → P q w) ∧ (∀ p, P p w ∨ P (negProperty p) w)
theorem principal_ultrafilter (g : E) (w : W) : LocalUltrafilter (principal g) w := by
  unfold LocalUltrafilter principal
  refine ⟨?_, ?_, ?_, ?_, ?_⟩
  · trivial
  · intro h
    exact h
  · intro p q hp hq
    exact ⟨hp, hq⟩
  · intro p q himp hp
    exact himp g hp
  · intro p
    exact em (p g w)
theorem all_positive_has_no_godlike (w : W) (hall : ∀ p, P p w) : ¬ ∃ x, godlike P x w := by
  intro h
  rcases h with ⟨x, hx⟩
  have hfalse : P (fun (_ : E) (_ : W) => False) w := hall (fun _ _ => False)
  exact hx (fun _ _ => False) hfalse

end Emergent
