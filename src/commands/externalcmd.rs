use pathsearch::find_executable_in_path;
use std::process::Command;
use std::path::Path;

pub fn externalcmd(cmd: &str, args: &[&str]) {
    let exe_path = find_executable_in_path(cmd)
        .or_else(|| Some(Path::new(cmd).to_path_buf()))
        .filter(|path| path.exists() && path.is_file());

    if let Some(path) = exe_path {
        match Command::new(path).args(args).spawn() {
            Ok(mut child) => {
                if let Err(err) = child.wait() {
                    eprintln!("Error waiting for process: {}", err);
                }
            }
            Err(err) => eprintln!("Failed to execute '{}': {}", cmd, err),
        }
    } else {
        eprintln!("{}: command not found", cmd);
    }
}
