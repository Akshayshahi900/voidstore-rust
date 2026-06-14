use std::net::{TcpListener, TcpStream};
use std::io::{Read , Write};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind the connection");

    println!("Server listening on port 8080");

    for stream in listener.incoming(){
        match stream {
            Ok(stream) =>{
                println!("new connection: {}" , stream.peer_addr().unwrap());
                handle_client(stream);
            }

            Err(e)=>{
               eprintln!("Connection failed :{}" , e);  
            }
        }
    }

}

fn handle_client(mut stream:TcpStream){
    let mut  buffer = [0; 1024];

    match stream.read(&mut buffer){
        Ok(bytes_read) =>{
            println!(
                "Recieved:{}",
                String::from_utf8_lossy(&buffer[..bytes_read])
            );

            let response = "hello from Rust tcp server!";

            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) =>{
            eprintln!("Failed to read:{}", e);
        }
    }
}