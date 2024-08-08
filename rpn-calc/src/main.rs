mod rpn;
use std::io::{self, Read, Write};

fn main() {
    let mut calc = rpn::RpnCalc::new();

    println!("\n-- RPN Calculator --");
    println!("Type \"help\" for options, or \"exit\" to quit.");

    let mut console = io::stdin();
    let mut input = String::new();
    while input != "exit" {
        input.clear();
        print!("Input command: ");
        let _ = io::stdout().flush();
        console.read_line(&mut input).unwrap_or_else(|_| {panic!("I/O error");});
        let input = input.trim().to_lowercase();

        println!("Got command {input}");
    }
}
