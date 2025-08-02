/*
   Client sends a broadcast request to all the discoverable peers in the local network
   need a timeout to manage connection requests so that we don't do this all the time
*/
use clap::{Parser, Subcommand};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpSocket, UdpSocket};
use tokio::time;
use tokio::time::{Duration, timeout};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Discovery {
        #[arg(short, long)]
        port: String,
    },
}
#[tokio::main]
async fn main() {
    let cmd = Cli::parse().command.unwrap();
    match &cmd {
        Command::Discovery { port } => {
            let discovery_port = port.parse().unwrap();
            let discovery_socket = UdpSocket::bind(("0.0.0.0", discovery_port)).await.unwrap();
            run_discovery_service(discovery_socket).await;

            let _tcp_listener = TcpListener::bind("0.0.0.0:0").await.unwrap();
            let client_socket = UdpSocket::bind(("0.0.0.0", 0)).await.unwrap();

            let peer_list = discover_peers(&client_socket, discovery_port)
                .await
                .unwrap();
        }
    };
}

async fn discover_peers(
    client_socket: &UdpSocket,
    discovery_port: u16,
) -> Result<Vec<SocketAddr>, Box<dyn std::error::Error>> {
    println!("discovery port: {}", client_socket.local_addr().unwrap());
    // allowing the socket to send boradcast requests to the gateway
    client_socket.set_broadcast(true).unwrap();

    // broadcase request to peers
    client_socket
        .send_to(b"P2P_REQ", format!("255.255.255.255:{}", discovery_port))
        .await
        .unwrap();

    let discovery_start_instance = time::Instant::now();
    let discovery_timeout = time::Duration::from_secs(5);
    println!("Waiting for discovery for {:#?} seconds", discovery_timeout);

    let mut peers = vec![];
    while discovery_start_instance.elapsed() < discovery_timeout {
        let mut buf = [0; 1024];
        if let Ok(Ok((len, addr))) =
            timeout(Duration::from_millis(5), client_socket.recv_from(&mut buf)).await
        {
            if b"P2P_RES" == &buf[..len] {
                peers.push(addr);
            }
        }
    }
    println!("Discovered peers: {:#?}", peers);
    Ok(peers)
}

async fn run_discovery_service(discovery_socket: UdpSocket) {
    println!("Creating the discovery service");
    tokio::spawn(async move {
        let mut buf = [0; 1025];

        while let Ok((len, addr)) = discovery_socket.recv_from(&mut buf).await {
            if b"P2P_REQ" == &buf[..len] {
                let _ = discovery_socket.send_to(b"P2P_RES", addr).await;
            }
        }
    });
}
