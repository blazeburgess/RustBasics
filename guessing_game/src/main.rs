// 'prelude' section is always at the top
extern crate rand; // extern for dependencies

use std::io; // library that allows user input
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    // Uncomment for debugging:
    // println!("The secret number is: {}", secret_number);

    loop { // inputing 'quit' command at prompt can break an infinite loop
        println!("Please input your guess.");

        let mut guess = String::new(); // let creates variable bindings; mut makes the binding have mutable references

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        // io::stdin() returns a handle, but .readline(&mut guess) actually takes
        // user input. .expect("Failed to read line") is split for aesthetic reasons.
        // Its purpose is to handle errors and return a relevant message.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };      // explicitly converts guessed string to a number so it can 
                // be compared with secret_number.
                // Trimming removes any whitespace and the \n from user input, 
                // parse converts to a number, expect again handles errors.

        println!("You guessed: {}", guess); // {} is a standin for a variable in
        // strings. Works with several varables as in:
        //   println!("x and y: {} and {}", x, y);
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"), // Ordering is an enum
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You win!"); // use ',' within methods, but ';' here
                break;
            }
        }
    }
}
