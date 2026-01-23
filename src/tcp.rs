
use std::net::{TcpListener, TcpStream, SocketAddr};

use std::io::{Read, Write};
use std::thread;
use std::time::Duration;


// create pub fun to init the listener
pub fn start_listener(ip_addr: SocketAddr){
     let listener = TcpListener::bind(ip_addr).expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
             Ok(stream) => {
                // Spawn per-connection thread for now
                // TODO: This code is temporary and will be replaced with a proper async runtime
                // Thread per connection will exhaust resources quickly under load consider using queues and worker pools
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => { eprintln!("Failed to accept connection: {}", e);}
        }
    }
}

// handle incoming connections and the TCP behavior, also read the bytes packages comming to the data stream
 fn handle_connection( mut stream: TcpStream) {
            let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));
            let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
            let _ = stream.set_nodelay(true);
            let _ = stream.set_nonblocking(true);
            let mut buffer = [0u8; 1024];
            //TODO: Read data in chunks and process accordingly, currently just reads the first chunk
            match stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    println!("Received {} bytes", bytes_read);
                    //TODO: Write data into the appropriate destination
                    let _ = stream.write_all(&buffer[..bytes_read]);
                }
                Err(e) => {
                    eprintln!("Failed to read from connection: {}", e);
                }
            }
    }

