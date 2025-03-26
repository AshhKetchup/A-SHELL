use pathsearch::find_executable_in_path;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::io::Write;
use std::fs::OpenOptions;

pub fn externalcmd(cmd: &str, args: &Vec<&str>) -> Result<(), String>{
    let exe_path = find_executable_in_path(cmd)
        .or_else(|| Some(Path::new(cmd).to_path_buf()))
        .filter(|path| path.exists() && path.is_file());
    let mut errtowrite = false;
    let mut overwrite = true;
    let (new_args, mut file) = if let Some(index) = args.iter().position(|arg| arg.to_string() == ">" || arg.to_string() == "1>" || arg.to_string() == "2>" || arg.to_string() == ">>" || arg.to_string() == "1>>" || arg.to_string() == "2>>")
    {
        if args[index] == "2>"{
            errtowrite = true;
        }
        if args[index] == ">>" || args[index] == "1>>" || args[index] == "2>>" {
            overwrite = false;
        }
        if let Some(filename) = args.get(index + 1) {
            let mut file = OpenOptions::new()
                .create(true)   // Create if not exists
                .write(true)    // Open for writing
                .truncate(overwrite) // Overwrite existing content
                .append(!overwrite)
                .open(filename).expect("unable to open/create file");

            //println!("Redirecting output to: {}", filename);
            // modify args
            let new_args: Vec<&str> = args.iter().take(index).map(|s| *s).collect();
            (new_args, Some(file))
        }
        else{
            return Err(String::from("Missing filename"));
        }
    } else {
        (args.iter().map(|s| *s).collect(), None)
    };
    if let Some(_path) = exe_path {
        let output = Command::new(cmd).args(new_args).output().map_err(|err| err.to_string())?;
        //println!("{:?}", output);
        let out = String::from_utf8_lossy(&output.stdout).trim_end_matches('\n').to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim_end_matches('\n').to_string();

        if let Some(ref mut file) = file {
            if !out.is_empty() && !errtowrite {
                writeln!(file, "{}\n", out).map_err(|err| err.to_string())?;
            }
            if !stderr.is_empty() && !errtowrite {
                println!("{}", stderr);
            }
            if errtowrite && !stderr.is_empty() {
                writeln!(file, "{}\n", stderr).map_err(|err| err.to_string())?;
            }
            if errtowrite && !out.is_empty() {
                println!("{}", out);
            }
        } else {
            if !stderr.is_empty() {
                eprintln!("{}", stderr);
            } else if !out.is_empty() {
                println!("{}", out);
            }
        }
        Ok(())
    } else {
        eprintln!("{}: command not found", cmd);
        Err(String::from("command not found"))
    }

}