mod tcp;
use std::net::SocketAddr;

fn main() {
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    println!("Starting load balancer on {}", addr);

    // Start TCP listener
    tcp::start_listener(addr);

}
