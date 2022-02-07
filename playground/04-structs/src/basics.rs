pub fn run() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);

    // struct update syntax
    let user2 = User {
        email: String::from("anotheremail@example.com"),
        ..user1
    };
    println!("{}", user2.username);

    // tuple struct
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    // unit-like structs
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // field init shorthand
        username,
        active: true,
        sign_in_count: 1,
    }
}
