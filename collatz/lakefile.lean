import Lake
open Lake DSL

package collatz where
  leanOptions := #[⟨`autoImplicit, false⟩]

@[default_target]
lean_lib Collatz
