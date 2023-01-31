use std::{io::{BufReader, BufRead}, fs::File};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args[0].starts_with("-") {
        let file = &args[1..];
        if args[0].contains("w") {
            println!("Number of words: {}", words(file));
        } else if args[0].contains("l") {
            println!("Number of lines: {}", lines(file));
        } else if args[0].contains("c") {
            println!("Number of characters: {}", chars(file));
        }
    } else {
        println!("Number of words: {}", words(&args));
        println!("Number of lines: {}", lines(&args));
        println!("Number of characters: {}", chars(&args));
    }
}

fn lines(files: &[String]) -> usize {
    let mut count = 0;
    for filename in files {
        let f = File::open(filename).unwrap();
        let buffer = BufReader::new(f);
        count += buffer.lines().count();
    }
    count
}

fn words(files: &[String]) -> usize {
    let mut count = 0;
    for filename in files {
        let f = File::open(filename).unwrap();
        let buffer = BufReader::new(f);

        for line in buffer.lines() {
            count += line.unwrap().split_whitespace().count();
        }
    }
    count
}

fn chars(files: &[String]) -> usize {
    let mut count = 0;
    for filename in files {
        let f = File::open(filename).unwrap();
        let buffer = BufReader::new(f);
        
        for line in buffer.lines() {
            count += line.unwrap().chars().count();
        }
    }
    count
}