/*
Crate: Smallest amount of code that Rust compiler considers at a time
Crates contain modules

Forms of Crate
1. Binary Crate: executable program (has main)
2. Library Crate: define functionality (doesn't have main)

Crate Root: A source file that Rust compiler starts from

Package: bundle of one or more crates
Packages contain Cargo.toml (Cargo.toml -> Has information of how to build crate)

Packages can contain as many binary crates, but at most only one library crate
Packages must contain at least one crate
*/

use crate::garden::vegetables::Asparagus;

// let compiler include the code it finds in src/garden.rs
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
