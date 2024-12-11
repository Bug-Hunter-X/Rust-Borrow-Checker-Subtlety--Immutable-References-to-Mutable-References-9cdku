The issue isn't about *solving* the problem by modifying through `z` because that violates Rust's borrowing rules,  and rightfully so. The solution is to refactor the code to avoid the situation.  A better approach would avoid the immutable reference to the mutable reference, if that level of indirection isn't necessary:

```rust
fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Modify directly through the mutable reference y.

    println!("x = {}", x); // Prints x = 6
}
```

Alternatively, if you truly need both an immutable and mutable reference, you'll need to split the code into separate scopes where each reference is used to avoid conflicting borrow rules:

```rust
fn main() {
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1; //Modifying x
    }
    {
        let z = &x; //Only reading x here
        println!("x = {}", *z);
    }
}
```