#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashMap;
use pathsearch::find_executable_in_path;

mod commands;

use commands::{ShellCommand, Echo, Exit, Cd, Pwd};
use commands::externalcmd::externalcmd;

fn parse(input: &str, commands: &HashMap<&str, Box<dyn ShellCommand>>){
    let mut parts = input.split_whitespace();
   

    if let Some(command) = parts.next() {
        let args: Vec<&str> = parts.collect();

        match command {
            "type" if !args.is_empty() => {
                let cmd_name = args[0];
                if cmd_name == "type"{
                    println!("type is a shell builtin")
                }
                else{
                match commands.get(cmd_name) {
                    Some(_) => println!("{} is a shell builtin", cmd_name),
                    None => match find_executable_in_path(cmd_name) {
                        Some(path) => println!("{} is {}", cmd_name, path.to_str().unwrap()),
                        None => println!("{}: not found", cmd_name),
                    },
                }
                }
            }
            _ => match commands.get(command) {
                Some(cmd) => cmd.execute(&args),
                None => {
                    externalcmd(command, &args);
                }
            },
        }
    }
}

fn main() {
    let mut commands: HashMap<&str, Box<dyn ShellCommand>> = HashMap::new();
    commands.insert("echo", Box::new(Echo));
    commands.insert("exit", Box::new(Exit));
    commands.insert("pwd", Box::new(Pwd)); 
    commands.insert("cd", Box::new(Cd));
    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

    // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        parse(input, &commands);
    }
}
