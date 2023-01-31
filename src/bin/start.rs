use std::fs;

fn main() -> std::io::Result<()> {
    let mut lines = 10;

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args[0].starts_with("-") {
        let num_lines = &args[0][1..]; // ampster = ampersand string slice
        let update_lines: usize = num_lines.parse().unwrap();
        lines = update_lines;
    }

    for arg in &args[1..] {
        let f = fs::read_to_string(arg)?;
        for line in f.lines().take(lines) {
            println!("{line}");
        }
    }
    Ok(())
}