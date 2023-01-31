use std::fs::{self};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.len() {
        0..=1 => {
            println!("Usage: duplicate filename newfilename")
        }
        _ => {
            // copy(args[0].as_str(), args[1].as_str());
            match fs::copy(&args[0], &args[1]) {
                Ok(_) => {},
                Err(_e) => println!("{_e}")
            }
        }
    }
}