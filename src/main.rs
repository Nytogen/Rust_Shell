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
        let command = full_command[0].trim();
        let arguments: Vec<&str> = full_command[1..].to_vec();

        if command == "echo"{
            print!("{}", &input[5..]);
        } else if command == "type" {
            handle_type(arguments[0].trim());
        } else if command == "pwd" {
            let _pwd_output = handle_pwd();
        } else if command == "cd" {
            handle_cd(arguments[0].trim());
        }
        else {
            handle_external_program(command, arguments);
        }
    }
}

fn handle_type(command: &str) {
    //Refactor in future
    if command == "echo" {
        println!("echo is a shell builtin");
    } else if command == "exit" {
        println!("exit is a shell builtin");
    } else if command == "type" {
        println!("type is a shell builtin");
    } else if command == "pwd" {
        println!("pwd is a shell builtin");
    } else if command == "cd" {
        println!("cd is a shell builtin");
    }
    else {
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

fn handle_pwd() -> std::io::Result<()>{
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}

fn handle_cd(new_path: &str){
    let new_directory = Path::new(new_path);
    assert!(env::set_current_dir(&new_directory).is_ok());
}