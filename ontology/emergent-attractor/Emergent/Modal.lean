/-!
Explicit-parameter, actualist shallow modal embedding.
The Scott proof terms are adapted from Benzmüller, arXiv:2609.26806,
anc/Notes/ScottVariantHOML.lean (retrieved 2026-10-07).
No global modal or ontological postulates are introduced.
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
  intro hn
  have hneg : P (negProperty p) w := A.a2 p (negProperty p) w hp
    (fun z hz x hxa hx => fun _ => hn ⟨z, hz, x, hxa, hx⟩)
  exact (A.a1 p w).mpr hneg hp

theorem possible_godlike (A : ScottAxioms R actual P) (w : W) :
    diamond R (fun z => ∃ x, actual x z ∧ godlike P x z) w :=
  positive_possible A (godlike P) w (A.a3 w)

theorem witnessed_positive (A : ScottAxioms R actual P) {x : E} {w : W}
    (hg : godlike P x w) {p : Property E W} (hp : p x w) : P p w := by
  apply Classical.byContradiction
  intro hn
  exact hg (negProperty p) ((A.a1 p w).mp hn) hp

theorem godlike_essence (A : ScottAxioms R actual P) {x : E} {w : W}
    (hg : godlike P x w) : essence R actual (godlike P) x w := by
  refine ⟨hg, ?_⟩
  intro q hq z hz y _ hgy
  exact hgy q (A.a4 q w (witnessed_positive A hg hq) z hz)

theorem scott_necessary (A : ScottAxioms R actual P)
    (symm : ∀ w z, R w z → R z w) (w : W) :
    box R (fun z => ∃ x, actual x z ∧ godlike P x z) w := by
  have promote : ∀ z, (∃ x, actual x z ∧ godlike P x z) →
      box R (fun t => ∃ x, actual x t ∧ godlike P x t) z := by
    intro z ⟨x, _, hg⟩
    exact hg (necessaryExistence R actual) (A.a5 z)
      (godlike P) (godlike_essence A hg)
  obtain ⟨z, hz, hzg⟩ := possible_godlike A w
  exact promote w (promote z hzg w (symm w z hz))

theorem scott_modal_collapse (A : ScottAxioms R actual P)
    (symm : ∀ w z, R w z → R z w) (q : MProp W) (w : W) : q w → box R q w := by
  intro hq z hz
  obtain ⟨x, _, hx⟩ := scott_necessary A symm z w (symm w z hz)
  obtain ⟨y, hya, hy⟩ := scott_necessary A symm w z hz
  exact (godlike_essence A hx).2 (fun _ t => q t) hq z hz y hya hy

theorem scott_unique (A : ScottAxioms R actual P) {x y : E} {w : W}
    (hx : godlike P x w) (hy : godlike P y w) : x = y := by
  have hp : P (fun z _ => x = z) w := witnessed_positive A hx rfl
  exact hy _ hp

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
    subst x
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
