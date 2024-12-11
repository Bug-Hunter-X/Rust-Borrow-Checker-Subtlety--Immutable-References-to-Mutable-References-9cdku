# Rust Borrow Checker Subtlety: Immutable References to Mutable References

This repository demonstrates a subtle aspect of Rust's borrow checker.  While a mutable reference allows modification, an immutable reference to that mutable reference can indirectly prevent modifications, leading to unexpected compile-time errors.

The `bug.rs` file contains code that exhibits this behavior. The `bugSolution.rs` file is intentionally left empty as there's no straightforward solution to directly modify 'x' through 'z' without violating borrowing rules. The solution lies in re-structuring the code to avoid creating an immutable reference to a mutable reference if modification through that reference is required.