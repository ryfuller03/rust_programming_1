use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.len() {
        0..=1 => {
            println!("Usage: findtext desired_text filenames");
        }
        _ => {
            for filepath in &args[1..] {
                let f = fs::read_to_string(filepath).unwrap();
                for line in f.lines() {
                    if line.contains(&args[0].as_str()) {
                        println!("{line}");
                    }
                }
            }
        }
    }
}