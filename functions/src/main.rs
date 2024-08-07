fn main() {
    println!("Hello, world!");

    another_function();
    parameter_function(5);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        // Return value w/o semicolon
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

// MUST declare the type of each parameter
fn parameter_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Function w/ return value
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // Return value w/o semicolon
    x + 1
}