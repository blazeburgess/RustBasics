/// Conventional and not terribly complex
let x = 5;

if x == 5 {
    println!("x is five!");
} // simplest if statement; one choice, two paths

/// with `else` condition:
let x = 5;

if x == 5 {
    println!("x is five!");
} else {
    println!("x is not five :(");
}

/// uses `else if` for more conditions
let x = 5;

if x == 5 {
    println!("x is five!");
} else if x == 6 {
    println!("x is six!");
} else {
    println!("x is not five or six :(");
}

/// can be used in some non-standard ways:
let x = 5;

let y = if x == 5 {
    10    
} else {
    15
}
/// the above can also be (better) written:
let x = 5;
let y = if x == 5 { 10 } else { 15 }; // y: i32

/// The reasons the two previous examples work is that `if` is an 
/// expression, which calculates value from the last expression in
/// it's specific branch.
///
/// `if`s without `else`s return a value of `()`
