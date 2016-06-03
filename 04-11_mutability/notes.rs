/// # Mutability
///
/// Mutability is not conventional in Rust. This is most obvious in that
/// it is not default, as in:
let x = 5;
x = 6; // error

/// but is rather introduced by the `mut` keyword:
let mut x = 5;

x = 6; // works
