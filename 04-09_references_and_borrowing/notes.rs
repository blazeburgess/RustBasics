/// # References and Borrowing
/// The last section ended with this function:
fn foo(v1: Vec<i32>,  v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    // manipulate v1 and v2 here
    
    // hand back ownership
    (v1, v2, 42)
}

let v1 = vec![1, 2, 3];
let v2 = vec![1, 2, 3];

let (v1, v2, answer) = foo(v1, v2);

/// This could be rendered much simpler by using borrowing:
fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // manipulate v1 and v2 here
    
    // return the answer
    42
}

let v1 = vec![1, 2, 3];
let v2 = vec![1, 2, 3];

let answer = foo(&v1, &v2);

// v1 and v2 can still be used down here

/// This works because the function only takes the reference of the 
/// `Vec<i32>` and we only passed in the references to `v1` and `v2`
/// respectively. Instead of shifting ownership, this let foo() 
/// borrow the vectors without change to the original bindings.
///
/// This also means that the vectors cannot be changed inside of foo()
fn foo(v: &Vec<i32>) {
    v.push(5);
}

let v = vec!;

foo(&v);
/// The above returns:
///     error: cannot borrow immutable borrowed content `*v` as mutable
///     v.push(5);
///     ^
/// because pushing a value mutates the vector
///
/// ## &mut references
/// `mutable reference`s allow mutations while borrowing.
let mut x = 5;
{
    let y = &mut x;
    *y += 1;
}

println!("{}", x);
/// This prints `6`.
///
/// The reason is that `y` is a mutable reference to x, which had to be
/// mutable as well to allow for this. We used `*y` to point to the 
/// contents `y` referenced and added 1. The parentheses are necessary 
/// because this requires it's own scope. If removed, this returns:
///
///     error: cannot borrow `x` as immutable because it is also
///     borrowed as mutable
///         println!("{}", x);
///                        ^
///     note: previous borrow of `x` occurs here; the mutable
///     borrow prevents subsequent moves, borrows, or modifications
///     of `x` until the borrow ends
///         let y = &mut x;
///                      ^
///
///     note: previous borrow ends here
///     fn main() {
///
///     }
///     ^
///
/// ## The rules
///     1) Any borrow can only last as long as the scope of the owner
///     2) You can only have one of the following at a given time:
///         - one or more references (&T) to a resource
///         - exactly one mutable reference
/// 
/// This is all to prevent a data race, defined:
/// 
///     There is a 'data race' when two or more pointers access
///     the same memory location at the same time, where at least
///     one of them is writing, and the operations are not
///     synchronized.
///
/// References are not so constrained because none of them are writing,
/// but we can only have one &mut at a time, making a 'data race'
/// impossible.
///
/// ## Thinking in Scopes
fn main() {
    let mut x = 5;

    let y = &mut x;

    *y += 1;

    println!("{}", x);
}
/// As stated above this returns:
///     error: cannot borrow `x` as immutable because it is also
///     borrwed as mutable
///         println!("{}", x);
///                        ^
/// 
/// This is the result of a rule violation: having a &mut T point to `x`,
/// which keeps us from making other &Ts. The part of this error that
/// states
///     note: previous borrow ends here
///     fn main() {
///
///     }
///     ^
/// hints at the issue being one of scope.
