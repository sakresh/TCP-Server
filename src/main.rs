use std::io::{Read, Write};
use std::thread::spawn;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Unable to read the buffer");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request received {}", request);
    let response = "Connection established !".as_bytes();
    stream.write(response).expect("Cannot write to the client");
}

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to the address");
    println!("Server listening on 127.0.0.1");
    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                spawn( || handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection :{}", e);
            }
        }
    }
}
