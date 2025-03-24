use crate::commands::ShellCommand;

pub struct Echo;

impl ShellCommand for Echo {
    fn execute(&self, args: &[&str]) {
        println!("{}", args.join(""));
    }
}
