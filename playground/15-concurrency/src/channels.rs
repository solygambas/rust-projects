// Go: “Do not communicate by sharing memory; instead, share memory by communicating.”
// mpsc stands for multiple producer, single consumer
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    let (tx, rx) = mpsc::channel(); // transmitter, receiver

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap(); // unwrap to panic in case of an error // ownership of val is transferred
    // });
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // Sending Multiple Values and Seeing the Receiver Waiting
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     println!("Got: {}", received);
    // }

    // Creating Multiple Producers by Cloning the Transmitter
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
