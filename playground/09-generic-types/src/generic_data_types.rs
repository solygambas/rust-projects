// The process of monomorphization makes Rust’s generics extremely efficient at runtime.

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// pub fn run() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
//     let char_list = vec!['y', 'm', 'a', 'q'];
//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// structs
// struct Point<T> {
//     x: T,
//     y: T,
// }

// pub fn run() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let integer_and_float = Point { x: 5, y: 4.0 };
// }

// enums
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// in method definition
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

pub fn run() {
    let integer = Point { x: 5, y: 10 };
    println!("integer.x = {}", integer.x());
}
