// returns function according to convention
fn foo(x: i32) -> i32 {
    return x;

    // all code below `return` will not run
    // this is meaningless:
    x + 1
}

// `return` can be used as the last line of a function, but it is 
// considered better style to use the expression-based method.
// so:
fn foo(x: i32) -> i32 {
    return x + 1;
}  // works, but poor style

fn foo(x: i32) -> i32 {
    x + 1
} // good style
