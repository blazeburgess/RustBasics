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
