use std::net::*;
use std::thread;

fn main() {
    let mut server: TcpListener = TcpListener::bind("127.0.0.1:6000").unwrap();

    for client in server.incoming() {
        match client {
            Ok(client) => { 
                thread::spawn(move || { handle_client(client); }); 
            }
            Err(err) => { eprintln!("error: {:?}", err); }
        }
    }
}

fn handle_client(client: TcpStream) {
    
}