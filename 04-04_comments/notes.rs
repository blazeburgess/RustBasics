/// two types of comments:
///     -line comments
///     -doc comments
/// line comments use 2 `/` and explain the code
/// doc comments use 3 `/` and support Markdown notation
/// for example:
///
/// Adds one to the number given.
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #    x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}
/// One more type of doc comment uses `//!` and is used for comments
/// with items (crates, modules, functions), often found in crates
/// root (lib.rs) or modules root (mod.rs), as in:

//! # The Rust Standard Library
//!
//! The Rust Standard Library provides the essential runtime
//! functionality for building portable Rust software.

/// `rustdoc` can be run to generate html documentation from doc comments
