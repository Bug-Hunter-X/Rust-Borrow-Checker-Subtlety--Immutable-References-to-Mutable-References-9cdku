fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y;    // z is an immutable reference to y, which is a mutable reference to x

    *y += 1; // This is allowed, as y is a mutable reference
    // *z += 1; // This is a compile-time error, as z is an immutable reference

    println!("x = {}", x); // Prints x = 6
}