use pathsearch::find_executable_in_path;
use std::process::Command;
use std::path::{Path, PathBuf};

pub fn externalcmd(cmd: &str, args: &[&str]) {
    let exe_path: Option<PathBuf> = find_executable_in_path(cmd)
        .or_else(|| Some(Path::new(cmd).to_path_buf()))
        .filter(|path| path.exists() && path.is_file());

    if let Some(path) = exe_path {
        let exe_name = path.file_name()
            .unwrap_or_else(|| path.as_os_str()) // Get only the filename
            .to_string_lossy();

        match Command::new(&path).args(args).spawn() {
            Ok(mut child) => {
                if let Err(err) = child.wait() {
                    eprintln!("Error waiting for process: {}", err);
                }
            }
            Err(err) => eprintln!("Failed to execute '{}': {}", exe_name, err),
        }
    } else {
        eprintln!("{}: command not found", cmd);
    }
}
