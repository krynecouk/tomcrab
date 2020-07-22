use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::{env, fs, process};
use tokio::task;
use tomcrab::config::Config;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("missing path to tomcrab configuration file");
        process::exit(1);
    }

    let toml_path = &args[1];
    let toml = fs::read_to_string(toml_path).expect("unable to find toml config");
    let config = Config::from_toml(&toml).expect("unable to parse toml config");
    let addrs: Vec<SocketAddr> = config.server.socket_addresses();
    let listener = TcpListener::bind(&addrs[..]).unwrap_or_else(|err| {
        eprintln!("unable to start server: {:?}", err);
        process::exit(1);
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("connection successfully established: {:?}", &stream);
                let location = config.server.location.clone();
                task::spawn(handle_connection(stream, location));
            }
            Err(e) => {
                eprintln!("unable to establish connection: {:?}", e);
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream, location: String) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", location)
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html".to_string())
    };

    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
