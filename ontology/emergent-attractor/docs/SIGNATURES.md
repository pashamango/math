# Theorem signatures

Exact explicit signatures extracted from the seven source modules. Definitions, implicit type parameters and namespace variables must also be read in the linked source. This inventory does not replace Lean elaboration or audit.

## `Emergent.recursive_target_godlike`

Source: [Emergent/Bridge.lean](../Emergent/Bridge.lean)

```lean
theorem recursive_target_godlike {E : Type v} {W : Type u}
    (F : W → Theory (Property E W) →o Theory (Property E W)) (a : W → E)
    (safe : ∀ w, F w (truthTheory (a w) w) ⊆ truthTheory (a w) w) (w : W) :
    godlike (recursivePos F) (a w) w
```

## `Emergent.recursive_target_coherent`

Source: [Emergent/Bridge.lean](../Emergent/Bridge.lean)

```lean
theorem recursive_target_coherent {E : Type v} {W : Type u}
    (F : W → Theory (Property E W) →o Theory (Property E W)) (a : W → E)
    (safe : ∀ w, F w (truthTheory (a w) w) ⊆ truthTheory (a w) w) (w : W) :
    Coherent negProperty (leastTheory (F w))
```

## `Emergent.recursive_modal_emergence`

Source: [Emergent/Bridge.lean](../Emergent/Bridge.lean)

```lean
theorem recursive_modal_emergence {E : Type v} {W : Type u} (R : W → W → Prop)
    (F : W → Theory (Property E W) →o Theory (Property E W))
    (f : W → E → E) (a : W → E) (rank : W → E → Nat)
    (safe : ∀ w, F w (truthTheory (a w) w) ⊆ truthTheory (a w) w)
    (zero : ∀ w x, rank w x = 0 → x = a w)
    (decrease : ∀ w x, x ≠ a w → rank w (f w x) < rank w x)
    (x : W → E) (w : W) :
    box R (fun z => ∃ n, n ≤ rank z (x z) ∧
      godlike (recursivePos F) (orbit (f z) n (x z)) z) w
```

## `Emergent.recursive_emergence_persists`

Source: [Emergent/Bridge.lean](../Emergent/Bridge.lean)

```lean
theorem recursive_emergence_persists {E : Type v} {W : Type u}
    (F : W → Theory (Property E W) →o Theory (Property E W))
    (f : W → E → E) (a : W → E)
    (safe : ∀ w, F w (truthTheory (a w) w) ⊆ truthTheory (a w) w)
    (fixed : ∀ w, f w (a w) = a w) (w : W) (x : E) (n : Nat)
    (hit : orbit (f w) n x = a w) (m : Nat) :
    godlike (recursivePos F) (orbit (f w) (m + n) x) w
```

## `Emergent.orbit_fixed`

Source: [Emergent/Dynamics.lean](../Emergent/Dynamics.lean)

```lean
theorem orbit_fixed {S : Type u} {f : S → S} {a : S}
    (hf : f a = a) (n : Nat) : orbit f n a = a
```

## `Emergent.orbit_add`

Source: [Emergent/Dynamics.lean](../Emergent/Dynamics.lean)

```lean
theorem orbit_add {S : Type u} (f : S → S) (m n : Nat) (x : S) :
    orbit f (m + n) x = orbit f m (orbit f n x)
```

## `Emergent.rank_reaches`

Source: [Emergent/Dynamics.lean](../Emergent/Dynamics.lean)

```lean
theorem rank_reaches {S : Type u} (f : S → S) (a : S) (rank : S → Nat)
    (zero : ∀ x, rank x = 0 → x = a)
    (decrease : ∀ x, x ≠ a → rank (f x) < rank x) (x : S) :
    ∃ n, n ≤ rank x ∧ orbit f n x = a
```

## `Emergent.reaches_persists`

Source: [Emergent/Dynamics.lean](../Emergent/Dynamics.lean)

```lean
theorem reaches_persists {S : Type u} {f : S → S} {a x : S}
    (hf : f a = a) {n : Nat} (hn : orbit f n x = a) (m : Nat) :
    orbit f (m + n) x = a
```

## `Emergent.modal_rank_reaches`

Source: [Emergent/Dynamics.lean](../Emergent/Dynamics.lean)

```lean
theorem modal_rank_reaches {W : Type u} {S : Type v} (R : W → W → Prop)
    (f : W → S → S) (a : W → S) (rank : W → S → Nat)
    (zero : ∀ w x, rank w x = 0 → x = a w)
    (decrease : ∀ w x, x ≠ a w → rank w (f w x) < rank w x)
    (x : W → S) (w : W) : box R (fun z => Reaches (f z) (a z) (x z)) w
```

## `Emergent.fixed_does_not_imply_attraction`

Source: [Emergent/Dynamics.lean](../Emergent/Dynamics.lean)

```lean
theorem fixed_does_not_imply_attraction :
    (id : Bool → Bool) false = false ∧ ¬ Reaches id false true
```

## `Emergent.complement_no_fixed`

Source: [Emergent/Dynamics.lean](../Emergent/Dynamics.lean)

```lean
theorem complement_no_fixed : ¬ ∃ x : Bool, (!x) = x
```

## `Emergent.contraction_attractor`

Source: [Emergent/Enrichment.lean](../Emergent/Enrichment.lean)

```lean
theorem contraction_attractor {S : Type u} [MetricSpace S] [CompleteSpace S] [Nonempty S]
    (f : S → S) {K : NNReal} (hf : ContractingWith K f) :
    ∃ a, Function.IsFixedPt f a ∧
      (∀ x, Tendsto (fun n : ℕ => f^[n] x) atTop (𝓝 a)) ∧
      (∀ b, Function.IsFixedPt f b → b = a)
```

## `Emergent.contraction_eventual_neighborhood`

Source: [Emergent/Enrichment.lean](../Emergent/Enrichment.lean)

```lean
theorem contraction_eventual_neighborhood {S : Type u} [MetricSpace S]
    [CompleteSpace S] [Nonempty S] (f : S → S) {K : NNReal}
    (hf : ContractingWith K f) (x : S) (epsilon : ℝ) (he : 0 < epsilon) :
    ∃ N : ℕ, ∀ n, N ≤ n → dist (f^[n] x) (hf.fixedPoint f) < epsilon
```

## `Emergent.quarterTurn_squared`

Source: [Emergent/Enrichment.lean](../Emergent/Enrichment.lean)

```lean
theorem quarterTurn_squared (z : ℂ) : quarterTurn (quarterTurn z) = -z
```

## `Emergent.quarterTurn_four`

Source: [Emergent/Enrichment.lean](../Emergent/Enrichment.lean)

```lean
theorem quarterTurn_four (z : ℂ) :
    quarterTurn (quarterTurn (quarterTurn (quarterTurn z))) = z
```

## `Emergent.almost_sure_not_necessary`

Source: [Emergent/Enrichment.lean](../Emergent/Enrichment.lean)

```lean
theorem almost_sure_not_necessary :
    (Measure.dirac (0 : ℕ)) {n : ℕ | n = 0} = 1 ∧
    ¬ box (fun (_ _ : ℕ) => True) (fun n => n = 0) 0
```

## `Emergent.positive_possible`

Source: [Emergent/Modal.lean](../Emergent/Modal.lean)

```lean
theorem positive_possible (A : ScottAxioms R actual P) (p : Property E W) (w : W)
    (hp : P p w) : diamond R (fun z => ∃ x, actual x z ∧ p x z) w
```

## `Emergent.possible_godlike`

Source: [Emergent/Modal.lean](../Emergent/Modal.lean)

```lean
theorem possible_godlike (A : ScottAxioms R actual P) (w : W) :
    diamond R (fun z => ∃ x, actual x z ∧ godlike P x z) w
```

## `Emergent.witnessed_positive`

Source: [Emergent/Modal.lean](../Emergent/Modal.lean)

```lean
theorem witnessed_positive (A : ScottAxioms R actual P) {x : E} {w : W}
    (hg : godlike P x w) {p : Property E W} (hp : p x w) : P p w
```

## `Emergent.godlike_essence`

Source: [Emergent/Modal.lean](../Emergent/Modal.lean)

```lean
theorem godlike_essence (A : ScottAxioms R actual P) {x : E} {w : W}
    (hg : godlike P x w) : essence R actual (godlike P) x w
```

## `Emergent.scott_necessary`

Source: [Emergent/Modal.lean](../Emergent/Modal.lean)

```lean
theorem scott_necessary (A : ScottAxioms R actual P)
    (symm : ∀ w z, R w z → R z w) (w : W) :
    box R (fun z => ∃ x, actual x z ∧ godlike P x z) w
```

## `Emergent.scott_modal_collapse`

Source: [Emergent/Modal.lean](../Emergent/Modal.lean)

```lean
theorem scott_modal_collapse (A : ScottAxioms R actual P)
    (symm : ∀ w z, R w z → R z w) (q : MProp W) (w : W) : q w → box R q w
```

## `Emergent.scott_unique`

Source: [Emergent/Modal.lean](../Emergent/Modal.lean)

```lean
theorem scott_unique (A : ScottAxioms R actual P) {x y : E} {w : W}
    (hx : godlike P x w) (hy : godlike P y w) : x = y
```

## `Emergent.principal_godlike`

Source: [Emergent/Modal.lean](../Emergent/Modal.lean)

```lean
theorem principal_godlike (g : E) (w : W) : godlike (principal g) g w
```

## `Emergent.principal_characterization`

Source: [Emergent/Modal.lean](../Emergent/Modal.lean)

```lean
theorem principal_characterization (g x : E) (w : W) :
    godlike (principal g) x w ↔ x = g
```

## `Emergent.principal_complement`

Source: [Emergent/Modal.lean](../Emergent/Modal.lean)

```lean
theorem principal_complement (g : E) (p : Property E W) (w : W) :
    (¬ principal g p w) ↔ principal g (negProperty p) w
```

## `Emergent.principal_witnessed`

Source: [Emergent/Modal.lean](../Emergent/Modal.lean)

```lean
theorem principal_witnessed (g : E) (p : Property E W) (w : W) :
    p g w ↔ principal g p w
```

## `Emergent.principal_ultrafilter`

Source: [Emergent/Modal.lean](../Emergent/Modal.lean)

```lean
theorem principal_ultrafilter (g : E) (w : W) : LocalUltrafilter (principal g) w
```

## `Emergent.all_positive_has_no_godlike`

Source: [Emergent/Modal.lean](../Emergent/Modal.lean)

```lean
theorem all_positive_has_no_godlike (w : W)
    (hall : ∀ p, P p w) : ¬ ∃ x, godlike P x w
```

## `Emergent.Models.model_ultrafilter`

Source: [Emergent/Models.lean](../Emergent/Models.lean)

```lean
theorem model_ultrafilter (w : Bool) : LocalUltrafilter positivity w
```

## `Emergent.Models.model_godlike`

Source: [Emergent/Models.lean](../Emergent/Models.lean)

```lean
theorem model_godlike (w : Bool) : godlike positivity false w
```

## `Emergent.Models.model_global_attraction`

Source: [Emergent/Models.lean](../Emergent/Models.lean)

```lean
theorem model_global_attraction (w x : Bool) : Reaches (step w) false x
```

## `Emergent.Models.model_no_collapse`

Source: [Emergent/Models.lean](../Emergent/Models.lean)

```lean
theorem model_no_collapse : contingent false ∧ ¬ box access contingent false
```

## `Emergent.Models.model_positivity_not_rigid`

Source: [Emergent/Models.lean](../Emergent/Models.lean)

```lean
theorem model_positivity_not_rigid :
    ∃ p : Property Bool Bool, positivity p false ∧ ¬ box access (positivity p) false
```

## `Emergent.Models.model_no_necessary_actuality`

Source: [Emergent/Models.lean](../Emergent/Models.lean)

```lean
theorem model_no_necessary_actuality :
    ¬ box access (fun w => ∃ x, actual x w ∧ godlike positivity x w) false
```

## `Emergent.Models.attraction_ultrafilter_countermodel`

Source: [Emergent/Models.lean](../Emergent/Models.lean)

```lean
theorem attraction_ultrafilter_countermodel :
    (∀ w, LocalUltrafilter positivity w) ∧
    (∀ w, godlike positivity false w) ∧
    (∀ w x, Reaches (step w) false x) ∧
    ¬ box access contingent false ∧
    ¬ box access (fun w => ∃ x, actual x w ∧ godlike positivity x w) false
```

## `Emergent.Models.future_realization_without_present`

Source: [Emergent/Models.lean](../Emergent/Models.lean)

```lean
theorem future_realization_without_present :
    (∀ x : Bool, realized (orbit (step false) 1 x) 1) ∧
    ¬ ∃ x : Bool, realized x 0
```

## `Emergent.Models.toy_laws_distinct`

Source: [Emergent/Models.lean](../Emergent/Models.lean)

```lean
theorem toy_laws_distinct : toyLaw false 2 ≠ toyLaw true 2
```

## `Emergent.Models.toy_laws_reach`

Source: [Emergent/Models.lean](../Emergent/Models.lean)

```lean
theorem toy_laws_reach (fast : Bool) (x : Nat) :
    ∃ n, n ≤ x ∧ orbit (toyLaw fast) n x = 0
```

## `Emergent.Models.scott_singleton_model`

Source: [Emergent/Models.lean](../Emergent/Models.lean)

```lean
theorem scott_singleton_model : ScottAxioms singletonR singletonActual singletonPos
```

## `Emergent.SeededExample.seed_positive`

Source: [Emergent/SeededExample.lean](../Emergent/SeededExample.lean)

```lean
theorem seed_positive : zeroProperty ∈ leastTheory rule
```

## `Emergent.SeededExample.derived_positive`

Source: [Emergent/SeededExample.lean](../Emergent/SeededExample.lean)

```lean
theorem derived_positive : boundProperty ∈ leastTheory rule
```

## `Emergent.SeededExample.rule_safe`

Source: [Emergent/SeededExample.lean](../Emergent/SeededExample.lean)

```lean
theorem rule_safe (w : Bool) :
    rule (truthTheory 0 w) ⊆ truthTheory 0 w
```

## `Emergent.SeededExample.unique_recursive_target`

Source: [Emergent/SeededExample.lean](../Emergent/SeededExample.lean)

```lean
theorem unique_recursive_target (x : Nat) (w : Bool) :
    godlike (recursivePos (fun _ => rule)) x w ↔ x = 0
```

## `Emergent.SeededExample.example_coherent`

Source: [Emergent/SeededExample.lean](../Emergent/SeededExample.lean)

```lean
theorem example_coherent (w : Bool) :
    Coherent negProperty (leastTheory rule)
```

## `Emergent.SeededExample.example_emergence`

Source: [Emergent/SeededExample.lean](../Emergent/SeededExample.lean)

```lean
theorem example_emergence (x : Nat) (w : Bool) :
    box Models.access (fun z => ∃ n, n ≤ x ∧
      godlike (recursivePos (fun _ => rule)) (orbit (Models.toyLaw z) n x) z) w
```

## `Emergent.leastTheory_fixed`

Source: [Emergent/SelfReference.lean](../Emergent/SelfReference.lean)

```lean
theorem leastTheory_fixed {α : Type u} (F : Theory α →o Theory α) :
    F (leastTheory F) = leastTheory F
```

## `Emergent.leastTheory_minimal`

Source: [Emergent/SelfReference.lean](../Emergent/SelfReference.lean)

```lean
theorem leastTheory_minimal {α : Type u} (F : Theory α →o Theory α)
    (T : Theory α) (hT : F T = T) : leastTheory F ⊆ T
```

## `Emergent.seed_survives`

Source: [Emergent/SelfReference.lean](../Emergent/SelfReference.lean)

```lean
theorem seed_survives {α : Type u} (F : Theory α →o Theory α) (seed : Theory α)
    (hseed : ∀ T, seed ⊆ F T) : seed ⊆ leastTheory F
```

## `Emergent.leastTheory_coherent`

Source: [Emergent/SelfReference.lean](../Emergent/SelfReference.lean)

```lean
theorem leastTheory_coherent {α : Type u} (F : Theory α →o Theory α)
    (complement : α → α) (safe : Theory α)
    (hsafe : F safe ⊆ safe) (hcoherent : Coherent complement safe) :
    Coherent complement (leastTheory F)
```

## `Emergent.identity_extremes`

Source: [Emergent/SelfReference.lean](../Emergent/SelfReference.lean)

```lean
theorem identity_extremes {α : Type u} :
    leastTheory (OrderHom.id : Theory α →o Theory α) = ∅ ∧
    greatestTheory (OrderHom.id : Theory α →o Theory α) = Set.univ
```

## `Emergent.identity_not_unique`

Source: [Emergent/SelfReference.lean](../Emergent/SelfReference.lean)

```lean
theorem identity_not_unique :
    leastTheory (OrderHom.id : Theory Bool →o Theory Bool) ≠
    greatestTheory (OrderHom.id : Theory Bool →o Theory Bool)
```

## `Emergent.leastTheory_symmetry`

Source: [Emergent/SelfReference.lean](../Emergent/SelfReference.lean)

```lean
theorem leastTheory_symmetry {α : Type u} (F : Theory α →o Theory α)
    (e : Theory α ≃o Theory α) (commutes : ∀ T, e (F T) = F (e T)) :
    e (leastTheory F) = leastTheory F
```
