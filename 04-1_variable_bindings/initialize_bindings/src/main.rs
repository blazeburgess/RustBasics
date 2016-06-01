fn main() {
    let x: i32; // this initializes but doesn't assign the variable 'x'
    println!("Hello, world!"); // this builds with a warning but runs fine
    // the following (uncommented) will not compile because it calls an
    // unassiged variable:
    //     println!("The value of x is {}:", x);
}
