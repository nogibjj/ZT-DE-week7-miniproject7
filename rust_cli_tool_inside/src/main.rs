use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "greet" => {
            if args.len() < 3 {
                println!("Please provide a name to greet.");
            } else {
                println!("Hello, {}!", args[2]);
            }
        }
        "add" => {
            if args.len() < 4 {
                println!("Please provide two numbers to add.");
            } else {
                let num1: i32 = args[2].parse().unwrap_or(0);
                let num2: i32 = args[3].parse().unwrap_or(0);
                println!("The sum is: {}", num1 + num2);
            }
        }
        _ => {
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Usage:");
    println!("rust_cli_tool greet [NAME]");
    println!("rust_cli_tool add [NUM1] [NUM2]");
}

