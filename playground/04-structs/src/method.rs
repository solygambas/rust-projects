#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // can be useful as a getter
    fn width(&self) -> bool {
        self.width > 0
    }

    // more parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn run() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // false

    // call an associated function
    let sq = Rectangle::square(3);
    println!("The square is {:?}", sq);
}
