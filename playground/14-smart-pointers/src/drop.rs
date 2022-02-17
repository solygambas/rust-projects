struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn run() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created: {} and {}.", c.data, d.data);
    // c.drop();
    drop(c); // call std::mem::drop to explicitly drop a value before it goes out of scope
    println!("CustomSmartPointer dropped before the end of main.");
}
