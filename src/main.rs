#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashMap;
use pathsearch::find_executable_in_path;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::env;
use dirs;

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

struct Pwd;
impl ShellCommand for Pwd{
    fn execute(&self, _args: &[&str]){
        match env::current_dir(){
            Ok(path) => println!("{}", path.display()),
            Err(e) => println!("pwd: error: {}", e),
        }
    }
}

struct Cd;
impl ShellCommand for Cd{
    fn execute(&self, args: &[&str]){
        let path = if args[0] == "~"{
            dirs::home_dir()
        }
        else if args[0].starts_with("~/"){
            dirs::home_dir().map(|home| home.join(args[0].trim_start_matches("`/")))
        }
        else{
            Some(PathBuf::from(args[0]))
        };

        if let Some(path) = path { 
        if let Err(e) = env::set_current_dir(&path){
            let e = match e.kind() {
                io::ErrorKind::NotFound => "No such file or directory",
                io::ErrorKind::PermissionDenied => "Permission denied",
                _ => "an error occured",
            };
            println!("cd: {}: {}", args[0], e);
        }
        }
        else {
            eprintln!("cd: could not determine home directory");
        }
    }
}

fn externalcmd(cmd: &str, args: &Vec<&str>) -> Result<(), String>{
    let exe_path = find_executable_in_path(cmd)
        .or_else(|| Some(Path::new(cmd).to_path_buf()))
        .filter(|path| path.exists() && path.is_file());

    if let Some(_path) = exe_path {
        let child = Command::new(cmd).args(args).spawn().map_err(|err| err.to_string());

        child?.wait().map_err(|err| err.to_string())?;
        Ok(())
    } 
    else{
        println!("{}: command not found", cmd);
        Err(format!("{}: command not found", cmd))
    }
}

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
