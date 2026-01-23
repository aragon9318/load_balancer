mod tcp;
use std::net::SocketAddr;

fn main() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Starting load balancer on {}", addr);

    // Start TCP listener
    tcp::start_listener(addr);

}
