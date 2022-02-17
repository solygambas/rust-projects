use std::ops::Deref;

pub fn run() {
    let x = 5;
    // let y = &x;
    // let y = Box::new(x);
    // let y = MyBox::new(x);
    // assert_eq!(5, x);
    // assert_eq!(5, *y); // follow the reference to the value it’s pointing to (hence dereference)
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

// Defining Our Own Smart Pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Treating a Type Like a Reference by Implementing the Deref Trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implicit Deref Coercions with Functions and Methods
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
