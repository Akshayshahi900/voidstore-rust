use std::collections::HashMap;
use std::collections::btree_map::Keys;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

struct Database {
    store: HashMap<String, String>,
}

impl Database {
    fn execute(&mut self, cmd: Command) -> String {
        match cmd {
            Command::Set(key, value) => {
                self.store.insert(key, value);
                "OK".to_string()
            }
            Command::Get(key) => self.store.get(&key).cloned().unwrap_or("(nil)".to_string()),
            Command::Del(key) => {
                if self.store.remove(&key).is_some() {
                    "1".to_string()
                } else {
                    "0".to_string()
                }
            }
        }
    }
}
enum Command {
    Set(String, String),
    Get(String),
    Del(String),
}
fn main() {
    let mut db = Database {
        store: HashMap::new(),
    };

    db.execute(Command::Set("name".into(), "karan aujla".into()));

    println!("{}", db.execute(Command::Get("name".into())));

    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind the connection");

    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }

            Err(e) => {
                eprintln!("Connection failed :{}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            println!(
                "Recieved:{}",
                String::from_utf8_lossy(&buffer[..bytes_read])
            );

            let response = "hello from Rust tcp server!";

            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => {
            eprintln!("Failed to read:{}", e);
        }
    }
}

fn parse_command(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    match parts.as_slice() {
        ["SET", key, value] => Some(Command::Set(key.to_string(), value.to_string())),
        ["GET", key] => Some(Command::Get(key.to_string())),
        ["DEL", key] => Some(Command::Del(key.to_string())),

        _ => None,
    }
}
