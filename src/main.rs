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

fn cli(args: Vec<String>) {
    let mut total = 0;
    let mut numbers: Vec<i32> = vec![];

    // Turn every number in the args to a i32 to put in the vec
    for item in &args {
        if item.parse::<i32>().is_ok() {
            numbers.push(item.parse::<i32>().unwrap());
        }
    }

    // Check what operation the user wants to perform
    if args.contains(&String::from("--add")) {
        for item in numbers { // Calculate the result
            total += item;
        }
    } else if args.contains(&String::from("--sub")) {
        total = numbers.remove(0); 
        for item in numbers {
            total -= item;
        }
    } else if args.contains(&String::from("--mult")) {
        total = numbers.remove(0);
        for item in numbers {
            total *= item;
        }
    } else if args.contains(&String::from("--div")) {
        total = numbers.remove(0);
        for item in numbers {
            total /= item;
        }
    } 

    println!("{total}");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        interactive();
    } else {
        cli(args);
    }
}
