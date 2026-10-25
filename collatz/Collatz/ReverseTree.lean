import Collatz.PhiWords

namespace Collatz.ReverseTree

/-- Reverse Collatz operator G0: doubles the number (forward step was n / 2). -/
def g0 (n : Nat) : Nat := 2 * n

/-- Validity condition for reverse Collatz operator G1:
    in forward Collatz, odd m produces 3m + 1. Since m is odd, 3m + 1 is congruent to 4 mod 6.
    Therefore, G1 is valid if and only if n ≡ 4 mod 6. -/
def isG1Valid (n : Nat) : Prop := n % 6 = 4

/-- Reverse Collatz operator G1: (n - 1) / 3 when n ≡ 4 mod 6. -/
def g1 (n : Nat) : Nat := (n - 1) / 3

theorem g0_pos {n : Nat} (hn : n > 0) : g0 n > 0 := by
  dsimp [g0]
  omega

theorem g1_pos {n : Nat} (hmod : isG1Valid n) : g1 n > 0 := by
  dsimp [isG1Valid] at hmod
  dsimp [g1]
  omega

theorem g1_odd {n : Nat} (hmod : isG1Valid n) : g1 n % 2 = 1 := by
  dsimp [isG1Valid] at hmod
  dsimp [g1]
  omega

theorem g1_mod6_ne_4 {n : Nat} (hmod : isG1Valid n) : ¬ isG1Valid (g1 n) := by
  intro h
  have h_odd := g1_odd hmod
  dsimp [isG1Valid] at h
  omega

theorem g1_consecutive_invalid {n : Nat} (hmod : isG1Valid n) : ¬ isG1Valid (g1 n) :=
  g1_mod6_ne_4 hmod

/-- Inductive predicate characterizing integers provably reachable from 1
    via valid reverse Collatz transitions. -/
inductive ReachableFromOne : Nat → Prop where
  | one : ReachableFromOne 1
  | step_g0 {n : Nat} (h : ReachableFromOne n) : ReachableFromOne (g0 n)
  | step_g1 {n : Nat} (h : ReachableFromOne n) (hmod : isG1Valid n) : ReachableFromOne (g1 n)

theorem reachable_pos {n : Nat} (h : ReachableFromOne n) : n > 0 := by
  induction h with
  | one => exact Nat.zero_lt_one
  | step_g0 _ ih => exact g0_pos ih
  | step_g1 _ hmod _ => exact g1_pos hmod

/-- Atomic reverse transition step. -/
inductive Step where
  | s0 : Step
  | s1 : Step
  deriving DecidableEq, Repr

abbrev Path := List Step

/-- Single step evaluation in reverse tree. -/
def evalStep (curr : Nat) (s : Step) : Option Nat :=
  match s with
  | Step.s0 => some (g0 curr)
  | Step.s1 => if curr % 6 = 4 then some (g1 curr) else none

/-- Path evaluation starting from a given node. -/
def evalPath : Nat → Path → Option Nat
  | curr, [] => some curr
  | curr, s :: rest =>
    match evalStep curr s with
    | some next => evalPath next rest
    | none => none

def validPath (curr : Nat) (p : Path) : Prop :=
  (evalPath curr p).isSome

theorem evalPath_sound_general (p : Path) :
    ∀ (start n : Nat), ReachableFromOne start → evalPath start p = some n → ReachableFromOne n := by
  induction p with
  | nil =>
    intro start n hreach h
    dsimp [evalPath] at h
    cases h
    exact hreach
  | cons st rest ih =>
    intro start n hreach h
    dsimp [evalPath] at h
    cases hs : evalStep start st with
    | none =>
      rw [hs] at h
      contradiction
    | some next =>
      rw [hs] at h
      have hreach_next : ReachableFromOne next := by
        cases st with
        | s0 =>
          dsimp [evalStep] at hs
          injection hs with hs_eq
          rw [← hs_eq]
          exact ReachableFromOne.step_g0 hreach
        | s1 =>
          dsimp [evalStep] at hs
          split at hs
          · rename_i hmod
            injection hs with hs_eq
            rw [← hs_eq]
            exact ReachableFromOne.step_g1 hreach hmod
          · contradiction
      exact ih next n hreach_next h

theorem evalPath_sound {p : Path} {n : Nat} (h : evalPath 1 p = some n) :
    ReachableFromOne n :=
  evalPath_sound_general p 1 n ReachableFromOne.one h

/-- Conversion of reverse tree path to word in alphabet {0, 1}. -/
def stepToBool : Step → Bool
  | Step.s0 => false
  | Step.s1 => true

def pathToWord (p : Path) : Collatz.PhiWords.Word :=
  p.map stepToBool

theorem pathToWord_nil : pathToWord [] = [] := rfl

theorem pathToWord_cons (s : Step) (p : Path) :
    pathToWord (s :: p) = stepToBool s :: pathToWord p := rfl

theorem pathToWord_injective : ∀ (p1 p2 : Path), pathToWord p1 = pathToWord p2 → p1 = p2
  | [], [] => fun _ => rfl
  | [], _ :: _ => fun h => by contradiction
  | _ :: _, [] => fun h => by contradiction
  | st1 :: r1, st2 :: r2 => by
    intro h
    dsimp [pathToWord] at h
    injection h with hs hr
    have hs_eq : st1 = st2 := by
      cases st1 <;> cases st2 <;> first | rfl | contradiction
    have hr_eq : r1 = r2 := pathToWord_injective r1 r2 hr
    rw [hs_eq, hr_eq]

/-- Predicate ensuring no consecutive s1 steps exist in a path. -/
def hasNoConsecutiveS1 : Path → Prop
  | [] => True
  | [_] => True
  | Step.s1 :: Step.s1 :: _ => False
  | Step.s0 :: s2 :: rest => hasNoConsecutiveS1 (s2 :: rest)
  | Step.s1 :: Step.s0 :: rest => hasNoConsecutiveS1 (Step.s0 :: rest)

theorem pathToWord_no11_of_hasNoConsecutiveS1 (p : Path) (h : hasNoConsecutiveS1 p) :
    Collatz.PhiWords.no11 (pathToWord p) := by
  induction p with
  | nil => exact Collatz.PhiWords.no11_nil
  | cons st1 rest ih =>
    cases rest with
    | nil =>
      cases st1
      · exact Collatz.PhiWords.no11_nil
      · exact Collatz.PhiWords.no11_one
    | cons st2 rest2 =>
      cases st1 <;> cases st2
      · dsimp [hasNoConsecutiveS1] at h
        have ih' := ih h
        change Collatz.PhiWords.no11 (false :: false :: pathToWord rest2)
        rw [Collatz.PhiWords.no11_false_cons]
        exact ih'
      · dsimp [hasNoConsecutiveS1] at h
        have ih' := ih h
        change Collatz.PhiWords.no11 (false :: true :: pathToWord rest2)
        rw [Collatz.PhiWords.no11_false_cons]
        exact ih'
      · dsimp [hasNoConsecutiveS1] at h
        have ih' := ih h
        change Collatz.PhiWords.no11 (true :: false :: pathToWord rest2)
        rw [Collatz.PhiWords.no11_true_false_cons]
        exact ih'
      · contradiction

theorem evalStep_s1_result_not_s1 (curr : Nat) (next : Nat) (h : evalStep curr Step.s1 = some next) :
    evalStep next Step.s1 = none := by
  dsimp [evalStep] at h
  split at h
  · rename_i hmod
    injection h with h_eq
    rw [← h_eq]
    have h_not : ¬ isG1Valid (g1 curr) := g1_consecutive_invalid hmod
    dsimp [evalStep]
    split
    · rename_i hmod_next
      exact False.elim (h_not hmod_next)
    · rfl
  · contradiction

theorem validPath_hasNoConsecutiveS1 (p : Path) :
    ∀ (curr : Nat), validPath curr p → hasNoConsecutiveS1 p := by
  induction p with
  | nil => intros; trivial
  | cons st1 rest ih =>
    cases rest with
    | nil => intros; trivial
    | cons st2 rest2 =>
      cases st1 <;> cases st2
      · intro curr h
        dsimp [hasNoConsecutiveS1]
        dsimp [validPath, evalPath] at h
        cases hs : evalStep curr Step.s0 with
        | none => rw [hs] at h; contradiction
        | some next =>
          rw [hs] at h
          apply ih next
          dsimp [validPath]
          exact h
      · intro curr h
        dsimp [hasNoConsecutiveS1]
        dsimp [validPath, evalPath] at h
        cases hs : evalStep curr Step.s0 with
        | none => rw [hs] at h; contradiction
        | some next =>
          rw [hs] at h
          apply ih next
          dsimp [validPath]
          exact h
      · intro curr h
        dsimp [hasNoConsecutiveS1]
        dsimp [validPath, evalPath] at h
        cases hs : evalStep curr Step.s1 with
        | none => rw [hs] at h; contradiction
        | some next =>
          rw [hs] at h
          apply ih next
          dsimp [validPath]
          exact h
      · intro curr h
        dsimp [validPath, evalPath] at h
        cases hs1 : evalStep curr Step.s1 with
        | none => rw [hs1] at h; contradiction
        | some next =>
          rw [hs1] at h
          have hs2 := evalStep_s1_result_not_s1 curr next hs1
          dsimp [evalPath] at h
          rw [hs2] at h
          contradiction

/-- Main structural lemma: every valid reverse Collatz tree path gives a word without 11. -/
theorem validPath_yields_no11 {curr : Nat} {p : Path} (h : validPath curr p) :
    Collatz.PhiWords.no11 (pathToWord p) :=
  pathToWord_no11_of_hasNoConsecutiveS1 p (validPath_hasNoConsecutiveS1 p curr h)

/-- Realizability characterization: the reverse map is NOT surjective onto no11 words.
    Specifically, [true] (corresponding to s1 from 1) satisfies no11, but is not realizable from 1
    because 1 % 6 = 1 ≠ 4. -/
theorem no11_not_all_realizable :
    ∃ (w : Collatz.PhiWords.Word), Collatz.PhiWords.no11 w ∧ ¬ ∃ (p : Path), pathToWord p = w ∧ validPath 1 p := by
  refine ⟨[true], Collatz.PhiWords.no11_one, ?_⟩
  intro ⟨p, hp_word, hp_valid⟩
  cases p with
  | nil => contradiction
  | cons st rest =>
    cases rest with
    | nil =>
      cases st with
      | s0 =>
        dsimp [pathToWord, stepToBool] at hp_word
        contradiction
      | s1 =>
        dsimp [validPath, evalPath, evalStep] at hp_valid
        revert hp_valid
        decide
    | cons st2 rest2 =>
      dsimp [pathToWord, stepToBool] at hp_word
      injection hp_word with _ hr
      contradiction

/-- Collatz conjecture formulated as an explicit hypothesis variable.
    No claim of proof: theorems conditioned on CollatzConjecture are strictly conditional. -/
def CollatzConjecture : Prop := ∀ (n : Nat), n > 0 → ReachableFromOne n

theorem conditional_reachable (hconjecture : CollatzConjecture) (n : Nat) (hn : n > 0) :
    ReachableFromOne n :=
  hconjecture n hn

end Collatz.ReverseTree
