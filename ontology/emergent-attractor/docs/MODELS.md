# Models, consistency, and vacuity

| Assumption set | Concrete witness | What this checks | What it does not check |
| --- | --- | --- | --- |
| Full five Scott hypotheses | `Models.scott_singleton_model`: `Unit` worlds/entities, universal accessibility and actuality | An explicit interpretation inhabits that hypothesis structure (relative to Lean foundations) | Nontrivial temporal worlds, physical existence, or absolute consistency of Lean |
| Local ultrafilter + God-like target + global finite attraction | `Models.attraction_ultrafilter_countermodel`: two Boolean worlds | These properties coexist with failure of necessary actuality and modal collapse | This model does not satisfy the full Scott hypotheses; positivity rigidity explicitly fails |
| Recursive seeded positivity + finite hitting | `SeededExample.rule`, `rule_safe`, `example_emergence` on natural numbers | An explicit nonconstant closure rule derives `x≤1` from `x=0`; two different toy laws reach target 0 | Target is selected by the seed; no recovery of an unspecified deity |
| Future but no present realization | `Models.future_realization_without_present` | Distinction is consistent in the toy definition | Predicate deliberately requires `0<t`; it does not explain why the real world should obey it |
| Arbitrary monotone operator | Identity on sets | Least and greatest fixed points differ | Monotonicity alone supplies neither uniqueness nor nonempty positivity |
| Fixed point without attraction | Identity on Boolean states | A different state never reaches the fixed point | Fixedness alone is insufficient |

For general `box R q w`, an empty successor set makes the universal statement vacuous. The bridge hypotheses hold at all worlds and a separate rank theorem gives per-world reachability, but the boxed syntax itself does not assert a successor. `possible_godlike` rules out successor-free worlds when the full Scott hypotheses hold.

`Coherent complement T` means no member occurs together with its chosen complement. It is not a general deductive consistency theorem for arbitrary rule operators/languages. A closed theory may be empty unless a seed or other inhabitance argument is supplied.

A natural rank gives finite hitting, including zero steps; an absorbing target is a separate condition. Metric contraction gives asymptotic convergence, not necessarily hitting. Modal accessibility has no built-in temporal direction. Review also notes that full Scott hypotheses plus symmetric accessibility force accessibility to the diagonal in this unrestricted shallow embedding: instantiate collapse with `q(t) := t=w`, then use possible existence for seriality. This is a review inference from the checked theorems, not an added Lean declaration.
