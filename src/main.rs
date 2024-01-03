use std::io::{self, Write};
use std::env;

fn interactive() {
    println!("Welcome to basic calculator. Your choices: \"add\", \"sub\", \"mult\", \"div\"");
    
    print!("Your choice: ");
    io::stdout().flush().unwrap(); // Print to the terminal and panic if we can't.

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

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
    let num1: i32 = num1.trim().parse().expect("Requires a number.");
    let num2: i32 = num2.trim().parse().expect("Requires a number.");

    // Perform the desired operation
    match input.as_str().trim() {
        "add" => {
            let res = num1 + num2;
            println!("{num1} + {num2} = {res}");
        },
        "sub" => {
            let res = num1 - num2;
            println!("{num1} - {num2} = {res}");
        },
        "mult" => {
            let res = num1 * num2;
            println!("{num1} * {num2} = {res}");
        },
        "div" => {
            let res = num1 / num2;
            println!("{num1} / {num2} = {res}");
        },
        rest => {
            println!("Invalid option '{rest}'!")
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
    let mode = args[1].as_str();
    let mut total = numbers.remove(0);

    for item in numbers {
        match mode {
            "add" => total += item,
            "sub" => total -= item,
            "mult" => total *= item,
            "div" => total /= item,
            opp => {
                println!("Invalid operation \"{opp}\".");
                return None;
            }
        }
    }

    println!("{total}");
    return Some(total);
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

    fn generate_args(operation: String, numbers: Vec<i32>) -> Vec<String> {
        let mut vec = vec![String::from("target/test"), operation];
    
        for item in numbers {
            vec.push(item.to_string());
        }
    
        return vec;
    }

    fn test_cli(operation: &str, numbers: Vec<i32>, solution: i32) {
        let args = generate_args(operation.to_string(), numbers);
        let result = cli(args);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), solution);
    }

    #[test]
    fn cli_addition() {
        test_cli("add", vec![1, 2], 3);
    }

    #[test]
    fn cli_subtraction() {
        test_cli("sub", vec![10, 3], 7);
    }

    #[test]
    fn cli_multiplication() {
        test_cli("mult", vec![5, 5], 25);
    }

    #[test]
    fn cli_division() {
        test_cli("div", vec![30, 3], 10);
    }
}