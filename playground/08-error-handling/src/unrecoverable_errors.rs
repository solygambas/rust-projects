pub fn run() {
    // panic!("crash and burn");
    // thread 'main' panicked at 'crash and burn', src\unrecoverable_errors.rs:2:5
    // 2nd line, 5th character

    let v = vec![1, 2, 3];
    v[99]; // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99'
}

// RUST_BACKTRACE=1 cargo run
//6: error_handling::unrecoverable_errors::run at .\src\unrecoverable_errors.rs:7
