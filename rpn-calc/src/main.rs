mod rpn;
use std::io::{self, Write};

fn main() {
    let mut command_calc = rpn::RpnCommand::new();

    println!("\n-- RPN Calculator --");
    println!("Type \"help\" for options, or \"exit\" to quit.");

    let console = io::stdin();
    let mut input = String::new();
    while input != "exit" {
        input.clear();
        print!("Input command: ");
        let _ = io::stdout().flush();
        console.read_line(&mut input).unwrap_or_else(|_| {panic!("I/O error");});
        input = input.trim().to_lowercase();
        command_calc.process_command(&input);
    }
}
