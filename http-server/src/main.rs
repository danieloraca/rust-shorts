use banner::crt_image;
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

mod banner;

#[derive(Serialize, Deserialize)]
struct Information {
    name: String,
    age: u8,
}

#[derive(Serialize, Deserialize)]
struct InformationList {
    infos: Vec<Information>,
}

fn get_json_response() -> String {
    let mut information_list: InformationList = InformationList {
        infos: vec![
            Information {
                name: "Alice".to_string(),
                age: 20,
            },
            Information {
                name: "Bob".to_string(),
                age: 25,
            },
            Information {
                name: "Charlie".to_string(),
                age: 30,
            },
        ],
    };

    information_list.infos.push(Information {
        name: "David".to_string(),
        age: 35,
    });

    match serde_json::to_value(&information_list) {
        Ok(object) => {
            format!("{}\n", object)
        }
        Err(e) => {
            format!("{}\n", e)
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let time = b"GET /time HTTP/1.1\r\n";
    let json = b"GET /json HTTP/1.1\r\n";

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

fn sleep_function() -> (String, String) {
    std::thread::sleep(std::time::Duration::from_secs(5));
    (
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n".to_string(),
        "Woke up from sleep!\n".to_string(),
    )
}

fn time_function() -> (String, String) {
    let now = Local::now();
    let current_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    (
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n".to_string(),
        format!("{}\n", current_time),
    )
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
