// Vectors are resizable arrays
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    // Re-assign value
    numbers[2] = 20;
    // Add on to vector
    numbers.push(6);
    println!("{:?}", numbers);
    // Pop off last value
    numbers.pop();
    println!("{:?}", numbers);
    // Get single value
    println!("Single value: {}", numbers[0]); // 1
    // Get vector length
    println!("Vector length: {}", numbers.len()); // 5
    // vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers)); // 24
    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice); // [1, 2]
    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers); // [2, 4, 40, 8, 10]
}