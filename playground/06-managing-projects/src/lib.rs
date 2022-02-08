mod front_of_house;

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // go to crate (root), the parent module of back_of_house
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// pub fn eat_at_restaurant() {
//     // Order a breakfast
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // Change your mind
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
//     // enum
//     let order1 = back_of_house::Appetizer::Soup;
//     let order1 = back_of_house::Appetizer::Salad;

// }

// use keyword
use crate::front_of_house::hosting;
// use self::front_of_house::hosting;

// full path for structs, enums, etc.
use std::collections::HashMap;

// as
use std::io::Result as IoResult;

// re-exporting
// pub use crate::back_of_house::Breakfast;

// external package
use rand::Rng;

// nested paths
// use std::{cmp::Ordering, io};
use std::io::{self, Write};

// glob operator
use std::collections::*;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    let mut map = HashMap::new();
    map.insert(1, 2);
}
