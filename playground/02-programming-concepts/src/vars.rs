pub fn run() {
    // mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // constants
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x); // 12
    }
    println!("The value of x is: {}", x); // 6
    // change the type of the value but reuse the same name
    let spaces = "   "; // string
    let spaces = spaces.len(); // number
    // won't work with let mut spaces and spaces (not allowed to mutate a variable’s type)
}