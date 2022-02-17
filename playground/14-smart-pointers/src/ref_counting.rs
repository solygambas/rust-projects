#[derive(Debug)]

enum List {
    // Cons(i32, Rc<List>),
    Cons(Rc<RefCell<i32>>, Rc<List>), // By using RefCell<T>, we have an outwardly immutable List value.
    Nil,
}

use self::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a)); // 1
    // let b = Cons(3, Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a)); // 2
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    // }
    // println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2

    // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a); //     a after = Cons(RefCell { value: 15 }, Nil)
    println!("b after = {:?}", b); // b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    println!("c after = {:?}", c); // c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
}
