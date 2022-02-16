// cargo new 10-automated-tests --name=automated_tests --lib
// cargo test

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
        // self.width < other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn exploration() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    use super::*; // anything we define in the outer module is available to this tests module

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
}
