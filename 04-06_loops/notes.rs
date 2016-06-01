/// # Three kinds of iteration:
///     -loop
///     -while
///     -for
loop {
    println!("Loop forever!");
} // infinite loop is the simplest loop possible

/// # While:
let mut x = 5; // mut x: i32
let mut done = false; //mut done: bool

while !done {
    x += x -3;

    println!("{}", x);

    if x % 5 == 0 {
        done = true;
    }
} // while loops used when unsure how many loops are needed

/// It is considered better form to make an infinite loop by:
///     loop {
///
///     }
/// than:
///     while true {
///
///     }
/// though both work
///
/// # For:
for x in 0..10 {
    println!("{}", x); // x: i32
}
/// the abstract form of the above is:
///     for `var` in `expression` {
///         `code`
///     }
///
/// # Enumerate:
for (i, j) in (5..10).enumerate() {
    println!("i = {} and j = {}", i, j);
}
/// `.enumerate()` tracks the number of times already looped. The 
/// range to iterate over (5..10) must be in parentheses and the
/// output for the above function is:
///     i = 0 and j = 5
///     i = 1 and j = 6
///     i = 2 and j = 7
///     i = 3 and j = 8
///     i = 4 and j = 9
/// Another example:
let lines = "hello\nworld".lines();

for (linenumber, line) in lines.enumerate() {
    println!("{}: {}", linenumber, line);
}
/// output would be:
///     0: hello
///     1: world
///
/// # Ending an interation early:
/// It is considered better to `break` or `return` a loop than use 
/// the `while bool` in the example above `while !done {...}`
let mut x = 5;

loop {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 { break; }
}
/// `continue` pushes the loop to the next iteration
for x in 0..10 {
    if x % 2 == 0 { continue; }
    println!("{}", x)''
}

/// # Loop labels:
/// For nested loops it is possible to specify whether a given statement
/// refers to the inner or outer function:
'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // continues `x` loop
        if y % 2 == 0 { continue 'inner; } // continues `y` loop
        println!("x: {}, y: {}", x, y);
    }
}
