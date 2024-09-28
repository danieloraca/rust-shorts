use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

use http_server::banner::crt_image;
use http_server::informations::{build_large_json, get_json_response};
use http_server::simple_functions::{sleep_function, time_function};

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let time = b"GET /time HTTP/1.1\r\n";
    let json = b"GET /json HTTP/1.1\r\n";
    let large_json = b"GET /large_json HTTP/1.1\r\n";

    let mut html_content = String::new();

    // root
    let (status_line, content) = if buffer.starts_with(get) {
        println!("GET /");
        if let Ok(mut file) = File::open("./public/hello.html") {
            file.read_to_string(&mut html_content).unwrap();
            (
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n".to_string(),
                html_content,
            )
        } else {
            (
                "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\n\r\n".to_string(),
                "Not Found :(\n".to_string(),
            )
        }
    // sleep
    } else if buffer.starts_with(sleep) {
        println!("GET /sleep");
        sleep_function()
    // time
    } else if buffer.starts_with(time) {
        println!("GET /time");
        time_function()
    } else if buffer.starts_with(json) {
        println!("GET /json");
        (
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n".to_string(),
            get_json_response(),
        )
    } else if buffer.starts_with(large_json) {
        println!("GET /large_json");
        (
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n".to_string(),
            build_large_json(),
        )
    } else {
        println!("GET /404");
        (
            "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain\r\n\r\n".to_string(),
            "Not Found\n".to_string(),
        )
    };

    let response = format!("{}{}", status_line, content);
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let _ = crt_image();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }
    }

    Ok(())
}
