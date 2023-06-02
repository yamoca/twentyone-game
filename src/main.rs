use std::io;
use std::io::Write;



/* ---------------------------------------------

        TODO:
                merge input and get_valid_input
                    does it matter?`1
                    just get rid of get valid input and place logic inside main game loop
                
                find way for "choice" to be immutable (fix up while loop (for loop?))

*/

fn main() {
    loop {
        println!("Welcome to twentyone");
        println!("1. Play");
        println!("2. Exit");
        let choice: u8 = match input("choose an option: ").trim().parse::<u8>() {
            Ok(n) if n >= 1 && n <= 2 => n,
            _ => {
                println!("Invalid input, enter a valid number");
                main(); //hacky - fix at later date
                break;
            }
        };

        if choice == 1 { game(); }
        else { std::process::exit(2000) }
    }
    
}

fn game() {
    let mut score: u8 = 0;
    loop {
        let mut choice: u8 = get_valid_input();
        while choice >  0 {
            choice -= 1;
            score += 1;
            println!("{}", score);

            if score == 21 {
                println!("you lose");
                main(); //hacky find another way 
            }   
        }

    }

    
}

fn get_valid_input() -> u8 {
    loop {
        let num = input("enter increase to the total (1-3): ");
        match num.trim().parse::<u8>() {
            Ok(n) if n >= 1 && n <= 3 => return n,
            _ => println!("invalid"),
        }
    }
}

fn input(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap(); //remove newline character 

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    input = input.to_string();

    return input
}