use std::net::Ipv4Addr;
use std::str::FromStr;
use tokio::net::{TcpListener, TcpSocket, UdpSocket};

async fn discover_peers(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // Ipv4Addr::from_str(&format!("127.0.0.1:{port}")).unwrap();
    let socket = UdpSocket::bind("192.168.0.1:8888").await.unwrap();
    loop {}
}
