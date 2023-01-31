use std::fs::copy;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.len() {
        0..=1 => {
            println!("Usage: duplicate filename newfilename")
        }
        _ => {
            // copy(args[0].as_str(), args[1].as_str());
            match copy(args[0].as_str(), args[1].as_str()) {
                Ok(_) => {},
                Err(e) => println!("Error: {e}")
            }
        }
    }
}