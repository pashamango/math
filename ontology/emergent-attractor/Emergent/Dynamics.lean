import Emergent.Modal

namespace Emergent
universe u v

/-- Iteration by first step, definitionally convenient for rank induction. -/
def orbit {S : Type u} (f : S → S) : Nat → S → S
  | 0, x => x
  | n + 1, x => orbit f n (f x)

def Reaches {S : Type u} (f : S → S) (a x : S) : Prop :=
  ∃ n, orbit f n x = a

theorem orbit_fixed {S : Type u} {f : S → S} {a : S}
    (hf : f a = a) (n : Nat) : orbit f n a = a := by
  induction n with
  | zero => rfl
  | succ n ih => simpa [orbit, hf] using ih

theorem orbit_add {S : Type u} (f : S → S) (m n : Nat) (x : S) :
    orbit f (m + n) x = orbit f m (orbit f n x) := by
  induction n generalizing x with
  | zero => simp [orbit]
  | succ n ih => simpa [Nat.add_succ, orbit] using ih (f x)

/-- A strict natural-number rank gives finite hitting, with an explicit bound. -/
theorem rank_reaches {S : Type u} (f : S → S) (a : S) (rank : S → Nat)
    (zero : ∀ x, rank x = 0 → x = a)
    (decrease : ∀ x, x ≠ a → rank (f x) < rank x) (x : S) :
    ∃ n, n ≤ rank x ∧ orbit f n x = a := by
  have aux : ∀ k x, rank x = k → ∃ n, n ≤ rank x ∧ orbit f n x = a := by
    intro k
    induction k using Nat.strongRecOn with
    | ind k ih =>
      intro x hx
      by_cases hz : rank x = 0
      · exact ⟨0, Nat.zero_le _, zero x hz⟩
      by_cases hxa : x = a
      · exact ⟨0, Nat.zero_le _, hxa⟩
      · have hd : rank (f x) < k := hx ▸ decrease x hxa
        obtain ⟨n, hn, he⟩ := ih (rank (f x)) hd (f x) rfl
        exact ⟨n + 1, Nat.succ_le_of_lt (Nat.lt_of_le_of_lt hn (decrease x hxa)), he⟩
  exact aux (rank x) x rfl

theorem reaches_persists {S : Type u} {f : S → S} {a x : S}
    (hf : f a = a) {n : Nat} (hn : orbit f n x = a) (m : Nat) :
    orbit f (m + n) x = a := by
  rw [orbit_add, hn]
  exact orbit_fixed hf m

theorem modal_rank_reaches {W : Type u} {S : Type v} (R : W → W → Prop)
    (f : W → S → S) (a : W → S) (rank : W → S → Nat)
    (zero : ∀ w x, rank w x = 0 → x = a w)
    (decrease : ∀ w x, x ≠ a w → rank w (f w x) < rank w x)
    (x : W → S) (w : W) : box R (fun z => Reaches (f z) (a z) (x z)) w := by
  intro z _
  obtain ⟨n, _, hn⟩ := rank_reaches (f z) (a z) (rank z) (zero z) (decrease z) (x z)
  exact ⟨n, hn⟩

/-- Identity has fixed points but does not drive a different state to them. -/
theorem fixed_does_not_imply_attraction :
    (id : Bool → Bool) false = false ∧ ¬ Reaches id false true := by
  refine ⟨rfl, ?_⟩
  intro ⟨n, hn⟩
  have hi := orbit_fixed (f := (id : Bool → Bool)) (a := true) rfl n
  rw [hi] at hn
  cases hn

/-- A two-cycle has no fixed point. -/
theorem complement_no_fixed : ¬ ∃ x : Bool, (!x) = x := by
  intro ⟨x, hx⟩
  cases x <;> cases hx

end Emergent
