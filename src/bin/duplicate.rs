use std::fs::copy;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        0..=1 => {
            println!("Usage: duplicate filename newfilename")
        }
        _ => {
            copy(args[1].as_str(), args[2].as_str());
        }
    }
}