use std::io;
use meval::eval_str;

fn main() {
    println!("Enhanced CLI Calculator");
    println!("Enter your calculation (e.g., 3 + 4 * (2 - 1)) or type 'exit' to quit:");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        if trimmed_input.eq_ignore_ascii_case("exit") || trimmed_input.eq_ignore_ascii_case("quit") {
            println!("Exiting calculator. Goodbye!");
            break;
        }

        if trimmed_input.is_empty() {
            continue;
        }

        match eval_str(trimmed_input) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => eprintln!("Error: Invalid expression. Details: {}", e),
        }
    }
}
