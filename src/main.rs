use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::io::Write;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buf;
    loop {
        buf = [0; 512];
        let _ = match stream.read(&mut buf) {
            Err(e) => panic!("Got an error: {}", e),
            Ok(m) => {
                if m == 0 {
                    break;
                }
                m
            }
        };

        match stream.write(&buf) {
            Err(_) => break,
            Ok(_) => continue,
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:12315").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }

            Err(e) => { 
                println!("Connected failed")
            }
        }
    }
}
