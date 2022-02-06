pub fn run() {
    println!("Hello, world!");
    // another_function(5);
    // print_labeled_measurement(5, 'h');

    // statements and expressions
    // let y = {
    //     let x = 3; // statement
    //     x + 1 // expression
    // };
    // println!("The value of y is: {}", y);

    // return value
    let x = five();
    println!("The value of x is: {}", x);
}

// fn another_function(x: i32) {
//     println!("The value of x is: {}", x);
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {}{}", value, unit_label);
// }

// return value
fn five() -> i32 {
    5
}