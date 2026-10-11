import Emergent.Bridge
import Emergent.Models

namespace Emergent.SeededExample

def zeroProperty : Property Nat Bool := fun x _ => x = 0
def boundProperty : Property Nat Bool := fun x _ => x ≤ 1

/-- Seed 'x=0', retain acquired properties, and derive 'x≤1' from the seed.
The operator actually reads its own current theory; it is not a constant map. -/
def rule : Theory (Property Nat Bool) →o Theory (Property Nat Bool) where
  toFun T := (T ∪ {zeroProperty}) ∪ {p | p = boundProperty ∧ zeroProperty ∈ T}
  monotone' := by
    intro A B h p hp
    rcases hp with (hp | hp) | ⟨hp, hz⟩
    · exact Or.inl (Or.inl (h hp))
    · exact Or.inl (Or.inr hp)
    · exact Or.inr ⟨hp, h hz⟩

theorem seed_positive : zeroProperty ∈ leastTheory rule := by
  rw [← leastTheory_fixed rule]
  exact Or.inl (Or.inr rfl)

theorem derived_positive : boundProperty ∈ leastTheory rule := by
  rw [← leastTheory_fixed rule]
  exact Or.inr ⟨rfl, seed_positive⟩

theorem rule_safe (w : Bool) :
    rule (truthTheory 0 w) ⊆ truthTheory 0 w := by
  intro p hp
  rcases hp with (hp | hp) | ⟨hp, _⟩
  · exact hp
  · have he : p = zeroProperty := hp
    subst p
    rfl
  · subst p
    exact Nat.zero_le 1

theorem unique_recursive_target (x : Nat) (w : Bool) :
    godlike (recursivePos (fun _ => rule)) x w ↔ x = 0 := by
  constructor
  · intro hg
    exact hg zeroProperty seed_positive
  · intro hx
    subst x
    exact recursive_target_godlike (fun _ => rule) (fun _ => 0) rule_safe w

theorem example_coherent (w : Bool) :
    Coherent negProperty (leastTheory rule) :=
  recursive_target_coherent (fun _ => rule) (fun _ => 0) rule_safe w

/-- Every initial state, under either distinct toy law, reaches the unique
recursively God-like target within its initial natural-number rank. -/
theorem example_emergence (x : Nat) (w : Bool) :
    box Models.access (fun z => ∃ n, n ≤ x ∧
      godlike (recursivePos (fun _ => rule)) (orbit (Models.toyLaw z) n x) z) w := by
  intro z _
  obtain ⟨n, hn, he⟩ := Models.toy_laws_reach z x
  exact ⟨n, hn, (unique_recursive_target _ z).mpr he⟩

end Emergent.SeededExample
