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
///
/// Between the `<>` of the above explicit version are ny generic 
/// parameters, including but not limited to lifetime. Two references
/// would look like this:
fn bar <'a, 'b>(...)
/// which is then used for parameters
... (x: &'a i32) // immutable

... (x: &'a mut i32) //mutable

/// ## In structs
struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let y = &5; // same as `let _y = 5; let y = &_y;`
    let f = Foo { x: y };

    println!("{}"m f.x);
}

/// As shown above, structs have lifetimes similarly to those of functions
struct Foo<'a> { ... } // declares lifetime

x: &'a i32 // uses previously declared lifetime

/// ## impl blocks
struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo <'a> {
    fn x(&self) -> & 'a i32 { self.x }
}

fn main() {
    let y = &5; // again, equivalent to `let _y = 5; let y = &_y`
    let f = Foo { x: y }

    println!("x is: {}", f.x());
}
/// In the `impl` line Foo needs a lifetime declared for it. The structure
/// is that `impl<'a>` declares the lifetime and `Foo<'a>` uses it.
///
/// ## Multiple lifetimes
fn x_or_y<'a>(x: &'a str, y: &'a str) {...} // multiple ref to one lifetime

fn x_or_y<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {...} // multiple lts

/// ## Thinking in scopes
///
/// visualization:
fn main() {
    let y = &5; // -+ y goes into scope
                //  |
                //  |
    // code     //  |
                //  |
                // -+ y goes out of scope
}

/// adding Foo struct:
struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let y = &5;           // -+ y goes into scope
    let f = Foo { x: y }; // -+ f goes into scope
                          //  |
    // code               //  |
                          //  |
}                         // -+ f and y go out of scope

/// Being mindful of scope prevents errors, as in the following case:
struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let x;                    // -+ x goes into scope
                              //  |
    {                         //  |
        let y = &5;           // ---+ y goes into scope
        let f = Foo { x: y }; // ---+ f goes into scope
        x = &f.x;             //  | | error here
    }                         //  |
                              //  |
        println!("{}, x")     //  |
}                             // -+ x goes out of scope
/// The error is that `x` is made a reference to `f.x`, which has a 
/// shorter scope than itself. This problem of scope is discussed in
/// terms of lifetimes to clarify what's actually going on.
///
/// ## 'static
let x: &'static str = "Hello, world.";

/// static lifetimes are special because they run the entire length of 
/// the program. This is the type for string literals or global variables
static FOO: i32 = 5;
let x: &'static i32 = &FOO; // this adds an i32 to binary data seg and
        // makes a reference `x`

/// ## Lifetime Elision
/// Rust supports local type inference inside of functions, but not in
/// item signatures. Lifetime Elisions are made by a secondary inference
/// algorithm. It works according to three rules:
///
///     - Each elided lifetime in a function's arguments becomes 
///         a distinct lifetime parameter.
///     - If there is exactly one input lifetime, elided or 
///         otherwise, that lifetime is assigned to all 
///         elided lifetimes in the return values of that
///         function
///     - If there are multiple input lifetimes, but one of them
///         is &self or &mut self, the lifetime of `self` is 
///         assigned to all elided out lifetimes.
/// 
/// In all other conditions, eliding an output lifetime is an error.
///
/// ### Exaples
fn print(s: &str); // elided
fn print<'a>(s: &'a str); // expanded

fn debug(lvl: u32, s: &str); // elided
fn debug<'a>(lvl: u32, s: &'a str); //expanded

// In the case immediately above, `lvl` does not need a lifetime because
// it is not a reference (determined by `&`). Only things with some 
// relation to references require a lifetime

fn substr(s: &str, until: u32) -> &str; // elided
fn substr<'a>(s: &'a str, until: u32) -> &'a str; // explanded

fn get_str() -> &str; // ILLEGAL, no inputs

fn frob(s: &str, t: &str) -> *str; // ILLEGAL, two inputs
fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str // Expanded: Output
        // lifetime is ambiguous

fn get_mut(&mut self) -> &mut T; // elided
fn get_mut<'a>(&'a mut self) -> &'a mut T; // expanded

fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command; // elided
fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // expanded

fn new(buf: &mut [u8]) -> BufWriter; // elided
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>; // expanded
