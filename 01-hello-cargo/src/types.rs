// Primitive types
// Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (unsigned: no negative values)
// Floats: f32, f64
// Boolean (bool)
// Characters (char)
// Tuples
// Arrays

// Rust is a statically typed language, but it can infer what type we want to use (it is not required to set the type).

pub fn run() {
// Default is "i32"
let x = 1;
// Default is "f64"
let y = 2.5;
// Add explicit type
let z: i64 = 4545445454545;
// Find max size
println!("Max i32: {}", std::i32::MAX); // 2147483647
println!("Max i64: {}", std::i64::MAX); // 9223372036854775807
// Boolean
// let is_active = true;
let is_active: bool = true;
// Get boolean from expression
let is_greater: bool = 10 > 5; // true
// Char with single quotes
let a1 = 'a';
let face = '\u{1F600}'; // emoji unicode: https://unicode.org/emoji/charts/full-emoji-list.html
println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}