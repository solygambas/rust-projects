// Arrays are fixed lists where elements are the same data type
use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // Re-assign value
    numbers[2] = 20;
    println!("{:?}", numbers);
    // Get single value
    println!("Single value: {}", numbers[0]); // 1
    // Get array length
    println!("Array length: {}", numbers.len()); // 5
    // Arrays are stack allocated
    // println!("Array occupies {} bytes", std::mem::size_of_val(&numbers)); // 24
    println!("Array occupies {} bytes", mem::size_of_val(&numbers)); // 24
    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice); // [1, 2]
}