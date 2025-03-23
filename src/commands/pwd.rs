use std::env;
use crate::commands::ShellCommand;
    
pub struct Pwd;
impl ShellCommand for Pwd{
    fn execute(&self, _args: &[&str]){
        match env::current_dir(){
            Ok(path) => println!("{}", path.display()),
            Err(e) => println!("pwd: error: {}", e),
        }
    }
}
