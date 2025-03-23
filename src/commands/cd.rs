use crate::commands::ShellCommand;
use std::path::{PathBuf};
use std::io;
use std::env;
use dirs;


pub struct Cd;
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



