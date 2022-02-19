pub fn run() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer); // 12

    let list_of_numbers = vec![1, 2, 3];
    // let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop,
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
