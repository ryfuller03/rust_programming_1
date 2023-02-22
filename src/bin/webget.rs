use openssl::ssl::{SslConnector, SslMethod};
use std::{io::{Write, BufReader, BufRead}, net::TcpStream, fs::File};

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.len() {
        1 => {
            let req = Request::new(&args[0]);
            let get_req = create_get(&req);
            println!("{}", get_req);
            match send_message(&req.host, 443, &get_req, &req.filename) {
                Ok(_) => {},
                Err(e) => println!("Error: {e}")
            }
            
        }
        _ => println!("Usage: webget [url]"),
    }
    
}

fn send_message(host: &str, port: usize, message: &str, filename: &str) -> anyhow::Result<()> {
    let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
    let connector = SslConnector::builder(SslMethod::tls())?.build();
    let mut stream = connector.connect(host, tcp)?;
    stream.write(message.as_bytes())?;
    let stream_buffer = BufReader::new(stream);
    let mut header = true;
    let mut file_lines = Vec::new();
    for line in stream_buffer.lines() {
        let final_line = line.unwrap().clone();
        if header {
            println!("{final_line}");
            if final_line.len() == 0 {
                header = false;
            }
        } else {
            write!(&mut file_lines, "{}\n", final_line)?;
        }
    }
    let final_filename = match filename.rfind("/") {
        Some(_) => {
            let (_unneeded, rest) = filename.split_at(filename.rfind("/").expect("Couldn't find /") + 1);
            rest
        },
        None => filename,
    };
    let mut new_file = File::create(final_filename)?;
    new_file.write_all(&file_lines)?;
    Ok(())
}

fn create_get(req: &Request) -> String {
    format!("GET {} HTTP/1.1\nHost: {}\nConnection: Close\r\n\r\n", req.filename, req.host)
}

struct Request {
    host: String,
    filename: String
}

impl Request {
    fn new(user_input: &str) -> Self {
        let input = user_input.trim();

        let (_unneeded, rest) = input.split_at(input.find("://").expect("couldn't find start of host") + 3);
        let (url_part, file_part) = rest.split_at(rest.find("/").expect("couldn't find correct split for host and file") + 1);

        let host = url_part[..(url_part.len()-1)].to_owned();
        let filename = file_part.to_owned();
        

        Request { host: host, filename: filename }
    }
}