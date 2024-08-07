// Constant is immutable, but there are some differences with `let`
// Constant never can be mutable
// Constant can be declared in any scope
// Constant's type should be annotated
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // x is immutable variable by default
    // Use `mut` keyword
    let mut x = 5;
    println!("The value of x is: {x}");
    // If x is immutable, this line will invoke compile error
    /*
    error[E0384]: cannot assign twice to immutable variable `x`
        --> main.rs:19:5
        |
        |     let x = 5;
        |         -
        |         |
        |         first assignment to `x`
        |         help: consider making this binding mutable: `mut x`
        ...
        |     x = 6;
        |     ^^^^^ cannot assign twice to immutable variable
    */
    x = 6;
    println!("The value of x is: {x}");

    // y is immutable variable
    let y = 5;

    // Re-declare y using original value of y
    // Shadowing
    // Shadowing allows us to store different type of value in variable
    // Mutable variable doesn't allow it
    let y = y + 1;

    {
        // Re-declare y in inner scope
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
}
