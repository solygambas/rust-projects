// use self::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]

// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),
//             Nil => None,
//         }
//     }
// }

// pub fn run() {
//     let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
//     println!("a initial rc count = {}", Rc::strong_count(&a));
//     println!("a next item = {:?}", a.tail());

//     let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
//     println!("a rc count after b creation = {}", Rc::strong_count(&a));
//     println!("b initial rc count = {}", Rc::strong_count(&b));
//     println!("b next item = {:?}", b.tail());

//     // We modify a so it points to b instead of Nil, creating a cycle
//     if let Some(link) = a.tail() {
//         *link.borrow_mut() = Rc::clone(&b);
//     }

//     println!("b rc count after changing a = {}", Rc::strong_count(&b));
//     println!("a rc count after changing a = {}", Rc::strong_count(&a));

//     // Uncomment the next line to see that we have a cycle;
//     // Rust will try to print this cycle with a pointing to b pointing to a and so forth until it overflows the stack
//     // println!("a next item = {:?}", a.tail());
// }

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn run() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // leaf parent = None
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    ); // leaf strong = 1, weak = 0

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        ); // branch strong = 1, weak = 1 (for leaf.parent pointing to branch with a Weak<Node>)

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        ); // leaf strong = 2, weak = 0 (branch now has a clone of the Rc<Node> of leaf stored in branch.children)
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // leaf parent = None, branch is out of scope // we don’t get any memory leaks!
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    ); // leaf strong = 1, weak = 0
}
