use std::fs::{self};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.len() {
        0 => {
            println!("Usage: order filenames \n OR \n order -r filenames");
        }
        _ => {
            // find out if it needs to be reversed or not
            if args[0].contains("-r") {
                print_lines(true, &args[1..]);
            } else {
                print_lines(false, &args);
            }
        }
    }
}

fn print_lines(reversed: bool, files: &[String]) {
    let mut lines: Vec<String> = Vec::new();
    for file in files {
        let f = fs::read_to_string(file).unwrap();
        for line in f.lines() {
            lines.push(line.to_owned());
        }
    }
    
    lines.sort();
    if reversed == true {
        lines.reverse();
    }
    for line in lines {
        println!("{line}");
    }
}