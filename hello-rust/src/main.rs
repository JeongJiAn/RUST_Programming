// compile : rustc main.rs
// in Windows, main.rs  main.exe  main.pdb
// in Linux/Mac, main.rs  main
// execute : ./main or cargo run main.rs

// fn FUNCTION_NAME() {} : Function definition
// main() : First code that runs in every executable Rust program
fn main() {
    println!("Hello, world!");
}
/*
Features
1. Indent w/ 4 tabs
2. `println!` -> runs Rust macro, w/o ! -> runs function
3. String "Hello, world"
4. End the line with ;
*/