use std::{fs::{self, remove_file, rename}, io};

fn main() {
    for arg in std::env::args() {
        println!("{arg}");
    }
}

fn dir() -> std::io::Result<()> {
    for entry in fs::read_dir(".")? {
        let dir = entry?;
        println!("{:?}", dir.path());
    }
    Ok(())
}

fn destroy() {
    for arg in std::env::args() {
        remove_file(arg);
    }
}

fn newname() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        0 => {
            println!("Usage: newname (current_name new_name)");
        }
        _ => {
            
        }
    }
}