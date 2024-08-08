use std::{error::Error as StdError, fmt::Display};

pub struct RpnCommand {
    calc: RpnCalc
}

impl RpnCommand {
    pub fn new() -> RpnCommand {
        RpnCommand { calc: RpnCalc::new() }
    }

    pub fn process_command(&mut self, command: &str) {
        let (command_type, command) = command.split_once(" ")
            .unwrap_or_else(|| (command, ""));
        match command_type {
            "help" => self.output_help(),
            "calc" => self.calc(command),
            "last" => self.last(),
            ""     => println!("Please input a command."),
            _      => { println!("Unrecognized command.")}
        };
    }

    fn calc(&mut self, command: &str) {
        match self.calc.calc(command) {
            Ok(result) => println!("Result was: '{result}'."),
            Err(err) => println!("Error: {:?}", err)
        }
    }

    fn last(&self) {
        if self.calc.last_input().is_some() && self.calc.last_result().is_some() {
          println!("Last equation was {} = {}", 
            self.calc.last_input().unwrap(), 
            self.calc.last_result().unwrap());
        } else {
            println!("No equations computed yet.");
        }
    }

    fn output_help(&self) {
        println!("\nCommand List:");
        println!("\tcalc <equation>: Calculates the RPN string following 'calc'.");
        println!("\tlast: Outputs information of the last calculation.");
        println!("\thelp: Outputs help information.");
        println!("\texit: Exits program.\n");
    }
}

pub struct RpnCalc {
    last_input: Option<String>,
    last_result: Option<i32>,
    stack: Vec<i32>
}

impl RpnCalc {
    pub fn new() -> RpnCalc {
        RpnCalc { last_input: None, last_result: None, stack: vec![] }
    }

    pub fn last_input(&self) -> Option<String> {
        match &self.last_input {
            Some(input) => Some(input.clone()),
            None => None
        }
    }

    pub fn last_result(&self) -> Option<i32> {
        self.last_result
    }

    pub fn calc(&mut self, input: &str) -> Result<i32, Box<dyn StdError>> {
        let input_str = input.to_string();
        self.last_input = Some(input_str.clone());
        let input_items = input_str.split_whitespace();
        
        for item in input_items {
            match item.to_string().parse::<i32>() {
                Ok(value) => {
                    self.stack.push(value)
                },
                Err(_) => {
                    self.calc_inner(self.validate_operator(item)?)?;
                }
            };
        }

        if self.stack.len() == 1 {
            self.last_result = self.stack.pop();
            Ok(self.last_result.unwrap())
        } else {
            Err(Box::new(Error {err_type: ErrorType::Parse}))
        }
    }

    fn calc_inner(&mut self, op: char) -> Result<(), Box<dyn StdError>> {
        if self.stack.len() < 2 {
            return Err(Box::new(Error {err_type: ErrorType::Parse}));
        }

        let val1 = self.stack.pop().unwrap();
        let val2 = self.stack.pop().unwrap();

        match op {
            '+' => self.stack.push(val1 + val2),
            '-' => self.stack.push(val1 - val2),
            '/' => self.stack.push(val1 / val2),
            '*' => self.stack.push(val1 * val2),
            _ => return Err(Box::new(Error{err_type: ErrorType::Operator}))
        }

        Ok(())
    }

    fn validate_operator(&self, value: &str) -> Result<char, Box<dyn StdError>> {
        let value = match value.chars().nth(0) {
            Some(char) => char,
            None => return Err(Box::new(Error{err_type: ErrorType::Operator}))
        };

        match value {
            '+' => Ok('+'),
            '-' => Ok('-'),
            '/' => Ok('/'),
            '*' => Ok('*'),
            _ => Err(Box::new(Error{err_type: ErrorType::Operator}))
        }
    }
}

#[derive(Debug)]
enum ErrorType {
    Parse,
    Operator
}

#[derive(Debug)]
struct Error {
    err_type: ErrorType
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return Ok(());
    }
}

impl StdError for Error {}