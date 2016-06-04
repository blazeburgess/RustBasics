/// # Structs
///
/// `structs` are a way to structure complex data types according to
/// their needs. For example, coordinates that would be done manually
let origin_x = 0;
let origin_y = 0;
/// could be rendered in a struct:
struct Point {
    x: i32,
    y: i32.
}

fn main() {
    let origin = Point { x: 0, y: 0 }; // origin: Point

    println!("The origin is at ({}, {})", origin.x, origin.y)
}
/// structs, by convention, begin with a capital letter. We can create an 
/// instance of the struct with `let` and a `key: value` structure to fill
/// each field
///
/// These values are immutable by default, but that can be overridden:
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0 };

    point.x = 5;

    println!("The point is at ({}, {})", point.x, point.y);
} // prints: `The point is at (5, 0)`

/// Mutability cannot, however, be implemented at the level of fields
struct Point {
    mut x: i32, // error here
    y: i32,
}
/// This is because mutability is a product of a binding, not structure.
/// Rust implements it this way to simplify certain processes and give
/// flexibility to instantiating (im)mutability
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0 };

    point.x = 5;

    let point = point; // now immutable

    point.y = 6; // causes error
}

/// Structures can contain &mut pointers, allowing for certain kinds of
/// mutations
struct Point {
    x: i32,
    y: i32,
}

struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0 };

    {
        let r = PointRef { x: &mut point.x, y: &mut point.y };

        *r.x = 5;
        *r.y = 6;
    }

    assert_eq!(5, point.x);
    assert_eq!(6, point.y);
}

/// ## Update Syntax
///
/// structs can include `..` to indicate a copy of some other structs 
/// values.
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

let mut point = Point3d { x: 0, y: 0, z: 0 }

point = Point3d { y: 1, .. point };
/// The above code changes the value of `y`, but copies (in this case,
/// retains) the value for `x` and `z`.
///
/// As implied, this works just as well with different instatiations of
/// the struct
let origin = Point3d { x: 0, y: 0, z: 0 };
let point = Point3d { z: 1, x: 2, .. origin }; // copies `y`

/// ## Tuple structs
///
/// tuple structs are a data type that mixes tuples and structs. They look
/// like this
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

/// The structs themselves have names, but the fields are anonymous. It is
/// generally better to use struct than a tuple struct, even in the cases
/// of the above examples for sake of clearer distinctions
struct Color {
    red: i32,
    blue: i32,
    green: i32,
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

/// The case where tuple structs become useful is when you need a 'newtype'
/// pattern (a struct with one variable that has semantic importance as 
/// a structure).
struct Inches(i32);

let length = Inches(10);

let Inches(integer_length) = length;
println!("length is {} inches", integer_length);

/// Extraction of inner integer types can be done, as above, with a 
/// destructuring `let`, which above assigns 10 to `integer_length`
///
/// ## Unit-like structs
///
/// A struct with no members is called 'unit-like' because of its 
/// resemblance to an empty tuple `()`, which is sometimes call a unit.
/// This is used to define a new type, as with tuple structs, but is
/// rarely useful except in compination with libraries or other features.
/// In these instances a primary use of these structs is to handle events
struct Electron;

let x = Electron;
