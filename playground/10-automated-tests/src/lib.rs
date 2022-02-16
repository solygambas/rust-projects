// cargo new 10-automated-tests --name=automated_tests --lib
// cargo test
// cargo test -- --test-threads=1 // will take longer, but the tests won't interfere with each other
// cargo test -- --show-output // see printed values for passing tests as well
// cargo test one_hundred // pass the name of any test function to cargo test to run only that test
// cargo test add // Filtering to Run Multiple Tests, 2 tests are prefixed with add
// cargo test -- --ignored // run only the ignored test

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//         // self.width < other.width && self.height > other.height
//     }
// }

pub fn add_two(a: i32) -> i32 {
    // a + 2
    // a + 3
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// pub fn greeting(name: &str) -> String {
//     // format!("Hello {}!", name)
//     String::from("Hello!")
// }

// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         // if value < 1 || value > 100 {
//         // if value < 1 {
//         //     // test did not panic as expected
//         //     panic!(
//         //         "Guess value must be greater than or equal to 1, got {}.",
//         //         value
//         //     );
//         // } else if value > 100 {
//         //     panic!(
//         //         "Guess value must be less than or equal to 100, got {}.",
//         //         value
//         //     );
//         // }
//         if value < 1 {
//             panic!(
//                 "Guess value must be less than or equal to 100, got {}.",
//                 value
//             );
//         } else if value > 100 {
//             panic!(
//                 "Guess value must be greater than or equal to 1, got {}.",
//                 value
//             );
//         }
//         Guess { value }
//     }
// }

#[cfg(test)] // the configuration option is test
mod tests {
    // #[test]
    // fn exploration() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    use super::*; // anything we define in the outer module is available to this tests module

    // Checking Results with the assert! Macro
    // #[test]
    // fn larger_can_hold_smaller() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 1,
    //     };
    //     assert!(larger.can_hold(&smaller));
    // }

    // #[test]
    // fn smaller_cannot_hold_larger() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 1,
    //     };
    //     assert!(!smaller.can_hold(&larger));
    // }

    // Testing Equality with the assert_eq! and assert_ne! Macros
    // #[test]
    // fn it_adds_two() {
    //     // assert_eq!(4, add_two(2)); // 'assertion failed: `(left == right)`  left: `4`, right: `5`'
    //     assert_ne!(4, add_two(2));
    // }

    // Adding Custom Failure Messages
    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     // assert!(result.contains("Carol"));
    //     assert!(
    //         result.contains("Carol"),
    //         "Greeting did not contain name, value was `{}`", // Hello!
    //         result
    //     );
    // }

    // Checking for Panics with should_panic
    // #[test]
    // // #[should_panic]
    // #[should_panic(expected = "Guess value must be less than or equal to 100")]
    // // a substring is enough
    // // we could have write Guess value must be less than or equal to 100, got 200.
    // // panic message: `"Guess value must be greater than or equal to 1, got 200."`,
    // // expected substring: `"Guess value must be less than or equal to 100"`
    // fn greater_than_100() {
    //     Guess::new(200);
    // }

    // Using Result<T, E> in Tests
    // #[test]
    // fn it_works() -> Result<(), String> {
    //     if 2 + 2 == 4 {
    //         Ok(())
    //     } else {
    //         Err(String::from("two plus two does not equal four"))
    //     }
    // }

    // Running Single Tests
    // #[test]
    // fn add_two_and_two() {
    //     assert_eq!(4, add_two(2));
    // }

    // #[test]
    // fn add_three_and_two() {
    //     assert_eq!(5, add_two(3));
    // }

    // #[test]
    // fn one_hundred() {
    //     assert_eq!(102, add_two(100));
    // }

    // // Ignoring Some Tests Unless Specifically Requested
    // #[test]
    // #[ignore]
    // fn expensive_test() {
    //     // code that takes an hour to run
    // }

    // Testing a private function is permitted by Rust
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
