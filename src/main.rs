#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;
use std::process::Command;

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
        }

        let full_command: Vec<&str> = input.split(" ").collect();
        let command: &str = full_command[0];
        let arguments: Vec<&str> = full_command[1..].to_vec();

        if *command == *"echo"{
            handle_echo(arguments[0].trim());
        } else if *command == *"type" {
            handle_type(arguments[0].trim());
        } else {
            handle_external_program(command, arguments);
        }
    }
}

fn handle_echo(argument: &str){
    println!("{}", argument);
}

fn handle_type(command: &str) {
    if command == "echo" {
        println!("echo is a shell builtin");
    } else if command == "exit" {
        println!("exit is a shell builtin");
    } else if command == "type" {
        println!("type is a shell builtin");
    } else {
        handle_type_external(command);
    }

}

fn handle_type_external(command: &str){
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

fn handle_external_program(exec_name: &str, arguments: Vec<&str>){
    let new_program = Command::new(exec_name)
        .args(arguments)
        .output();

    if new_program.is_err(){
        println!("{}: command not found", exec_name.trim());
    } else { 
        let new_program_stdout= new_program.unwrap().stdout;
        let mut new_program_stdout_string: String = String::from_utf8(new_program_stdout).unwrap();
        new_program_stdout_string = new_program_stdout_string.replace("\n\n", "\n");
        print!("{}", new_program_stdout_string);
    }
}