/// # Lifetimes
/// The concept of lifetimes is implemented to stop certain issues that 
/// borrowing and referencing creates. One example is:
///    1) A resource is created
///    2) A reference is made to that resource
///    3) The original resource is deallocated
///    4) There's an attempt to use the reference
/// By the end of step three the reference is already pointing to an 
/// invalid resource, so to prevent dangling pointers Rust utilizes the 
/// `lifetime` aspect of ownership.
///
/// This can be explicit or implicit:
fn foo(x: &i32) {
    
} // implicit

fn bar<'a>(x: &'a i32) {
    
} // explicit
/// Literally `'a` is to be read as `the lifetime a`. Every reference 
/// technically has a lifetime, but this is not a problem due to elision
/// handled by the compiler
