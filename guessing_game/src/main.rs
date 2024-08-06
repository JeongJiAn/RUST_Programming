// including Input/Output library
// std library holds io library
// By default, every Rust programs have std library(prelude)
// If you want to use other library, use `use` keyword
use std::io;
// Add dependency in Cargo.tml
// Rand Number Generator
use rand::Rng;
// `Ordering` type is enum (Less, Greater, Equal)
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // immutable variable
    // 1 to 100
    // thread_rng() -> Returns random number that is local to the current thread and seeded by the OS
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    // Infinite Loop
    loop {
        println!("Please input your guess.");

        // `let` : Create variable
        // `mut` : mutable
        // let -> immutable variable, let mut -> mutable variable
        // immutable is default
        // String::new() -> Create new, empty
        // new is associated function fo String type
        let mut guess = String::new();
    
        // io::stdin() -> Get user input
        // If not used `use std::io`, std::io::stdin() instead 
        io::stdin()
            // Read user input and append it to variable(parameter)
            // `&` means reference
            // This functions returns `Result` type (enum -> Ok, Err)
            .read_line(&mut guess)
            // If value of Return type is Err, call expect()
            // Crash the program and display the message
            // If you don't use expect(), warning message will be shown in compilation process
            .expect("Failed to read line");
    
        // Shadow the previous value of `guess` with new one
        // Shadowing : reuse the guess variable name
        // unsigned int (32bit number)
        // trim() -> Eliminate white characters
        // parse() -> Converts String to another type
        // `:` -> Annotate variable's type (u32)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        // Formatted print
        println!("You guessed: {}", guess);
    
        // cmp() -> compares two values
        // Returns Ordering type
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }
    }
    
}
