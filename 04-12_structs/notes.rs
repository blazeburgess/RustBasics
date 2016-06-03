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
