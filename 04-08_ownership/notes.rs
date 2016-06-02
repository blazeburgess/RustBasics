/// # Ownership
/// Three overarching concepts within ownership:
///     - ownership (this)
///     - borrowing (associated with `references`)
///     - lifetimes (advanced borrowing)
/// ## Meta
/// Two notes about ownership:
///     - Rust runs these abstractions at compile time, meaning that
///         these features do not have any run-time cost
///     - This has proven a difficult concept to new users, rejecting
///         code that the programmer sees no issue with. This is 
///         called `fighting the borrow checker`
///
/// ## Ownership
fn foo() {
    let v = vec![1, 2, 3];
}
/// This fuction creates vector `v` on the stack when it comes into 
/// scope, i.e. inside the function, and allocates space on the heap
/// for `v`s elements.
///
/// When vector `v` goes out of scope, at the end of foo(), Rust 
/// deterministiclly cleans up everything related to the vector,
/// including the memory allocation on the heap.
///
/// ## Move semantics
/// Rust enforces exactly one binding per resource
let v = vec![1, 2, 3];

let v2 = v; // this reassigns the vec to `v2` from `v`

println!("v[0] is: {}", v[0]); // returns "error: use of moved value: 'v'

/// This is also true when moved with functions
fn take (v: Vec<i32>) {
    // imagine code here
}

let v = vec![1, 2, 3];

take(v);

println!("v[0] is: {}", v[0]); // returns "error: use of moved value 'v'"

/// # The details
let x = 10;
/// Rust allocate memory for `x`, specifically an integer [i32], on 
/// the stack, copies the pattern representing `10` to allocated
/// memory ad binds `x` to that memory region.
///
/// But here:
let v = vec![1, 2, 3];

let mut v2 = v;
/// memory is allocated for vector object `v` on the stack, on the
/// heap for the actual data `[1, 2, 3]`. `v2` is added to the stack
/// but, by a shallow copy, it points to the same allocated memory on 
/// the heap.
///
/// Because Rust does not allow two pointers to the same heap memory
/// allocation (for fear of a data race when both are accessible), `v`
/// is no longer accessible once the heap memory is reallocated to `v2`
///
/// ## Copy types
/// Copy allows what ownership transfer does not.
let v = 1; // v: i32

let v2 = v; // v2: i32

println!("v is: {}", v); // this works
/// Since v is an i32 in the above example the `let` equation actually
/// implements a full copy, meaning that `v` and `v2` are both accessible.
///
/// Bool types work the same way, allowing these two programs to compile 
/// wihout issue:
// first
fn main() {
    let a = 5;

    let _y = double(a);
    println!("{}", a);
}

fn double(x: i32) -> i32 {
    x * 2
}

// second
fn main() {
    let a = true;

    let _y = change_truth(a);
    println!("{}", a);
}

fn change_truth(x: bool) -> bool {
    !x
}
/// If these types did not implement the Copy trait, compiling this 
/// would have returned: "error: use of moved value: `a`" like before
fn foo(v: Vec<i32>) -> Vec<i32> {
    // mess with v here
    
    // return ownership
    v
}
/// This, when done manually, is tedious as shown below. There is a 
/// feature (borrowing) that makes this much easier, covered in the
/// next section.
///
/// Manually:
fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // mess with v and v2 here
    
    // return ownership, return result
    (v1, v2, 42)
}

let v1 = vec![1, 2, 3];
let v2 = vec![1, 2, 3];

let (v1, v2, answer) = foo(v1, v2);
