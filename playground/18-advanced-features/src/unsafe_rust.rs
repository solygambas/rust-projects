// use std::slice;

pub fn run() {
    // Dereferencing a Raw Pointer
    // let mut num = 5;
    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32;
    // unsafe {
    //     println!("r1 is {}", *r1); // r1 is 5
    //     println!("r2 is {}", *r2); // r2 is 5
    // }

    // Calling an Unsafe Function or Method
    // unsafe {
    //     dangerous();
    // }

    // let mut v = vec![1, 2, 3, 4, 5, 6];
    // let r = &mut v[..];
    // let (a, b) = r.split_at_mut(3);
    // assert_eq!(a, &mut [1, 2, 3]);
    // assert_eq!(b, &mut [4, 5, 6]);

    // Using extern Functions to Call External Code
    // unsafe {
    //     println!("Absolute value of -3 according to C: {}", abs(-3));
    // }

    // Accessing or Modifying a Mutable Static Variable
    println!("name is: {}", HELLO_WORLD);
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER); // 3
    }
}

// unsafe fn dangerous() {}

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();
//     let ptr = slice.as_mut_ptr();
//     assert!(mid <= len);

//     // (&mut slice[..mid], &mut slice[mid..]) // cannot borrow `*slice` as mutable more than once at a time
//     // Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice
//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
//         )
//     }
// }

// extern "C" {
//     fn abs(input: i32) -> i32;
// }

// #[no_mangle]
// pub extern "C" fn call_from_c() {
//     println!("Just called a Rust function from C!");
// }

static HELLO_WORLD: &str = "Hello, world";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Implementing an Unsafe Trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

// Accessing Fields of a Union
// A union is similar to a struct, but only one declared field is used in a particular instance at one time.
