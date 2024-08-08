/*
Ownership : Set of rules how Rust programs manages memory
How to manage memory
1. Garbage Collector - Java
2. Explicitly allocate and free the memory - C/C++
3. Ownership (new!) - Rust

In Ownership, memory is managed in COMPILE TIME
Compiler checks the set of rules(ownership) and if any of the rules are violated, compile error occurs.

Ownership Rules
1. Each value has an owner
2. There can only be one owner at a time
3. When owner goes out of scope, value will be dropped


*/

fn main() {
    // Literals can't be mutable
    // Literals are stored in stack (compile-time)
    let s = "hello";

    // String can be mutable
    // String is stored in heap (run-time)
    // String::from() -> requests the memory
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print `hello, world!`

    {
        // s is valid from this point forward
        let s = String::from("hello"); 
        // do stuff with s
    } // this scope is now over, and s is no longer valid (freed)
    // When the scope is over, drop() function is called -> return the memory of s

    // s1 is a pointer of string "hello"
    let s1 = String::from("hello");
    // s1's value(string) can't be copied by simple assignment
    // s1 and s2 point the same memory address of heap -> cause dangling pointer
    let s2 = s1;
    // Using s1 is not allowed (compile error)
    // s1 is MOVED to s2, so s1 is invalidated
    // println!("{s1}, world!");
    // Instead, using s2 is allowed
    // This is kind of shallow copy (not deep copy)
    println!("{s2}, world!");

    // If you wan't deep copy, use clone()
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // s comes into scope
    let s = String::from("hello");  
    // s's value moves into the function and so is no longer valid here
    takes_ownership(s);             

    // x comes into scope
    let x = 5;                      
    // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
    makes_copy(x);                  

    // gives_ownership moves its return value into s1
    let s1 = gives_ownership();         

    // s2 comes into scope
    let s2 = String::from("hello");     

    // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    let s3 = takes_and_gives_back(s2);  

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");
}

// some_string comes into scope
fn takes_ownership(some_string: String) { 
    println!("{some_string}");
    // Here, some_string goes out of scope and `drop` is called. 
    // The backing memory is freed.
} 

// some_integer comes into scope
fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
    // Here, some_integer goes out of scope. Nothing special happens.
} 

// gives_ownership will move its return value into the function that calls it
fn gives_ownership() -> String {             

    // some_string comes into scope
    let some_string = String::from("yours");

    // some_string is returned and moves out to the calling function
    some_string                              
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    // len() returns the length of a String
    let length = s.len(); 

    // Returning multiple values using tuple
    (s, length)
}