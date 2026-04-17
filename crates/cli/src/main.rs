use std::process;

use cli::input;

fn main() {
    let input = input::read("Enter a non-negative number:").unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    let parsed_input = input::parse::<u16>(&input).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    println!("You entered {parsed_input}.");
}
