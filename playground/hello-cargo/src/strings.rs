// Primitive str = immutable fixed-length string
// String = growable

pub fn run() {
    let mut hello = String::from("Hello ");
    // Get length
    println!("Length: {}", hello.len());
    // Add char
    hello.push('W');
    // Add string
    hello.push_str("orld!");
    println!("{}", hello);
    // Get capacity in bytes
    println!("Capacity: {}", hello.capacity());
    // Check if empty
    println!("Is empty: {}", hello.is_empty()); // false
    // Contains
    println!("Contains 'World': {}", hello.contains("World")); // true
    // Replace
    println!("Replace: {}", hello.replace("World", "There")); // Hello There
    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);
    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}