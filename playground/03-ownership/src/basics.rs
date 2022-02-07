// pub fn run() {
//     // String: for a value we don't know when we write our code
//     let mut s = String::from("hello"); // pointer, length and capacity are in the stack, the heap holds the content "hello"
//     s.push_str(", world!");
//     println!("{}", s);
// } // drop is called by Rust: s is cleaned from memory

// pub fn run() {
//     // stack data
//     let x = 5;
//     let y = x; // shallow copy
//     println!("x = {}, y = {}", x, y);
//     // heap data
//     let s1 = String::from("hello");
//     let s2 = s1.clone(); // without clone, s1 is dropped to avoid "double free error"
//     println!("s1 = {}, s2 = {}", s1, s2);
// }

pub fn run() {
    let s = String::from("hello");
    takes_ownership(s);
    // s is no longer valid here (borrow of moved value)
    let x = 5;
    makes_copy(x); // shallow copy
    println!("{}", x); // x is still valid
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
