pub fn run() {
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear(); // index becomes invalid

    // string slice
    // let s = String::from("hello world");
    // let hello = &s[0..5]; // &s[..5]
    // let world = &s[6..11]; // &s[6..]
    // let slice = &s[0..s.len()]; // &s[..]

    let s2 = String::from("hello world");
    let word2 = first_word(&s2); // we can pass references or slices
    println!("The first word is: {}", word2); // hello

    // array slice
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
