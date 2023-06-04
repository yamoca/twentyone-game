use std::io;
use std::io::Write;



/* ---------------------------------------------

        TODO:
                find way for "choice" to be immutable (fix up while loop (for loop?))

*/

fn main() {
    loop {
        println!("Welcome to twentyone");
        println!("1. Play");
        println!("2. Exit");

        let choice: u8 = get_valid_input("choose an option: ", 1, 2);

        if choice == 1 { game(); }
        else { std::process::exit(2000) }
    }
}

fn game() {
    let mut score: u8 = 0;
    loop {
        let mut choice: u8 = get_valid_input("enter increase to the total (1-3): ", 1, 3);
        while choice >  0 {
            choice -= 1;
            score += 1;
            println!("{}", score);

            if score == 21 {
                println!("you lose");
                main();
            }   
        }
    }
}

fn get_valid_input(text: &str, low_bound: u8, up_bound: u8) -> u8 {
    loop {
        print!("{}", text);
        io::stdout().flush().unwrap(); //remove newline character 

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
        input = input.to_string();

        //let num = input("enter increase to the total (1-3): ");
        match input.trim().parse::<u8>() {
            Ok(n) if n >= low_bound && n <= up_bound => return n,
            _ => println!("Invalid input, please enter a number between {} and {}", low_bound, up_bound),
        }
    }
}