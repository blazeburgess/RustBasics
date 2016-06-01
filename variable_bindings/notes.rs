// simplest form of binding
fn main() {
    let x = 5;
}

// pattern binding
fn main() {
    let (x, y) = (1, 2);    
}

// with type annotation
fn main() {
    let x: i32 = 5;
}
// leaving type annotation out leaves type assignment to rust's inference
// meaning the above is equivalent to:
fn main() {
    let x = 5; // x: i32
}

// bindings are immutable by default, so the following reassignment
// will break the program:
fn main() {
    let x = 5;
    x = 10; // returns "error: re-assignment of immutable variable 'x'"
}

// mutable bindings are set with 'mut'
fn main() {
    let mut x = 5; // mut x: i32
    x = 10;
}
