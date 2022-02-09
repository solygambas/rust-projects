// use std::fs::File;
use std::fs;
// use std::io::ErrorKind;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

// pub fn run() {
// let f = File::open("hello.txt");

// with match
// let f = match f {
//     Ok(file) => file,
//     // Err(error) => panic!("Problem opening the file: {:?}", error),
//     Err(error) => match error.kind() {
//         ErrorKind::NotFound => match File::create("hello.txt") {
//             Ok(fc) => fc,
//             Err(e) => panic!("Problem creating the file: {:?}", e),
//         },
//         other_error => {
//             panic!("Problem opening the file: {:?}", other_error)
//         }
//     },
// };

// with closures
// let f = File::open("hello.txt").unwrap_or_else(|error| {
//     if error.kind() == ErrorKind::NotFound {
//         File::create("hello.txt").unwrap_or_else(|error| {
//             panic!("Problem creating the file: {:?}", error);
//         })
//     } else {
//         panic!("Problem opening the file: {:?}", error);
//     }
// });

// unwrap
// let f = File::open("hello.txt").unwrap(); // generic panic

// expect
// let f = File::open("hello.txt").expect("Failed to open hello.txt"); // custom message
// }

// propagating errors
// pub fn run() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// with the ? operator
// pub fn run() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// chaining methods
// pub fn run() -> Result<String, io::Error> {
//     let mut s = String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }

// with fs module
pub fn run() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// with option
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// with a trait object
fn any_kind_of_error() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?; //  with ?, an Err value can be returned early
    Ok(())
}
