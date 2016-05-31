// 'prelude' section is always at the top
extern crate rand; // extern for dependencies

use std::io; // library that allows user input
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new(); // let creates variable bindings; mut makes the binding have mutable references

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    // io::stdin() returns a handle, but .readline(&mut guess) actually takes
    // user input. .expect("Failed to read line") is split for aesthetic reasons.
    // Its purpose is to handle errors and return a relevant message.

    println!("You guessed: {}", guess); // {} is a standin for a variable in
    // strings. Works with several varables as in:
    //   println!("x and y: {} and {}", x, y);
}
