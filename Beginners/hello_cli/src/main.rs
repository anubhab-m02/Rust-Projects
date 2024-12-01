use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let name = &args[1];
        println!("Hello, {}!", name);
    } else {
        print!("Please enter your name: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        let name = name.trim();

        println!("Hello, {}!", name);
    }
}
