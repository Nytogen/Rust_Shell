#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    loop{    
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        


        if input.trim() == "exit 0" {
            break;
        } else if input.trim()[..4] == *"echo"{
            print!("{}", &input[5..]);
        } else if input.trim()[..4] == *"type" {
            let mut command = String::new();
            command = input.trim()[5..].to_owned();
            if command == *"echo" {
                println!("echo is a shell builtin");
            } else if command == *"exit" {
                println!("exit is a shell builtin");
            } else if command == *"type" {
                println!("type is a shell builtin");
            } else{
                println!("{}: not found", command);
            }
        } else {
            println!("{}: command not found", input.trim())
        }
    }
}
