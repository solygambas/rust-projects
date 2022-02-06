pub fn run() {
    // floating-point types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // boolean
    let t = true;
    let f: bool = false;

    // character type
    let z = 'Z';
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    // tuples
    let tup = (500, 6.4, "John");
    let (x, y, z) = tup; // destructuring
    println!("The value of y is {}", y);
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    // if you try a[5], runtime error - index out of bounds: the len is 4 but the index is 5
}