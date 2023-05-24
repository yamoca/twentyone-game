use std::io;
use std::io::Write;

fn main() {
    loop {
        println!("Welcome to twentyone");
        println!("1. Play");
        println!("2. Exit");
        let choice: u8 = input("choose an option: ").parse::<u8>().unwrap();
        match choice {
            1 => game(),
            2 => std::process::exit(000000),
            _ => println!("please choose a valid option (1 or 2)")
        } 
    }
    
}

fn game() {

}

fn input(text: &str) -> &str {
    print!("{}", text);
    io::stdout().flush().unwrap(); //remove newline character 

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    input = input.trim();
    return input;
}