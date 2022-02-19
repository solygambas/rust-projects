pub fn run() {
    // Creating Type Synonyms with Type Aliases
    // type Kilometers = i32;
    // let x: i32 = 5;
    // let y: Kilometers = 5;
    // println!("x + y = {}", x + y); // 10

    // let f: Thunk = Box::new(|| println!("hi"));
}

// type Thunk = Box<dyn Fn() + Send + 'static>;

// fn takes_long_type(f: Thunk) {
//     // --snip--
// }

// The Never Type that Never Returns
// fn bar() -> ! {
//     print!("forever ");
//     loop {
//         print!("and ever ");
//     }
// }
