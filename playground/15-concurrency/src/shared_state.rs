use std::sync::{Arc, Mutex}; // Arc stands for atomically reference counted, safe to share across threads vs Rc (which is not Send nor Sync)
use std::thread;

pub fn run() {
    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }
    // println!("m = {:?}", m); // m = Mutex { data: 6, poisoned: false, .. }

    // Sharing a Mutex<T> Between Multiple Threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap()); // 10
}
