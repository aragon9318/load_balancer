mod tests {
  
    use std::net::{ SocketAddr, TcpStream};
    use std::io::Write;
    use std::io::Read;
    use std::net::Shutdown;
    use std::time::Duration;
    use std::time::Instant;
  
    #[test]
    fn test_server_data_handling_large_payload() {
        // The server should respond to the large payload correctly,
        //  this because the buffer is 1024 bytes but the file is larger than that
        let addr: SocketAddr = "127.0.0.1:8080".parse().expect("Invalid address");
        let large_payload = vec![1u8; 1024*10*10]; // 100KB payload
        let connected_stream = connect_to_server(addr, &large_payload);
        println!(
    "sent={} received={}",
    large_payload.len(),
    connected_stream.len()
);
        assert_eq!(connected_stream, large_payload, "Received bytes do not match sent bytes");
        }
    #[test]
    fn test_tcp_server_echo() {
        //Currently the server echos back the received bytes
        let addr: SocketAddr = "127.0.0.1:8080".parse().expect("Invalid address");
        let payload = vec![1u8; 1024];
        let connected_stream = connect_to_server(addr, &payload);
        assert_eq!(connected_stream, payload, "Received bytes do not match sent bytes");
    }
    
    
    fn connect_to_server(addr: SocketAddr, message: &[u8])-> Vec<u8>  { 
        let mut client = TcpStream::connect(addr).expect("Could not connect to server");
        let start = Instant::now();
        let timeout = Duration::from_secs(1);
        let _ = client.write_all(&message);
        let _ =client.shutdown(Shutdown::Write);
        let mut buffer = [0u8; 1024];
        let mut received = Vec::with_capacity(message.len());
         while received.len() < message.len() && start.elapsed() < timeout{
            let bytes_read = client.read(&mut buffer).unwrap();;
            received.extend_from_slice(&buffer[..bytes_read]);
        }
        
        return received;
        
    }
    

}