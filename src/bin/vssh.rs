use std::{io::Write, env::set_current_dir, ffi::CString};
use anyhow;
use nix::{sys::wait::waitpid,unistd::{fork, ForkResult, execvp}};

fn main() {
    loop {
        match process_line() {
            Ok(keep_going) => {
                if !keep_going {
                    break;
                }
            },
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
}

fn process_line() -> anyhow::Result<bool> {
    let mut user_input = String::new();
    let current_dir = std::env::current_dir().unwrap();
    let current_dir_final = current_dir.display();
    print!("{current_dir_final} ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut user_input)?;
    let input = user_input.trim();
    if input == "exit" {
        Ok(false)
    } else if input.starts_with("cd") {
        let words: Vec<&str> = input.split_whitespace().collect();
        match set_current_dir(words[1]) {
            Ok(_) => {},
            Err(e) => {println!("Error: {e}")}
        }
        Ok(true)
    } else if input.is_empty() {
        Ok(true)
    } else {
        fork_function(input);
        Ok(true)
    }
}

fn externalize(command: &str) -> Vec<CString> {
    command.split_whitespace().map(|s| CString::new(s).unwrap()).collect()
}

fn fork_function(command: &str) {
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child, .. }) => {
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            let cmd = externalize(command);
            match execvp::<CString>(cmd[0].as_c_str(), &cmd) {
                Ok(_) => {},
                Err(e) => println!("Error: {e}")
            }
        }
        Err(_) => println!("Fork failed.")
    }
}