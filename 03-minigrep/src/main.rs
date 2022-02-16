// cargo run to poem.txt > output.txt // redirect output to a file

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args); // ["target\\debug\\minigrep.exe", "needle", "haystack"]
    // let query = &args[1];
    // let filename = &args[2];

    // if args.len() < 3 {
    //     panic!("not enough arguments");
    // }

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        // Write error messages to standard error instead of standard output
        process::exit(1);
    });
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
