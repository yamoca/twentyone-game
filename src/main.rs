use std::io;
use std::io::Write;

fn main() {
    loop {
        println!("Welcome to twentyone");
        println!("1. Play");
        println!("2. Exit");
        let choice: u8 = match input("choose an option: ").trim().parse::<u8>() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input, enter a valid number");
                main(); //hacky - fix at later date
                break;
            }
        };

        match choice {
            1 => {
                game();
                break;
            }
            2 => std::process::exit(000000),
            _ => println!("Failed to match input to choice, enter a valid number between 1 and 2")
        } 
    }
    
}

fn game() {
    println!("HAHAH");
}

fn input(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap(); //remove newline character 

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    input = input.to_string();

    return input
}