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
