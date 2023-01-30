use std::fs::rename;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        0..=1 => {
            println!("Usage: newname current new");
        }
        _ => {
            match rename(args[1].as_str(), args[2].as_str()) {
                Ok(()) => {},
                Err(e) => println!("Error renaming: {e}")
            }
        }
    }
}