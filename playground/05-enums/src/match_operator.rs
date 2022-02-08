#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

pub fn run() {
    // value_in_cents(Coin::Penny);
    // value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five); // 6
    let none = plus_one(None); // None

    dice_roll(9);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            1
        }
    }
}

// matching with Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// catch-all patterns
fn dice_roll(number: u8) {
    match number {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other), //  catch-all pattern
        // _ => reroll(), // placeholder pattern
        _ => (), // empty tuple, do nothing
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
fn move_player(num_spaces: u8) {
    println!("{}", num_spaces);
}
