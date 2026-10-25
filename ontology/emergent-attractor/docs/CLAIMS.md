# Claims and proof obligations

The package contains 53 conditional theorem declarations. The complete [signature inventory](SIGNATURES.md) gives all explicit hypotheses; source definitions and namespace variables remain authoritative. The axiom audit lists kernel foundation dependencies, not the truth of theorem arguments.

| Public claim | Main declarations | Required data / hypotheses | Supported conclusion and boundary |
| --- | --- | --- | --- |
| Scott-style modal existence | `positive_possible`, `possible_godlike`, `scott_necessary` | Five fields of `ScottAxioms`; symmetry of `R` for necessity | Existence relative to the supplied `actual`, `P`, and modal frame; no empirical identification of these with reality |
| Scott branch has modal collapse | `scott_modal_collapse` | Same full Scott hypotheses and symmetry | Every proposition true at a world is true at accessible worlds; this branch does not establish nontrivial directed future emergence |
| God-like witnesses are unique | `scott_unique` | Scott hypotheses and two godlike witnesses at one world | Equality of those witnesses; existence is not established by this theorem alone |
| Principal positivity has a witness | `principal_godlike`, `principal_characterization`, `principal_ultrafilter` | A supplied element `g`; positivity defined by truth at `g` | Definitional characterization, not discovery of an independently unknown being |
| Recursive positivity admits a coherent target | `recursive_target_godlike`, `recursive_target_coherent` | Monotone rule operator, supplied target `a`, safety inside its truth theory | Least closure contains only properties true at that target; coherence excludes complementary pairs |
| Recursive theories have fixed points | `leastTheory_fixed`, `leastTheory_minimal`, `seed_survives`, `leastTheory_coherent`, `leastTheory_symmetry` | Monotonicity; seed/safety/symmetry hypotheses where stated | Order-theoretic fixed points; not geometric fractality or unrestricted logical consistency |
| A rank gives bounded finite hitting | `rank_reaches`, `modal_rank_reaches`, `recursive_modal_emergence` | Natural-number rank; zero implies target; strict decrease away from target; safety for the bridge | At most initial rank many transitions to the supplied target; bound permits zero steps |
| Reached properties persist | `reaches_persists`, `recursive_emergence_persists` | Target is absorbing and already reached | Persistence requires the absorbing hypothesis; hitting alone does not supply it |
| A contraction has a unique attracting fixed point | `contraction_attractor`, `contraction_eventual_neighborhood` | Nonempty complete metric space; `ContractingWith K f` | Standard mathlib contraction results; asymptotic convergence need not be finite hitting |
| Several tempting bridges fail | `fixed_does_not_imply_attraction`, `identity_not_unique`, `attraction_ultrafilter_countermodel`, `almost_sure_not_necessary` | Explicit concrete examples | Fixedness, uniqueness, attraction, necessity and probability one are distinct |
| Future realization can differ from present realization | `future_realization_without_present` | Toy predicate `realized x t := x = false ∧ 0 < t` | A modeled possibility; future-only realization is encoded in the definition, not proved for the universe |
| A nonconstant recursive example is inhabited | `seed_positive`, `derived_positive`, `rule_safe`, `unique_recursive_target`, `example_coherent`, `example_emergence` | Explicit rule seeded with `x=0`; two specified toy laws | Rule reads its current theory; unique target 0 is encoded by the seed; no physical law claim |
| Complex quarter-turn has period dividing four | `quarterTurn_squared`, `quarterTurn_four` | Multiplication by `Complex.I` | Algebraic fact, not a modal or metaphysical bridge |
| Making every property positive fails | `all_positive_has_no_godlike` | Positivity includes every property, including False | No godlike witness; the essay's universal goodness cannot be read as this formal hypothesis |

## Status of the essay's two proposals

“God is in our future” motivates the temporal toy predicate and dynamics. No theorem identifies a mathematical state with a deity or proves a future-only deity. `R` is accessibility, not a time arrow.

“Goodness is God, and God is goodness” motivates principal and recursive positivity. Principal positivity chooses a witness in advance; recursive safety also supplies a target. Neither construction proves a unique moral semantics from self-reference alone. The identity-operator counterexample explicitly shows nonuniqueness.

The full Scott branch is a reference formalization, not a conjunction with all recursive/dynamical examples. Our proposed interpretive program remains a research question. Standard fixed-point/contraction results and Scott's argument are credited to their existing mathematical sources; no priority claim is made for them.
