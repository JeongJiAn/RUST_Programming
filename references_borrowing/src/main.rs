fn main() {
    let s1 = String::from("hello");

    // `&` is reference keyword
    let len = calculate_length(&s1);
    // s1 is not dropped after calling function because s1 is passed using reference

    println!("The length of '{s1}' is {len}.");

    // immutable reference's value can't be changed
    //let s = String::from("hello");
    //change(&s);

    let mut s = String::from("hello");
    // mutable reference is `&mut`
    change(&mut s);

    // You can create only one mutable variable at a time
    /*
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
    error[E0499]: cannot borrow `s` as mutable more than once at a time
        --> src/main.rs:5:14
        |
        |     let r1 = &mut s;
        |              ------ first mutable borrow occurs here
        |     let r2 = &mut s;
        |              ^^^^^^ second mutable borrow occurs here
        |
        |     println!("{}, {}", r1, r2);
        |                        -- first borrow later used here
    */

    // Also, if you want to borrow variable, you must use the variable before borrowing
    /*
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
    error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
        --> src/main.rs:6:14
        |
        |     let r1 = &s; // no problem
        |              -- immutable borrow occurs here
        |     let r2 = &s; // no problem
        |     let r3 = &mut s; // BIG PROBLEM
        |              ^^^^^^ mutable borrow occurs here
        |
        |     println!("{}, {}, and {}", r1, r2, r3);
        |                                -- immutable borrow later used here
    */

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

/*
fn change(some_string: &String) {
    some_string.push_str(", world");
}
*/

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}