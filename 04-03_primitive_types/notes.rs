// Booleans
let x = true; // type inference
let y: bool = false;

// char
let x = 'x';
let two_hearts = '💕';
// a char type represents a single Unicode scalar value, taking four
// bytes rather than one
// chars can also be created with a single quotation mark in front
//
//Numeric types
let x = 42; // inferred type i32
let x = 1.0; //inferred type f64
// a four-bit signed number is any from -8 to +7, whereas an unsigned
// ranges from 0 to +15. Signed types begin with an i, such as:
//   i8, i16, i32, i64, isize
// while unsigned begin with u:
//   u8, u16, u32, u64, usize
// fixed-size types end with a number specifying the number of bits
// they represent. whether signed or unsigned. They include:
//   8, 16, 32, 64
// variable-size types determine representation by the size of a
// pointer. They also have a signed and unsigned variety:
//   isize, usize
// floating point numbers have two representations, one in 32 bits,
// the other in 64:
//   f32, f64
//
//   Arrays
let a = [1, 2, 3]; // inferred a: [i32; 3]
let mut m = [1, 2, 3]; // inferred m: [i32; 3]
// arrays have [T; N] type, N representing the compile-time 
// constant for the length of the array (T is explained in the 
// section on generics, 4.18)
let a = [0; 20]; // a: [i32; 20]
// above initializes array `a` with every element having the same 
// value
let a = [1, 2, 3];
println!("a has {} elements", a.len()); // prints number of elems

let names = ["Graydon", "Brian", "Niko"]; // inferred names: [&str; 3]
println!("The second name is: {}", names[1]); // indexed conventionally
        // will print "The second name is: Brian"

// Slices
// a `slice` creates a reference to another data structure. It 
// provides an efficient and safe way to use some portion of an 
// array without needlessly copying it. Slices have a defined length
// but can be mutable or immutable. Slices also have a type &[T], 
// which is, again, covered in generics (4.18).
let a = [0, 1, 2, 3, 4];
let complete = &a[..]; // slice contains all elements of `a`
let middle = &a[1..4]; // slice contains elements between 1 (inclusive) 
        // and 4 (not inclusive), so: [1, 2, 3]
// str
let hello = "Hello, world!"; // inferred type
let hello: &'static str = "Hello, world!"; //explicit
// str representation has two components:
//     -pointer
//     -length
// these can be accesed with the `.as_ptr()`  and `len()` methods
use std::slice;
use std::str;

let story = "Once upon a time...";

let ptr = story.as_ptr();
let len = story.len();

assert_eq!(19, len); // story has nineteen bytes

// a str can be reconstructed from the ptr and length, but this is unsafe
// as it depends on the validity of both components
let s = unsafe {
    // we build a &[u8]
    let slice = slice::from_raw_parts(ptr, len);

    // then converts slice into string slice
    str::from_utf8(slice)
};

assert_eq!(s, Ok(story));

// Tuples
let x = (1, "hello"); // inferred type
let x: (i32, &str) = (1, "hello");
// - - - - - -

let mut x = (1, 2); // x: (i32, i32)
let y = (2, 3); // y: (i32, i32)

x = y // one tuple can be assigned into another

let (x, y, z) = (1, 2, 3);
printl!("x is {}", x); // fields are accessed through a `destructuring
        // let`
//
// Single element tuples and parentheses can be disambiguated with `,`
(0,); // single-element tuple
(0); // zero in parentheses

// Tuple Indexing
// begins with zero, as conventional, but uses `.` rather than `[]`
let tuple = (1, 2, 3);

let x = tuple.0;
let y = tuple.1;
let z = tuple.2;

println!("x is {}", x);

// Functions
// functions are also a type
fn foo(x: i32) -> i32 { x }

let x: fn(i32) -> i32 = foo; // x is a function pointer
        // it takes an i32 and returns an i32
