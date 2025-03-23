pub trait ShellCommand {
    fn execute(&self, args: &[&str]);
}

// Import all commands
pub mod echo;
pub mod exit;
pub mod cd;
pub mod pwd;
pub mod externalcmd;

// Re-export commands for easy access in main.rs
pub use echo::Echo;
pub use exit::Exit;
pub use cd::Cd;
pub use pwd::Pwd;
