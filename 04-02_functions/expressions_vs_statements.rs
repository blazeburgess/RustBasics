// expresions return a value, statements do not.
// for rust 'let' variable bining is not an expression
fn main() {
    let x = (let y = 5);
}
// the above function raises the error:
//   expected identifier, found keyword 'let'
// because 'let' only introduces statements while the
// compiler was looking to create an expression.
// 
// however, assignment to an already bound variable is
// an expression as in the following case:
fn main() {
    let mut y = 5;
    let x = (y = 6); // x gains the value `()`, not `6`
        // because an assigned value can only have one
        // owner
}

// An expression statement can be seen in the file 
// add_one.rs. where there is no semi-colon:
//   fn add_one(x: i32) -> i32 {
//       x + 1
//   }
