mod front_of_house {
    pub mod hosting {
        // let front_of_house refers to it
        pub fn add_to_waitlist() {} // let hosting refers to it
                                    // fn seat_at_table() {}
    }

    // mod serving {
    //     fn take_order() {}
    //     fn serve_order() {}
    //     fn take_payment() {}
    // }
}

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
                pub toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change your mind
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // enum
    let order1 = back_of_house::Appetizer::Soup;
    let order1 = back_of_house::Appetizer::Salad;

}
