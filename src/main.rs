use std::env;
use std::io::{self, Write};
use std::num::ParseIntError;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn from_str(name: &str) -> Option<Operation> {
        match name {
            "add" => Some(Operation::Add),
            "sub" | "subtract" => Some(Operation::Subtract),
            "mult" | "multiply" => Some(Operation::Multiply),
            "div" | "divide" => Some(Operation::Divide),
            _ => None,
        }
    }
}

fn interactive() {
    println!("Welcome to basic calculator. Your choices: \"add\", \"sub\", \"mult\", \"div\", or \"exit\" to exit.");

    print!("Your choice: ");
    io::stdout().flush().unwrap(); // Print to the terminal and panic if we can't.

    let mut input_operation = String::new();
    io::stdin()
        .read_line(&mut input_operation)
        .expect("Failed to read input.");

    let input_operation = input_operation.trim();

    if input_operation == "exit" {
        println!("Goodbye!");
        return;
    }

    let operation = Operation::from_str(input_operation.trim());

    if operation.is_none() {
        println!("Invalid operation.");
        return;
    }

    let operation = operation.unwrap();

    print!("First number: ");
    io::stdout().flush().unwrap();

    // Get the first number
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read input.");

    print!("Second Number: ");
    io::stdout().flush().unwrap();

    // Get the second number
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read input.");

    // Parse the strings into integers
    let num1: Result<i32, ParseIntError> = num1.trim().parse();
    let num2: Result<i32, ParseIntError> = num2.trim().parse();

    let parse_err = "Error: Cannot convert the specified number(s) to integers.";

    // Handle the error instead of crashing the program so the user
    // understands what went wrong
    if let Err(_) = num1 {
        println!("{parse_err}");
        return;
    } else if let Err(_) = num2 {
        println!("{parse_err}");
        return;
    }
    let num1 = num1.unwrap();
    let num2 = num2.unwrap();
    

    // Perform the desired operation
    match operation {
        Operation::Add => {
            let res = num1 + num2;
            println!("{num1} + {num2} = {res}");
        }
        Operation::Subtract => {
            let res = num1 - num2;
            println!("{num1} - {num2} = {res}");
        }
        Operation::Multiply => {
            let res = num1 * num2;
            println!("{num1} * {num2} = {res}");
        }
        Operation::Divide => {
            let res = num1 / num2;
            println!("{num1} / {num2} = {res}");
        }
    }
}

fn cli(args: Vec<String>) -> Option<i32> {
    let mut numbers: Vec<i32> = vec![];

    // Turn every number in the args to a i32 to put in the vec
    for item in &args {
        let possible_num = item.parse::<i32>();
        if let Ok(number) = possible_num {
            numbers.push(number);
        }
    }

    // Assume the operation is the first argument
    // Also assume it is not a flag, should be just a word
    let operation = Operation::from_str(args[1].trim());
    let mut total = numbers.remove(0);

    if operation.is_none() {
        println!("Invalid operation.");
        return None;
    }
    let operation = operation.unwrap();

    for item in numbers {
        match operation {
            Operation::Add => total += item,
            Operation::Subtract => total -= item,
            Operation::Multiply => total *= item,
            Operation::Divide => total /= item,
        }
    }

    println!("{total}");
    Some(total)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        interactive();
    } else {
        cli(args);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operation_to_string(operation: Operation) -> String {
        match operation {
            Operation::Add => String::from("add"),
            Operation::Subtract => String::from("sub"),
            Operation::Multiply => String::from("mult"),
            Operation::Divide => String::from("div"),
        }
    }

    fn generate_args(operation: Operation, numbers: Vec<i32>) -> Vec<String> {
        let mut vec = vec![String::from("target/test"), operation_to_string(operation)];

        for item in numbers {
            vec.push(item.to_string());
        }

        return vec;
    }

    fn test_cli(operation: Operation, numbers: Vec<i32>, solution: i32) {
        let args = generate_args(operation, numbers);
        let result = cli(args);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), solution);
    }

    #[test]
    fn cli_addition() {
        test_cli(Operation::Add, vec![1, 2], 3);
    }

    #[test]
    fn cli_subtraction() {
        test_cli(Operation::Subtract, vec![10, 3], 7);
    }

    #[test]
    fn cli_multiplication() {
        test_cli(Operation::Multiply, vec![5, 5], 25);
    }

    #[test]
    fn cli_division() {
        test_cli(Operation::Divide, vec![30, 3], 10);
    }
}
