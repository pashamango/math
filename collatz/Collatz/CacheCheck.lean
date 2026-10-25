import Collatz.PhiWords
import Collatz.ReverseTree

namespace Collatz.CacheCheck

open Collatz.ReverseTree

/-- A node provably reachable from 1 in the reverse Collatz tree.
    Both S and τ are defined exclusively on CertifiedNode instances,
    preventing any unconditional claims about arbitrary natural numbers. -/
structure CertifiedNode where
  val : Nat
  pos : val > 0
  reachable : ReachableFromOne val

/-- Canonical root node at 1. -/
def rootNode : CertifiedNode := {
  val := 1
  pos := Nat.zero_lt_one
  reachable := ReachableFromOne.one
}

theorem rootNode_val : rootNode.val = 1 := rfl

/-- Scale function S on provably reachable nodes.
    Defined as the positive integer magnitude. -/
def S (node : CertifiedNode) : Nat :=
  node.val

theorem S_pos (node : CertifiedNode) : S node > 0 := node.pos

theorem S_root : S rootNode = 1 := rfl

/-- Reciprocal coordinate τ = 1 / S on provably reachable nodes.
    Represented as a certified unit fraction with denominator S.
    NOTE: τ is an algebraic coordinate on certified nodes; it is NOT
    a divergence detector for general unproven orbits. -/
structure Tau where
  den : Nat
  den_pos : den > 0
  reachable : ReachableFromOne den
  deriving DecidableEq, Repr

def tau (node : CertifiedNode) : Tau := {
  den := S node
  den_pos := S_pos node
  reachable := node.reachable
}

theorem tau_den (node : CertifiedNode) : (tau node).den = S node := rfl

/-- Partial order on unit fractions: 1 / d1 ≤ 1 / d2 ↔ d2 ≤ d1. -/
def tau_le (t1 t2 : Tau) : Prop := t2.den ≤ t1.den

/-- Strict order on unit fractions: 1 / d1 < 1 / d2 ↔ d2 < d1. -/
def tau_lt (t1 t2 : Tau) : Prop := t2.den < t1.den

theorem tau_le_refl (t : Tau) : tau_le t t := Nat.le_refl t.den

theorem tau_le_trans {t1 t2 t3 : Tau} (h12 : tau_le t1 t2) (h23 : tau_le t2 t3) :
    tau_le t1 t3 :=
  Nat.le_trans h23 h12

theorem tau_root : tau rootNode = ⟨1, Nat.zero_lt_one, ReachableFromOne.one⟩ := rfl

/-- Monotonicity of τ upon descent to 1 as a lemma of S:
    When scale S decreases, τ increases monotonically. -/
theorem tau_mono_of_S_descent (u v : CertifiedNode) (h : S v ≤ S u) :
    tau_le (tau u) (tau v) := h

theorem tau_strict_mono_of_S_descent (u v : CertifiedNode) (h : S v < S u) :
    tau_lt (tau u) (tau v) := h

theorem tau_le_root (node : CertifiedNode) : tau_le (tau node) (tau rootNode) := by
  dsimp [tau_le, tau, rootNode, S]
  exact node.pos

theorem tau_eq_root_iff (node : CertifiedNode) : tau node = tau rootNode ↔ node.val = 1 := by
  constructor
  · intro h
    have hden : (tau node).den = (tau rootNode).den := by rw [h]
    dsimp [tau, S, rootNode] at hden
    exact hden
  · intro h
    cases node with | mk val pos reach =>
    dsimp [tau, S, rootNode]
    subst h
    rfl

/-- Single step verifier for reverse tree path.
    Runs in O(1) arithmetic operations. -/
def checkStep (curr : Nat) (s : Step) : Option Nat :=
  match s with
  | Step.s0 => some (g0 curr)
  | Step.s1 => if curr % 6 = 4 then some (g1 curr) else none

theorem checkStep_eq_evalStep (curr : Nat) (s : Step) :
    checkStep curr s = evalStep curr s := rfl

/-- Linear path checker:
    Verifies a sequence of reverse tree steps starting from a given node.
    Runs in exactly O(|w|) operations where |w| is the length of path w. -/
def verifyPath (start : Nat) : Path → Option Nat
  | [] => some start
  | s :: rest =>
    match checkStep start s with
    | some next => verifyPath next rest
    | none => none

theorem verifyPath_eq_evalPath (start : Nat) (p : Path) :
    verifyPath start p = evalPath start p := by
  induction p generalizing start with
  | nil => rfl
  | cons s rest ih =>
    dsimp [verifyPath, evalPath]
    rw [checkStep_eq_evalStep]
    cases evalStep start s with
    | none => rfl
    | some next => exact ih next

/-- Soundness of the linear path checker:
    Any integer accepted by verifyPath starting from 1 is provably reachable. -/
theorem verifyPath_sound (p : Path) {n : Nat} (h : verifyPath 1 p = some n) :
    ReachableFromOne n := by
  rw [verifyPath_eq_evalPath] at h
  exact evalPath_sound h

theorem verifyPath_pos (p : Path) {n : Nat} (h : verifyPath 1 p = some n) :
    n > 0 :=
  reachable_pos (verifyPath_sound p h)

/-- The step count complexity of verifyPath is strictly proportional to path length.
    NOTE: Linear certificate verification is an elementary property of explicit witnesses
    and implies nothing regarding P vs NP. -/
def stepsCount (p : Path) : Nat := p.length

theorem verifyPath_step_bound (p : Path) : stepsCount p = p.length := rfl

/-- Cache entry associating a natural number with a claimed reachability path witness. -/
structure CacheEntry where
  val : Nat
  path : Path
  deriving Repr

/-- Consistency check for a cache entry: verifies that path witness evaluates to val. -/
def isCacheEntryValid (entry : CacheEntry) : Bool :=
  match verifyPath 1 entry.path with
  | some res => res == entry.val
  | none => false

/-- Certified construction of a node admitted by the cache guard. -/
theorem cache_guard_sound (entry : CacheEntry) (h : isCacheEntryValid entry = true) :
    ReachableFromOne entry.val := by
  dsimp [isCacheEntryValid] at h
  split at h
  · rename_i res hres
    have h_eq : res = entry.val := beq_iff_eq.mp h
    have hreach := verifyPath_sound entry.path hres
    rw [← h_eq]
    exact hreach
  · contradiction

theorem cache_entry_pos (entry : CacheEntry) (h : isCacheEntryValid entry = true) :
    entry.val > 0 :=
  reachable_pos (cache_guard_sound entry h)

/-- Construction of a CertifiedNode from any cache entry passing the guard. -/
def certifiedOfCacheEntry (entry : CacheEntry) (h : isCacheEntryValid entry = true) :
    CertifiedNode := {
  val := entry.val
  pos := cache_entry_pos entry h
  reachable := cache_guard_sound entry h
}

theorem certifiedOfCacheEntry_val (entry : CacheEntry) (h : isCacheEntryValid entry = true) :
    (certifiedOfCacheEntry entry h).val = entry.val := rfl

/-- Consistency guard for an entire cache collection:
    Every admitted entry must pass the linear path checker. -/
def checkCache (entries : List CacheEntry) : Bool :=
  entries.all isCacheEntryValid

theorem checkCache_sound (entries : List CacheEntry) (h : checkCache entries = true) :
    ∀ entry ∈ entries, ReachableFromOne entry.val := by
  intro entry hentry
  dsimp [checkCache] at h
  rw [List.all_eq_true] at h
  have hvalid := h entry hentry
  exact cache_guard_sound entry hvalid

end Collatz.CacheCheck
