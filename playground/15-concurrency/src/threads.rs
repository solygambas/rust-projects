use std::thread;
// use std::time::Duration;

// pub fn run() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//     // handle.join().unwrap(); // The main thread will wait for the spawned thread to finish
//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
//     handle.join().unwrap(); // Waiting for All Threads to Finish Using join Handles
// }

// Using move Closures with Threads
pub fn run() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // force the closure to take ownership of v
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}
