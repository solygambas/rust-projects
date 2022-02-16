use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args); // ["target\\debug\\minigrep.exe", "needle", "haystack"]
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}
