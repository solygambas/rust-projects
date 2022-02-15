fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // the lifetime of the reference returned by the longest function
    // is the same as the smaller of the lifetimes of the references passed in
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn run() {
    // let x = 5;
    // let r = &x;
    // println!("r: {}", r);

    // generic lifetimes
    // let string1 = String::from("abcd");
    // let string2 = "xyz";
    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let s: &'static str = "I have a static lifetime.";
}
