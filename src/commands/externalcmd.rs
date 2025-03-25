use pathsearch::find_executable_in_path;
use std::process::Command;
use std::path::{Path, PathBuf};



pub fn externalcmd(cmd: &str, args: &Vec<&str>) -> Result<(), String>{
    let exe_path = find_executable_in_path(cmd)
        .or_else(|| Some(Path::new(cmd).to_path_buf()))
        .filter(|path| path.exists() && path.is_file());

    if let Some(_path) = exe_path {
        let child = Command::new(cmd).args(args).spawn().map_err(|err| err.to_string());
        child?.wait().map_err(|err| err.to_string())?;
        Ok(())
    }
    else{
        eprintln!("{}: command not found", cmd);
        Err(String::from("command not found"))
    }
}