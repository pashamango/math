namespace Collatz.PhiWords

abbrev Word := List Bool

def fib : Nat → Nat
  | 0 => 0
  | 1 => 1
  | n + 2 => fib (n + 1) + fib n

theorem fib_add_two (n : Nat) : fib (n + 2) = fib (n + 1) + fib n := rfl

def no11 : Word → Prop
  | [] => True
  | [_] => True
  | true :: true :: _ => False
  | false :: b2 :: rest => no11 (b2 :: rest)
  | true :: false :: rest => no11 (false :: rest)

theorem no11_nil : no11 [] := by trivial
theorem no11_one : no11 [true] := by trivial
theorem no11_two : no11 [true, false] := by trivial
theorem no11_three : no11 [true, false, false] := by trivial
theorem no11_four : no11 [true, false, true] := by trivial
theorem no11_five : no11 [true, false, false, false] := by trivial
theorem no11_six : no11 [true, false, false, true] := by trivial
theorem no11_seven : no11 [true, false, true, false] := by trivial
theorem no11_eight : no11 [true, false, false, false, false] := by trivial

theorem no11_false_cons (b2 : Bool) (rest : Word) :
    no11 (false :: b2 :: rest) ↔ no11 (b2 :: rest) := by
  cases b2 <;> rfl

theorem no11_true_false_cons (rest : Word) :
    no11 (true :: false :: rest) ↔ no11 (false :: rest) := by
  rfl

theorem no11_of_cons (b : Bool) (w : Word) (h : no11 (b :: w)) : no11 w := by
  match w with
  | [] => trivial
  | [b2] => cases b2 <;> trivial
  | b2 :: b3 :: rest =>
    cases b
    · change no11 (false :: b2 :: b3 :: rest) at h
      cases b2 <;> exact h
    · cases b2
      · change no11 (true :: false :: b3 :: rest) at h
        exact h
      · change no11 (true :: true :: b3 :: rest) at h
        contradiction

theorem not_no11_true_true (rest : Word) :
    ¬ no11 (true :: true :: rest) := by
  intro h; exact h

theorem not_no11_false_true_true (rest : Word) :
    ¬ no11 (false :: true :: true :: rest) := by
  intro h
  have : no11 (true :: true :: rest) := no11_of_cons false (true :: true :: rest) h
  exact not_no11_true_true rest this

def countOnes : Word → Nat
  | [] => 0
  | true :: rest => countOnes rest + 1
  | false :: rest => countOnes rest

theorem countOnes_append (l1 l2 : Word) :
    countOnes (l1 ++ l2) = countOnes l1 + countOnes l2 := by
  induction l1 with
  | nil => simp [countOnes]
  | cons b l1 ih =>
    cases b <;> simp [countOnes, ih] <;> omega

inductive Step011 : Word → Word → Prop
  | step (u v : Word) :
      Step011 (u ++ [false, true, true] ++ v) (u ++ [true, false, false] ++ v)

theorem step011_countOnes {w w' : Word} (h : Step011 w w') :
    countOnes w = countOnes w' + 1 := by
  cases h with
  | step u v =>
    simp [countOnes_append, countOnes]
    omega

theorem step011_countOnes_lt {w w' : Word} (h : Step011 w w') :
    countOnes w' < countOnes w := by
  have := step011_countOnes h
  omega

theorem step011_length {w w' : Word} (h : Step011 w w') :
    w.length = w'.length := by
  cases h with
  | step u v =>
    simp [List.length_append]

def evalLSBAt : Nat → List Bool → Nat
  | _, [] => 0
  | k, false :: rest => evalLSBAt (k + 1) rest
  | k, true :: rest => fib (k + 2) + evalLSBAt (k + 1) rest

theorem evalLSBAt_append (k : Nat) (l1 l2 : List Bool) :
    evalLSBAt k (l1 ++ l2) = evalLSBAt k l1 + evalLSBAt (k + l1.length) l2 := by
  induction l1 generalizing k with
  | nil =>
    simp [evalLSBAt]
  | cons b l1 ih =>
    cases b
    · simp [evalLSBAt]
      rw [ih (k + 1)]
      have : k + 1 + l1.length = k + (l1.length + 1) := by omega
      rw [this]
    · simp [evalLSBAt]
      rw [ih (k + 1)]
      have : k + 1 + l1.length = k + (l1.length + 1) := by omega
      rw [this]
      omega

theorem evalLSBAt_011_eq_100 (k : Nat) :
    evalLSBAt k [true, true, false] = evalLSBAt k [false, false, true] := by
  change fib (k + 2) + fib (k + 3) = fib (k + 4)
  have h := fib_add_two (k + 2)
  have h1 : k + 2 + 1 = k + 3 := by omega
  have h2 : k + 2 + 2 = k + 4 := by omega
  rw [h1, h2] at h
  omega

def evalFib (w : Word) : Nat := evalLSBAt 0 w.reverse

theorem evalFib_nil : evalFib [] = 0 := rfl
theorem evalFib_one : evalFib [true] = 1 := rfl
theorem evalFib_two : evalFib [true, false] = 2 := rfl
theorem evalFib_three : evalFib [true, false, false] = 3 := rfl
theorem evalFib_four : evalFib [true, false, true] = 4 := rfl
theorem evalFib_five : evalFib [true, false, false, false] = 5 := rfl
theorem evalFib_six : evalFib [true, false, false, true] = 6 := rfl
theorem evalFib_seven : evalFib [true, false, true, false] = 7 := rfl
theorem evalFib_eight : evalFib [true, false, false, false, false] = 8 := rfl

theorem evalFib_reduction_equiv :
    evalFib [false, true, true] = evalFib [true, false, false] := rfl

theorem step011_evalFib {w w' : Word} (h : Step011 w w') :
    evalFib w = evalFib w' := by
  cases h with
  | step u v =>
    dsimp [evalFib]
    simp only [List.reverse_append]
    have hrev1 : ([false, true, true] : List Bool).reverse = [true, true, false] := rfl
    have hrev2 : ([true, false, false] : List Bool).reverse = [false, false, true] := rfl
    rw [hrev1, hrev2]
    have H1 := evalLSBAt_append 0 (v.reverse) ([true, true, false] ++ u.reverse)
    have H2 := evalLSBAt_append 0 (v.reverse) ([false, false, true] ++ u.reverse)
    rw [Nat.zero_add] at H1 H2
    rw [H1, H2]
    have H3 := evalLSBAt_append (v.reverse.length) [true, true, false] u.reverse
    have H4 := evalLSBAt_append (v.reverse.length) [false, false, true] u.reverse
    have hl1 : ([true, true, false] : List Bool).length = 3 := rfl
    have hl2 : ([false, false, true] : List Bool).length = 3 := rfl
    rw [hl1] at H3
    rw [hl2] at H4
    rw [H3, H4]
    rw [evalLSBAt_011_eq_100]

def addPhi : (Nat × Nat) → (Nat × Nat) → (Nat × Nat)
  | (a1, b1), (a2, b2) => (a1 + a2, b1 + b2)

def phiPower : Nat → Nat × Nat
  | 0 => (1, 0)
  | 1 => (0, 1)
  | k + 2 => addPhi (phiPower (k + 1)) (phiPower k)

theorem phiPower_add_two (k : Nat) :
    phiPower (k + 2) = addPhi (phiPower (k + 1)) (phiPower k) := rfl

def evalPhiLSBAt : Nat → List Bool → Nat × Nat
  | _, [] => (0, 0)
  | k, false :: rest => evalPhiLSBAt (k + 1) rest
  | k, true :: rest => addPhi (phiPower k) (evalPhiLSBAt (k + 1) rest)

theorem addPhi_zero (p : Nat × Nat) : addPhi p (0, 0) = p := by
  cases p; simp [addPhi]

theorem addPhi_assoc (p1 p2 p3 : Nat × Nat) :
    addPhi (addPhi p1 p2) p3 = addPhi p1 (addPhi p2 p3) := by
  cases p1; cases p2; cases p3; simp [addPhi]; omega

theorem addPhi_comm (p1 p2 : Nat × Nat) :
    addPhi p1 p2 = addPhi p2 p1 := by
  cases p1; cases p2; simp [addPhi]; omega

theorem evalPhiLSBAt_append (k : Nat) (l1 l2 : List Bool) :
    evalPhiLSBAt k (l1 ++ l2) = addPhi (evalPhiLSBAt k l1) (evalPhiLSBAt (k + l1.length) l2) := by
  induction l1 generalizing k with
  | nil =>
    simp [evalPhiLSBAt, addPhi]
  | cons b l1 ih =>
    cases b
    · simp [evalPhiLSBAt]
      rw [ih (k + 1)]
      have : k + 1 + l1.length = k + (l1.length + 1) := by omega
      rw [this]
    · simp [evalPhiLSBAt]
      rw [ih (k + 1)]
      have : k + 1 + l1.length = k + (l1.length + 1) := by omega
      rw [this]
      rw [addPhi_assoc]

theorem evalPhiLSBAt_011_eq_100 (k : Nat) :
    evalPhiLSBAt k [true, true, false] = evalPhiLSBAt k [false, false, true] := by
  change addPhi (phiPower k) (addPhi (phiPower (k + 1)) (0, 0)) =
         addPhi (phiPower (k + 2)) (0, 0)
  rw [addPhi_zero, addPhi_zero]
  rw [phiPower_add_two]
  rw [addPhi_comm]

def evalPhi (w : Word) : Nat × Nat := evalPhiLSBAt 0 w.reverse

theorem evalPhi_nil : evalPhi [] = (0, 0) := rfl
theorem evalPhi_phi0 : evalPhi [true] = (1, 0) := rfl
theorem evalPhi_phi1 : evalPhi [true, false] = (0, 1) := rfl
theorem evalPhi_phi2 : evalPhi [true, false, false] = (1, 1) := rfl
theorem evalPhi_phi3 : evalPhi [true, false, false, false] = (1, 2) := rfl
theorem evalPhi_phi4 : evalPhi [true, false, false, false, false] = (2, 3) := rfl

theorem evalPhi_reduction_equiv :
    evalPhi [false, true, true] = evalPhi [true, false, false] := rfl

theorem step011_evalPhi {w w' : Word} (h : Step011 w w') :
    evalPhi w = evalPhi w' := by
  cases h with
  | step u v =>
    dsimp [evalPhi]
    simp only [List.reverse_append]
    have hrev1 : ([false, true, true] : List Bool).reverse = [true, true, false] := rfl
    have hrev2 : ([true, false, false] : List Bool).reverse = [false, false, true] := rfl
    rw [hrev1, hrev2]
    have H1 := evalPhiLSBAt_append 0 (v.reverse) ([true, true, false] ++ u.reverse)
    have H2 := evalPhiLSBAt_append 0 (v.reverse) ([false, false, true] ++ u.reverse)
    rw [Nat.zero_add] at H1 H2
    rw [H1, H2]
    have H3 := evalPhiLSBAt_append (v.reverse.length) [true, true, false] u.reverse
    have H4 := evalPhiLSBAt_append (v.reverse.length) [false, false, true] u.reverse
    have hl1 : ([true, true, false] : List Bool).length = 3 := rfl
    have hl2 : ([false, false, true] : List Bool).length = 3 := rfl
    rw [hl1] at H3
    rw [hl2] at H4
    rw [H3, H4]
    rw [evalPhiLSBAt_011_eq_100]

inductive Step : Word → Word → Prop
  | head011 (v : Word) :
      Step (false :: true :: true :: v) (true :: false :: false :: v)
  | head11 (v : Word) :
      Step (true :: true :: v) (true :: false :: false :: v)
  | cons (b : Bool) {w w' : Word} :
      Step w w' → Step (b :: w) (b :: w')

theorem step_not_no11 {w w' : Word} (h : Step w w') : ¬ no11 w := by
  induction h with
  | head011 v =>
    exact not_no11_false_true_true v
  | head11 v =>
    exact not_no11_true_true v
  | cons b hstep ih =>
    intro hno11
    have hw := no11_of_cons b _ hno11
    exact ih hw

theorem no11_irreducible (w : Word) (h : no11 w) : ¬ ∃ w', Step w w' := by
  intro ⟨w', hstep⟩
  exact step_not_no11 hstep h

theorem step_of_not_no11 (w : Word) (h : ¬ no11 w) : ∃ w', Step w w' :=
  match w with
  | [] => by exfalso; exact h trivial
  | [b] => by exfalso; cases b <;> exact h trivial
  | true :: true :: rest =>
    ⟨true :: false :: false :: rest, Step.head11 rest⟩
  | false :: true :: true :: rest =>
    ⟨true :: false :: false :: rest, Step.head011 rest⟩
  | false :: true :: [] =>
    by exfalso; exact h trivial
  | false :: false :: rest =>
    have hnot : ¬ no11 (false :: rest) := by
      intro hno11
      have : no11 (false :: false :: rest) := by
        change no11 (false :: rest) at hno11
        exact hno11
      exact h this
    have ⟨w', hstep⟩ := step_of_not_no11 (false :: rest) hnot
    ⟨false :: w', Step.cons false hstep⟩
  | false :: true :: false :: rest =>
    have hnot : ¬ no11 (true :: false :: rest) := by
      intro hno11
      have : no11 (false :: true :: false :: rest) := by
        change no11 (true :: false :: rest) at hno11
        exact hno11
      exact h this
    have ⟨w', hstep⟩ := step_of_not_no11 (true :: false :: rest) hnot
    ⟨false :: w', Step.cons false hstep⟩
  | true :: false :: rest =>
    have hnot : ¬ no11 (false :: rest) := by
      intro hno11
      have : no11 (true :: false :: rest) := by
        change no11 (false :: rest) at hno11
        exact hno11
      exact h this
    have ⟨w', hstep⟩ := step_of_not_no11 (false :: rest) hnot
    ⟨true :: w', Step.cons true hstep⟩
termination_by w.length

theorem irreducible_iff_no11 (w : Word) : (¬ ∃ w', Step w w') ↔ no11 w := by
  constructor
  · intro h
    by_cases hno11 : no11 w
    · exact hno11
    · have ⟨w', hstep⟩ := step_of_not_no11 w hno11
      exfalso; exact h ⟨w', hstep⟩
  · intro h
    exact no11_irreducible w h

theorem zeckendorf_zero : ∃ w, no11 w ∧ evalFib w = 0 := ⟨[], no11_nil, evalFib_nil⟩
theorem zeckendorf_one : ∃ w, no11 w ∧ evalFib w = 1 := ⟨[true], no11_one, evalFib_one⟩
theorem zeckendorf_two : ∃ w, no11 w ∧ evalFib w = 2 := ⟨[true, false], no11_two, evalFib_two⟩
theorem zeckendorf_three : ∃ w, no11 w ∧ evalFib w = 3 := ⟨[true, false, false], no11_three, evalFib_three⟩
theorem zeckendorf_four : ∃ w, no11 w ∧ evalFib w = 4 := ⟨[true, false, true], no11_four, evalFib_four⟩
theorem zeckendorf_five : ∃ w, no11 w ∧ evalFib w = 5 := ⟨[true, false, false, false], no11_five, evalFib_five⟩
theorem zeckendorf_six : ∃ w, no11 w ∧ evalFib w = 6 := ⟨[true, false, false, true], no11_six, evalFib_six⟩
theorem zeckendorf_seven : ∃ w, no11 w ∧ evalFib w = 7 := ⟨[true, false, true, false], no11_seven, evalFib_seven⟩
theorem zeckendorf_eight : ∃ w, no11 w ∧ evalFib w = 8 := ⟨[true, false, false, false, false], no11_eight, evalFib_eight⟩

theorem bergman_phi0 : ∃ w, no11 w ∧ evalPhi w = (1, 0) := ⟨[true], no11_one, evalPhi_phi0⟩
theorem bergman_phi1 : ∃ w, no11 w ∧ evalPhi w = (0, 1) := ⟨[true, false], no11_two, evalPhi_phi1⟩
theorem bergman_phi2 : ∃ w, no11 w ∧ evalPhi w = (1, 1) := ⟨[true, false, false], no11_three, evalPhi_phi2⟩
theorem bergman_phi3 : ∃ w, no11 w ∧ evalPhi w = (1, 2) := ⟨[true, false, false, false], no11_five, evalPhi_phi3⟩
theorem bergman_phi4 : ∃ w, no11 w ∧ evalPhi w = (2, 3) := ⟨[true, false, false, false, false], no11_eight, evalPhi_phi4⟩

end Collatz.PhiWords
