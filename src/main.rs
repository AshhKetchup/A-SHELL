#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashMap;
use pathsearch::find_executable_in_path;
use std::process::Command;

trait ShellCommand{
    fn execute(&self, args: &[&str]);
}

struct Echo;
impl ShellCommand for Echo{
    fn execute(&self, args: &[&str]){
        println!("{}", args.join(" "));
    }
}

struct Exit;
impl ShellCommand for Exit{
    fn execute(&self, args: &[&str]){
        if !args.is_empty() && args[0] == "0"{
            std::process::exit(0);
        }
    }
}

fn externalcmd(cmd: &str, args: &Vec<&str>){
    if let Some(path) = find_executable_in_path(cmd){
        let status = Command::new(path).args(args).spawn().expect("Failed to execute command").wait();

        if let Err(e) = status{
            println!("Error: {}", e);
        }
    } 
    else{
        println!("{}: command not found", cmd);
    }
}

fn parse(input: &str, commands: &HashMap<&str, Box<dyn ShellCommand>>){
    let mut parts = input.split_whitespace();

    if let Some(command) = parts.next() {
        let args: Vec<&str> = parts.collect();

        match command {
            "type" if !args.is_empty() => {
                let cmd_name = args[0];

                match commands.get(cmd_name) {
                    Some(_) => println!("{} is a shell builtin", cmd_name),
                    None => match find_executable_in_path(cmd_name) {
                        Some(path) => println!("{} is {}", cmd_name, path.to_str().unwrap()),
                        None => println!("{}: not found", cmd_name),
                    },
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
    // Uncomment this block to pass the first stage
    let mut commands: HashMap<&str, Box<dyn ShellCommand>> = HashMap::new();
    commands.insert("echo", Box::new(Echo));
    commands.insert("exit", Box::new(Exit));
    
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
