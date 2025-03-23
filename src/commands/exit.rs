use crate::commands::ShellCommand;
use std::process;

pub struct Exit;

impl ShellCommand for Exit {
    fn execute(&self, args: &[&str]) {
        if !args.is_empty() && args[0] == "0" {
            process::exit(0);
        }
    }
}

