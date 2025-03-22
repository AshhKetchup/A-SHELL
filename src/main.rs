#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashMap;

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

        let mut parts = input.split_whitespace();
        if let Some(command) = parts.next() {
            let args: Vec<&str> = parts.collect();
            if command == "type" {
                if commands.get(args[0]).is_some() || args[0] == "type" {
                    println!("{} is a shell builtin", args[0]);
                }
                else{
                    println!("{}: not found", args[0]);
                }
            }else{
                if let Some(cmd) = commands.get(command){
                    cmd.execute(&args);
                }
                else{
                    println!("{}: command not found", command);
                }
            }
        }
    }
}
