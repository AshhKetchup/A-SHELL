#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashMap;
use pathsearch::find_executable_in_path;

mod commands;
mod util;

use commands::{ShellCommand, Echo, Exit, Cd, Pwd};
use commands::externalcmd::externalcmd;

fn parse(input: &str, commands: &HashMap<&str, Box<dyn ShellCommand>>){
    let mut parts: Vec<String>= util::parse_input(input);
    //println!("{:?}", parts);

    if parts.is_empty() {
        return;
    }

    let command = parts[0].as_str();
    let args: Vec<&str> = parts.iter().skip(1).map(String::as_str).collect();
    //println!("{:?}", command);
    //println!("args: {:?}", args);
    match command {
        "type" => { if !args.is_empty() {
                let cmd_name = args[0];
                if cmd_name == "type" || cmd_name == "echo" {
                    println!("{cmd_name} is a shell builtin")
                } else {
                    match commands.get(cmd_name) {
                        Some(_) => println!("{} is a shell builtin", cmd_name),
                        None => match find_executable_in_path(cmd_name) {
                            Some(path) => println!("{} is {}", cmd_name, path.to_str().unwrap()),
                            None => println!("{}: not found", cmd_name),
                        },
                    }
                }
            }
            else {
                println!("no args for {}", command);
            }
        }
        "echo" => {
            let echo = Echo;
            echo.execute(input);
        }
        _ => match commands.get(command) {
            Some(cmd) => {
                //println!("{} is a shell cmd", command);
                cmd.execute(&args)
            },
            None => {
                //println!("{}: not in commands", command);
                externalcmd(command, &args).expect("Failed to execute command(external)");
            }
        },
    }
}

fn main() {
    let mut commands: HashMap<&str, Box<dyn ShellCommand>> = HashMap::new();
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