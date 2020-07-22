use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::{fs, process, thread, time::Duration};
use tokio::task;

#[tokio::main]
async fn main() {
    // todo: take from TOML configuration file
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 80)),
        SocketAddr::from(([127, 0, 0, 1], 443)),
        SocketAddr::from(([127, 0, 0, 1], 8888)),
    ];

    let listener = TcpListener::bind(&addrs[..]).unwrap_or_else(|err| {
        eprintln!("unable to start server: {:?}", err);
        process::exit(1);
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("connection successfully established: {:?}", &stream);
                task::spawn(handle_connection(stream));
            }
            Err(e) => {
                eprintln!("unable to establish connection: {:?}", e);
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let get_sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        // todo: take from TOML configuration file
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(get_sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
