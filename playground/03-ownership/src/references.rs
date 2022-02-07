pub fn run() {
    let s1 = String::from("hello");
    // borrowing
    let len = calculate_length(&s1); // & refers to some value without taking ownership of it (no need to return)
    println!("The length of '{}' is {}", s1, len);

    // mutable reference
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2); // hello, world
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, nothing happens to s1 because it does not have ownership on it

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
