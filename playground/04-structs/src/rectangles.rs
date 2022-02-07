// pub fn run() {
//     let width1 = 30;
//     let height1 = 50;
//     println!(
//         "The area of the rectangle is {} square pixels",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// refactoring with tuples
// pub fn run() {
//     let rect1 = (30, 50);
//     println!("The area of the rectangle is {} square pixels", area(rect1));
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// refactoring with structs

#[derive(Debug)] // debug trait

struct Rectangle {
    width: u32,
    height: u32,
}

pub fn run() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1); // on several lines
    dbg!(&rect1); // figure out what your code is doing
    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
