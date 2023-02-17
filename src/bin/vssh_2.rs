use std::{io::Write, env::set_current_dir, ffi::CString};
use anyhow;
use nix::{sys::{wait::waitpid, stat::Mode},unistd::{fork, ForkResult, execvp, pipe, close, dup2}, fcntl::{OFlag, open}};

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
    }
    else {
        let input_struct = Command::new(input);
        fork_function(input_struct);
        Ok(true)
    }
}

fn externalize(command: &str) -> Vec<CString> {
    command.split_whitespace().map(|s| CString::new(s).unwrap()).collect()
}

fn fork_function(command: Command) {
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child}) => {
            if command.background {
                println!("ID: {child}");
            }
            else {
                waitpid(child, None).unwrap();
            }
        }
        Ok(ForkResult::Child) => {
            piping(command).expect("Initial pipe failed");
        }
        Err(e) => {println!("Error: {e}");}
    }
}

fn piping(command: Command) -> anyhow::Result<()> {
    let mut descriptor = 1;
    if command.output_file != None {
        let flags: OFlag = [OFlag::O_CREAT, OFlag::O_WRONLY, OFlag::O_TRUNC].iter().copied().collect();
        descriptor = open(command.output_file.unwrap().as_str(), flags, Mode::S_IWUSR).expect("Failed to open output_file");
    }
    for cmd in command.commands.iter().skip(1).rev() {
        descriptor = pipe_fork(cmd, descriptor).expect("Error in forking the pipe.");
    }
    if command.input_file != None {
        let filename = open(command.input_file.unwrap().as_str(), OFlag::O_RDONLY, Mode::S_IRUSR).expect("Failed to open input file");
        dup2(filename, 0)?;
    }
    dup2(descriptor, 1)?;
    let current = externalize(command.commands[0].as_str());
    execvp(&current[0], &current)?;
    Ok(())
}

fn pipe_fork(cmd: &String, descriptor: i32) -> anyhow::Result<i32> {
    let (inp, outp) = pipe()?;
    match unsafe {fork()}? {
        ForkResult::Parent { child: _ } => {
            close(outp)?;
            dup2(inp, 0)?;
            dup2(descriptor, 1)?;
            let current = externalize(cmd.as_str());
            execvp(&current[0], &current)?;
            unreachable!() // necessary, thanks Simon!
        }
        ForkResult::Child => {
            close(inp)?;
            Ok(outp)
        }
    }
}

struct Command {
    background: bool,
    input_file: Option<String>,
    output_file: Option<String>,
    commands: Vec<String>
}

impl Command {
    fn new(user_input: &str) -> Self {
        let mut input = user_input.trim();
        let mut background = false;
        let mut user_commands = Vec::new();
        let mut commands = vec![];
        let mut input_file = None;
        let mut output_file = None;
        if input.ends_with("&") {
            background = true;
            input = &input[..input.len() - 1];
        }
        if input.contains("|") {
            for command in input.split("|") {
                user_commands.push(command.trim().to_owned());
            }
        } else {
            user_commands.push(input.to_owned());
        }

        let mut first: String = user_commands[0].clone();

        match input.find("<") {
            Some(split) => {
                let (file, name) = user_commands[0].split_at(split);
                first = file.to_owned();
                input_file = Some(name[1..].trim().to_owned());
            }
            None => {
                input_file = None;
            }
        }

        match input.find(">") {
            Some(split) => {
                let (file, name) = user_commands[0].split_at(split);
                input = file;
                output_file = Some(name[1..].trim().to_owned());
            }
            None => {
                output_file = None;
            }
        }
        commands.push(first);
        commands.extend(user_commands.to_owned());

        Command {
            background,
            output_file,
            input_file,
            commands,
            
        }

    }
}