use std::{fs::{self, ReadDir, DirEntry, read_dir}, io::Error, env};

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
        let args: Vec<String> = env::args().collect();
        
    }
}