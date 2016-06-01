/// # Vectors
/// Definition:
///      a dynamic or `growable` array of generic Vec<T> type, meaning
///      that it can be of any type
let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>

let v = vec![0; 10]; // v is an array of 10 zeros

/// Indexing is standard
let v = vec![1, 2, 3, 4, 5];

println!("The third element of v is {}", v[2]);

/// but index retrieval must be with usize types:
let v = vec![1, 2, 3, 4, 5];

let i: usize = 0; // valid index
let j: i32 = 0; // invalid index

v[i]; // works
v[j]; // fails
v[0]; // inferred `usize`, works

/// ## Out-of-bounds Access
let v = vec![1, 2, 3];
println!("Item 7 is {}", v[7]); // panics with error: `index out of bounds`

/// out-of-bounds errors can be handled:
let v = vec![1, 2, 3];

match v.get(7) {
    Some(x) => println!("Item 7 is {}", x),
    None => println!("Sorry, this vector is too short.")
}

/// ## Iterating
/// Three ways to iterate with `for`
let mut v  = vec![1, 2, 3, 4, 5];

// first
for i in &v {
    println!("A reference to {}", i)
}

//second
for i in &mut v {
    println!("A mutable reference to {}", i)
}

// third
for i in v {
    println!("Take ownership of the vector and its element {}", i);
}

/// Taking ownership, as in the third method, does not allow use of that
/// same vector again.
///
/// Thus, this fails to compile:
let v = vec![1, 2, 3, 4, 5];

for i in v {
    println!("Take ownership of the vector and its element {}", i);
}

for i in v {
    println!("Take ownership of the vector and its element {}", i);
} // this is invalid

/// while multiple reference iterations compile without issue:
let v = vec![1, 2, 3, 4, 5];

for i in &v {
    println!("This is a reference to {}", i);
}

for i in &v {
    println!("This is a reference to {}", i);
}

/// More vector methods can be found at the relevant API docs:
///     https://doc.rust-lang.org/std/vec/
