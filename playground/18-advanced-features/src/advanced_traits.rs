// Specifying Placeholder Types in Trait Definitions with Associated Types
// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

// struct Counter {
//     count: u32,
// }

// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {}
// }

// Default Generic Type Parameters and Operator Overloading
// use std::ops::Add;

// #[derive(Debug, Copy, Clone, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Add for Point {
//     type Output = Point;

//     fn add(self, other: Point) -> Point {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// trait Add<Rhs = Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
//     // The Rhs generic type parameter (short for “right hand side”) defines the type of the rhs parameter in the add method.
// }

// struct Millimeters(u32);
// struct Meters(u32);

// impl Add<Meters> for Millimeters {
//     type Output = Millimeters;

//     fn add(self, other: Meters) -> Millimeters {
//         Millimeters(self.0 + (other.0 * 1000))
//     }
// }

// pub fn run() {
//     assert_eq!(
//         Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
//         Point { x: 3, y: 3 }
//     );
// }

// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
// trait Pilot {
//     fn fly(&self);
// }

// trait Wizard {
//     fn fly(&self);
// }

// struct Human;

// impl Pilot for Human {
//     fn fly(&self) {
//         println!("This is your captain speaking.");
//     }
// }

// impl Wizard for Human {
//     fn fly(&self) {
//         println!("Up!");
//     }
// }

// impl Human {
//     fn fly(&self) {
//         println!("*waving arms furiously*");
//     }
// }

// pub fn run() {
//     let person = Human;
//     Pilot::fly(&person); // This is your captain speaking.
//     Wizard::fly(&person); // Up!
//     person.fly(); // *waving arms furiously*
// }

// trait Animal {
//     fn baby_name() -> String;
// }

// struct Dog;

// impl Dog {
//     fn baby_name() -> String {
//         String::from("Spot")
//     }
// }

// impl Animal for Dog {
//     fn baby_name() -> String {
//         String::from("puppy")
//     }
// }

// pub fn run() {
//     // println!("A baby dog is called a {}", Dog::baby_name()); // A baby dog is called a Spot
//     // println!("A baby dog is called a {}", Animal::baby_name()); //  Rust can’t figure out which implementation of Animal::baby_name we want
//     println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // A baby dog is called a puppy
//     // <Type as Trait>::function(receiver_if_method, next_arg, ...);
// }

// Using Supertraits to Require One Trait’s Functionality Within Another Trait
// use std::fmt;

// trait OutlinePrint: fmt::Display {
//     fn outline_print(&self) {
//         let output = self.to_string();
//         let len = output.len();
//         println!("{}", "*".repeat(len + 4));
//         println!("*{}*", " ".repeat(len + 2));
//         println!("* {} *", output);
//         println!("*{}*", " ".repeat(len + 2));
//         println!("{}", "*".repeat(len + 4));
//     }
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// // impl OutlinePrint for Point {}
// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

// Using the Newtype Pattern to Implement External Traits on External Types
// use std::fmt;

// struct Wrapper(Vec<String>);

// impl fmt::Display for Wrapper {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.0.join(", "))
//     }
// }

// pub fn run() {
//     let w = Wrapper(vec![String::from("hello"), String::from("world")]);
//     println!("w = {}", w);
// }
