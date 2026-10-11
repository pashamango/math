import Mathlib.Order.FixedPoints

namespace Emergent
universe u

/-- Positivity as a set of property codes, not a claim about physical truth. -/
abbrev Theory (α : Type u) := Set α

noncomputable def leastTheory {α : Type u} (F : Theory α →o Theory α) : Theory α := F.lfp
noncomputable def greatestTheory {α : Type u} (F : Theory α →o Theory α) : Theory α := F.gfp

theorem leastTheory_fixed {α : Type u} (F : Theory α →o Theory α) :
    F (leastTheory F) = leastTheory F := F.map_lfp

theorem leastTheory_minimal {α : Type u} (F : Theory α →o Theory α)
    (T : Theory α) (hT : F T = T) : leastTheory F ⊆ T := F.lfp_le_fixed hT

theorem seed_survives {α : Type u} (F : Theory α →o Theory α) (seed : Theory α)
    (hseed : ∀ T, seed ⊆ F T) : seed ⊆ leastTheory F := by
  rw [← leastTheory_fixed F]
  exact hseed _

/-- Coherence is a separate invariant, not a consequence of monotonicity. -/
def Coherent {α : Type u} (complement : α → α) (T : Theory α) : Prop :=
  ∀ p, p ∈ T → complement p ∉ T

theorem leastTheory_coherent {α : Type u} (F : Theory α →o Theory α)
    (complement : α → α) (safe : Theory α)
    (hsafe : F safe ⊆ safe) (hcoherent : Coherent complement safe) :
    Coherent complement (leastTheory F) := by
  have hbound : leastTheory F ⊆ safe := F.lfp_le hsafe
  intro p hp hcp
  exact hcoherent p (hbound hp) (hbound hcp)

/-- Identity recursion leaves all candidates fixed; it selects no unique theory. -/
theorem identity_extremes {α : Type u} :
    leastTheory (OrderHom.id : Theory α →o Theory α) = ∅ ∧
    greatestTheory (OrderHom.id : Theory α →o Theory α) = Set.univ := by
  constructor
  · apply Set.Subset.antisymm
    · exact (OrderHom.id : Theory α →o Theory α).lfp_le (by rfl)
    · exact Set.empty_subset _
  · apply Set.Subset.antisymm
    · exact Set.subset_univ _
    · exact (OrderHom.id : Theory α →o Theory α).le_gfp (by rfl)

theorem identity_not_unique :
    leastTheory (OrderHom.id : Theory Bool →o Theory Bool) ≠
    greatestTheory (OrderHom.id : Theory Bool →o Theory Bool) := by
  obtain ⟨hl, hg⟩ := identity_extremes (α := Bool)
  rw [hl, hg]
  intro h
  have : false ∈ (∅ : Set Bool) := h.symm ▸ Set.mem_univ false
  exact this

/-- Symmetry of the rule operator transfers to its selected least fixed point.
This is an order-symmetry result; geometric fractality requires further data. -/
theorem leastTheory_symmetry {α : Type u} (F : Theory α →o Theory α)
    (e : Theory α ≃o Theory α) (commutes : ∀ T, e (F T) = F (e T)) :
    e (leastTheory F) = leastTheory F := by
  change e F.lfp = F.lfp
  have hforward : F (e F.lfp) = e F.lfp := by
    rw [← commutes, F.map_lfp]
  have hback : F (e.symm F.lfp) = e.symm F.lfp := by
    apply e.injective
    rw [commutes, e.apply_symm_apply, F.map_lfp]
  apply le_antisymm
  · simpa only [e.apply_symm_apply] using e.monotone (F.lfp_le_fixed hback)
  · exact F.lfp_le_fixed hforward

end Emergent
