fn main() {
    print_sum(5, 6);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}
// last function only works because of explicit type assignment
// had the function read:
//   fn print_sum(x, y) { ... }
// It would have raised an error: 
//   expected one of `!`, `:`, or `@`, found ')'
// Rust is specifically designed to have type inference within 
// functions, but to force declaration on functions/parameters.
