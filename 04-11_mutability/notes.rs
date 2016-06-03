/// # Mutability
///
/// Mutability is not conventional in Rust. This is most obvious in that
/// it is not default, as in:
let x = 5;
x = 6; // error

/// but is rather introduced by the `mut` keyword:
let mut x = 5;

x = 6; // works
/// Since the last variable binding was mutable, it was possible to change
/// what `x` points to. This is not changing the value at location `x`,  
/// Rather it changes the i32 to which `x` is bound.
///
/// To change what the binding itself points to, use a mutable reference:
let mut x = 5;
let y = &mut x;
/// As `y` is an immutable binding to a mutable reference, it cannot be
/// reassigned (`y = &mut z` would now fail), but you can change what
/// `y` is bound to (`*y = 5` would still work).
///
/// If you need both, just combine the concepts:
let mut x = 5;
let mut y = &mut x;

/// Also, because `mut` is part of a pattern, this is allowed:
let (mut x, y) = (5, 6);

fn foo(mut x: i32) { ... }

/// ## Interior vs. Exterior Mutability
///
/// `Immutable` in Rust actually means it has `exterior mutability`:
use std::sync::Arc;

let x = Arc::new(5);
let y  = x.clone();
/// Calling `clone()` actually requires an update to the reference count
/// but this was called without any `mut`s. This is because of the base
/// rules of borrowing in Rust:
///     
///     You may have one or the other of these, but not both
///     at once:
///     - one or more references (&T) to a resource
///     - exactly one mutable reference (&mut T)
///
/// In the above example, it is considered safe to have two pointers
/// because any mutations are contained within the structure itself.
///
/// Interior mutability, opposite of this, are found in modules like
/// std::cell
use std::cell::RefCell;

let x = RefCell::new(42);

let y = x.borrow_mut();

/// RefCell actually gives `&mut` references to whatever is inside of it
/// via `borrow_mut()`
///
/// This does create some risks and RefCell will only enforce these at 
/// runtime. As such the following will result in a `panic`
use std::cell::RefCell;

let x = RefCell::new(42);

let y = x.borrow_mut();
let z = x.borrow_mut();

/// ## Field-level mutability
///
/// As mutability is a property of a borrow (&mut) or a binding (let mut),
/// structs, for example, cannot have some fields mutable and others
/// immutable:
struct Point {
    x: i32,
    mut y: i32 // error here
}

/// This is because a struct's mutability is in its binding.
struct Point {
    x: i32,
    y: i32
}

let mut a = point { x: 5, y: 6 };

a.x = 10;

let b = Point { x: 5, y: 6 };

b.x = 10 // returns "error: cannot assign to immutable field `b.x`
