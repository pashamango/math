import Emergent.SelfReference
import Emergent.Dynamics

namespace Emergent
universe u v

def truthTheory {E : Type v} {W : Type u} (a : E) (w : W) : Theory (Property E W) :=
  {p | p a w}

noncomputable def recursivePos {E : Type v} {W : Type u}
    (F : W → Theory (Property E W) →o Theory (Property E W)) : Positivity E W :=
  fun p w => p ∈ leastTheory (F w)

theorem recursive_target_godlike {E : Type v} {W : Type u}
    (F : W → Theory (Property E W) →o Theory (Property E W)) (a : W → E)
    (safe : ∀ w, F w (truthTheory (a w) w) ⊆ truthTheory (a w) w) (w : W) :
    godlike (recursivePos F) (a w) w := by
  intro p hp
  exact (F w).lfp_le (safe w) hp

theorem recursive_target_coherent {E : Type v} {W : Type u}
    (F : W → Theory (Property E W) →o Theory (Property E W)) (a : W → E)
    (safe : ∀ w, F w (truthTheory (a w) w) ⊆ truthTheory (a w) w) (w : W) :
    Coherent negProperty (leastTheory (F w)) := by
  apply leastTheory_coherent (F w) negProperty (truthTheory (a w) w) (safe w)
  exact fun _ hp hnp => hnp hp

/-- Closure safety and rank descent jointly give eventual realization of every
recursively positive property in every accessible world. No Scott A4/A5 or
necessary-current-actuality conclusion is assumed or derived. -/
theorem recursive_modal_emergence {E : Type v} {W : Type u} (R : W → W → Prop)
    (F : W → Theory (Property E W) →o Theory (Property E W))
    (f : W → E → E) (a : W → E) (rank : W → E → Nat)
    (safe : ∀ w, F w (truthTheory (a w) w) ⊆ truthTheory (a w) w)
    (zero : ∀ w x, rank w x = 0 → x = a w)
    (decrease : ∀ w x, x ≠ a w → rank w (f w x) < rank w x)
    (x : W → E) (w : W) :
    box R (fun z => ∃ n, n ≤ rank z (x z) ∧
      godlike (recursivePos F) (orbit (f z) n (x z)) z) w := by
  intro z _
  obtain ⟨n, hn, he⟩ := rank_reaches (f z) (a z) (rank z) (zero z) (decrease z) (x z)
  exact ⟨n, hn, he.symm ▸ recursive_target_godlike F a safe z⟩

/-- If the target is absorbing, realized positive properties stay realized. -/
theorem recursive_emergence_persists {E : Type v} {W : Type u}
    (F : W → Theory (Property E W) →o Theory (Property E W))
    (f : W → E → E) (a : W → E)
    (safe : ∀ w, F w (truthTheory (a w) w) ⊆ truthTheory (a w) w)
    (fixed : ∀ w, f w (a w) = a w) (w : W) (x : E) (n : Nat)
    (hit : orbit (f w) n x = a w) (m : Nat) :
    godlike (recursivePos F) (orbit (f w) (m + n) x) w := by
  rw [reaches_persists (fixed w) hit m]
  exact recursive_target_godlike F a safe w

end Emergent
