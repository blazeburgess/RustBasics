// variable bindings can point to functions, with or without
// type inference
fn plus_one(i: i32) -> i32 {
    i + 1
} // simple function in expression style

let f: fn(i32) -> i32 = plus_one; // without type inference

let f = plus_one; // with type inference

let six = f(5); // calls the function through variable `f`
