// https://doc.rust-lang.org/std/vec/struct.Vec.html

pub fn run() {
    // creating
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];

    // updating
    // let mut v = Vec::new();
    // v.push(5);
    // v.push(7);

    // reading
    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("The third element is {}", third);
    // match v.get(2) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }

    // deleting
    // let mut v = vec![1, 2, 3, 4, 5];
    // v.pop();

    // iterating
    // let v = vec![100, 32, 57];
    // for integer in &v {
    //     println! {"{}", integer}; // 100 32 57
    // }
    // let mut v = vec![100, 32, 57];
    // for integer in &mut v {
    //     *integer += 50;
    //     println! {"{}", integer}; // 150 82 107
    // }

    // using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}
