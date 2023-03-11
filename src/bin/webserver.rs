use std::{net::{TcpListener, TcpStream}, thread, io::{Read, Write, BufReader}, sync::{Arc, Mutex}, path::PathBuf, env, fs::File};

use anyhow::anyhow;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut stream_enabled = false;
    if args.len() > 1 {
        println!("Usage: webserver -s [number of bytes] OR webserver");
    }
    if args[0].contains("-s") {
        println!("Streaming enabled");
        stream_enabled = true
    }
    let total_req = Arc::new(Mutex::new(0));
    let reqs = Arc::new(Mutex::new(0));
    let listener = TcpListener::bind("localhost:8888")?;
    for stream in listener.incoming() {
        let total_req_2 = total_req.clone();
        let reqs_2 = reqs.clone();
        let mut stream = stream?;
        thread::spawn(move || {
            {
                let mut counter = total_req_2.lock().unwrap();
                *counter += 1;
            }
            match handle_client(&mut stream, stream_enabled) {
                Ok(_) => {
                    let mut counter = reqs_2.lock().unwrap();
                    *counter += 1;
                },
                Err(e) => println!("Error: {e}"),
            }
            {
                let total_req_3 = total_req_2.clone();
                let total_lock = total_req_3.lock().unwrap();
                println!("Total requests: {}", *total_lock);
            }
            {
                let reqs_3 = reqs_2.clone();
                let reqs_lock = reqs_3.lock().unwrap();
                println!("Valid requests: {}", *reqs_lock);
            }
        });
    }
    Ok(())
}

fn handle_client(stream: &mut TcpStream, stream_enabled: bool) -> anyhow::Result<()> {
    let mut message = String::new();
    loop {
        let mut buffer = [0u8; 500];
        let bytes = stream.read(&mut buffer)?;
        let bytes_string = std::str::from_utf8(&buffer[..bytes])?;
        message.push_str(bytes_string);
        if message.contains("\r\n\r\n") || message.contains("\n\n") {
            break;
        }
    }
    println!("client IP Address: {}", stream.local_addr()?);
    println!("Read {} bytes from {}", message.as_bytes().len(), stream.local_addr()?);
    println!("{message}");
    let file_string = get_file_string(message.as_str())?;
    let mut final_file_path = env::current_dir()?;
    final_file_path.push(file_string.as_str());
    if !(PathBuf::from(file_string.as_str()).exists()) {
        stream.write(b"HTTP/1.1 404 Not Found\n\n<html><body>403</body></html>")?;
        println!("{}", final_file_path.display());
        return Err(anyhow!("404 Not Found."));
    }
    if !(final_file_path.starts_with(env::current_dir()?)) {
        stream.write(b"HTTP/1.1 403 Forbidden\n\n<html><body>403</body></html>")?;
        return Err(anyhow!("403 Forbidden."));
    }
    
    let mut file = File::open(PathBuf::from(file_string.as_str()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let file_length = contents.as_bytes().len();
    let header = format!("HTTP/1.1 200 OK\nContent-Type: text/html; charset=UTF-8\nContent-Length: {}\n\n", file_length);
    println!("{header}");
    if !stream_enabled {
        streamm(stream, header, file_string.as_str())?;
    } else {
        send(stream, header, file_string.as_str())?;
    }
    Ok(())
}

fn get_file_string(message: &str) -> anyhow::Result<String> {
    let mut done = false;
    for something in message.split_whitespace() {  // not sure what to call variable when splitting by whitespace...something it is!
        if done {
            return Ok(String::from(something[1..].to_owned()));
        }
        if something == "GET" {
            done = true;
        }
    }
    Err(anyhow!("get_file_string failed."))
}

fn send(stream: &mut TcpStream, header: String, file: &str) -> anyhow::Result<()> {
    let mut file= File::open(PathBuf::from(file))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    stream.write(format!("{header}{contents}").as_bytes())?;
    Ok(())
}

fn streamm(stream: &mut TcpStream, header: String, file: &str) -> anyhow::Result<()> {
    let mut file = File::open(PathBuf::from(file))?;
    let mut reader = BufReader::new(file);
    loop {
        let mut buffer = [0u8; 500];
        let bytes_read = reader.read(&mut buffer)?;
        stream.write(&mut buffer[..bytes_read])?;
        if bytes_read < 500 {
            break;
        }
    }
    Ok(())
}