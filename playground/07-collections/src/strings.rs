pub fn run() {
    // creating
    let mut s = String::new();
    let data = "initial contents";
    let s2 = data.to_string();
    let s3 = "initial contents".to_string();
    let s4 = String::from("initial contents");

    // updating
    let mut s5 = String::from("foo");
    s5.push_str("bar"); // foobar
    let mut s6 = String::from("lo");
    s.push('l'); // lol

    // concatenating
    let s7 = String::from("Hello, ");
    let s8 = String::from("world!");
    let s9 = s7 + &s8; // Hello, world!  // note s1 has been moved here and can no longer be used: fn add(self, s: &str)

    let s10 = String::from("tic");
    let s11 = String::from("tac");
    let s12 = String::from("toe");
    let s13 = format!("{}-{}-{}", s10, s11, s12); // tic-tac-toe

    // slicing
    let hello = "Здравствуйте";
    let s14 = &hello[0..4]; // 4 bytes, each character is 2 bytes: Зд

    // iterating
    // Unicode scalar values
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // raw byte
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
