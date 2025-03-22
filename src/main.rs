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

fn main() {
    // Uncomment this block to pass the first stage
    let mut commands: HashMap<&str, Box<dyn ShellCommand>> = HashMap::new();
    commands.insert("echo", Box::new(Echo));

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
            if command == "exit"{
                if !args.is_empty() && args[0] == "0"{
                    break;
                }
                else{
                    continue;
                }
            }
            if let Some(cmd) = commands.get(command){
                cmd.execute(&args);
            }
            else{
                println!("{}: command not found", command);
            }
        }
    }
}
