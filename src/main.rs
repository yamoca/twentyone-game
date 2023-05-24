use std::io;
use std::io::Write;

fn main() {
    input("enter yo ball size: ");
}

fn input(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap(); //remove newline character 

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    return input;
}