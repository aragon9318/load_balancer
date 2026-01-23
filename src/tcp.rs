
use std::net::{TcpListener, TcpStream, SocketAddr,Shutdown};

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
            let _ = stream.set_nonblocking(false);
            let mut buffer = [0u8; 1024];
            //TODO: Implement proper connection handling logic, reading bytes in a loop might hit blocks even when set_nonblocking is true
            loop {
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            return; // Connection closed
                }
                        println!("Received {} bytes", bytes_read);
                        //TODO: Write data into the appropriate destination
                        let _ = stream.write_all(&buffer[..bytes_read]);
                    }
                    // No data available yet, continue the loop
                    
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                        println!("Read timed out, closing connection");
                        break; 
                    }
                    Err(e) => {
                        eprintln!("Failed to read from connection: {} {}", e, e.kind());
                    }
                }
            }
            stream.shutdown(Shutdown::Write).ok();
           
            
          
    }


