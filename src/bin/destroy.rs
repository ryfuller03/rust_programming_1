use std::fs::remove_file;

fn main() {
    for arg in std::env::args().skip(1) {
        // match statement here
        match remove_file(arg) {
            Ok(()) => {},
            Err(e) => println!("Error when reading files: {e}")
        }
    }
}