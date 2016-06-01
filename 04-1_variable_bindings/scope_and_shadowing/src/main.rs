// Scope
fn main() {
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and the value of y is {}", x, y);
        // this works as both x and y are in scope
    }
    println!("The value of x is {} and value of y is {}", x, y);
    // this fails because y is not in scope
}

// Shadowing
fn main() {
    let x: i32 = 8;
    {
        println!("{}", x); // prints "8"
        let x = 12; // shadows x value in this scope
        println!("{}", x); // prints shadowed value "12"
    }
    println!("{}", x); // prints original value "8"
    let x = 42;
    println!("{}", x); // prints "42"
}

// Shadowing vs. Mutability
fn main() {
    let mut x: i32 = 1;
    x = 7;
    let x = x; // makes x immutable and bound to 7

    let y = 4;
    let y = "I can also be bound to text!"; // changes type of variable y
}
