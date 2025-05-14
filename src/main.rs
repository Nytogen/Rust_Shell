#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;

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
            let command:String = input.trim()[5..].to_owned();
            handle_type(&command);
        } else {
            println!("{}: command not found", input.trim())
        }
    }
}

fn handle_type(command: &str) {
    if command == "echo" {
        println!("echo is a shell builtin");
    } else if command == "exit" {
        println!("exit is a shell builtin");
    } else if command == "type" {
        println!("type is a shell builtin");
    } else {
        handle_type_exec(command);
    }

}

fn handle_type_exec(command: &str){
    let key = "PATH";
    let paths = env::var(key).unwrap();
    let path_list: Vec<&str> = paths.split(":").collect();

    let mut curr_path: String;
    let mut found = false;

    for path in path_list{
        curr_path = path.to_owned();
        curr_path.push_str("/");
        curr_path.push_str(command);
        if Path::new(&curr_path).exists(){
            found = true;
            println!("{} is {}", command, Path::new(&curr_path).display());
            break;
        }
    }   
    
    if !found {
        println!("{}: not found", command);
    }
}
